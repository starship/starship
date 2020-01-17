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
#[cfg(not(test))]
pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
    internal_exec_cmd(&cmd, &args)
}

#[cfg(test)]
pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
    let command = match args.len() {
        0 => String::from(cmd),
        _ => format!("{} {}", cmd, args.join(" ")),
    };

    match command.as_str() {
        "dummy_command" => Some(CommandOutput {
            stdout: String::from("stdout ok!"),
            stderr: String::from("stderr ok!"),
        }),
        // If we don't have a mocked command fall back to executing the command
        _ => internal_exec_cmd(&cmd, &args),
    }
}

// Convenience function to wrap ANSI color sequences (ECMA-48 SGR in `man console_codes`)
pub fn wrap_colorseq_for_shell(ansi: String, shell: &str) -> String {
    const ESCAPE_BEGIN: char = '\u{1b}';
    const ESCAPE_END: char = 'm';
    wrap_seq_for_shell(ansi, shell, ESCAPE_BEGIN, ESCAPE_END)
}

/// Many shells cannot deal with raw unprintable characters and miscompute the cursor position,
/// leading to strange visual bugs like duplicated/missing chars. This wraps some escape seqs
/// in shell-specific escapes to avoid this.
pub fn wrap_seq_for_shell(
    ansi: String,
    shell: &str,
    escape_begin: char,
    escape_end: char,
) -> String {
    let mut escaped = false;
    let final_string: String = ansi
        .chars()
        .map(|x| {
            if x == escape_begin {
                escaped = true;
                match shell {
                    "bash" => String::from("\u{5c}\u{5b}\u{1b}"), // => \[ESC
                    "zsh" => String::from("\u{25}\u{7b}\u{1b}"),  // => %{ESC
                    _ => x.to_string(),
                }
            } else if x == escape_end {
                if escaped {
                    escaped = false;
                    match shell {
                        "bash" => String::from("m\u{5c}\u{5d}"), // => m\]
                        "zsh" => String::from("m\u{25}\u{7d}"),  // => m%}
                        _ => x.to_string(),
                    }
                } else {
                    x.to_string()
                }
            } else {
                x.to_string()
            }
        })
        .collect();
    final_string
}

fn internal_exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
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
    fn exec_mocked_command() {
        let result = exec_cmd("dummy_command", &[]);
        let expected = Some(CommandOutput {
            stdout: String::from("stdout ok!"),
            stderr: String::from("stderr ok!"),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_no_output() {
        let result = internal_exec_cmd("true", &[]);
        let expected = Some(CommandOutput {
            stdout: String::from(""),
            stderr: String::from(""),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_output_stdout() {
        let result = internal_exec_cmd("/bin/echo", &["-n", "hello"]);
        let expected = Some(CommandOutput {
            stdout: String::from("hello"),
            stderr: String::from(""),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_output_stderr() {
        let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello >&2"]);
        let expected = Some(CommandOutput {
            stdout: String::from(""),
            stderr: String::from("hello\n"),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_output_both() {
        let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello; echo world >&2"]);
        let expected = Some(CommandOutput {
            stdout: String::from("hello\n"),
            stderr: String::from("world\n"),
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn exec_with_non_zero_exit_code() {
        let result = internal_exec_cmd("false", &[]);
        let expected = None;

        assert_eq!(result, expected)
    }
}
