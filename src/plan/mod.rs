//! A prompt plan derived from configuration.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::config::{
    ModuleConfig, StarshipConfig, Style, get_palette, parse_style_string_with_palette,
};
use crate::configs::fill::FillConfig;
use crate::configs::{PROMPT_ORDER, Palette, StarshipRootConfig};
use crate::context::Target;
use crate::escaping::Destination;
use crate::formatter::model::{FormatElement, StyleElement, VariableHolder};
use crate::formatter::parse_format_string;
use crate::segment::Segment;

/// The format string used when the configured one cannot be used at all.
const FALLBACK_FORMAT: &str = ">";

/// The variable that stands for "every module the format string does not name".
const ALL_MODULES_VARIABLE: &str = "all";

/// The module that emits a stretchable run of padding.
const FILL_MODULE: &str = "fill";

/// The module that emits nothing but a line break.
const LINE_BREAK_MODULE: &str = "line_break";

/// The name of a module as it may appear in a format string.
///
/// Not every name is a module that exists — a format string may name anything —
/// so this is a name that prompt rendering will *try* to turn into segments,
/// not a proof that it can.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleName(String);

impl ModuleName {
    /// Wraps a name taken from a format string or from the default ordering.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// The name as it is spelled in a format string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModuleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// The position of one distinct module in a [`Plan`].
///
/// This is deliberately not a module name: the plan interns every name once,
/// so rendering can update a slot in constant time without a second lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ModuleSlot(usize);

impl ModuleSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

/// The modules that can make a conditional visible.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Predicate(Vec<ModuleSlot>);

impl Predicate {
    fn of(module: ModuleSlot) -> Self {
        Self(vec![module])
    }

    fn union(&mut self, other: Self) {
        for module in other.0 {
            if !self.0.contains(&module) {
                self.0.push(module);
            }
        }
    }
}

/// Literal text together with the style that configuration gives it.
///
/// The text never contains a line break: line breaks are structural and are
/// held by [`Node::LineBreak`] instead.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticText {
    text: String,
    style: Option<Style>,
}

/// The characters a fill repeats, and the style it is drawn with.
#[derive(Clone, Debug, PartialEq)]
pub struct FillTemplate {
    symbol: String,
    style: Option<Style>,
}

/// One piece of a [`Plan`].
#[derive(Clone, Debug, PartialEq)]
enum Node {
    /// Literal text with the style configuration gives it.
    Static(StaticText),
    /// A module's output, to be supplied per render.
    Slot {
        /// The module whose segments fill the slot.
        slot: ModuleSlot,
        /// The style of the enclosing text group, which the module's segments
        /// take on wherever they carry no style of their own.
        inherited_style: Option<Style>,
    },
    /// Padding that stretches to whatever width the line has left over.
    ///
    /// A fill's characters come from configuration; only how many of them are
    /// painted is decided per render, and that is decided by the width of
    /// everything around it rather than by anything a module produces. So a
    /// fill has no per-render value of its own and contributes nothing to any
    /// predicate.
    Fill {
        /// What to repeat, and in which style.
        template: FillTemplate,
    },
    /// A body that shows only when at least one module in `predicate` renders
    /// something non-empty.
    ///
    /// The predicate is never empty: a body that no module can ever bring into
    /// view is dropped at build time instead, and one that always shows is
    /// inlined.
    Conditional {
        /// The modules whose emptiness decides whether the body shows.
        predicate: Predicate,
        /// What to render when it does.
        body: Vec<Self>,
    },
    /// The end of a visual line.
    LineBreak,
}

/// Everything [`Plan::build`] is allowed to look at.
///
/// There is deliberately no way to reach a [`crate::context::Context`] from
/// here: the fields are the parsed configuration, the destination whose escaping
/// rules apply, and which of the prompts is being built. That is what makes "a
/// plan is a function of configuration" a fact about the types rather than a
/// promise in a comment.
pub struct PromptConfiguration<'configuration> {
    starship_configuration: &'configuration StarshipConfig,
    root_configuration: &'configuration StarshipRootConfig,
    destination: Destination,
    target: &'configuration Target,
}

impl<'configuration> PromptConfiguration<'configuration> {
    /// Bundles the configuration a plan is built from.
    pub fn new(
        starship_configuration: &'configuration StarshipConfig,
        root_configuration: &'configuration StarshipRootConfig,
        destination: Destination,
        target: &'configuration Target,
    ) -> Self {
        Self {
            starship_configuration,
            root_configuration,
            destination,
            target,
        }
    }
}

