//! Streaming prompt frames.

use std::collections::BTreeMap;
#[cfg(test)]
use std::io::BufRead;
use std::io::{self, Write};
use std::time::Duration;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::context::Shell;
use crate::damage::cursor::CursorNeutral;
use crate::escaping::shell_prompt_escape;
use crate::module::painted::Painted;
use crate::utils::wrap_colorseq_for_shell;

const NULL_BYTE: u8 = b'\0';
const NEWLINE: &[u8] = b"\n";
const EMPTY_SLICE: &[u8] = &[];

const EVENT_READY: &[u8] = b"READY";
const EVENT_PATCH: &[u8] = b"PATCH";
const EVENT_COMPLETE: &[u8] = b"COMPLETE";
const EVENT_HEARTBEAT: &[u8] = b"HEARTBEAT";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum FrameEncoding {
    #[default]
    Compact,
    #[value(name = "json")]
    JsonLines,
}

/// Bytes written directly to the terminal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawTerminalPayload(String);

impl RawTerminalPayload {
    pub fn repaint(repaint: &CursorNeutral) -> Self {
        let repaint_string =
            String::from_utf8(repaint.as_bytes().to_vec()).expect("repaints are valid UTF-8");
        debug_assert!(!repaint_string.contains(char::from(NULL_BYTE)));
        Self(repaint_string)
    }

    pub fn prompt(painted: &Painted) -> Self {
        Self(painted.to_string().replace(char::from(NULL_BYTE), ""))
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

    #[cfg(test)]
    pub const fn prompt(&self) -> &PromptVariablePayload {
        match self {
            Self::Replace(prompt) | Self::Repaint { prompt, .. } => prompt,
        }
    }

    #[cfg(test)]
    pub const fn repaint(&self) -> Option<&RawTerminalPayload> {
        match self {
            Self::Replace(_) => None,
            Self::Repaint { repaint, .. } => Some(repaint),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessId(u32);

impl ProcessId {
    #[must_use]
    pub fn of_this_process() -> Self {
        Self(std::process::id())
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Module resolution timings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timings(BTreeMap<String, u64>);

impl Timings {
    /// An empty set of timings, usable in a `const` context.
    #[must_use]
    pub const fn empty() -> Self {
        Self(BTreeMap::new())
    }

    pub fn record(&mut self, module: &str, elapsed: Duration) {
        let elapsed_microseconds = u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX);
        self.0
            .entry(module.to_owned())
            .and_modify(|recorded| *recorded = (*recorded).max(elapsed_microseconds))
            .or_insert(elapsed_microseconds);
    }

    pub fn set(&mut self, module: &str, cost_microseconds: u64) {
        self.0.insert(module.to_owned(), cost_microseconds);
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

    fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("a map of strings to integers always serializes")
    }

    pub fn from_json(payload: &[u8]) -> Option<Self> {
        serde_json::from_slice(payload).ok()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServerEvent {
    /// The first prompt paint.
    Ready {
        prompt: PromptVariablePayload,
        /// Lets the shell stop a stream blocked in a module.
        process_id: ProcessId,
    },
    Patch(Patch),
    /// Initial rendering completed.
    Complete(Timings),
    /// Keeps an idle pipe observable.
    Heartbeat,
}

impl ServerEvent {
    #[cfg(test)]
    pub fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        self.write(FrameEncoding::Compact, writer)
    }

    pub fn write(&self, encoding: FrameEncoding, writer: &mut impl Write) -> io::Result<()> {
        match encoding {
            FrameEncoding::Compact => self.write_compact(writer),
            FrameEncoding::JsonLines => self.write_json(writer),
        }
    }

    fn write_compact(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Ready { prompt, process_id } => {
                let process_id_string = process_id.get().to_string();
                write_frame(
                    writer,
                    EVENT_READY,
                    prompt.as_bytes(),
                    process_id_string.as_bytes(),
                )
            }
            Self::Patch(Patch::Replace(prompt)) => {
                write_frame(writer, EVENT_PATCH, prompt.as_bytes(), EMPTY_SLICE)
            }
            Self::Patch(Patch::Repaint { prompt, repaint }) => {
                write_frame(writer, EVENT_PATCH, prompt.as_bytes(), repaint.as_bytes())
            }
            Self::Complete(timings) => write_frame(
                writer,
                EVENT_COMPLETE,
                &timings.to_json_bytes(),
                EMPTY_SLICE,
            ),
            Self::Heartbeat => write_frame(writer, EVENT_HEARTBEAT, EMPTY_SLICE, EMPTY_SLICE),
        }
    }

    fn write_json(&self, writer: &mut impl Write) -> io::Result<()> {
        #[derive(Serialize)]
        #[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
        enum JsonView<'a> {
            Ready {
                prompt: &'a str,
                process_id: u32,
            },
            Patch {
                prompt: &'a str,
                #[serde(skip_serializing_if = "Option::is_none")]
                repaint: Option<&'a str>,
            },
            Complete {
                timings: &'a Timings,
            },
            Heartbeat,
        }

        let payload = match self {
            Self::Ready { prompt, process_id } => JsonView::Ready {
                prompt: &prompt.0,
                process_id: process_id.get(),
            },
            Self::Patch(Patch::Replace(prompt)) => JsonView::Patch {
                prompt: &prompt.0,
                repaint: None,
            },
            Self::Patch(Patch::Repaint { prompt, repaint }) => JsonView::Patch {
                prompt: &prompt.0,
                repaint: Some(&repaint.0),
            },
            Self::Complete(timings) => JsonView::Complete { timings },
            Self::Heartbeat => JsonView::Heartbeat,
        };

        serde_json::to_writer(&mut *writer, &payload)?;
        writer.write_all(NEWLINE)?;
        writer.flush()
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
                let prompt = PromptVariablePayload(parse_text(first)?);
                let process_id = std::str::from_utf8(&second)
                    .ok()
                    .and_then(|string| string.parse().ok())
                    .map(ProcessId)
                    .ok_or("invalid process id")?;

                Ok(Some(Self::Ready { prompt, process_id }))
            }
            EVENT_PATCH => {
                let prompt = PromptVariablePayload(parse_text(first)?);
                let patch = if second.is_empty() {
                    Patch::Replace(prompt)
                } else {
                    Patch::Repaint {
                        prompt,
                        repaint: RawTerminalPayload(parse_text(second)?),
                    }
                };

                Ok(Some(Self::Patch(patch)))
            }
            EVENT_COMPLETE if second.is_empty() => Timings::from_json(&first)
                .map(Self::Complete)
                .map(Some)
                .ok_or("invalid timings"),
            EVENT_HEARTBEAT if first.is_empty() && second.is_empty() => Ok(Some(Self::Heartbeat)),
            _ => Err("invalid event"),
        }
    }
}

fn write_frame(
    writer: &mut impl Write,
    keyword: &[u8],
    first: &[u8],
    second: &[u8],
) -> io::Result<()> {
    for field in [keyword, first, second] {
        debug_assert!(!field.contains(&NULL_BYTE));
        writer.write_all(field)?;
        writer.write_all(&[NULL_BYTE])?;
    }
    writer.flush()
}

#[cfg(test)]
fn read_field(reader: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
    let mut field = Vec::new();
    let bytes_read = reader.read_until(NULL_BYTE, &mut field)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    if field.ends_with(&[NULL_BYTE]) {
        field.pop();
        Ok(Some(field))
    } else {
        Err(io::ErrorKind::UnexpectedEof.into())
    }
}

#[cfg(test)]
fn parse_text(field: Vec<u8>) -> Result<String, &'static str> {
    String::from_utf8(field).map_err(|_| "invalid utf-8 text")
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

    fn compact_fields(event: &ServerEvent) -> Vec<Vec<u8>> {
        let mut bytes = Vec::new();
        event
            .write_to(&mut bytes)
            .expect("writing to a vector cannot fail");
        assert_eq!(Some(&0), bytes.last());
        let mut fields: Vec<_> = bytes.split(|byte| *byte == 0).map(<[u8]>::to_vec).collect();
        assert_eq!(4, fields.len());
        assert!(fields.pop().expect("trailing field").is_empty());
        fields
    }

    #[test]
    fn json_lines_are_typed_and_self_delimiting() {
        let event = ServerEvent::Patch(Patch::repainting_cells(
            prompt_payload("10% off"),
            repaint_payload(),
        ));
        let mut bytes = Vec::new();
        event
            .write(FrameEncoding::JsonLines, &mut bytes)
            .expect("a vector is writable");

        assert_eq!(Some(&b'\n'), bytes.last());
        let json: serde_json::Value = serde_json::from_slice(&bytes).expect("one JSON object");
        assert_eq!("PATCH", json["kind"]);
        assert_eq!("10% off", json["prompt"]);
        assert!(json["repaint"].as_str().is_some());
    }

    #[test]
    fn compact_events_are_three_nul_delimited_fields() {
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
            let fields = compact_fields(&event);
            assert_eq!(3, fields.len());
            assert_eq!(keyword, fields[0]);
        }
    }

