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

const ESTIMATED_STYLE_STRING_LENGTH: usize = 16;

/// The name of a module as it may appear in a format string.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleName(String);

impl ModuleName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
        self.0.extend(other.0);
        self.0.sort_unstable();
        self.0.dedup();
    }
}

/// Literal text together with the style that configuration gives it.
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
    Static(StaticText),
    Slot {
        slot: ModuleSlot,
        inherited_style: Option<Style>,
    },
    Fill {
        template: FillTemplate,
    },
    Conditional {
        predicate: Predicate,
        body: Vec<Self>,
    },
    LineBreak,
}

pub struct PromptConfiguration<'configuration> {
    starship_configuration: &'configuration StarshipConfig,
    root_configuration: &'configuration StarshipRootConfig,
    destination: Destination,
    target: &'configuration Target,
}

impl<'configuration> PromptConfiguration<'configuration> {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleUse {
    pub module: ModuleName,
    slot: ModuleSlot,
}

impl ModuleUse {
    pub(crate) fn slot(&self) -> ModuleSlot {
        self.slot
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    nodes: Vec<Node>,
    modules: Vec<ModuleUse>,
    referenced_modules: BTreeSet<ModuleName>,
}

impl Plan {
    pub fn build(configuration: &PromptConfiguration<'_>) -> Self {
        let palette = get_palette(
            &configuration.root_configuration.palettes,
            configuration.root_configuration.palette.as_deref(),
        );

        let (elements, referenced_modules) = select_format(configuration);

        let mut builder = Builder {
            starship_configuration: configuration.starship_configuration,
            palette,
            destination: configuration.destination,
            referenced_modules: &referenced_modules,
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

    pub fn referenced_modules(&self) -> &BTreeSet<ModuleName> {
        &self.referenced_modules
    }

    pub fn module_uses(&self) -> &[ModuleUse] {
        &self.modules
    }

    fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}

#[derive(Clone)]
pub struct PromptState<'plan> {
    plan: &'plan Plan,
    resolved: Vec<Vec<Segment>>,
}

/// An attempt to record a module result in a state for another plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordError {
    ForeignModuleUse,
}

impl fmt::Display for RecordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("module use does not belong to this prompt plan")
    }
}

impl std::error::Error for RecordError {}

impl<'plan> PromptState<'plan> {
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
    pub fn record(
        &mut self,
        module: &ModuleUse,
        segments: Vec<Segment>,
    ) -> Result<(), RecordError> {
        if !self
            .plan
            .module_uses()
            .iter()
            .any(|candidate| std::ptr::eq(candidate, module))
        {
            return Err(RecordError::ForeignModuleUse);
        }

        self.resolved[module.slot.0] = segments;
        Ok(())
    }

    fn resolved_segments(&self, slot: ModuleSlot) -> &[Segment] {
        &self.resolved[slot.0]
    }

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
                } => {
                    segments.extend(self.resolved_segments(*slot).iter().cloned().map(
                        |mut segment| {
                            segment.set_style_if_empty(*inherited_style);
                            segment
                        },
                    ));
                }
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

    fn is_filled(&self, slot: ModuleSlot) -> bool {
        self.resolved_segments(slot)
            .iter()
            .any(|segment| !segment.value().is_empty())
    }
}

