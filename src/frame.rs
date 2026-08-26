//! Streaming prompt frames.
//!
//! Every event is the same shape: a keyword and two payload fields, each ended
//! by a NUL byte (`KEYWORD\0first\0second\0`). There is one encoding and no
//! flag to pick another.
//!
//! NUL is the one byte a payload may never contain. It is stripped at the
//! source, alongside carriage returns, so prompts carry their line breaks as
//! ordinary newlines and a shell has nothing to unescape. That leaves "read
//! until the next NUL" as the only thing a shell has to be able to do, and
//! every shell can, one way or another: `read -r -d ''` in zsh and bash,
//! `read -z` in fish, `bytes split 0x[00]` in nushell, `IndexOf` over a block
//! read in PowerShell, and a NUL scan of the buffer in xonsh and Clink's Lua.

use std::collections::BTreeMap;
#[cfg(test)]
use std::io::BufRead;
use std::io::{self, Write};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::context::Shell;
use crate::damage::cursor::CursorNeutral;
use crate::escaping::shell_prompt_escape;
use crate::module::painted::Painted;
use crate::utils::wrap_colorseq_for_shell;

/// Separates the fields of a frame, and never appears inside one.
const FIELD_TERMINATOR: u8 = b'\0';

/// Bytes that would corrupt framing or the terminal if they reached a payload.
/// NUL is the field terminator; a carriage return would drag the cursor back
/// over freshly drawn cells; the unit separator is reserved for packing the
/// timings payload and must not be mistaken for prompt text.
const PAYLOAD_FORBIDDEN: &[char] = &['\0', '\r', '\u{1f}'];

const EMPTY_FIELD: &[u8] = &[];

const EVENT_READY: &[u8] = b"READY";
const EVENT_PATCH: &[u8] = b"PATCH";
const EVENT_COMPLETE: &[u8] = b"COMPLETE";
const EVENT_HEARTBEAT: &[u8] = b"HEARTBEAT";
const EVENT_RIGHT: &[u8] = b"RIGHT";

/// Bytes written directly to the terminal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawTerminalPayload(String);

impl RawTerminalPayload {
    /// The prompt as it should appear on screen, with only the bytes that would
    /// corrupt framing or the cursor removed. Line breaks stay as newlines.
    pub fn prompt(painted: &Painted) -> Self {
        Self(Self::sanitized(&painted.to_string()))
    }

    /// The incremental cursor-relative bytes that turn one paint into the next.
    pub fn repaint(repaint: &CursorNeutral) -> Self {
        let bytes = String::from_utf8_lossy(repaint.as_bytes()).into_owned();
        debug_assert!(
            !bytes.contains(char::from(FIELD_TERMINATOR)),
            "a repaint must not contain the field terminator"
        );
        Self(bytes)
    }

