use std::io;
use std::process;

/// Replace the `Command` import when building tests with a mock of `Command`
macro_rules! use_command {
    () => {
        #[cfg(not(test))]
        use std::process::Command;

        #[cfg(test)]
        use crate::mocks::Command;
    };
}

pub struct Command {
    binary: &'static str,
}

pub struct Output {
    pub stdout: Vec<u8>,
}

impl Command {
    pub fn new(binary_name: &'static str) -> Self {
        Self {
            binary: binary_name,
        }
    }

    pub fn arg(&self, _argument_name: &str) -> &Self {
        &self
    }

    pub fn output(&self) -> io::Result<Output> {
        let output = match self.binary {
            "ruby" => "ruby 2.5.5p456 (2018-03-28 revision 63024) [universal.x86_64-darwin18]",

            _ => panic!("Unknown binary"),
        };

        Ok(Output {
            stdout: output.to_string().into_bytes(),
        })
    }
}