    #[test]
    fn compact_payloads_preserve_newlines() {
        let two_lines = Painted::paint(
            &[
                Segment::from_text(None, "first").remove(0),
                Segment::LineTerm,
                Segment::from_text(None, "second").remove(0),
            ],
            None,
        );
        let event = ready(PromptVariablePayload::escaped_for(
            &RawTerminalPayload::prompt(&two_lines),
            Shell::Unknown,
        ));

        assert_eq!(compact_fields(&event)[1], b"first\nsecond");
    }

    #[test]
    fn terminal_payloads_discard_nul() {
        let payload = RawTerminalPayload::prompt(&painted("a\0b"));
        assert_eq!(b"ab", payload.as_bytes());
    }

    #[test]
    fn compact_events_preserve_empty_fields() {
        let event = ready(PromptVariablePayload::escaped_for(
            &RawTerminalPayload::prompt(&Painted::default()),
            Shell::Unknown,
        ));

        let fields = compact_fields(&event);
        assert!(fields[1].is_empty());
        assert!(!fields[2].is_empty());
    }

    #[test]
    fn timings_round_trip_through_their_event() {
        let mut timings = Timings::default();
        timings.record("character", Duration::from_micros(12));
        timings.record("git_status", Duration::from_millis(180));

        let fields = compact_fields(&ServerEvent::Complete(timings.clone()));
        let read = Timings::from_json(&fields[1]).expect("timings are JSON");

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
    fn empty_timings_are_still_valid_json() {
        let timings = Timings::default();

        assert!(timings.is_empty());
        assert_eq!(compact_fields(&ServerEvent::Complete(timings))[1], b"{}");
    }
}