    fn sanitized(text: &str) -> String {
        text.replace(PAYLOAD_FORBIDDEN, "")
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// Bytes assigned to a shell prompt variable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PromptVariablePayload(String);

impl PromptVariablePayload {
    pub fn escaped_for(payload: &RawTerminalPayload, shell: Shell) -> Self {
        let escaped = shell_prompt_escape(payload.0.as_str(), shell);
        Self(wrap_colorseq_for_shell(escaped, shell))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// What the shell ends up drawing after it expands this variable: the
    /// prompt escapes resolved back to the plain bytes they protected.
    #[cfg(test)]
    pub fn as_terminal_bytes_under(&self, shell: Shell) -> RawTerminalPayload {
        let escape_tokens = match shell {
            Shell::Bash => Some(('\\', '[', ']')),
            Shell::Zsh | Shell::Tcsh => Some(('%', '{', '}')),
            _ => None,
        };

        let Some((intro, start, end)) = escape_tokens else {
            return RawTerminalPayload(self.0.clone());
        };

        let mut expanded = String::with_capacity(self.0.len());
        let mut characters = self.0.chars();

        while let Some(character) = characters.next() {
            if character != intro {
                expanded.push(character);
                continue;
            }

            match characters.next() {
                Some(next) if next == start || next == end => continue,
                Some(next) => expanded.push(next),
                None => {
                    expanded.push(intro);
                    break;
                }
            }
        }

        RawTerminalPayload(expanded)
    }
}

/// A change to what the prompt shows: either the whole prompt, or the cells
/// that a cell-precise shell can repaint in place beside the prompt they leave.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Patch {
    Replace(PromptVariablePayload),
    Repaint {
        prompt: PromptVariablePayload,
        repaint: RawTerminalPayload,
    },
}

impl Patch {
    pub const fn whole_prompt(prompt: PromptVariablePayload) -> Self {
        Self::Replace(prompt)
    }

    pub const fn repainting_cells(
        prompt: PromptVariablePayload,
        repaint: RawTerminalPayload,
    ) -> Self {
        Self::Repaint { prompt, repaint }
    }

    pub const fn prompt(&self) -> &PromptVariablePayload {
        match self {
            Self::Replace(prompt) | Self::Repaint { prompt, .. } => prompt,
        }
    }

    /// The incremental bytes a cell-precise shell would apply, if any.
    #[cfg(test)]
    pub const fn repaint(&self) -> Option<&RawTerminalPayload> {
        match self {
            Self::Replace(_) => None,
            Self::Repaint { repaint, .. } => Some(repaint),
        }
    }
}

/// A process id. A stream announces its own (zsh cannot discover it: `$!` is
/// unset for a process substitution, and `coproc` is a single global slot a
/// plugin may already own); the snapshot transport also carries the id of the
/// shell it must signal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessId(u32);

impl ProcessId {
    #[must_use]
    pub fn of_this_process() -> Self {
        Self(std::process::id())
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Module resolution timings, carried between prompts of one shell session.
///
/// The wire form is flat pairs — `<name>\t<microseconds>`, joined by `\x1f` —
/// which the shell passes back verbatim as `--timings=<payload>` without ever
/// parsing it. A payload lives inside one frame field, so its separators are
/// free to be anything but the field terminator; module names are sanitized on
/// the way in so a configured name can neither forge a pair nor split one.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timings(BTreeMap<String, u64>);

impl Timings {
    const PAIR_SEPARATOR: char = '\u{1f}';
    const VALUE_SEPARATOR: char = '\t';

    /// An empty set of timings, usable in a `const` context.
    #[must_use]
    pub const fn empty() -> Self {
        Self(BTreeMap::new())
    }

    /// Drops every byte the framing or the packed form reserves, so a module
    /// name — which configuration may make nearly anything — stays one field
    /// of one pair.
    fn sanitize(module: &str) -> String {
        module
            .chars()
            .filter(|character| !matches!(character, '\0' | '\u{1f}' | '\t' | '\n' | '\r'))
            .collect()
    }

    pub fn record(&mut self, module: &str, elapsed: Duration) {
        let elapsed_microseconds = u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX);
        self.0
            .entry(Self::sanitize(module))
            .and_modify(|recorded| *recorded = (*recorded).max(elapsed_microseconds))
            .or_insert(elapsed_microseconds);
    }

    pub fn set(&mut self, module: &str, cost_microseconds: u64) {
        self.0.insert(Self::sanitize(module), cost_microseconds);
    }

    /// The two sides of one prompt, as the single payload the shell hands back.
    ///
    /// A module a side never drew has no entry to contribute, and one both drew
    /// keeps the slower reading, which is what [`record`](Self::record) does
    /// within a side. Names arrive already sanitized.
    #[must_use]
    pub fn merged_with(mut self, other: &Self) -> Self {
        for (module, &cost_microseconds) in &other.0 {
            self.0
                .entry(module.clone())
                .and_modify(|recorded| *recorded = (*recorded).max(cost_microseconds))
                .or_insert(cost_microseconds);
        }
        self
    }

    pub fn get(&self, module: &str) -> Option<u64> {
        self.0.get(module).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, u64)> {
        self.0.iter().map(|(module, cost)| (module.as_str(), *cost))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn to_wire_string(&self) -> String {
        let mut wire = String::new();
        for (module, cost) in &self.0 {
            if !wire.is_empty() {
                wire.push(Self::PAIR_SEPARATOR);
            }
            wire.push_str(module);
            wire.push(Self::VALUE_SEPARATOR);
            wire.push_str(&cost.to_string());
        }
        wire
    }

    pub fn from_wire(payload: &[u8]) -> Option<Self> {
        let text = std::str::from_utf8(payload).ok()?;
        let mut map = BTreeMap::new();
        for pair in text.split(Self::PAIR_SEPARATOR) {
            let (module, cost) = pair.split_once(Self::VALUE_SEPARATOR)?;
            map.insert(module.to_owned(), cost.parse().ok()?);
        }
        Some(Self(map))
    }

    // Retained for reading sessions that hand back a payload from an older
    // starship; anything unrecognized degrades to "no estimates".
    pub fn from_json(payload: &[u8]) -> Option<Self> {
        serde_json::from_slice(payload).ok()
    }
}

/// One event in a streamed prompt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServerEvent {
    /// The first prompt paint, plus the stream's own process id.
    Ready {
        prompt: PromptVariablePayload,
        process_id: ProcessId,
    },
    /// A refinement of the prompt already on screen.
    Patch(Patch),
    /// Initial rendering completed; carries the timings to hand to the next prompt.
    Complete(Timings),
    /// Payload-free liveness, so an idle pipe stays observable.
    Heartbeat,
    /// The right prompt, whole, from the same stream as the left.
    ///
    /// One renderer serves both sides, so the right side needs no first paint
    /// of its own to distinguish and no process id to announce: every one of
    /// its paints, first or later, is the same statement that the right prompt
    /// is now this. That is why the pair of sides costs one new keyword rather
    /// than a second set of them.
    RightPrompt(PromptVariablePayload),
}

impl ServerEvent {
    pub fn write_to(&self, writer: &mut dyn Write) -> io::Result<()> {
        self.with_fields(|fields| write_compact(fields, writer))
    }

    /// The three wire fields of this event — keyword, then two payloads —
    /// handed to `visit` while the materialized ones (a stringified process id,
    /// the packed timings) are still alive, so the shape of an event lives in
    /// exactly one place.
    fn with_fields<R>(&self, visit: impl FnOnce([&[u8]; 3]) -> R) -> R {
        match self {
            Self::Ready { prompt, process_id } => {
                let process_id_string = process_id.get().to_string();
                visit([EVENT_READY, prompt.as_bytes(), process_id_string.as_bytes()])
            }
            Self::Patch(Patch::Replace(prompt)) => {
                visit([EVENT_PATCH, prompt.as_bytes(), EMPTY_FIELD])
            }
            Self::Patch(Patch::Repaint { prompt, repaint }) => {
                visit([EVENT_PATCH, prompt.as_bytes(), repaint.as_bytes()])
            }
            Self::Complete(timings) => {
                let timings_wire = timings.to_wire_string();
                visit([EVENT_COMPLETE, timings_wire.as_bytes(), EMPTY_FIELD])
            }
            Self::Heartbeat => visit([EVENT_HEARTBEAT, EMPTY_FIELD, EMPTY_FIELD]),
            Self::RightPrompt(prompt) => visit([EVENT_RIGHT, prompt.as_bytes(), EMPTY_FIELD]),
        }
    }

    #[cfg(test)]
    pub fn read_from(reader: &mut impl BufRead) -> io::Result<Option<Self>> {
        let Some(keyword) = read_field(reader)? else {
            return Ok(None);
        };
        let first = read_field(reader)?.ok_or(io::ErrorKind::UnexpectedEof)?;
        let second = read_field(reader)?.ok_or(io::ErrorKind::UnexpectedEof)?;

        Self::parse_event(&keyword, first, second)
            .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))
    }

    #[cfg(test)]
    fn parse_event(
        keyword: &[u8],
        first: Vec<u8>,
        second: Vec<u8>,
    ) -> Result<Option<Self>, &'static str> {
        match keyword {
            EVENT_READY => {
                let prompt = PromptVariablePayload(parse_text(first));
                let process_id = std::str::from_utf8(&second)
                    .ok()
                    .and_then(|string| string.parse().ok())
                    .map(ProcessId)
                    .ok_or("invalid process id")?;

                Ok(Some(Self::Ready { prompt, process_id }))
            }
            EVENT_PATCH => {
                let prompt = PromptVariablePayload(parse_text(first));
                let patch = if second.is_empty() {
                    Patch::Replace(prompt)
                } else {
                    Patch::Repaint {
                        prompt,
                        repaint: RawTerminalPayload(parse_text(second)),
                    }
                };

                Ok(Some(Self::Patch(patch)))
            }
            EVENT_COMPLETE if second.is_empty() => {
                let timings = if first.is_empty() {
                    Timings::empty()
                } else {
                    Timings::from_wire(&first)
                        .or_else(|| Timings::from_json(&first))
                        .ok_or("invalid timings")?
                };
                Ok(Some(Self::Complete(timings)))
            }
            EVENT_HEARTBEAT if first.is_empty() && second.is_empty() => Ok(Some(Self::Heartbeat)),
            EVENT_RIGHT if second.is_empty() => {
                Ok(Some(Self::RightPrompt(PromptVariablePayload(parse_text(first)))) )
            }
            _ => Err("invalid event"),
        }
    }
}