enum Presence {
    Never,
    WhenAnyFilled(Predicate),
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

struct Builder<'build> {
    starship_configuration: &'build StarshipConfig,
    palette: Option<&'build Palette>,
    destination: Destination,
    referenced_modules: &'build BTreeSet<ModuleName>,
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
                    let escaped_text = self.destination.escape(text.clone());
                    push_literal(&escaped_text, inherited_style, nodes);
                }
                FormatElement::TextGroup(group) => {
                    let style = self.resolve_style(&group.style);
                    if group.format.is_empty() {
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
            Presence::Never => Presence::Never,
        }
    }

    fn build_variable(
        &mut self,
        name: &str,
        inherited_style: Option<Style>,
        nodes: &mut Vec<Node>,
    ) -> Presence {
        if name == ALL_MODULES_VARIABLE {
            return modules_expanded_by_all(self.referenced_modules)
                .into_iter()
                .fold(Presence::Never, |presence, module_name| {
                    if self.is_disabled(module_name.as_str()) {
                        presence
                    } else {
                        presence.merge(self.build_module(&module_name, inherited_style, nodes))
                    }
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
            LINE_BREAK_MODULE => {
                if self.is_disabled(LINE_BREAK_MODULE) {
                    return Presence::Never;
                }
                nodes.push(Node::LineBreak);
                Presence::Always
            }
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

    fn fill_template(&self, inherited_style: Option<Style>) -> Option<FillTemplate> {
        if self.is_disabled(FILL_MODULE) {
            return None;
        }

        let module_configuration = self.starship_configuration.get_module_config(FILL_MODULE);
        let configuration = FillConfig::try_load(module_configuration);

        if configuration.disabled {
            return None;
        }

        let resolved_style =
            parse_style_string_with_palette(configuration.style, self.palette).or(inherited_style);

        Some(FillTemplate {
            symbol: self.destination.escape(configuration.symbol),
            style: resolved_style,
        })
    }

    fn is_disabled(&self, module_name: &str) -> bool {
        self.starship_configuration
            .get_module_config(module_name)
            .and_then(|configuration_value| configuration_value.as_table())
            .and_then(|module_table| module_table.get("disabled"))
            .and_then(|disabled_flag| disabled_flag.as_bool())
            .unwrap_or(false)
    }

    fn resolve_style(&self, elements: &[StyleElement<'_>]) -> Option<Style> {
        let mut style_string = String::with_capacity(ESTIMATED_STYLE_STRING_LENGTH);

        for element in elements {
            match element {
                StyleElement::Text(text) => style_string.push_str(text.as_ref()),
                StyleElement::Variable(_) => {}
            }
        }

        parse_style_string_with_palette(&style_string, self.palette)
    }
}

/// Every module `$all` stands for, in prompt order, that the format string
/// did not already name explicitly.
#[must_use]
pub fn modules_expanded_by_all(referenced_modules: &BTreeSet<ModuleName>) -> Vec<ModuleName> {
    PROMPT_ORDER
        .iter()
        .map(|module_name| ModuleName::new(*module_name))
        .filter(|module_name| !referenced_modules.contains(module_name))
        .collect()
}

fn push_literal(text: &str, style: Option<Style>, nodes: &mut Vec<Node>) {
    let mut lines = text.split('\n');

    if let Some(first_line) = lines.next() {
        nodes.push(Node::Static(StaticText {
            text: first_line.to_owned(),
            style,
        }));
    }

    for subsequent_line in lines {
        nodes.push(Node::LineBreak);
        nodes.push(Node::Static(StaticText {
            text: subsequent_line.to_owned(),
            style,
        }));
    }
}

fn select_format<'configuration>(
    configuration: &PromptConfiguration<'configuration>,
) -> (Vec<FormatElement<'configuration>>, BTreeSet<ModuleName>) {
    let root = configuration.root_configuration;

    let (left_format, right_format, fallback_name) = match configuration.target {
        Target::Continuation => (&root.continuation_prompt[..], "", "continuation_prompt"),
        Target::Main => (&root.format[..], &root.right_format[..], "format"),
        Target::Right => (&root.format[..], &root.right_format[..], "right_format"),
        Target::Profile(profile_name) => {
            let profile = root
                .user_profiles
                .get(profile_name)
                .or_else(|| root.internal_profiles.get(profile_name));

            match profile {
                Some(format_string) => (format_string.as_str(), "", "profile"),
                None => {
                    log::error!("Profile {profile_name:?} not found");
                    return (fallback_format(), BTreeSet::new());
                }
            }
        }
    };

    let left_elements = parse_and_log(left_format, fallback_name);
    let right_elements = parse_and_log(right_format, "right_format");

    let mut referenced_modules = named_modules(&left_elements);
    referenced_modules.extend(named_modules(&right_elements));

    let selected_elements = match configuration.target {
        Target::Main | Target::Profile(_) | Target::Continuation => left_elements,
        Target::Right => right_elements,
    };

    (selected_elements, referenced_modules)
}

fn parse_and_log<'configuration>(
    format_string: &'configuration str,
    name: &str,
) -> Vec<FormatElement<'configuration>> {
    if format_string.is_empty() {
        return Vec::new();
    }

    match parse_format_string(format_string) {
        Ok(elements) => elements,
        Err(error) => {
            log::error!("Error parsing {name}: {error}");
            fallback_format()
        }
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
                state.record(module_use, segments.clone()).unwrap();
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

        state
            .record(alpha, Segment::from_text(None, "first"))
            .unwrap();
        state
            .record(alpha, Segment::from_text(None, "second"))
            .unwrap();

        assert_eq!("second", render_markup(&state));
    }

    // ---------------------------------------------------------------- conditionals

    #[test]
    fn a_conditional_keeps_its_literal_with_its_module() {
        let plan = default_plan("($alpha literal)");
        let mut state = PromptState::empty(&plan);
        state
            .record(module(&plan, "alpha"), Segment::from_text(None, "A"))
            .unwrap();

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
        state
            .record(module(&plan, "alpha"), Segment::from_text(None, "A"))
            .unwrap();

        assert_eq!("literal A", render_markup(&state));
    }

    #[test]
    fn a_conditional_shows_when_any_of_its_modules_is_filled() {
        let plan = default_plan("($alpha$beta)");
        let mut state = PromptState::empty(&plan);

        assert_eq!("", render_markup(&state));

        state
            .record(module(&plan, "beta"), Segment::from_text(None, "B"))
            .unwrap();
        assert_eq!("B", render_markup(&state));
    }

    #[test]
    fn an_empty_segment_does_not_fill_a_module() {
        let plan = default_plan("($alpha)");
        let mut state = PromptState::empty(&plan);
        state
            .record(module(&plan, "alpha"), Segment::from_text(None, ""))
            .unwrap();

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
        state
            .record(module(&plan, "alpha"), Segment::from_text(None, "A"))
            .unwrap();

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