/// A distinct module in a plan, in first-paint order.
///
/// A module named multiple times is represented once. Its private slot keeps
/// the rendered value tied to that one plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleUse {
    /// The module to run.
    pub module: ModuleName,
    slot: ModuleSlot,
}

impl ModuleUse {
    pub(crate) fn slot(&self) -> ModuleSlot {
        self.slot
    }
}

/// The prompt, built from configuration alone.
#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    nodes: Vec<Node>,
    modules: Vec<ModuleUse>,
    referenced_modules: BTreeSet<ModuleName>,
}

impl Plan {
    /// Builds the plan for one prompt.
    ///
    /// Everything this reads is configuration; nothing it reads can vary between
    /// two renders of the same prompt.
    pub fn build(configuration: &PromptConfiguration<'_>) -> Self {
        // Palette resolution: the one configuration-dependent input to style
        // resolution. Read once, then threaded through every style in the plan.
        let palette = get_palette(
            &configuration.root_configuration.palettes,
            configuration.root_configuration.palette.as_deref(),
        );

        let (elements, referenced_modules) = select_format(configuration);
        let modules_expanded_by_all = modules_expanded_by_all(&referenced_modules);

        let mut builder = Builder {
            starship_configuration: configuration.starship_configuration,
            palette,
            destination: configuration.destination,
            modules_expanded_by_all: &modules_expanded_by_all,
            slots: BTreeMap::new(),
            modules: Vec::new(),
        };

        let mut nodes = Vec::new();
        builder.build_elements(&elements, None, &mut nodes);

        Self {
            nodes,
            modules: builder.modules,
            referenced_modules,
        }
    }

    /// The modules the format strings name, including `all` if they name it.
    ///
    /// Both the main and the right format contribute, because a module named by
    /// either one is not one that `$all` should supply again.
    pub fn referenced_modules(&self) -> &BTreeSet<ModuleName> {
        &self.referenced_modules
    }

    /// Every module the plan renders, distinct, in first-paint order.
    ///
    /// See [`ModuleUse`] for what that order guarantees.
    pub fn module_uses(&self) -> &[ModuleUse] {
        &self.modules
    }

    fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}

/// What each module of one plan resolved to, for one render.
///
/// The state is keyed by *what a value is about* rather than by where it goes,
/// and it can only be made from the plan it belongs to — [`PromptState::empty`]
/// is the sole constructor and the sole place the two are ever associated. So
/// there is no way to hand one plan's values to another plan's structure: no
/// function takes a plan and a state as two arguments, because a state already
/// holds its own.
///
/// A module with no value renders nothing, which is also what a module that is
/// disabled or produced nothing renders — the three are indistinguishable by
/// design. A module that produced an *empty* segment is a different thing
/// again: the segment is still there, still carries a style, and a following
/// `prev_fg` or `prev_bg` can still see it.
#[derive(Clone)]
pub struct PromptState<'plan> {
    plan: &'plan Plan,
    resolved: Vec<Vec<Segment>>,
}

impl<'plan> PromptState<'plan> {
    /// A render of `plan` in which no module has resolved yet.
    pub fn empty(plan: &'plan Plan) -> Self {
        Self {
            plan,
            resolved: vec![Vec::new(); plan.modules.len()],
        }
    }

    /// Takes what a module resolved to into this render.
    ///
    /// The module is named by a borrow of the plan's own copy of the name, so
    /// what is recorded cannot be about a prompt other than the one being
    /// rendered.
    pub fn record(&mut self, module: &ModuleUse, segments: Vec<Segment>) {
        self.resolved[module.slot.0] = segments;
    }

    /// What a module resolved to, or nothing if it has not resolved.
    fn resolved_segments(&self, slot: ModuleSlot) -> &[Segment] {
        &self.resolved[slot.0]
    }

    /// Fills the plan in, producing the segments the prompt is painted from.
    pub fn render(&self) -> Vec<Segment> {
        let mut segments = Vec::new();
        self.render_nodes(self.plan.nodes(), &mut segments);
        segments
    }

    fn render_nodes(&self, nodes: &[Node], segments: &mut Vec<Segment>) {
        for node in nodes {
            match node {
                Node::Static(text) => {
                    segments.push(Segment::text(text.style, text.text.clone()));
                }
                Node::LineBreak => segments.push(Segment::LineTerm),
                Node::Slot {
                    slot,
                    inherited_style,
                    ..
                } => segments.extend(self.resolved_segments(*slot).iter().cloned().map(
                    |mut segment| {
                        segment.set_style_if_empty(*inherited_style);
                        segment
                    },
                )),
                Node::Fill { template } => {
                    segments.push(Segment::fill(template.style, template.symbol.clone()));
                }
                Node::Conditional { predicate, body } => {
                    if predicate.0.iter().any(|slot| self.is_filled(*slot)) {
                        self.render_nodes(body, segments);
                    }
                }
            }
        }
    }

