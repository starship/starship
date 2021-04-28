use process_control::{ChildExt, Timeout};
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::context::Shell;

/// Return the string contents of a file
pub fn read_file<P: AsRef<Path>>(file_name: P) -> Result<String> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;
    Ok(data)
}

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}

impl PartialEq for CommandOutput {
    fn eq(&self, other: &Self) -> bool {
        self.stdout == other.stdout && self.stderr == other.stderr
    }
}

/// Execute a command and return the output on stdout and stderr if successful
#[cfg(not(test))]
pub fn exec_cmd(cmd: &str, args: &[&str], time_limit: Duration) -> Option<CommandOutput> {
    internal_exec_cmd(&cmd, &args, time_limit)
}

#[cfg(test)]
pub fn exec_cmd(cmd: &str, args: &[&str], time_limit: Duration) -> Option<CommandOutput> {
    let command = match args.len() {
        0 => String::from(cmd),
        _ => format!("{} {}", cmd, args.join(" ")),
    };
    match command.as_str() {
        "crystal --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Crystal 0.35.1 (2020-06-19)

LLVM: 10.0.0
Default target: x86_64-apple-macosx\n",
            ),
            stderr: String::default(),
        }),
        "dart --version" => Some(CommandOutput {
            stdout: String::default(),
            stderr: String::from(
                "Dart VM version: 2.8.4 (stable) (Wed Jun 3 12:26:04 2020 +0200) on \"macos_x64\"",
            ),
        }),
        "deno -V" => Some(CommandOutput {
            stdout: String::from("deno 1.8.3\n"),
            stderr: String::default()
        }),
        "dummy_command" => Some(CommandOutput {
            stdout: String::from("stdout ok!\n"),
            stderr: String::from("stderr ok!\n"),
        }),
        "elixir --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Erlang/OTP 22 [erts-10.6.4] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1] [hipe]

Elixir 1.10 (compiled with Erlang/OTP 22)\n",
            ),
            stderr: String::default(),
        }),
        "elm --version" => Some(CommandOutput {
            stdout: String::from("0.19.1\n"),
            stderr: String::default(),
        }),
        "go version" => Some(CommandOutput {
            stdout: String::from("go version go1.12.1 linux/amd64\n"),
            stderr: String::default(),
        }),
        "helm version --short --client" => Some(CommandOutput {
            stdout: String::from("v3.1.1+gafe7058\n"),
            stderr: String::default(),
        }),
        s if s.ends_with("java -Xinternalversion") => Some(CommandOutput {
            stdout: String::from("OpenJDK 64-Bit Server VM (13.0.2+8) for bsd-amd64 JRE (13.0.2+8), built on Feb  6 2020 02:07:52 by \"brew\" with clang 4.2.1 Compatible Apple LLVM 11.0.0 (clang-1100.0.33.17)"),
            stderr: String::default(),
        }),
        "scalac -version" => Some(CommandOutput {
            stdout: String::from("Scala compiler version 2.13.5 -- Copyright 2002-2020, LAMP/EPFL and Lightbend, Inc."),
            stderr: String::default(),
        }),
        "julia --version" => Some(CommandOutput {
            stdout: String::from("julia version 1.4.0\n"),
            stderr: String::default(),
        }),
        "kotlin -version" => Some(CommandOutput {
            stdout: String::from("Kotlin version 1.4.21-release-411 (JRE 14.0.1+7)\n"),
            stderr: String::default(),
        }),
        "kotlinc -version" => Some(CommandOutput {
            stdout: String::from("info: kotlinc-jvm 1.4.21 (JRE 14.0.1+7)\n"),
            stderr: String::default(),
        }),
        "lua -v" => Some(CommandOutput{
            stdout: String::from("Lua 5.4.0  Copyright (C) 1994-2020 Lua.org, PUC-Rio\n"),
            stderr: String::default(),
        }),
        "luajit -v" => Some(CommandOutput{
            stdout: String::from("LuaJIT 2.0.5 -- Copyright (C) 2005-2017 Mike Pall. http://luajit.org/\n"),
            stderr: String::default(),
        }),
        "nim --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Nim Compiler Version 1.2.0 [Linux: amd64]
