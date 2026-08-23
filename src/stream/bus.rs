//! The pure batching policy for prompt refinements.

use std::time::{Duration, Instant};

use crate::module::painted::{Painted, Run, RunKind, TerminalWidth};
use crate::print::UnicodeWidthGraphemes;

use super::schedule::ArrivalSchedule;

/// The maximum delay a batch may add to a refinement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BusWindow(Duration);

impl BusWindow {
    #[must_use]
    pub const fn from_milliseconds(milliseconds: u64) -> Self {
        Self(Duration::from_millis(milliseconds))
    }

    #[must_use]
    pub const fn duration(self) -> Duration {
        self.0
    }
}

/// Whether a refinement moves cells that follow it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reflow {
    None,
    Shifts,
}

impl Reflow {
    pub fn between(previous: &Painted, next: &Painted) -> Self {
        if previous.line_count() != next.line_count()
            || previous
                .lines()
                .zip(next.lines())
                .any(|(previous, next)| line_width(previous) != line_width(next))
        {
            Self::Shifts
        } else {
            Self::None
        }
    }
}

fn line_width(line: &[Run]) -> TerminalWidth {
    TerminalWidth(
        line.iter()
            .filter(|run| run.kind() != RunKind::LineTerminator)
            .map(|run| run.text().width_graphemes())
            .sum(),
    )
}

/// What the caller should do with an arriving refinement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    DrawNow,
    Hold,
}

#[derive(Clone, Debug)]
enum Policy {
    Reactive,
    Predicted {
        schedule: ArrivalSchedule,
        started: Instant,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Holding {
    Nothing,
    Until(Instant),
}

/// A state machine that batches arriving refinements without knowing their text.
#[derive(Clone, Debug)]
pub struct Bus {
    window: BusWindow,
    policy: Policy,
    holding: Holding,
}

impl Bus {
    #[must_use]
    pub fn fixed(window: BusWindow) -> Self {
        Self {
            window,
            policy: Policy::Reactive,
            holding: Holding::Nothing,
        }
    }

    #[must_use]
    pub fn scheduled(window: BusWindow, schedule: ArrivalSchedule, started: Instant) -> Self {
        Self {
            window,
            policy: Policy::Predicted { schedule, started },
            holding: Holding::Nothing,
        }
    }

    /// Admits one refinement. An open batch accepts everything until `release`.
    pub fn admit(&mut self, reflow: Reflow, now: Instant) -> Verdict {
        if self.holding != Holding::Nothing {
            return Verdict::Hold;
        }

        let Some(deadline) = self.deadline_for(reflow, now) else {
            return Verdict::DrawNow;
        };
        self.holding = Holding::Until(deadline);
        Verdict::Hold
    }

    fn deadline_for(&self, reflow: Reflow, now: Instant) -> Option<Instant> {
        let window_deadline = || now.checked_add(self.window.duration()).unwrap_or(now);
        match &self.policy {
            Policy::Reactive => (reflow == Reflow::Shifts && !self.window.duration().is_zero())
                .then(window_deadline),
            Policy::Predicted { schedule, started } => {
                let elapsed = now.saturating_duration_since(*started);
                let batch = schedule.expected_batch_at(elapsed)?;
                batch.has_later_arrival.then(|| {
                    now.checked_add(
                        batch
                            .closes
                            .saturating_sub(elapsed)
                            .min(self.window.duration()),
                    )
                    .unwrap_or(now)
                })
            }
        }
    }

    /// The time an open batch must be emitted, if any.
    pub fn deadline(&self) -> Option<Instant> {
        match self.holding {
            Holding::Nothing => None,
            Holding::Until(deadline) => Some(deadline),
        }
    }

    /// Releases the held batch exactly once.
    pub fn release(&mut self) -> bool {
        let held = std::mem::replace(&mut self.holding, Holding::Nothing);
        held != Holding::Nothing
    }
}

#[cfg(test)]
mod tests {
    use super::super::schedule::PredictedArrival;
    use super::*;
    use crate::segment::Segment;

    const WINDOW: BusWindow = BusWindow::from_milliseconds(100);

    fn painted(text: &str) -> Painted {
        Painted::paint(&Segment::from_text(None, text), Some(TerminalWidth(40)))
    }

    #[test]
    fn fills_make_a_width_change_screen_stable() {
        let before = Painted::paint(
            &[
                Segment::from_text(None, "a").remove(0),
                Segment::fill(None, "."),
            ],
            Some(TerminalWidth(20)),
        );
        let after = Painted::paint(
            &[
                Segment::from_text(None, "long").remove(0),
                Segment::fill(None, "."),
            ],
            Some(TerminalWidth(20)),
        );
        assert_eq!(Reflow::None, Reflow::between(&before, &after));
    }

    #[test]
    fn an_unknown_reflow_opens_one_bounded_batch() {
        let now = Instant::now();
        let mut bus = Bus::fixed(WINDOW);
        assert_eq!(Verdict::Hold, bus.admit(Reflow::Shifts, now));
        assert_eq!(Some(now + WINDOW.duration()), bus.deadline());
        assert!(bus.release());
        assert!(!bus.release());
    }

    #[test]
    fn zero_window_draws_immediately() {
        assert_eq!(
            Verdict::DrawNow,
            Bus::fixed(BusWindow::from_milliseconds(0)).admit(Reflow::Shifts, Instant::now())
        );
    }

    #[test]
    fn a_prediction_holds_only_while_another_arrival_is_expected() {
        let now = Instant::now();
        let schedule = ArrivalSchedule::of(
            [
                PredictedArrival::after(Duration::from_millis(10)),
                PredictedArrival::after(Duration::from_millis(40)),
            ],
            WINDOW,
        );
        let mut bus = Bus::scheduled(WINDOW, schedule, now);
        assert_eq!(
            Verdict::Hold,
            bus.admit(Reflow::None, now + Duration::from_millis(10))
        );
        assert!(bus.release());
        assert_eq!(
            Verdict::DrawNow,
            bus.admit(Reflow::None, now + Duration::from_millis(40))
        );
    }

    #[test]
    fn changing_a_line_width_is_a_reflow() {
        assert_eq!(
            Reflow::Shifts,
            Reflow::between(&painted("a"), &painted("long"))
        );
    }
}
