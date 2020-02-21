use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use std::process::Command;

use crate::context::Shell;

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
        "crystal --version" => Some(CommandOutput {
            stdout: String::from("Crystal 0.32.1 (2019-12-18)"),
            stderr: String::default(),
        }),
        "dummy_command" => Some(CommandOutput {
            stdout: String::from("stdout ok!"),
            stderr: String::from("stderr ok!"),
        }),
        "elm --version" => Some(CommandOutput {
            stdout: String::from("0.19.1"),
            stderr: String::default(),
        }),
        "go version" => Some(CommandOutput {
            stdout: String::from("go version go1.12.1 linux/amd64"),
            stderr: String::default(),
        }),
        "node --version" => Some(CommandOutput {
            stdout: String::from("v12.0.0"),
            stderr: String::default(),
        }),
        "php -r echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION.'.'.PHP_RELEASE_VERSION;" => {
            Some(CommandOutput {
                stdout: String::from("7.3.8"),
                stderr: String::default(),
            })
        }
        "ruby -v" => Some(CommandOutput {
            stdout: String::from("ruby 2.5.1p57 (2018-03-29 revision 63029) [x86_64-linux-gnu]"),
            stderr: String::default(),
        }),
        "stack ghc -- --numeric-version --no-install-ghc" => Some(CommandOutput {
            stdout: String::from("8.6.5"),
            stderr: String::default(),
        }),
        // If we don't have a mocked command fall back to executing the command
        _ => internal_exec_cmd(&cmd, &args),
    }
}

/// Wraps ANSI color escape sequences in the shell-appropriate wrappers.
pub fn wrap_colorseq_for_shell(ansi: String, shell: Shell) -> String {
    const ESCAPE_BEGIN: char = '\u{1b}';
    const ESCAPE_END: char = 'm';
    wrap_seq_for_shell(ansi, shell, ESCAPE_BEGIN, ESCAPE_END)
}

/// Many shells cannot deal with raw unprintable characters and miscompute the cursor position,
/// leading to strange visual bugs like duplicated/missing chars. This function wraps a specified
/// sequence in shell-specific escapes to avoid these problems.
pub fn wrap_seq_for_shell(
    ansi: String,
    shell: Shell,
    escape_begin: char,
    escape_end: char,
) -> String {
    const BASH_BEG: &str = "\u{5c}\u{5b}"; // \[
    const BASH_END: &str = "\u{5c}\u{5d}"; // \]
    const ZSH_BEG: &str = "\u{25}\u{7b}"; // %{
    const ZSH_END: &str = "\u{25}\u{7d}"; // %}

    // ANSI escape codes cannot be nested, so we can keep track of whether we're
    // in an escape or not with a single boolean variable
    let mut escaped = false;
    let final_string: String = ansi
        .chars()
        .map(|x| {
            if x == escape_begin && !escaped {
                escaped = true;
                match shell {
                    Shell::Bash => format!("{}{}", BASH_BEG, escape_begin),
                    Shell::Zsh => format!("{}{}", ZSH_BEG, escape_begin),
                    _ => x.to_string(),
                }
            } else if x == escape_end && escaped {
                escaped = false;
                match shell {
                    Shell::Bash => format!("{}{}", escape_end, BASH_END),
                    Shell::Zsh => format!("{}{}", escape_end, ZSH_END),
                    _ => x.to_string(),
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
        let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello"]);
        let expected = Some(CommandOutput {
            stdout: String::from("hello\n"),
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

    #[test]
    fn test_color_sequence_wrappers() {
        let test0 = "\x1b2mhellomynamekeyes\x1b2m"; // BEGIN: \x1b     END: m
        let test1 = "\x1b]330;mlol\x1b]0m"; // BEGIN: \x1b     END: m
        let test2 = "\u{1b}J"; // BEGIN: \x1b     END: J
        let test3 = "OH NO"; // BEGIN: O    END: O
        let test4 = "herpaderp";
        let test5 = "";

        let zresult0 = wrap_seq_for_shell(test0.to_string(), Shell::Zsh, '\x1b', 'm');
        let zresult1 = wrap_seq_for_shell(test1.to_string(), Shell::Zsh, '\x1b', 'm');
        let zresult2 = wrap_seq_for_shell(test2.to_string(), Shell::Zsh, '\x1b', 'J');
        let zresult3 = wrap_seq_for_shell(test3.to_string(), Shell::Zsh, 'O', 'O');
        let zresult4 = wrap_seq_for_shell(test4.to_string(), Shell::Zsh, '\x1b', 'm');
        let zresult5 = wrap_seq_for_shell(test5.to_string(), Shell::Zsh, '\x1b', 'm');

        assert_eq!(&zresult0, "%{\x1b2m%}hellomynamekeyes%{\x1b2m%}");
        assert_eq!(&zresult1, "%{\x1b]330;m%}lol%{\x1b]0m%}");
        assert_eq!(&zresult2, "%{\x1bJ%}");
        assert_eq!(&zresult3, "%{OH NO%}");
        assert_eq!(&zresult4, "herpaderp");
        assert_eq!(&zresult5, "");

        let bresult0 = wrap_seq_for_shell(test0.to_string(), Shell::Bash, '\x1b', 'm');
        let bresult1 = wrap_seq_for_shell(test1.to_string(), Shell::Bash, '\x1b', 'm');
        let bresult2 = wrap_seq_for_shell(test2.to_string(), Shell::Bash, '\x1b', 'J');
        let bresult3 = wrap_seq_for_shell(test3.to_string(), Shell::Bash, 'O', 'O');
        let bresult4 = wrap_seq_for_shell(test4.to_string(), Shell::Bash, '\x1b', 'm');
        let bresult5 = wrap_seq_for_shell(test5.to_string(), Shell::Bash, '\x1b', 'm');

        assert_eq!(&bresult0, "\\[\x1b2m\\]hellomynamekeyes\\[\x1b2m\\]");
        assert_eq!(&bresult1, "\\[\x1b]330;m\\]lol\\[\x1b]0m\\]");
        assert_eq!(&bresult2, "\\[\x1bJ\\]");
        assert_eq!(&bresult3, "\\[OH NO\\]");
        assert_eq!(&bresult4, "herpaderp");
        assert_eq!(&bresult5, "");
    }
}