Compiled at 2020-04-03
Copyright (c) 2006-2020 by Andreas Rumpf
git hash: 7e83adff84be5d0c401a213eccb61e321a3fb1ff
active boot switches: -d:release\n",
            ),
            stderr: String::default(),
        }),
        "node --version" => Some(CommandOutput {
            stdout: String::from("v12.0.0\n"),
            stderr: String::default(),
        }),
        "ocaml -vnum" => Some(CommandOutput {
            stdout: String::from("4.10.0\n"),
            stderr: String::default(),
        }),
        "opam switch show --safe" => Some(CommandOutput {
            stdout: String::from("default\n"),
            stderr: String::default(),
        }),
        "esy ocaml -vnum" => Some(CommandOutput {
            stdout: String::from("4.08.1\n"),
            stderr: String::default(),
        }),
        "perl -e printf q#%vd#,$^V;" => Some(CommandOutput {
            stdout: String::from("5.26.1"),
            stderr: String::default(),
        }),
        "php -nr echo PHP_MAJOR_VERSION.\".\".PHP_MINOR_VERSION.\".\".PHP_RELEASE_VERSION;" => {
            Some(CommandOutput {
                stdout: String::from("7.3.8"),
                stderr: String::default(),
            })
        }
        "purs --version" => Some(CommandOutput {
            stdout: String::from("0.13.5\n"),
            stderr: String::default(),
        }),
        "pyenv version-name" => Some(CommandOutput {
            stdout: String::from("system\n"),
            stderr: String::default(),
        }),
        "python --version" => None,
        "python2 --version" => Some(CommandOutput {
            stdout: String::default(),
            stderr: String::from("Python 2.7.17\n"),
        }),
        "python3 --version" => Some(CommandOutput {
            stdout: String::from("Python 3.8.0\n"),
            stderr: String::default(),
        }),
        "red --version" => Some(CommandOutput {
            stdout: String::from("0.6.4\n"),
            stderr: String::default()
        }),
        "ruby -v" => Some(CommandOutput {
            stdout: String::from("ruby 2.5.1p57 (2018-03-29 revision 63029) [x86_64-linux-gnu]\n"),
            stderr: String::default(),
        }),
        "swift --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51)
Target: x86_64-apple-darwin19.4.0\n",
            ),
            stderr: String::default(),
        }),
        "vagrant --version" => Some(CommandOutput {
            stdout: String::from("Vagrant 2.2.10\n"),
            stderr: String::default(),
        }),
        "zig version" => Some(CommandOutput {
            stdout: String::from("0.6.0\n"),
            stderr: String::default(),
        }),
        "cmake --version" => Some(CommandOutput {
            stdout: String::from(
                "\
cmake version 3.17.3

CMake suite maintained and supported by Kitware (kitware.com/cmake).\n",
            ),
            stderr: String::default(),
        }),
        "dotnet --version" => Some(CommandOutput {
            stdout: String::from("3.1.103"),
            stderr: String::default(),
        }),
        "dotnet --list-sdks" => Some(CommandOutput {
            stdout: String::from("3.1.103 [/usr/share/dotnet/sdk]"),
            stderr: String::default(),
        }),
        "terraform version" => Some(CommandOutput {
            stdout: String::from("Terraform v0.12.14\n"),
            stderr: String::default(),
        }),
        s if s.starts_with("erl -noshell -eval") => Some(CommandOutput {
            stdout: String::from("22.1.3\n"),
            stderr: String::default(),
        }),
        // If we don't have a mocked command fall back to executing the command
        _ => internal_exec_cmd(&cmd, &args, time_limit),
    }
}

/// Wraps ANSI color escape sequences in the shell-appropriate wrappers.
pub fn wrap_colorseq_for_shell(mut ansi: String, shell: Shell) -> String {
    // Bash might interepret baskslashes, backticks and $
    // see #658 for more details
    if shell == Shell::Bash {
        ansi = ansi.replace('\\', r"\\");
        ansi = ansi.replace('$', r"\$");
        ansi = ansi.replace('`', r"\`");
    }

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
    if !matches!(shell, Shell::Bash | Shell::Zsh | Shell::Tcsh) {
        return ansi;
    }

    const BASH_BEG: &str = "\u{5c}\u{5b}"; // \[
    const BASH_END: &str = "\u{5c}\u{5d}"; // \]
    const ZSH_BEG: &str = "\u{25}\u{7b}"; // %{
    const ZSH_END: &str = "\u{25}\u{7d}"; // %}
    const TCSH_BEG: &str = "\u{25}\u{7b}"; // %{
    const TCSH_END: &str = "\u{25}\u{7d}"; // %}

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
                    Shell::Tcsh => format!("{}{}", TCSH_BEG, escape_begin),
                    _ => x.to_string(),
                }
            } else if x == escape_end && escaped {
                escaped = false;
                match shell {
                    Shell::Bash => format!("{}{}", escape_end, BASH_END),
                    Shell::Zsh => format!("{}{}", escape_end, ZSH_END),
                    Shell::Tcsh => format!("{}{}", escape_end, TCSH_END),
                    _ => x.to_string(),
                }
            } else {
                x.to_string()
            }
        })
        .collect();
    final_string
}

