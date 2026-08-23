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

/// Streaming event encoding.
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
        let repaint =
            String::from_utf8(repaint.as_bytes().to_vec()).expect("repaints are valid UTF-8");
        debug_assert!(!repaint.contains('\0'));
        Self(repaint)
    }

    pub fn prompt(painted: &Painted) -> Self {
        let mut prompt = painted.to_string();
        prompt.retain(|character| character != '\0');
        Self(prompt)
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
        let expansion = match shell {
            Shell::Bash => Some(('\\', '[', ']')),
            Shell::Zsh | Shell::Tcsh => Some(('%', '{', '}')),
            _ => None,
        };
        let Some((introducer, zero_width_start, zero_width_end)) = expansion else {
            return RawTerminalPayload(self.0.clone());
        };

        let mut expanded = String::with_capacity(self.0.len());
        let mut characters = self.0.chars();
        while let Some(character) = characters.next() {
            if character != introducer {
                expanded.push(character);
                continue;
            }
            match characters.next() {
                Some(next) if next == zero_width_start || next == zero_width_end => {}
                Some(next) => expanded.push(next),
                None => expanded.push(introducer),
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
    pub fn whole_prompt(prompt: PromptVariablePayload) -> Self {
        Self::Replace(prompt)
    }

    pub fn repainting_cells(prompt: PromptVariablePayload, repaint: RawTerminalPayload) -> Self {
        Self::Repaint { prompt, repaint }
    }

    #[cfg(test)]
    pub fn prompt(&self) -> &PromptVariablePayload {
        match self {
            Self::Replace(prompt) | Self::Repaint { prompt, .. } => prompt,
        }
    }

    #[cfg(test)]
    pub fn repaint(&self) -> Option<&RawTerminalPayload> {
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
        const NOTHING: &[u8] = &[];
        match self {
            Self::Ready { prompt, process_id } => {
                let process_id = process_id.get().to_string();
                write_frame(writer, "READY", prompt.as_bytes(), process_id.as_bytes())
            }
            Self::Patch(Patch::Replace(prompt)) => {
                write_frame(writer, "PATCH", prompt.as_bytes(), NOTHING)
            }
            Self::Patch(Patch::Repaint { prompt, repaint }) => {
                write_frame(writer, "PATCH", prompt.as_bytes(), repaint.as_bytes())
            }
            Self::Complete(timings) => {
                let timings = timings.to_json_bytes();
                write_frame(writer, "COMPLETE", &timings, NOTHING)
            }
            Self::Heartbeat => write_frame(writer, "HEARTBEAT", NOTHING, NOTHING),
        }
    }

    fn write_json(&self, writer: &mut impl Write) -> io::Result<()> {
        serde_json::to_writer(&mut *writer, &JsonEvent::from(self))?;
        writer.write_all(b"\n")?;
        writer.flush()
    }

    #[cfg(test)]
    pub fn read_from(reader: &mut impl BufRead) -> io::Result<Option<Self>> {
        let Some(keyword) = read_field(reader)? else {
            return Ok(None);
        };
        let first = read_field(reader)?.ok_or(io::ErrorKind::UnexpectedEof)?;
        let second = read_field(reader)?.ok_or(io::ErrorKind::UnexpectedEof)?;
        let malformed = |message| io::Error::new(io::ErrorKind::InvalidData, message);

        match keyword.as_slice() {
            b"READY" => Ok(Some(Self::Ready {
                prompt: PromptVariablePayload(text(first)?),
                process_id: ProcessId(
                    std::str::from_utf8(&second)
                        .ok()
                        .and_then(|field| field.parse().ok())
                        .ok_or_else(|| malformed("invalid process id"))?,
                ),
            })),
            b"PATCH" => {
                let prompt = PromptVariablePayload(text(first)?);
                Ok(Some(Self::Patch(if second.is_empty() {
                    Patch::Replace(prompt)
                } else {
                    Patch::Repaint {
                        prompt,
                        repaint: RawTerminalPayload(text(second)?),
                    }
                })))
            }
            b"COMPLETE" if second.is_empty() => Timings::from_json(&first)
                .map(Self::Complete)
                .map(Some)
                .ok_or_else(|| malformed("invalid timings")),
            b"HEARTBEAT" if first.is_empty() && second.is_empty() => Ok(Some(Self::Heartbeat)),
            _ => Err(malformed("invalid event")),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
enum JsonEvent<'a> {
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

impl<'a> From<&'a ServerEvent> for JsonEvent<'a> {
    fn from(event: &'a ServerEvent) -> Self {
        match event {
            ServerEvent::Ready { prompt, process_id } => Self::Ready {
                prompt: &prompt.0,
                process_id: process_id.get(),
            },
            ServerEvent::Patch(Patch::Replace(prompt)) => Self::Patch {
                prompt: &prompt.0,
                repaint: None,
            },
            ServerEvent::Patch(Patch::Repaint { prompt, repaint }) => Self::Patch {
                prompt: &prompt.0,
                repaint: Some(&repaint.0),
            },
            ServerEvent::Complete(timings) => Self::Complete { timings },
            ServerEvent::Heartbeat => Self::Heartbeat,
        }
    }
}

fn write_frame(
    writer: &mut impl Write,
    keyword: &str,
    first: &[u8],
    second: &[u8],
) -> io::Result<()> {
    for field in [keyword.as_bytes(), first, second] {
        debug_assert!(!field.contains(&0));
        writer.write_all(field)?;
        writer.write_all(b"\0")?;
    }
    writer.flush()
}

#[cfg(test)]
fn read_field(reader: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
    let mut field = Vec::new();
    match reader.read_until(0, &mut field)? {
        0 => Ok(None),
        _ if field.pop() == Some(0) => Ok(Some(field)),
        _ => Err(io::ErrorKind::UnexpectedEof.into()),
    }
}

#[cfg(test)]
fn text(field: Vec<u8>) -> io::Result<String> {
    String::from_utf8(field).map_err(|_| io::ErrorKind::InvalidData.into())
}

/// A module resolution latency.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Microseconds(pub u64);

impl Microseconds {
    #[must_use]
    pub fn duration(self) -> Duration {
        Duration::from_micros(self.0)
    }
}

impl From<Duration> for Microseconds {
    fn from(duration: Duration) -> Self {
        Self(u64::try_from(duration.as_micros()).unwrap_or(u64::MAX))
    }
}

/// Module resolution timings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timings(BTreeMap<String, Microseconds>);

impl Timings {
    pub fn record(&mut self, module: &str, elapsed: Duration) {
        let elapsed = Microseconds::from(elapsed);
        let recorded = self.0.entry(module.to_owned()).or_default();
        *recorded = (*recorded).max(elapsed);
    }

    pub fn set(&mut self, module: &str, cost: Microseconds) {
        self.0.insert(module.to_owned(), cost);
    }

    pub fn get(&self, module: &str) -> Option<Microseconds> {
        self.0.get(module).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, Microseconds)> {
        self.0.iter().map(|(module, cost)| (module.as_str(), *cost))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("a map of strings and integers always serializes")
    }

    pub fn from_json(payload: &[u8]) -> Option<Self> {
        serde_json::from_slice(payload).ok()
    }
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
        assert_eq!(Some(Microseconds(12)), read.get("character"));
        assert_eq!(Some(Microseconds(180_000)), read.get("git_status"));
        assert_eq!(2, read.len());
    }

    #[test]
    fn a_module_recorded_twice_keeps_the_longer_latency() {
        // `directory` renders twice (instant estimate, then real); keep the real one.
        let mut timings = Timings::default();
        timings.record("directory", Duration::from_micros(40));
        timings.record("directory", Duration::from_millis(9));
        timings.record("directory", Duration::from_micros(50));

        assert_eq!(Some(Microseconds(9_000)), timings.get("directory"));
    }

    #[test]
    fn empty_timings_are_still_valid_json() {
        let timings = Timings::default();

        assert!(timings.is_empty());
        assert_eq!(compact_fields(&ServerEvent::Complete(timings))[1], b"{}");
    }
}