    /// Whether a module rendered anything a conditional would count as present.
    fn is_filled(&self, slot: ModuleSlot) -> bool {
        self.resolved_segments(slot)
            .iter()
            .any(|segment| !segment.value().is_empty())
    }
}

/// The modules `$all` stands for: every module in the default ordering that the
/// format strings do not already name.
pub fn modules_expanded_by_all(referenced_modules: &BTreeSet<ModuleName>) -> Vec<ModuleName> {
    PROMPT_ORDER
        .iter()
        .map(|module| ModuleName::new(*module))
        .filter(|module| !referenced_modules.contains(module))
        .collect()
}

/// What a part of the plan contributes to the visibility of a conditional that
/// contains it.
///
/// This mirrors, at build time, the question the renderer used to ask at render
/// time: does any variable inside this conditional have a non-empty value?
/// Literal text answers "no" — it never brings a conditional into view — and a
/// module answers "only if it rendered something". A fill and a line break
/// answer from configuration alone, which is why neither is ever named in a
/// predicate.
enum Presence {
    /// Nothing here can bring a conditional into view.
    Never,
    /// A conditional shows exactly when one of these modules is filled.
    WhenAnyFilled(Predicate),
    /// A conditional containing this always shows.
    Always,
}

impl Presence {
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Always, _) | (_, Self::Always) => Self::Always,
            (Self::Never, only) | (only, Self::Never) => only,
            (Self::WhenAnyFilled(mut left), Self::WhenAnyFilled(right)) => {
                left.union(right);
                Self::WhenAnyFilled(left)
            }
        }
    }
}

/// Turns format elements into plan nodes.
struct Builder<'build> {
    starship_configuration: &'build StarshipConfig,
    palette: Option<&'build Palette>,
    destination: Destination,
    modules_expanded_by_all: &'build [ModuleName],
    slots: BTreeMap<ModuleName, ModuleSlot>,
    modules: Vec<ModuleUse>,
}

