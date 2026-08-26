//! Shell capabilities for streamed prompts.

use std::fmt;

use clap::ValueEnum;

use crate::context::{Shell, Target};
use crate::damage::Damage;
use crate::frame::{Patch, PromptVariablePayload, RawTerminalPayload};
use crate::module::painted::{Painted, TerminalWidth};

/// A shell's streaming capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    Static,
    PromptReplace,
    CellPrecise,
}

impl Tier {
    pub fn of(shell: Shell) -> Self {
        match shell {
            Shell::Zsh => Self::CellPrecise,

            Shell::Fish
            | Shell::Pwsh
            | Shell::PowerShell
            | Shell::Nu
            | Shell::Xonsh
            | Shell::Cmd => Self::PromptReplace,

            Shell::Elvish => Self::Static,

            Shell::Bash | Shell::Ion | Shell::Tcsh | Shell::Unknown => Self::Static,
        }
    }

    #[cfg(test)]
    pub fn for_prompt(shell: Shell, target: &Target) -> Self {
        Self::of(shell).for_target(target)
    }

    fn for_target(self, target: &Target) -> Self {
        match target {
            Target::Main => self,
            Target::Right | Target::Continuation | Target::Profile(_) => {
                self.min(Self::PromptReplace)
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn can_refine(self) -> bool {
        self.refinement().is_some()
    }

    pub(crate) fn refinement(self) -> Option<RefinementTier> {
        match self {
            Self::Static => None,
            Self::PromptReplace => Some(RefinementTier::PromptReplace),
            Self::CellPrecise => Some(RefinementTier::CellPrecise),
        }
    }
}

/// The capabilities available once a stream has committed to refining.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RefinementTier {
    PromptReplace,
    CellPrecise,
}

/// Requested stream transport.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum StreamingTransport {
    #[default]
    Auto,
    Ble,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransportMismatch {
    requested: StreamingTransport,
    shell: Shell,
}

impl fmt::Display for TransportMismatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.requested {
            StreamingTransport::Auto => unreachable!("automatic transport accepts every shell"),
            StreamingTransport::Ble => write!(
                formatter,
                "--transport=ble requires STARSHIP_SHELL=bash, found {:#?}",
                self.shell
            ),
        }
    }
}

impl std::error::Error for TransportMismatch {}

impl StreamingTransport {
    pub(crate) fn tier(self, shell: Shell, target: &Target) -> Result<Tier, TransportMismatch> {
        let capability = match self {
            Self::Auto => Tier::of(shell),
            Self::Ble if shell == Shell::Bash => Tier::PromptReplace,
            Self::Ble => {
                return Err(TransportMismatch {
                    requested: self,
                    shell,
                });
            }
        };
        Ok(capability.for_target(target))
    }
}

