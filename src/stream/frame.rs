//! Prompt protocol: `KEYWORD\0first\0second\0` in UTF-8.
//! Fields preserve newlines; embedded NULs are stripped before writing.

use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frame<'a> {
    /// First frame: the stream PID, followed by an empty field.
    Process,
    /// Main and right prompts; replaces the previous pair.
    Prompt { main: &'a str, right: &'a str },
    /// Every module has rendered, and the prompt drawn shows it.
    Complete,
}

impl Frame<'_> {
    pub fn write_to(&self, output: &mut impl Write) -> io::Result<()> {
        let process_id;
        let fields = match self {
            Self::Process => {
                process_id = std::process::id().to_string();
                ["PROCESS", &process_id, ""]
            }
            Self::Prompt { main, right } => ["PROMPT", main, right],
            Self::Complete => ["COMPLETE", "", ""],
        };
        for field in fields {
            for part in field.split('\0') {
                output.write_all(part.as_bytes())?;
            }
            output.write_all(b"\0")?;
        }
        output.flush()
    }
}

pub trait Sink {
    fn send(&mut self, frame: Frame) -> io::Result<()>;
}

impl<W: Write> Sink for W {
    fn send(&mut self, frame: Frame) -> io::Result<()> {
        frame.write_to(self)
    }
}

#[cfg(test)]
pub fn read_frames(bytes: &[u8]) -> Vec<[String; 3]> {
    let fields: Vec<String> = String::from_utf8(bytes.to_vec())
        .expect("frames are text")
        .split_terminator('\0')
        .map(str::to_owned)
        .collect();
    assert_eq!(0, fields.len() % 3, "a frame has three fields: {fields:?}");
    fields
        .chunks(3)
        .map(|frame| frame.to_vec().try_into().expect("three fields"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn frames_preserve_text_and_strip_embedded_nuls(
            main in prop_oneof![any::<String>(), "[\\x00-\\x7f]{0,80}"],
            right in prop_oneof![any::<String>(), "[\\x00-\\x7f]{0,80}"],
        ) {
            let mut bytes = Vec::new();
            for frame in [
                Frame::Process,
                Frame::Prompt { main: &main, right: &right },
                Frame::Complete,
            ] {
                frame.write_to(&mut bytes).unwrap();
            }
            prop_assert_eq!(
                vec![
                    ["PROCESS".to_owned(), std::process::id().to_string(), String::new()],
                    ["PROMPT".to_owned(), main.replace('\0', ""), right.replace('\0', "")],
                    ["COMPLETE".to_owned(), String::new(), String::new()],
                ],
                read_frames(&bytes)
            );
        }

    }
}