impl Builder<'_> {
    fn build_elements(
        &mut self,
        elements: &[FormatElement<'_>],
        inherited_style: Option<Style>,
        nodes: &mut Vec<Node>,
    ) -> Presence {
        let mut presence = Presence::Never;

        for element in elements {
            match element {
                FormatElement::Text(text) => {
                    let escaped = self.destination.escape(text.clone());
                    push_literal(&escaped, inherited_style, nodes);
                }
                FormatElement::TextGroup(group) => {
                    let style = self.resolve_style(&group.style);
                    if group.format.is_empty() {
                        // An empty text group still produces a segment, so that a
                        // later `prev_fg` or `prev_bg` can read its style.
                        nodes.push(Node::Static(StaticText {
                            text: String::new(),
                            style,
                        }));
                    } else {
                        presence = presence.merge(self.build_elements(&group.format, style, nodes));
                    }
                }
                FormatElement::Variable(name) => {
                    presence = presence.merge(self.build_variable(name, inherited_style, nodes));
                }
                FormatElement::Conditional(body) => {
                    presence = presence.merge(self.build_conditional(body, inherited_style, nodes));
                }
            }
        }

        presence
    }

    fn build_conditional(
        &mut self,
        elements: &[FormatElement<'_>],
        inherited_style: Option<Style>,
        nodes: &mut Vec<Node>,
    ) -> Presence {
        let mut body = Vec::new();
        let presence = self.build_elements(elements, inherited_style, &mut body);

        match presence {
            Presence::Always => {
                // Configuration proves this always shows, so there is nothing
                // left to decide per render.
                nodes.append(&mut body);
                Presence::Always
            }
            Presence::WhenAnyFilled(predicate) => {
                nodes.push(Node::Conditional {
                    predicate: predicate.clone(),
                    body,
                });
                Presence::WhenAnyFilled(predicate)
            }
            Presence::Never => {
                // No module inside, and nothing statically present: the body
                // could never have shown, so it goes and takes whatever it
                // contained with it.
                Presence::Never
            }
        }
    }

    fn build_variable(
        &mut self,
        name: &str,
        inherited_style: Option<Style>,
        nodes: &mut Vec<Node>,
    ) -> Presence {
        if name == ALL_MODULES_VARIABLE {
            let expanded = self.modules_expanded_by_all;
            return expanded.iter().fold(Presence::Never, |presence, module| {
                presence.merge(self.build_module(module, inherited_style, nodes))
            });
        }

        let module = ModuleName::new(name);
        if self.is_disabled(module.as_str()) {
            return Presence::Never;
        }
        self.build_module(&module, inherited_style, nodes)
    }

    fn build_module(
        &mut self,
        module: &ModuleName,
        inherited_style: Option<Style>,
        nodes: &mut Vec<Node>,
    ) -> Presence {
        match module.as_str() {
            // A line break has no data behind it: configuration alone says
            // whether it is there, and it is never empty when it is.
            LINE_BREAK_MODULE => {
                if self.is_disabled(LINE_BREAK_MODULE) {
                    return Presence::Never;
                }
                nodes.push(Node::LineBreak);
                Presence::Always
            }
            // A fill's characters and style are configuration too; only its
            // width is decided per render.
            FILL_MODULE => {
                let Some(template) = self.fill_template(inherited_style) else {
                    return Presence::Never;
                };
                let presence = if template.symbol.is_empty() {
                    Presence::Never
                } else {
                    Presence::Always
                };
                nodes.push(Node::Fill { template });
                presence
            }
            _ => {
                let slot = self.intern(module);
                nodes.push(Node::Slot {
                    slot,
                    inherited_style,
                });
                Presence::WhenAnyFilled(Predicate::of(slot))
            }
        }
    }

    fn intern(&mut self, module: &ModuleName) -> ModuleSlot {
        if let Some(&slot) = self.slots.get(module) {
            return slot;
        }

        let slot = ModuleSlot(self.modules.len());
        self.slots.insert(module.clone(), slot);
        self.modules.push(ModuleUse {
            module: module.clone(),
            slot,
        });
        slot
    }

    /// The fill module's configuration, or `None` if it will not render.
    fn fill_template(&self, inherited_style: Option<Style>) -> Option<FillTemplate> {
        if self.is_disabled(FILL_MODULE) {
            return None;
        }

        let configuration =
            FillConfig::try_load(self.starship_configuration.get_module_config(FILL_MODULE));
        if configuration.disabled {
            return None;
        }

        Some(FillTemplate {
            symbol: configuration.symbol.to_owned(),
            // A fill with no style of its own takes on the enclosing text
            // group's style, exactly as any other module's segments would.
            style: parse_style_string_with_palette(configuration.style, self.palette)
                .or(inherited_style),
        })
    }

    /// Whether a module is switched off by `disabled = true`.
    fn is_disabled(&self, module: &str) -> bool {
        self.starship_configuration
            .get_module_config(module)
            .and_then(|value| value.as_table()?.get("disabled")?.as_bool())
            == Some(true)
    }

    /// Resolves a style specification into a style.
    ///
    /// A root format string has no style variables to substitute — only modules
    /// map those, and a prompt's own format string is not a module — so a style
    /// variable contributes the empty string, which is what the formatter did
    /// with an unmapped one too.
    fn resolve_style(&self, elements: &[StyleElement<'_>]) -> Option<Style> {
        let style_string: String = elements
            .iter()
            .map(|element| match element {
                StyleElement::Text(text) => text.as_ref(),
                StyleElement::Variable(_) => "",
            })
            .collect();

        parse_style_string_with_palette(&style_string, self.palette)
    }
}

/// Splits literal text on line breaks, which are structural rather than textual.
fn push_literal(text: &str, style: Option<Style>, nodes: &mut Vec<Node>) {
    for (index, piece) in text.split('\n').enumerate() {
        if index > 0 {
            nodes.push(Node::LineBreak);
        }
        nodes.push(Node::Static(StaticText {
            text: piece.to_owned(),
            style,
        }));
    }
}

/// Picks the format string this prompt is built from, and collects every module
/// the configured format strings name.
///
/// The module set is deliberately taken from *both* the main and the right
/// format even when only one of them is being built: a module named by either
/// is one that `$all` must not supply again.
fn select_format<'configuration>(
    configuration: &PromptConfiguration<'configuration>,
) -> (Vec<FormatElement<'configuration>>, BTreeSet<ModuleName>) {
    let root = configuration.root_configuration;

    if *configuration.target == Target::Continuation {
        return match parse_format_string(&root.continuation_prompt) {
            Ok(elements) => {
                let modules = named_modules(&elements);
                (elements, modules)
            }
            Err(error) => {
                log::error!("Error parsing continuation prompt: {error}");
                (fallback_format(), BTreeSet::new())
            }
        };
    }

    let (left_format, right_format): (&'configuration str, &'configuration str) =
        match configuration.target {
            Target::Main | Target::Right => (&root.format, &root.right_format),
            Target::Profile(name) => {
                match root
                    .user_profiles
                    .get(name)
                    .or_else(|| root.internal_profiles.get(name))
                {
                    Some(format) => (format, ""),
                    None => {
                        log::error!("Profile {name:?} not found");
                        return (fallback_format(), BTreeSet::new());
                    }
                }
            }
            Target::Continuation => {
                unreachable!("Continuation prompt should have been handled above")
            }
        };

    let left_elements = parse_format_string(left_format);
    let right_elements = parse_format_string(right_format);

    if let Err(ref error) = left_elements {
        let name = if let Target::Profile(profile_name) = configuration.target {
            format!("profile.{profile_name}")
        } else {
            "format".to_string()
        };
        log::error!("Error parsing {name:?}: {error}");
    }

    if let Err(ref error) = right_elements {
        log::error!("Error parsing right_format: {error}");
    }

    let modules: BTreeSet<ModuleName> = [&left_elements, &right_elements]
        .into_iter()
        .flatten()
        .flat_map(|elements| named_modules(elements))
        .collect();

    let selected = match configuration.target {
        Target::Main | Target::Profile(_) => left_elements,
        Target::Right => right_elements,
        Target::Continuation => {
            unreachable!("Continuation prompt should have been handled above")
        }
    };

    match selected {
        Ok(elements) => (elements, modules),
        Err(_) => (fallback_format(), BTreeSet::new()),
    }
}

fn fallback_format() -> Vec<FormatElement<'static>> {
    vec![FormatElement::Text(Cow::Borrowed(FALLBACK_FORMAT))]
}

