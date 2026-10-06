//! The frames a streamed prompt is written as.
//!
//! Every frame is a keyword and two fields, each ended by a NUL byte:
//! `KEYWORD\0first\0second\0`. A prompt never holds a NUL, so prompts travel
//! verbatim, line breaks and all, and reading up to the next NUL is all a
//! shell has to do: `read -d ''` in zsh and bash, `read -z` in fish,
//! `bytes split 0x[00]` in Nushell, and a search for the byte anywhere else.

use std::io::{self, Write};

/// One frame of a streamed prompt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frame<'a> {
    /// The id of the process streaming, so that a shell can stop it. It is
    /// always the first frame.
    Process,
    /// The main and right prompts as they are to be drawn now. The first is
    /// the first paint, and each after it refines it.
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
            // A NUL would end its field early, and nothing a prompt shows needs one.
            for part in field.split('\0') {
                output.write_all(part.as_bytes())?;
            }
            output.write_all(b"\0")?;
        }
        // A shell watching the pipe gets each frame as soon as it is written.
        output.flush()
    }
}

/// Where a stream sends its frames.
pub trait Sink {
    fn send(&mut self, frame: Frame) -> io::Result<()>;
}

/// Frames written out, for a shell that reads them from a pipe.
impl<W: Write> Sink for W {
    fn send(&mut self, frame: Frame) -> io::Result<()> {
        frame.write_to(self)
    }
}

/// The fields of every frame in `bytes`, three to a frame.
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

    fn written(frame: Frame) -> Vec<u8> {
        let mut bytes = Vec::new();
        frame.write_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn the_process_frame_names_the_process_streaming() {
        assert_eq!(
            [[
                "PROCESS".to_owned(),
                std::process::id().to_string(),
                String::new()
            ]],
            read_frames(&written(Frame::Process)).as_slice()
        );
    }

    proptest! {
        /// Whatever prompts hold, a shell that splits the stream at every NUL
        /// reads them back whole, less any NUL, and in a frame of its own.
        #[test]
        fn prompts_travel_verbatim_between_terminators(main in any::<String>(), right in any::<String>()) {
            let mut bytes = written(Frame::Prompt { main: &main, right: &right });
            bytes.extend(written(Frame::Complete));

            prop_assert_eq!(
                vec![
                    ["PROMPT".to_owned(), main.replace('\0', ""), right.replace('\0', "")],
                    ["COMPLETE".to_owned(), String::new(), String::new()],
                ],
                read_frames(&bytes)
            );
        }
    }
}
