use proptest::prelude::*;

use super::{Damage, terminal};
use crate::config::parse_style_string;
use crate::module::painted::{Painted, TerminalWidth};
use crate::segment::Segment;

fn prompt(parts: &[(bool, String)]) -> Painted {
    let mut segments: Vec<_> = parts
        .iter()
        .map(|(green, text)| {
            let style = green.then(|| parse_style_string("green", None).unwrap());
            Segment::from_text(style, text).remove(0)
        })
        .collect();
    segments.extend([Segment::LineTerm, Segment::from_text(None, "> ").remove(0)]);
    Painted::paint(&segments, Some(TerminalWidth(32)))
}

fn apply(terminal: &mut terminal::EmulatedTerminal, previous: &Painted, next: &Painted) {
    match Damage::between(previous, next, TerminalWidth(32)) {
        Damage::None => {}
        Damage::Repaint(repaint) => terminal.feed(repaint.as_bytes()),
        Damage::Full => terminal.redraw(&next.to_bytes()),
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn a_sequence_of_single_line_refinements_matches_full_rendering(
        updates in prop::collection::vec(
            prop::collection::vec((any::<bool>(), "[a-z]{0,6}"), 1..4),
            2..8,
        ),
    ) {
        let mut previous = prompt(&updates[0]);
        let mut terminal = terminal::EmulatedTerminal::blank(TerminalWidth(32));
        terminal.feed(&previous.to_bytes());

        for update in &updates[1..] {
            let next = prompt(update);
            prop_assert!(
                !matches!(Damage::between(&previous, &next, TerminalWidth(32)), Damage::Full),
                "single-line ASCII refinements must exercise incremental damage"
            );
            apply(&mut terminal, &previous, &next);
            prop_assert_eq!(terminal::fully_rendered(&next, TerminalWidth(32)), terminal.screen());
            previous = next;
        }
    }
}
