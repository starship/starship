use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use std::process::Command;

/// Return the string contents of a file
pub fn read_file<P: AsRef<Path>>(file_name: P) -> Result<String> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;
    Ok(data)
}

#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}

impl PartialEq for CommandOutput {
    fn eq(&self, other: &Self) -> bool {
        self.stdout == other.stdout && self.stderr == other.stderr
    }
}

/// Execute a command and return the output on stdout and stderr if sucessful
pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
    log::trace!("Executing command '{:?}' with args '{:?}'", cmd, args);
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            let stdout_string = String::from_utf8(output.stdout).unwrap();
            let stderr_string = String::from_utf8(output.stderr).unwrap();

            if !output.status.success() {
                log::trace!("Non-zero exit code '{:?}'", output.status.code());
                log::trace!("stdout: {}", stdout_string);
                log::trace!("stderr: {}", stderr_string);
                return None;
            }

            Some(CommandOutput {
                stdout: stdout_string,
                stderr: stderr_string,
            })
        }
        Err(_) => None,
    }
}

#[cfg(test)]
#[cfg(not(windows))] // While the exec_cmd should work on Windows these tests assume a Unix-like environment.
mod tests {
    use super::*;

    #[test]
    fn exec_no_output() {
        let result = exec_cmd("true", &[]);
        let expected = Some(CommandOutput {
            stdout: String::from(""),
            stderr: String::from(""),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_output_stdout() {
        let result = exec_cmd("/bin/echo", &["-n", "hello"]);
        let expected = Some(CommandOutput {
            stdout: String::from("hello"),
            stderr: String::from(""),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_output_stderr() {
        let result = exec_cmd("/bin/sh", &["-c", "echo hello >&2"]);
        let expected = Some(CommandOutput {
            stdout: String::from(""),
            stderr: String::from("hello\n"),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_output_both() {
        let result = exec_cmd("/bin/sh", &["-c", "echo hello; echo world >&2"]);
        let expected = Some(CommandOutput {
            stdout: String::from("hello\n"),
            stderr: String::from("world\n"),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_non_zero_exit_code() {
        let result = exec_cmd("false", &[]);
        let expected = None;

        assert_eq!(result, expected)
    }
}