fn internal_exec_cmd(cmd: &str, args: &[&str], time_limit: Duration) -> Option<CommandOutput> {
    log::trace!("Executing command {:?} with args {:?}", cmd, args);

    let full_path = match which::which(cmd) {
        Ok(full_path) => {
            log::trace!("Using {:?} as {:?}", full_path, cmd);
            full_path
        }
        Err(error) => {
            log::trace!("Unable to find {:?} in PATH, {:?}", cmd, error);
            return None;
        }
    };

    let start = Instant::now();

    let process = match Command::new(full_path)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(process) => process,
        Err(error) => {
            log::info!("Unable to run {:?}, {:?}", cmd, error);
            return None;
        }
    };

    match process.with_output_timeout(time_limit).terminating().wait() {
        Ok(Some(output)) => {
            let stdout_string = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    log::warn!("Unable to decode stdout: {:?}", error);
                    return None;
                }
            };
            let stderr_string = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    log::warn!("Unable to decode stderr: {:?}", error);
                    return None;
                }
            };

            log::trace!(
                "stdout: {:?}, stderr: {:?}, exit code: \"{:?}\", took {:?}",
                stdout_string,
                stderr_string,
                output.status.code(),
                start.elapsed()
            );

            if !output.status.success() {
                return None;
            }

            Some(CommandOutput {
                stdout: stdout_string,
                stderr: stderr_string,
            })
        }
        Ok(None) => {
            log::warn!("Executing command {:?} timed out.", cmd);
            log::warn!("You can set command_timeout in your config to a higher value to allow longer-running commands to keep executing.");
            None
        }
        Err(error) => {
            log::info!("Executing command {:?} failed by: {:?}", cmd, error);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_mocked_command() {
        let result = exec_cmd("dummy_command", &[], Duration::from_millis(500));
        let expected = Some(CommandOutput {
            stdout: String::from("stdout ok!\n"),
            stderr: String::from("stderr ok!\n"),
        });

        assert_eq!(result, expected)
    }

    // While the exec_cmd should work on Windows some of these tests assume a Unix-like
    // environment.

    #[test]
    #[cfg(not(windows))]
    fn exec_no_output() {
        let result = internal_exec_cmd("true", &[], Duration::from_millis(500));
        let expected = Some(CommandOutput {
            stdout: String::from(""),
            stderr: String::from(""),
        });

        assert_eq!(result, expected)
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_output_stdout() {
        let result =
            internal_exec_cmd("/bin/sh", &["-c", "echo hello"], Duration::from_millis(500));
        let expected = Some(CommandOutput {
            stdout: String::from("hello\n"),
            stderr: String::from(""),
        });

        assert_eq!(result, expected)
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_output_stderr() {
        let result = internal_exec_cmd(
            "/bin/sh",
            &["-c", "echo hello >&2"],
            Duration::from_millis(500),
        );
        let expected = Some(CommandOutput {
            stdout: String::from(""),
            stderr: String::from("hello\n"),
        });

        assert_eq!(result, expected)
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_output_both() {
        let result = internal_exec_cmd(
            "/bin/sh",
            &["-c", "echo hello; echo world >&2"],
            Duration::from_millis(500),
        );
        let expected = Some(CommandOutput {
            stdout: String::from("hello\n"),
            stderr: String::from("world\n"),
        });

        assert_eq!(result, expected)
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_non_zero_exit_code() {
        let result = internal_exec_cmd("false", &[], Duration::from_millis(500));
        let expected = None;

        assert_eq!(result, expected)
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_slow_command() {
        let result = internal_exec_cmd("sleep", &["500"], Duration::from_millis(500));
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

    #[test]
    fn test_bash_escape() {
        let test = "$(echo a)";
        assert_eq!(
            wrap_colorseq_for_shell(test.to_owned(), Shell::Bash),
            r"\$(echo a)"
        );
        assert_eq!(
            wrap_colorseq_for_shell(test.to_owned(), Shell::PowerShell),
            test
        );

        let test = r"\$(echo a)";
        assert_eq!(
            wrap_colorseq_for_shell(test.to_owned(), Shell::Bash),
            r"\\\$(echo a)"
        );
        assert_eq!(
            wrap_colorseq_for_shell(test.to_owned(), Shell::PowerShell),
            test
        );

        let test = r"`echo a`";
        assert_eq!(
            wrap_colorseq_for_shell(test.to_owned(), Shell::Bash),
            r"\`echo a\`"
        );
        assert_eq!(
            wrap_colorseq_for_shell(test.to_owned(), Shell::PowerShell),
            test
        );
    }
}