/// The refinement that turns `previous` into `next`, if anything changed.
pub(crate) fn patch(
    previous: &Painted,
    next: &Painted,
    terminal_width: TerminalWidth,
    tier: RefinementTier,
    shell: Shell,
) -> Option<Patch> {
    let prompt = || PromptVariablePayload::escaped_for(&RawTerminalPayload::prompt(next), shell);

    match Damage::between(previous, next, terminal_width) {
        Damage::None => None,
        Damage::Full => Some(Patch::whole_prompt(prompt())),
        Damage::Repaint(bytes) => Some(match tier {
            RefinementTier::PromptReplace => Patch::whole_prompt(prompt()),
            RefinementTier::CellPrecise => {
                Patch::repainting_cells(prompt(), RawTerminalPayload::repaint(&bytes))
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::tests::{painted, segments, text};
    use crate::segment::Segment;

    const EVERY_SHELL: &[Shell] = &[
        Shell::Bash,
        Shell::Fish,
        Shell::Ion,
        Shell::Pwsh,
        Shell::PowerShell,
        Shell::Zsh,
        Shell::Elvish,
        Shell::Tcsh,
        Shell::Nu,
        Shell::Xonsh,
        Shell::Cmd,
        Shell::Unknown,
    ];

    #[test]
    fn the_ladder_is_strictly_ordered() {
        assert!(Tier::Static < Tier::PromptReplace);
        assert!(Tier::PromptReplace < Tier::CellPrecise);
        assert!(!Tier::Static.can_refine());
        assert!(Tier::PromptReplace.can_refine());
        assert!(Tier::CellPrecise.can_refine());
    }

    #[test]
    fn every_shell_has_a_tier() {
        for &shell in EVERY_SHELL {
            let _ = Tier::of(shell);
        }
        assert_eq!(Tier::CellPrecise, Tier::of(Shell::Zsh));
    }

    /// The shells whose output this change must not alter at all.
    #[test]
    fn the_shells_that_cannot_be_refined_are_exactly_the_expected_ones() {
        let unrefinable: Vec<Shell> = EVERY_SHELL
            .iter()
            .copied()
            .filter(|shell| !Tier::of(*shell).can_refine())
            .collect();

        assert_eq!(
            vec![
                Shell::Bash,
                Shell::Ion,
                Shell::Elvish,
                Shell::Tcsh,
                Shell::Unknown,
            ],
            unrefinable,
            "every shell but zsh, fish, PowerShell, nu, xonsh and Clink is drawn synchronously today"
        );
    }

    /// A right prompt is placed by the shell, so it is never repainted cell by cell.
    #[test]
    fn only_the_main_prompt_is_ever_repainted_cell_by_cell() {
        assert_eq!(
            Tier::CellPrecise,
            Tier::for_prompt(Shell::Zsh, &Target::Main)
        );
        for target in [
            Target::Right,
            Target::Continuation,
            Target::Profile("anything".to_owned()),
        ] {
            assert_eq!(
                Tier::PromptReplace,
                Tier::for_prompt(Shell::Zsh, &target),
                "for {target:?}"
            );
        }

        // A shell that could not be refined at all is not promoted by this.
        for target in [Target::Main, Target::Right] {
            assert_eq!(Tier::Static, Tier::for_prompt(Shell::Bash, &target));
        }
    }

    #[test]
    fn ble_is_an_explicit_bash_prompt_replace_transport() {
        assert_eq!(
            Tier::PromptReplace,
            StreamingTransport::Ble
                .tier(Shell::Bash, &Target::Main)
                .unwrap()
        );
        assert_eq!(
            Tier::PromptReplace,
            StreamingTransport::Ble
                .tier(Shell::Bash, &Target::Right)
                .unwrap()
        );
        assert!(
            StreamingTransport::Ble
                .tier(Shell::Zsh, &Target::Main)
                .is_err()
        );
        assert_eq!(
            Tier::Static,
            StreamingTransport::Auto
                .tier(Shell::Bash, &Target::Main)
                .unwrap()
        );
    }

    const WIDTH: TerminalWidth = TerminalWidth(80);

    fn refine(previous: &Painted, next: &Painted, tier: RefinementTier, shell: Shell) -> Patch {
        patch(previous, next, WIDTH, tier, shell).expect("the prompts differ")
    }

    /// Two prompts of the same shape differing in one word: a run of damage.
    fn a_run_of_damage() -> (Painted, Painted) {
        (
            painted(&segments("[main](red) >"), WIDTH.0),
            painted(&segments("[work](red) >"), WIDTH.0),
        )
    }

    /// Two prompts of different shapes: nothing incremental expresses it.
    fn full_damage() -> (Painted, Painted) {
        (
            painted(&segments("one"), WIDTH.0),
            painted(&segments("a much longer prompt\nover two lines"), WIDTH.0),
        )
    }

    #[test]
    fn a_cell_precise_shell_is_sent_the_incremental_repaint() {
        let (previous, next) = a_run_of_damage();
        assert!(matches!(
            refine(&previous, &next, RefinementTier::CellPrecise, Shell::Zsh),
            Patch::Repaint { .. }
        ));
    }

    #[test]
    fn a_prompt_replace_shell_is_sent_a_whole_prompt_for_the_same_change() {
        let (previous, next) = a_run_of_damage();
        assert!(matches!(
            refine(&previous, &next, RefinementTier::PromptReplace, Shell::Fish),
            Patch::Replace(_)
        ));
    }

    /// Separate from the table above so promoting a shell to a higher tier doesn't change this test's meaning.
    #[test]
    fn the_rung_rather_than_the_shell_decides_what_is_sent() {
        let (previous, next) = a_run_of_damage();
        for shell in EVERY_SHELL {
            assert!(
                matches!(
                    refine(&previous, &next, RefinementTier::PromptReplace, *shell),
                    Patch::Replace(_)
                ),
                "for {shell:?}"
            );
            assert!(
                matches!(
                    refine(&previous, &next, RefinementTier::CellPrecise, *shell),
                    Patch::Repaint { .. }
                ),
                "for {shell:?}"
            );
        }
    }

    #[test]
    fn full_damage_degrades_to_a_whole_prompt_on_every_tier() {
        let (previous, next) = full_damage();
        for tier in [RefinementTier::PromptReplace, RefinementTier::CellPrecise] {
            assert!(
                matches!(
                    refine(&previous, &next, tier, Shell::Zsh),
                    Patch::Replace(_)
                ),
                "a full repaint must be a whole prompt at {tier:?}"
            );
        }
    }

    #[test]
    fn a_prompt_that_did_not_change_is_not_delivered_at_all() {
        let prompt = painted(&segments("[main](red) >"), WIDTH.0);
        for tier in [RefinementTier::PromptReplace, RefinementTier::CellPrecise] {
            assert_eq!(None, patch(&prompt, &prompt, WIDTH, tier, Shell::Zsh));
        }
    }

    /// Guards the doubled-percent-sign bug: a repaint is raw terminal bytes, never escaped.
    #[test]
    fn an_incremental_repaint_is_never_escaped_for_the_shell() {
        // Differ from the first character so the repaint's run covers the percent sign.
        let previous = painted(&segments("10% off >"), WIDTH.0);
        let next = painted(&segments("25% bad >"), WIDTH.0);

        let Some(Patch::Repaint { repaint, prompt }) = patch(
            &previous,
            &next,
            WIDTH,
            RefinementTier::CellPrecise,
            Shell::Zsh,
        ) else {
            panic!("a same-width change under zsh is an incremental repaint");
        };

        let bytes = String::from_utf8(repaint.as_bytes().to_vec()).expect("a repaint is text");
        assert!(
            bytes.contains("25%") && !bytes.contains("%%"),
            "a repaint under zsh must carry a single percent sign, but was {bytes:?}"
        );

        // Same bytes, opposite treatment: this prompt is for zsh to expand, so it is escaped.
        let variable = String::from_utf8(prompt.as_bytes().to_vec()).expect("a prompt is text");
        assert!(
            variable.contains("25%% bad"),
            "the prompt beside a repaint must be escaped for zsh, but was {variable:?}"
        );
        assert_eq!(
            RawTerminalPayload::prompt(&next),
            prompt.as_terminal_bytes_under(Shell::Zsh)
        );
    }

    /// A repaint always ships with its prompt, or the shell's prompt variable would describe a stale screen.
    #[test]
    fn a_repaint_always_arrives_with_the_prompt_the_screen_now_shows() {
        let (previous, next) = a_run_of_damage();
        let Some(Patch::Repaint { prompt, .. }) = patch(
            &previous,
            &next,
            WIDTH,
            RefinementTier::CellPrecise,
            Shell::Zsh,
        ) else {
            panic!("a same-width change under zsh is an incremental repaint");
        };

        assert_eq!(
            RawTerminalPayload::prompt(&next),
            prompt.as_terminal_bytes_under(Shell::Zsh),
            "the prompt beside a repaint must describe exactly what was painted"
        );
    }

    /// Guards zsh eating a percent sign: a whole prompt is assigned into `PROMPT`, which zsh re-expands, so it must be escaped.
    #[test]
    fn a_whole_prompt_is_always_escaped_for_the_shell() {
        let previous = painted(&segments("one"), WIDTH.0);
        let next = painted(&segments("10% off\nover two lines"), WIDTH.0);

        let Some(Patch::Replace(prompt)) = patch(
            &previous,
            &next,
            WIDTH,
            RefinementTier::CellPrecise,
            Shell::Zsh,
        ) else {
            panic!("a change of shape is a whole prompt");
        };

        let bytes = String::from_utf8(prompt.as_bytes().to_vec()).expect("a prompt is text");
        assert!(
            bytes.contains("10%% off"),
            "a whole prompt under zsh must double its percent signs, but was {bytes:?}"
        );

        // What zsh draws from this is exactly what the terminal would have shown directly.
        assert_eq!(
            RawTerminalPayload::prompt(&next),
            prompt.as_terminal_bytes_under(Shell::Zsh)
        );
    }

    /// Same pair of failures for bash, whose expansion eats backslashes and dollar signs instead.
    #[test]
    fn a_whole_prompt_survives_bash_prompt_expansion_unchanged() {
        let previous = painted(&segments("one"), WIDTH.0);
        let next = painted(
            &[
                text("", r"cost $5 `now` back\slash"),
                Segment::LineTerm,
                text("", "over two lines"),
            ],
            WIDTH.0,
        );

        let Some(Patch::Replace(prompt)) = patch(
            &previous,
            &next,
            WIDTH,
            RefinementTier::PromptReplace,
            Shell::Bash,
        ) else {
            panic!("a change of shape is a whole prompt");
        };

        assert_eq!(
            RawTerminalPayload::prompt(&next),
            prompt.as_terminal_bytes_under(Shell::Bash)
        );
    }

    /// A shell that expands nothing gets the rendered bytes untouched.
    #[test]
    fn a_shell_that_expands_nothing_gets_the_bytes_it_would_have_drawn() {
        let previous = painted(&segments("one"), WIDTH.0);
        let next = painted(&segments("10% off\nover two lines"), WIDTH.0);

        let Some(Patch::Replace(prompt)) = patch(
            &previous,
            &next,
            WIDTH,
            RefinementTier::PromptReplace,
            Shell::Fish,
        ) else {
            panic!("a change of shape is a whole prompt");
        };

        assert_eq!(
            RawTerminalPayload::prompt(&next).as_bytes(),
            prompt.as_bytes()
        );
    }
}