fn named_modules(elements: &[FormatElement<'_>]) -> BTreeSet<ModuleName> {
    elements
        .get_variables()
        .into_iter()
        .map(|name| ModuleName::new(name.into_owned()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::parse_style_string;
    use crate::formatter::StringFormatter;
    use crate::module::painted::Painted;
    use crate::segment::Segment;
    use std::collections::HashMap;

    /// A root configuration whose main format is `format`.
    ///
    /// `StarshipRootConfig` keeps a private field, so it cannot be built with
    /// struct update syntax from another module.
    fn root_configuration(format: &str) -> StarshipRootConfig {
        set_format(StarshipRootConfig::default(), format)
    }

    fn set_format(mut root: StarshipRootConfig, format: &str) -> StarshipRootConfig {
        root.format = format.to_owned();
        root
    }

    /// Builds the main prompt's plan, the way a prompt would.
    fn plan_with(root: &StarshipRootConfig, starship: &StarshipConfig) -> Plan {
        plan_for(root, starship, &Target::Main)
    }

    fn plan_for(root: &StarshipRootConfig, starship: &StarshipConfig, target: &Target) -> Plan {
        Plan::build(&PromptConfiguration::new(
            starship,
            root,
            Destination::RawTerminal,
            target,
        ))
    }

    fn default_plan(format: &str) -> Plan {
        plan_with(&root_configuration(format), &StarshipConfig::default())
    }

    /// The segments a render resolved so far renders to, described as markup.
    fn render_markup(state: &PromptState<'_>) -> String {
        Painted::paint(&state.render(), None).to_markup()
    }

    /// Resolves every module of a plan from a lookup table of name to segments.
    fn fill_from<'plan>(
        plan: &'plan Plan,
        table: &HashMap<&str, Vec<Segment>>,
    ) -> PromptState<'plan> {
        let mut state = PromptState::empty(plan);
        for module_use in plan.module_uses() {
            if let Some(segments) = table.get(module_use.module.as_str()) {
                state.record(module_use, segments.clone());
            }
        }
        state
    }

    fn module<'plan>(plan: &'plan Plan, name: &str) -> &'plan ModuleUse {
        plan.module_uses()
            .iter()
            .find(|module| module.module.as_str() == name)
            .expect("module is part of the plan")
    }

    fn styled(style: &str, text: &str) -> Vec<Segment> {
        Segment::from_text(parse_style_string(style, None), text)
    }

    // ---------------------------------------------------------------- equivalence

    /// The format strings the plan has to agree with the formatter on.
    const EQUIVALENT_FORMATS: &[&str] = &[
        "",
        "text only",
        "$alpha",
        "$alpha$beta",
        "$alpha $beta",
        "[$alpha](red bold)",
        "[$alpha](bg:blue)[$beta](fg:prev_bg)",
        "[](bg:#9A348E)[X](bg:prev_bg)",
        "[]()",
        "($alpha) between ($empty)",
        "($alpha ($empty)) and ($empty ($alpha))",
        "(literal only)",
        "[($empty)](red bold)",
        "[(@$empty)](red bold)",
        "line one\nline two",
        "[a\nb](green)",
        "$line_break",
        "a$line_break b",
        "($line_break)",
        "$fill",
        "left$fill right",
        "[$fill](red)",
        "($fill)",
        r"\\\[\$text\]\(red bold\)",
        "${custom.thing}",
        "$alpha[$beta](bold $style_variable)$alpha",
        "[[$alpha](red)](blue)",
        "(($alpha))",
        "$alpha$alpha",
    ];

    /// The values every variable takes, for both implementations.
    fn equivalence_table() -> HashMap<&'static str, Vec<Segment>> {
        HashMap::from([
            ("alpha", Segment::from_text(None, "A")),
            ("beta", styled("green", "B")),
            ("empty", Vec::new()),
            ("custom.thing", styled("italic", "C")),
            ("line_break", vec![Segment::LineTerm]),
            (
                "fill",
                vec![Segment::fill(parse_style_string("bold black", None), ".")],
            ),
        ])
    }

    #[test]
    fn a_plan_renders_what_the_formatter_renders() {
        let table = equivalence_table();

        for format in EQUIVALENT_FORMATS {
            let formatter_segments = StringFormatter::new(format)
                .expect("format string should parse")
                .map_variables_to_segments(|variable| {
                    table
                        .get(variable)
                        .cloned()
                        .map(Ok)
                        .or(Some(Ok(Vec::new())))
                })
                .parse(None, None)
                .expect("format string should evaluate");
            let expected = Painted::paint(&formatter_segments, None).to_markup();

            let plan = default_plan(format);
            let actual = render_markup(&fill_from(&plan, &table));

            assert_eq!(expected, actual, "rendering {format:?}");
        }
    }

    #[test]
    fn a_plan_renders_what_the_formatter_renders_for_every_terminal_width() {
        use crate::module::painted::TerminalWidth;

        let table = equivalence_table();
        let format = "left$fill middle$fill right";

        let formatter_segments = StringFormatter::new(format)
            .expect("format string should parse")
            .map_variables_to_segments(|variable| {
                table
                    .get(variable)
                    .cloned()
                    .map(Ok)
                    .or(Some(Ok(Vec::new())))
            })
            .parse(None, None)
            .expect("format string should evaluate");

        let plan = default_plan(format);
        let plan_segments = fill_from(&plan, &table).render();

        for width in [0, 1, 2, 10, 41, 80, 200] {
            assert_eq!(
                Painted::paint(&formatter_segments, Some(TerminalWidth(width))).to_string(),
                Painted::paint(&plan_segments, Some(TerminalWidth(width))).to_string(),
                "at width {width}"
            );
        }
    }

    // ---------------------------------------------------------------- modules

    /// The names of the modules a plan uses, in the order it reports them.
    fn used_modules(plan: &Plan) -> Vec<&str> {
        plan.module_uses()
            .iter()
            .map(|module_use| module_use.module.as_str())
            .collect()
    }

    #[test]
    fn module_uses_are_reported_in_first_paint_order() {
        let plan = default_plan("$alpha[$beta]($style)($gamma$delta)");

        assert_eq!(vec!["alpha", "beta", "gamma", "delta"], used_modules(&plan));
    }

    #[test]
    fn the_same_configuration_always_yields_the_same_plan() {
        let format = "$alpha($beta$fill)[$gamma](red)$all";
        assert_eq!(default_plan(format), default_plan(format));
    }

    #[test]
    fn each_distinct_module_has_one_plan_slot() {
        let plan = default_plan("$alpha($beta)$fill[$gamma](red)($delta$fill)$alpha");

        assert_eq!(vec!["alpha", "beta", "gamma", "delta"], used_modules(&plan));
    }

    #[test]
    fn a_module_named_twice_is_one_use_that_renders_in_every_position() {
        let plan = default_plan("$alpha$beta$alpha");

        assert_eq!(vec!["alpha", "beta"], used_modules(&plan));
        assert_eq!(
            "ABA",
            render_markup(&fill_from(
                &plan,
                &HashMap::from([
                    ("alpha", Segment::from_text(None, "A")),
                    ("beta", Segment::from_text(None, "B")),
                ])
            ))
        );
    }

    #[test]
    fn a_later_resolution_replaces_an_earlier_one() {
        let plan = default_plan("$alpha");
        let mut state = PromptState::empty(&plan);
        let alpha = module(&plan, "alpha");

        state.record(alpha, Segment::from_text(None, "first"));
        state.record(alpha, Segment::from_text(None, "second"));

        assert_eq!("second", render_markup(&state));
    }

    // ---------------------------------------------------------------- conditionals

    #[test]
    fn a_conditional_keeps_its_literal_with_its_module() {
        let plan = default_plan("($alpha literal)");
        let mut state = PromptState::empty(&plan);
        state.record(module(&plan, "alpha"), Segment::from_text(None, "A"));

        assert_eq!("A literal", render_markup(&state));
    }

    #[test]
    fn a_literal_inside_a_conditional_is_not_static() {
        let plan = default_plan("($alpha literal)");

        assert_eq!("", render_markup(&PromptState::empty(&plan)));
    }

    #[test]
    fn a_conditional_nothing_can_fill_is_dropped() {
        let plan = default_plan("(just literal text)");

        assert_eq!(&[] as &[Node], plan.nodes());
        assert!(plan.module_uses().is_empty());
    }

    #[test]
    fn a_dropped_conditional_takes_its_contents_with_it() {
        // A fill with nothing to repeat renders an empty segment, and a fill
        // never names a module in a predicate anyway, so nothing inside this
        // conditional can bring it into view and the whole thing goes — the
        // fill included, leaving the plan with only what followed it.
        let starship = StarshipConfig {
            config: Some(toml::toml! {
                [fill]
                symbol = ""
            }),
        };
        let plan = plan_with(&root_configuration("($fill)$alpha"), &starship);

        assert!(matches!(plan.nodes(), [Node::Slot { .. }]));
        assert_eq!(vec!["alpha"], used_modules(&plan));
    }

    #[test]
    fn a_conditional_configuration_always_shows_is_inlined() {
        assert_eq!(&[Node::LineBreak], default_plan("($line_break)").nodes());

        // A fill always renders its symbol, so a conditional holding one is
        // decided by configuration too.
        let plan = default_plan("($fill)");
        assert!(matches!(plan.nodes(), [Node::Fill { .. }]));
    }

    #[test]
    fn a_nested_conditional_is_visible_when_its_module_is_filled() {
        let plan = default_plan("(literal ($alpha))");
        let mut state = PromptState::empty(&plan);
        state.record(module(&plan, "alpha"), Segment::from_text(None, "A"));

        assert_eq!("literal A", render_markup(&state));
    }

    #[test]
    fn a_conditional_shows_when_any_of_its_modules_is_filled() {
        let plan = default_plan("($alpha$beta)");
        let mut state = PromptState::empty(&plan);

        assert_eq!("", render_markup(&state));

        state.record(module(&plan, "beta"), Segment::from_text(None, "B"));
        assert_eq!("B", render_markup(&state));
    }

    #[test]
    fn an_empty_segment_does_not_fill_a_module() {
        let plan = default_plan("($alpha)");
        let mut state = PromptState::empty(&plan);
        state.record(module(&plan, "alpha"), Segment::from_text(None, ""));

        assert_eq!("", render_markup(&state));
    }

    // ---------------------------------------------------------------- text

    #[test]
    fn literal_line_breaks_become_structure() {
        let plan = default_plan("a\nb");

        assert_eq!(
            vec![
                Node::Static(StaticText {
                    text: "a".to_owned(),
                    style: None
                }),
                Node::LineBreak,
                Node::Static(StaticText {
                    text: "b".to_owned(),
                    style: None
                }),
            ],
            plan.nodes()
        );
    }

    #[test]
    fn literal_text_is_escaped_for_the_destination_the_plan_was_built_for() {
        // `\$` is the format string's way of writing a literal dollar sign.
        let root = root_configuration(r"10% \$");
        let starship = StarshipConfig::default();

        for (destination, expected) in [
            (Destination::RawTerminal, "10% $"),
            (
                Destination::shell_prompt_variable(crate::context::Shell::Zsh),
                "10%% $",
            ),
            (
                Destination::shell_prompt_variable(crate::context::Shell::Bash),
                r"10% \$",
            ),
        ] {
            let plan = Plan::build(&PromptConfiguration::new(
                &starship,
                &root,
                destination,
                &Target::Main,
            ));
            let rendered = Painted::paint(&PromptState::empty(&plan).render(), None).to_string();
            assert_eq!(expected, rendered, "for {destination:?}");
        }
    }

    // ---------------------------------------------------------------- styles

    #[test]
    fn a_palette_is_applied_when_styles_are_resolved() {
        let mut root = root_configuration("[x](love)");
        root.palette = Some("rose".to_owned());
        root.palettes = HashMap::from([(
            "rose".to_owned(),
            HashMap::from([("love".to_owned(), "#EB6F92".to_owned())]),
        )]);
        let plan = plan_with(&root, &StarshipConfig::default());

        assert_eq!("[x](#EB6F92)", render_markup(&PromptState::empty(&plan)));
    }

    #[test]
    fn without_the_palette_the_same_style_string_means_something_else() {
        // The point of resolving palettes during plan construction: `love` is
        // not a colour on its own, so a style resolved without the palette is
        // not merely differently coloured, it is a different style.
        let plan = default_plan("[x](love)");

        assert_eq!("x", render_markup(&PromptState::empty(&plan)));
    }

    #[test]
    fn a_prev_reference_survives_into_the_plan_for_painting_to_resolve() {
        let plan = default_plan("[a](fg:red bg:blue)[b](fg:prev_bg)");

        assert_eq!(
            "[a](red bg:blue)[b](blue)",
            render_markup(&PromptState::empty(&plan))
        );
    }

    #[test]
    fn a_module_inherits_the_style_of_its_text_group() {
        let plan = default_plan("[$alpha](red bold)");
        let mut state = PromptState::empty(&plan);
        state.record(module(&plan, "alpha"), Segment::from_text(None, "A"));

        assert_eq!("[A](red bold)", render_markup(&state));
    }

    // ---------------------------------------------------------------- fill

    #[test]
    fn a_fill_takes_its_characters_and_style_from_configuration() {
        let starship = StarshipConfig {
            config: Some(toml::toml! {
                [fill]
                symbol = "*-"
                style = "bold green"
            }),
        };
        let plan = plan_with(&root_configuration("$fill"), &starship);

        assert_eq!(
            vec![Node::Fill {
                template: FillTemplate {
                    symbol: "*-".to_owned(),
                    style: parse_style_string("bold green", None),
                },
            }],
            plan.nodes()
        );
    }

    #[test]
    fn a_disabled_fill_leaves_nothing_behind() {
        let starship = StarshipConfig {
            config: Some(toml::toml! {
                [fill]
                disabled = true
            }),
        };
        let plan = plan_with(&root_configuration("$fill"), &starship);

        assert_eq!(&[] as &[Node], plan.nodes());
        assert!(plan.module_uses().is_empty());
    }

    #[test]
    fn a_disabled_line_break_leaves_nothing_behind() {
        let starship = StarshipConfig {
            config: Some(toml::toml! {
                [line_break]
                disabled = true
            }),
        };
        let plan = plan_with(&root_configuration("$line_break"), &starship);

        assert_eq!(&[] as &[Node], plan.nodes());
    }

    // ---------------------------------------------------------------- $all

    #[test]
    fn all_expands_to_the_modules_the_format_does_not_name() {
        let plan = default_plan("$directory$all");
        let uses = plan.module_uses();

        assert_eq!("directory", uses[0].module.as_str());
        assert_eq!(
            1,
            uses.iter()
                .filter(|module_use| module_use.module.as_str() == "directory")
                .count()
        );
    }

    #[test]
    fn all_expands_in_the_default_ordering() {
        let plan = default_plan("$all");
        let expanded = used_modules(&plan);

        // `line_break` and any module without data of its own are structural,
        // so they are not modules the plan runs; the rest keep the default
        // order.
        let mut remaining = expanded.iter();
        for module in PROMPT_ORDER.iter() {
            if *module == LINE_BREAK_MODULE {
                continue;
            }
            assert_eq!(
                Some(&module),
                remaining.next().as_ref(),
                "{module} out of order"
            );
        }
        assert_eq!(None, remaining.next());
    }

    #[test]
    fn the_right_format_also_claims_modules_from_all() {
        let mut root = root_configuration("$all");
        root.right_format = "$directory".to_owned();
        let plan = plan_with(&root, &StarshipConfig::default());

        assert!(!used_modules(&plan).contains(&"directory"));
    }

    // ---------------------------------------------------------------- targets

    #[test]
    fn each_target_selects_its_own_format() {
        let mut root = root_configuration("main");
        root.right_format = "right".to_owned();
        root.continuation_prompt = "continuation".to_owned();
        root.user_profiles = [("chosen".to_owned(), "profile".to_owned())]
            .into_iter()
            .collect();
        let starship = StarshipConfig::default();

        for (target, expected) in [
            (Target::Main, "main"),
            (Target::Right, "right"),
            (Target::Continuation, "continuation"),
            (Target::Profile("chosen".to_owned()), "profile"),
            // An unknown profile falls back to a bare prompt character.
            (Target::Profile("missing".to_owned()), ">"),
        ] {
            let plan = plan_for(&root, &starship, &target);
            assert_eq!(
                expected,
                render_markup(&PromptState::empty(&plan)),
                "for {target:?}"
            );
        }
    }

    #[test]
    fn an_unparsable_format_falls_back_to_a_bare_prompt_character() {
        let plan = default_plan("[unclosed");

        assert_eq!(">", render_markup(&PromptState::empty(&plan)));
        assert!(plan.referenced_modules().is_empty());
    }
}