/// Compact framing: each field followed by a NUL, then a flush, so a reader
/// watching the pipe sees a whole frame the instant it is produced.
fn write_compact(fields: [&[u8]; 3], writer: &mut dyn Write) -> io::Result<()> {
    for field in fields {
        debug_assert!(
            !field.contains(&FIELD_TERMINATOR),
            "a compact frame field must not contain the field terminator"
        );
        writer.write_all(field)?;
        writer.write_all(&[FIELD_TERMINATOR])?;
    }
    writer.flush()
}

#[cfg(test)]
fn read_field(reader: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
    let mut field = Vec::new();
    let bytes_read = reader.read_until(FIELD_TERMINATOR, &mut field)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    if field.ends_with(&[FIELD_TERMINATOR]) {
        field.pop();
        Ok(Some(field))
    } else {
        Err(io::ErrorKind::UnexpectedEof.into())
    }
}

#[cfg(test)]
fn parse_text(field: Vec<u8>) -> String {
    String::from_utf8(field).unwrap_or_else(|_| String::from("invalid utf-8"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Shell;
    use crate::damage::Column;
    use crate::damage::cursor::{CursorNeutral, CursorSafeText, RowsAbove};
    use crate::module::painted::TerminalWidth;
    use crate::segment::Segment;

    fn ready(prompt: PromptVariablePayload) -> ServerEvent {
        ServerEvent::Ready {
            prompt,
            process_id: ProcessId::of_this_process(),
        }
    }

    fn painted(text: &str) -> Painted {
        Painted::paint(&Segment::from_text(None, text), Some(TerminalWidth(80)))
    }

    fn prompt_payload(text: &str) -> PromptVariablePayload {
        PromptVariablePayload::escaped_for(
            &RawTerminalPayload::prompt(&painted(text)),
            Shell::Unknown,
        )
    }

    fn repaint_payload() -> RawTerminalPayload {
        RawTerminalPayload::repaint(&CursorNeutral::around(|body| {
            body.move_to(RowsAbove(1), Column(4));
            body.write_text(CursorSafeText::new("main").expect("plain text is cursor safe"));
        }))
    }

    /// Every frame is three NUL-terminated fields; splitting the written bytes
    /// on NUL leaves those three plus one trailing empty part.
    fn frame_fields(event: &ServerEvent) -> Vec<Vec<u8>> {
        let mut bytes = Vec::new();
        event
            .write_to(&mut bytes)
            .expect("writing to a vector cannot fail");
        assert_eq!(Some(&FIELD_TERMINATOR), bytes.last());
        let mut fields: Vec<_> = bytes
            .split(|&byte| byte == FIELD_TERMINATOR)
            .map(<[u8]>::to_vec)
            .collect();
        assert_eq!(4, fields.len());
        assert!(fields.pop().expect("trailing field").is_empty());
        fields
    }

    /// A field returns byte for byte no matter where its newlines fall, the
    /// cases most likely to be off by one. The leading-blank-line shape is the
    /// one an `add_newline` prompt opens with, and the one a reader that hunts
    /// for a terminator with a culture-sensitive comparison truncates to empty.
    #[test]
    fn fields_round_trip_every_newline_shape() {
        for original in [
            "",        // empty
            "x",       // one line, no terminator
            "x\n",     // a trailing newline
            "\n",      // a lone newline (two empty lines)
            "\n\nx",   // leading blank lines (the add_newline shape)
            "\n\n\n",  // only newlines
            "a\nb\nc", // several lines
        ] {
            let mut written = Vec::new();
            write_compact([b"PATCH", original.as_bytes(), EMPTY_FIELD], &mut written)
                .expect("writing to a vector cannot fail");

            let mut reader = std::io::Cursor::new(written);
            assert_eq!(
                Some(b"PATCH".to_vec()),
                read_field(&mut reader).expect("a keyword")
            );
            assert_eq!(
                Some(original.as_bytes().to_vec()),
                read_field(&mut reader).expect("a payload"),
                "round-trip changed {original:?}"
            );
            assert_eq!(
                Some(Vec::new()),
                read_field(&mut reader).expect("an empty payload")
            );
        }
    }

    #[test]
    fn frames_are_three_nul_terminated_fields() {
        let mut timings = Timings::default();
        timings.record("git_status", Duration::from_millis(180));

        let cases = [
            (ready(prompt_payload("> ")), b"READY".as_slice()),
            (
                ServerEvent::Patch(Patch::whole_prompt(prompt_payload("> "))),
                b"PATCH",
            ),
            (
                ServerEvent::Patch(Patch::repainting_cells(
                    prompt_payload("> "),
                    repaint_payload(),
                )),
                b"PATCH",
            ),
            (ServerEvent::Complete(timings), b"COMPLETE"),
            (ServerEvent::Heartbeat, b"HEARTBEAT"),
        ];

        for (event, keyword) in cases {
            let fields = frame_fields(&event);
            assert_eq!(3, fields.len());
            assert_eq!(keyword, fields[0]);
        }
    }

    #[test]
    fn prompt_line_breaks_travel_as_real_newlines() {
        let two_lines = Painted::paint(
            &[
                Segment::from_text(None, "first").remove(0),
                Segment::LineTerm,
                Segment::from_text(None, "second").remove(0),
            ],
            None,
        );

        let wire = RawTerminalPayload::prompt(&two_lines);
        assert_eq!(b"first\nsecond", wire.as_bytes());
        assert_eq!(
            frame_fields(&ready(PromptVariablePayload::escaped_for(
                &wire,
                Shell::Unknown
            )))[1],
            b"first\nsecond"
        );
    }

    #[test]
    fn terminal_payloads_discard_nul_and_carriage_return() {
        let payload = RawTerminalPayload::prompt(&painted("a\0b\rc"));
        assert_eq!(b"abc", payload.as_bytes());
    }

    #[test]
    fn frames_preserve_empty_fields() {
        let event = ready(PromptVariablePayload::escaped_for(
            &RawTerminalPayload::prompt(&Painted::default()),
            Shell::Unknown,
        ));

        let fields = frame_fields(&event);
        assert!(fields[1].is_empty());
        assert!(!fields[2].is_empty());

        let heartbeats = frame_fields(&ServerEvent::Heartbeat);
        assert!(heartbeats[1].is_empty() && heartbeats[2].is_empty());
    }

    #[test]
    fn timings_round_trip_through_their_event() {
        let mut timings = Timings::default();
        timings.record("character", Duration::from_micros(12));
        timings.record("git_status", Duration::from_millis(180));

        let fields = frame_fields(&ServerEvent::Complete(timings.clone()));
        let read = Timings::from_wire(&fields[1]).expect("timings are well-formed");

        assert_eq!(timings, read);
        assert_eq!(Some(12), read.get("character"));
        assert_eq!(Some(180_000), read.get("git_status"));
        assert_eq!(2, read.len());
    }

    #[test]
    fn a_module_recorded_twice_keeps_the_longer_latency() {
        // `directory` renders twice (instant estimate, then real); keep the real one.
        let mut timings = Timings::default();
        timings.record("directory", Duration::from_micros(40));
        timings.record("directory", Duration::from_millis(9));
        timings.record("directory", Duration::from_micros(50));

        assert_eq!(Some(9_000), timings.get("directory"));
    }

    #[test]
    fn empty_timings_are_an_empty_payload() {
        let timings = Timings::default();

        assert!(timings.is_empty());
        assert!(frame_fields(&ServerEvent::Complete(timings))[1].is_empty());
    }

    #[test]
    fn module_names_cannot_smuggle_separators_into_their_pairs() {
        let mut timings = Timings::default();
        timings.set("custom.a\tbogus", 7);

        let read =
            Timings::from_wire(timings.to_wire_string().as_bytes()).expect("sanitized names parse");
        assert_eq!(1, read.len());
        assert_eq!(Some(7), read.get("custom.abogus"));
    }

    #[test]
    fn malformed_timings_degrade_to_nothing() {
        for payload in [&b"no separator"[..], b"name\tnotanumber", b"a\t1\x1fbroken"] {
            assert!(Timings::from_wire(payload).is_none(), "{payload:?}");
        }
    }
}
