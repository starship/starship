pub mod env;
pub mod serde;
pub mod statusline;

use process_control::{ChildExt, Control};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::context::Context;
use crate::context::Shell;

/// Default timeout for command execution in milliseconds
pub const DEFAULT_COMMAND_TIMEOUT_MS: u64 = 500;

/// Create a `PathBuf` from an absolute path, where the root directory will be mocked in test
#[cfg(not(test))]
#[inline]
#[allow(dead_code)]
pub fn context_path<S: AsRef<OsStr> + ?Sized>(_context: &Context, s: &S) -> PathBuf {
    PathBuf::from(s)
}

/// Create a `PathBuf` from an absolute path, where the root directory will be mocked in test
#[cfg(test)]
#[allow(dead_code)]
pub fn context_path<S: AsRef<OsStr> + ?Sized>(context: &Context, s: &S) -> PathBuf {
    let requested_path = PathBuf::from(s);

    if requested_path.is_absolute() {
        let mut path = PathBuf::from(context.root_dir.path());
        path.extend(requested_path.components().skip(1));
        path
    } else {
        requested_path
    }
}

/// Return the string contents of a file
pub fn read_file<P: AsRef<Path> + Debug>(file_name: P) -> Result<String> {
    log::trace!("Trying to read from {file_name:?}");

    let result = fs::read_to_string(file_name);

    if result.is_err() {
        log::debug!("Error reading file: {result:?}");
    } else {
        log::trace!("File read successfully");
    }

    result
}

/// Write a string to a file
#[cfg(test)]
pub fn write_file<P: AsRef<Path>, S: AsRef<str>>(file_name: P, text: S) -> Result<()> {
    let file_name = file_name.as_ref();
    let text = text.as_ref();

    log::trace!("Trying to write {text:?} to {file_name:?}");
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
    {
        Ok(file) => file,
        Err(err) => {
            log::warn!("Error creating file: {err:?}");
            return Err(err);
        }
    };

    match file.write_all(text.as_bytes()) {
        Ok(()) => {
            log::trace!("File {file_name:?} written successfully");
        }
        Err(err) => {
            log::warn!("Error writing to file: {err:?}");
            return Err(err);
        }
    }
    file.sync_all()
}

/// Write contents to a file by first writing to a temporary file
/// and then move it to the target location in place
/// Only overwrites existing files if `force` is true
pub fn write_file_atomic<P: AsRef<Path>, S: AsRef<str>>(
    target_path: P,
    text: S,
    force: bool,
) -> std::result::Result<(), String> {
    let target_path = target_path.as_ref();
    let text = text.as_ref();

    log::trace!("Trying to write {text:?} to {target_path:?}");

    #[cfg_attr(not(unix), allow(unused_mut))]
    let mut builder = tempfile::Builder::new();

    // On Unix, the default permissions are too restrictive, so we need to relax them
    // This should be safe because we're creating a temporary file in the same directory
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let permissions = target_path
            .metadata()
            .as_ref()
            .map(fs::Metadata::permissions)
            .unwrap_or_else(|_| {
                let all_read_write = 0o666;
                std::fs::Permissions::from_mode(all_read_write)
            });

        builder.permissions(permissions);
    }

    let Some(parent_dir) = target_path.parent() else {
        return Err(format!(
            "Unable to determine parent directory of {target_path:?}"
        ));
    };

    let mut temp_file = builder
        .tempfile_in(parent_dir)
        .map_err(|e| format!("Error creating temporary file: {}", e))?;

    if let Err(err) = temp_file.write_all(text.as_bytes()) {
        return Err(format!("Error writing to temporary file: {}", err));
    }

    let result = if force {
        temp_file.persist(target_path)
    } else {
        temp_file.persist_noclobber(target_path)
    };

    result.map_err(|e| {
        if !force && e.error.kind() == ErrorKind::AlreadyExists {
            "Error saving file, use --force to overwrite existing configuration file".to_string()
        } else {
            format!("Error moving temporary file to target location: {e}")
        }
    })?;

    log::trace!("File {target_path:?} written successfully");

    Ok(())
}

/// Reads command output from stderr or stdout depending on to which stream program streamed it's output
pub fn get_command_string_output(command: CommandOutput) -> String {
    if command.stdout.is_empty() {
        command.stderr
    } else {
        command.stdout
    }
}

/// Attempt to resolve `binary_name` from and creates a new `Command` pointing at it
/// This allows executing cmd files on Windows and prevents running executable from cwd on Windows
/// This function also initializes std{err,out,in} to protect against processes changing the console mode
pub fn create_command<T: AsRef<OsStr>>(binary_name: T) -> Result<Command> {
    let binary_name = binary_name.as_ref();
    log::trace!("Creating Command for binary {binary_name:?}");

    let full_path = match which::which(binary_name) {
        Ok(full_path) => {
            log::trace!("Using {full_path:?} as {binary_name:?}");
            full_path
        }
        Err(error) => {
            log::trace!("Unable to find {binary_name:?} in PATH, {error:?}");
            return Err(Error::new(ErrorKind::NotFound, error));
        }
    };

    #[allow(clippy::disallowed_methods)]
    let mut cmd = Command::new(full_path);
    cmd.stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::null());

    Ok(cmd)
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

#[cfg(test)]
pub fn display_command<T: AsRef<OsStr> + Debug, U: AsRef<OsStr> + Debug>(
    cmd: T,
    args: &[U],
) -> String {
    std::iter::once(cmd.as_ref())
        .chain(args.iter().map(AsRef::as_ref))
        .map(|i| i.to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Execute a command and return the output on stdout and stderr if successful
pub fn exec_cmd<T: AsRef<OsStr> + Debug, U: AsRef<OsStr> + Debug>(
    cmd: T,
    args: &[U],
    time_limit: Duration,
) -> Option<CommandOutput> {
    log::trace!("Executing command {cmd:?} with args {args:?}");
    #[cfg(test)]
    if let Some(o) = mock_cmd(&cmd, args) {
        return o;
    }
    internal_exec_cmd(cmd, args, time_limit)
}

#[cfg(test)]
pub fn mock_cmd<T: AsRef<OsStr> + Debug, U: AsRef<OsStr> + Debug>(
    cmd: T,
    args: &[U],
) -> Option<Option<CommandOutput>> {
    let command = display_command(&cmd, args);
    let out = match command.as_str() {
        "bun --version" => Some(CommandOutput {
            stdout: String::from("0.1.4\n"),
            stderr: String::default(),
        }),
        "buf --version" => Some(CommandOutput {
            stdout: String::from("1.0.0"),
            stderr: String::default(),
        }),
        "cc --version" => Some(CommandOutput {
            stdout: String::from(
                "\
FreeBSD clang version 11.0.1 (git@github.com:llvm/llvm-project.git llvmorg-11.0.1-0-g43ff75f2c3fe)
Target: x86_64-unknown-freebsd13.0
Thread model: posix
InstalledDir: /usr/bin",
            ),
            stderr: String::default(),
        }),
        "gcc --version" => Some(CommandOutput {
            stdout: String::from(
                "\
cc (Debian 10.2.1-6) 10.2.1 20210110
Copyright (C) 2020 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
            ),
            stderr: String::default(),
        }),
        "clang --version" => Some(CommandOutput {
            stdout: String::from(
                "\
OpenBSD clang version 11.1.0
Target: amd64-unknown-openbsd7.0
Thread model: posix
InstalledDir: /usr/bin",
            ),
            stderr: String::default(),
        }),
        "c++ --version" => Some(CommandOutput {
            stdout: String::from(
                "\
c++ (GCC) 14.2.1 20240910
Copyright (C) 2024 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
            ),
            stderr: String::default(),
        }),
        "g++ --version" => Some(CommandOutput {
            stdout: String::from(
                "\
g++ (GCC) 14.2.1 20240910
Copyright (C) 2024 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
            ),
            stderr: String::default(),
        }),
        "clang++ --version" => Some(CommandOutput {
            stdout: String::from(
                "\
clang version 19.1.7
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin",
            ),
            stderr: String::default(),
        }),
        "cobc -version" => Some(CommandOutput {
            stdout: String::from(
                "\
cobc (GnuCOBOL) 3.1.2.0
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
Written by Keisuke Nishida, Roger While, Ron Norman, Simon Sobisch, Edward Hart
Built     Dec 24 2020 19:08:58
Packaged  Dec 23 2020 12:04:58 UTC
C version \"10.2.0\"",
            ),
            stderr: String::default(),
        }),
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
            stderr: String::default(),
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
        "fennel --version" => Some(CommandOutput {
            stdout: String::from("Fennel 1.2.1 on PUC Lua 5.4\n"),
            stderr: String::default(),
        }),
        "fossil branch current" => Some(CommandOutput {
            stdout: String::from("topic-branch"),
            stderr: String::default(),
        }),
        "fossil branch new topic-branch trunk" | "fossil update topic-branch" => {
            Some(CommandOutput {
                stdout: String::default(),
                stderr: String::default(),
            })
        }
        "fossil diff -i --numstat" => Some(CommandOutput {
            stdout: String::from(
                "\
         3          2 README.md
         3          2 TOTAL over 1 changed files",
            ),
            stderr: String::default(),
        }),
        "gleam --version" => Some(CommandOutput {
            stdout: String::from("gleam 1.0.0\n"),
            stderr: String::default(),
        }),
        "go version" => Some(CommandOutput {
            stdout: String::from("go version go1.12.1 linux/amd64\n"),
            stderr: String::default(),
        }),
        "ghc --numeric-version" => Some(CommandOutput {
            stdout: String::from("9.2.1\n"),
            stderr: String::default(),
        }),
        "helm version --short" => Some(CommandOutput {
            stdout: String::from("v3.1.1+gafe7058\n"),
            stderr: String::default(),
        }),
        s if s.ends_with("java -Xinternalversion") => Some(CommandOutput {
            stdout: String::from(
                "OpenJDK 64-Bit Server VM (13.0.2+8) for bsd-amd64 JRE (13.0.2+8), built on Feb  6 2020 02:07:52 by \"brew\" with clang 4.2.1 Compatible Apple LLVM 11.0.0 (clang-1100.0.33.17)",
            ),
            stderr: String::default(),
        }),
        "scala-cli version --scala" => Some(CommandOutput {
            stdout: String::from("3.4.1"),
            stderr: String::default(),
        }),
        "scalac -version" => Some(CommandOutput {
            stdout: String::from(
                "Scala compiler version 2.13.5 -- Copyright 2002-2020, LAMP/EPFL and Lightbend, Inc.",
            ),
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
        "lua -v" => Some(CommandOutput {
            stdout: String::from("Lua 5.4.0  Copyright (C) 1994-2020 Lua.org, PUC-Rio\n"),
            stderr: String::default(),
        }),
        "luajit -v" => Some(CommandOutput {
            stdout: String::from(
                "LuaJIT 2.0.5 -- Copyright (C) 2005-2017 Mike Pall. http://luajit.org/\n",
            ),
            stderr: String::default(),
        }),
        "mojo --version" => Some(CommandOutput {
            stdout: String::from("mojo 24.4.0 (2cb57382)\n"),
            stderr: String::default(),
        }),
        "nats context info --json" => Some(CommandOutput {
            stdout: String::from("{\"name\":\"localhost\",\"url\":\"nats://localhost:4222\"}"),
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
        "odin version" => Some(CommandOutput {
            stdout: String::from("odin version dev-2024-03:fc587c507\n"),
            stderr: String::default(),
        }),
        "opa version" => Some(CommandOutput {
            stdout: String::from(
                "Version: 0.44.0
Build Commit: e8d488f
Build Timestamp: 2022-09-07T23:50:25Z
Build Hostname: 119428673f4c
Go Version: go1.19.1
Platform: linux/amd64
WebAssembly: unavailable
",
            ),
            stderr: String::default(),
        }),
        "opam switch show --safe" => Some(CommandOutput {
            stdout: String::from("default\n"),
            stderr: String::default(),
        }),
        "typst --version" => Some(CommandOutput {
            stdout: String::from("typst 0.10 (360cc9b9)"),
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
        "pijul channel" => Some(CommandOutput {
            stdout: String::from("  main\n* tributary-48198"),
            stderr: String::default(),
        }),
        "pijul channel new tributary-48198" => Some(CommandOutput {
            stdout: String::default(),
            stderr: String::default(),
        }),
        "pijul channel switch tributary-48198" => Some(CommandOutput {
            stdout: String::from("Outputting repository ↖"),
            stderr: String::default(),
        }),
        "pixi --version" => Some(CommandOutput {
            stdout: String::from("pixi 0.33.0"),
            stderr: String::default(),
        }),
        "pulumi version" => Some(CommandOutput {
            stdout: String::from("1.2.3-ver.1631311768+e696fb6c"),
            stderr: String::default(),
        }),
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
        "quarto --version" => Some(CommandOutput {
            stdout: String::from("1.4.549\n"),
            stderr: String::default(),
        }),
        "R --version" => Some(CommandOutput {
            stdout: String::default(),
            stderr: String::from(
                r#"R version 4.1.0 (2021-05-18) -- "Camp Pontanezen"
Copyright (C) 2021 The R Foundation for Statistical Computing
Platform: x86_64-w64-mingw32/x64 (64-bit)\n

R is free software and comes with ABSOLUTELY NO WARRANTY.
You are welcome to redistribute it under the terms of the
GNU General Public License versions 2 or 3.
For more information about these matters see
https://www.gnu.org/licenses/."#,
            ),
        }),
        "raku --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Welcome to Rakudo™ v2021.12.
Implementing the Raku® Programming Language v6.d.
Built on MoarVM version 2021.12.\n",
            ),
            stderr: String::default(),
        }),
        "red --version" => Some(CommandOutput {
            stdout: String::from("0.6.4\n"),
            stderr: String::default(),
        }),
        "ruby -v" => Some(CommandOutput {
            stdout: String::from("ruby 2.5.1p57 (2018-03-29 revision 63029) [x86_64-linux-gnu]\n"),
            stderr: String::default(),
        }),
        "solc --version" => Some(CommandOutput {
            stdout: String::from(
                "solc, the solidity compiler commandline interface
Version: 0.8.16+commit.07a7930e.Linux.g++",
            ),
            stderr: String::default(),
        }),
        "solcjs --version" => Some(CommandOutput {
            stdout: String::from("0.8.15+commit.e14f2714.Emscripten.clang"),
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
        "v version" => Some(CommandOutput {
            stdout: String::from("V 0.2 30c0659"),
            stderr: String::default(),
        }),
        "xmake --version" => Some(CommandOutput {
            stdout: String::from(
                r"xmake v2.9.5+HEAD.0db4fe6, A cross-platform build utility based on Lua
Copyright (C) 2015-present Ruki Wang, tboox.org, xmake.io
                         _
    __  ___ __  __  __ _| | ______
    \ \/ / |  \/  |/ _  | |/ / __ \
     >  <  | \__/ | /_| |   <  ___/
    /_/\_\_|_|  |_|\__ \|_|\_\____|
                         by ruki, xmake.io
    👉  Manual: https://xmake.io/#/getting_started
    🙏  Donate: https://xmake.io/#/sponsor",
            ),
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
        _ => return None,
    };
    Some(out)
}

/// Many shells cannot deal with raw unprintable characters and miscompute the
/// cursor position, leading to strange visual bugs like duplicated/missing
/// chars. This function wraps ANSI escape sequences in shell-specific markers
/// to avoid these problems.
///
/// Handles:
/// - CSI sequences: `\x1b[...<final>` (terminated by final byte 0x40–0x7E)
/// - String-terminated sequences (OSC/DCS/SOS/PM/APC): `\x1b]...<term>` etc.,
///   terminated by BEL (0x07) or the two-byte String Terminator ST (ESC \)
/// - Other two-byte ESC sequences: `\x1b<X>` (single char after ESC)
pub fn wrap_colorseq_for_shell(ansi: String, shell: Shell) -> String {
    let (beg, end) = match shell {
        // \[ and \]
        Shell::Bash => ("\u{5c}\u{5b}", "\u{5c}\u{5d}"),
        // %{ and %}
        Shell::Tcsh | Shell::Zsh => ("\u{25}\u{7b}", "\u{25}\u{7d}"),
        _ => return ansi,
    };

    let mut result = String::with_capacity(ansi.len());
    let mut chars = ansi.chars().peekable();
    let mut state = EscapeState::Ground;

    while let Some(c) = chars.next() {
        match state {
            EscapeState::Ground => {
                if c == '\x1b' {
                    result.push_str(beg);
                    result.push('\x1b');
                    state = EscapeState::Escape;
                } else {
                    result.push(c);
                }
            }
            EscapeState::Escape => {
                result.push(c);
                match c {
                    // CSI: Control Sequence Introducer
                    '[' => state = EscapeState::Csi,
                    // String-terminated sequences per ECMA-48:
                    // ] = OSC, P = DCS, X = SOS, ^ = PM, _ = APC
                    ']' | 'P' | 'X' | '^' | '_' => state = EscapeState::String,
                    // All other two-byte ESC sequences (e.g. ESC M, ESC 7)
                    _ => {
                        result.push_str(end);
                        state = EscapeState::Ground;
                    }
                }
            }
            EscapeState::Csi => {
                result.push(c);
                // CSI parameters are 0x30–0x3F, intermediate bytes 0x20–0x2F.
                // A final byte in 0x40–0x7E terminates the sequence (ECMA-48 §5.4).
                if ('@'..='~').contains(&c) {
                    result.push_str(end);
                    state = EscapeState::Ground;
                }
            }
            EscapeState::String => {
                result.push(c);
                // String-terminated sequences end with BEL (0x07) or
                // ST (ESC \, i.e. \x1b followed by '\\').
                if c == '\x07' {
                    result.push_str(end);
                    state = EscapeState::Ground;
                } else if c == '\x1b' {
                    // Peek at next char: if it's '\\', this is ST
                    if let Some(&'\\') = chars.peek() {
                        chars.next();
                        result.push('\\');
                        result.push_str(end);
                        state = EscapeState::Ground;
                    }
                    // Otherwise the ESC is just part of the string payload.
                }
            }
        }
    }

    // If still inside an escape at end-of-string, close the shell wrapper
    // so the markers stay balanced even for truncated sequences.
    if !matches!(state, EscapeState::Ground) {
        result.push_str(end);
    }

    result
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EscapeState {
    /// Not inside any escape sequence
    Ground,
    /// Just saw ESC, waiting for type byte
    Escape,
    /// Inside a CSI sequence (\x1b[)
    Csi,
    /// Inside a string-terminated sequence (OSC/DCS/SOS/PM/APC)
    String,
}

fn internal_exec_cmd<T: AsRef<OsStr> + Debug, U: AsRef<OsStr> + Debug>(
    cmd: T,
    args: &[U],
    time_limit: Duration,
) -> Option<CommandOutput> {
    let mut cmd = create_command(cmd).ok()?;
    cmd.args(args);
    exec_timeout(&mut cmd, time_limit)
}

pub fn exec_timeout(cmd: &mut Command, time_limit: Duration) -> Option<CommandOutput> {
    let start = Instant::now();
    let process = match cmd.spawn() {
        Ok(process) => process,
        Err(error) => {
            log::info!("Unable to run {:?}, {:?}", cmd.get_program(), error);
            return None;
        }
    };
    match process
        .controlled_with_output()
        .time_limit(time_limit)
        .terminate_for_timeout()
        .wait()
    {
        Ok(Some(output)) => {
            let stdout_string = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    log::warn!("Unable to decode stdout: {error:?}");
                    return None;
                }
            };
            let stderr_string = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    log::warn!("Unable to decode stderr: {error:?}");
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
            log::warn!("Executing command {:?} timed out.", cmd.get_program());
            log::warn!(
                "You can set command_timeout in your config to a higher value to allow longer-running commands to keep executing."
            );
            None
        }
        Err(error) => {
            log::info!(
                "Executing command {:?} failed by: {:?}",
                cmd.get_program(),
                error
            );
            None
        }
    }
}

// Render the time into a nice human-readable string
pub fn render_time(raw_millis: u128, show_millis: bool) -> String {
    // Fast returns for zero cases to render something
    match (raw_millis, show_millis) {
        (0, true) => return "0ms".into(),
        (0..=999, false) => return "0s".into(),
        _ => (),
    }

    // Calculate a simple breakdown into days/hours/minutes/seconds/milliseconds
    let (millis, raw_seconds) = (raw_millis % 1000, raw_millis / 1000);
    let (seconds, raw_minutes) = (raw_seconds % 60, raw_seconds / 60);
    let (minutes, raw_hours) = (raw_minutes % 60, raw_minutes / 60);
    let (hours, days) = (raw_hours % 24, raw_hours / 24);

    // Calculate how long the string will be to allocate once in most cases
    let result_capacity = match raw_millis {
        1..=59 => 3,
        60..=3599 => 6,
        3600..=86399 => 9,
        _ => 12,
    } + if show_millis { 5 } else { 0 };

    let components = [(days, "d"), (hours, "h"), (minutes, "m"), (seconds, "s")];

    // Concat components ito result starting from the first non-zero one
    let result = components.iter().fold(
        String::with_capacity(result_capacity),
        |acc, (component, suffix)| match component {
            0 if acc.is_empty() => acc,
            n => acc + &n.to_string() + suffix,
        },
    );

    if show_millis {
        result + &millis.to_string() + "ms"
    } else {
        result
    }
}

/// Formats an integer into a human-readable string using SI prefixes (k, M, G, T)
pub fn humanize_int(n: u64) -> String {
    if n < 1000 {
        return n.to_string();
    }

    let n = n as f64;
    let units = ["k", "M", "G", "T", "P", "E"];
    let mut unit_idx = 0;
    let mut val = n / 1000.0;

    while val >= 1000.0 && unit_idx < units.len() - 1 {
        val /= 1000.0;
        unit_idx += 1;
    }

    if val < 10.0 {
        let s = format!("{:.1}{}", val, units[unit_idx]);
        if s.contains(".0") {
            s.replace(".0", "")
        } else {
            s
        }
    } else {
        format!("{:.0}{}", val, units[unit_idx])
    }
}

pub fn home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

const HEXTABLE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Encode a u8 slice into a hexadecimal string.
pub fn encode_to_hex(slice: &[u8]) -> String {
    // let mut j = 0;
    let mut dst = Vec::with_capacity(slice.len() * 2);
    for &v in slice {
        dst.push(HEXTABLE[(v >> 4) as usize] as u8);
        dst.push(HEXTABLE[(v & 0x0f) as usize] as u8);
    }
    String::from_utf8(dst).unwrap()
}

pub trait PathExt {
    /// Get device / volume info
    fn device_id(&self) -> Option<u64>;
}

#[cfg(windows)]
impl PathExt for Path {
    fn device_id(&self) -> Option<u64> {
        // Maybe it should use unimplemented!
        Some(42u64)
    }
}

#[cfg(not(windows))]
impl PathExt for Path {
    #[cfg(target_os = "linux")]
    fn device_id(&self) -> Option<u64> {
        use std::os::linux::fs::MetadataExt;
        Some(self.metadata().ok()?.st_dev())
    }

    #[cfg(all(unix, not(target_os = "linux")))]
    fn device_id(&self) -> Option<u64> {
        use std::os::unix::fs::MetadataExt;
        Some(self.metadata().ok()?.dev())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn render_time_test_0ms() {
        assert_eq!(render_time(0_u128, true), "0ms");
    }
    #[test]
    fn render_time_test_0s() {
        assert_eq!(render_time(0_u128, false), "0s");
    }
    #[test]
    fn render_time_test_500ms() {
        assert_eq!(render_time(500_u128, true), "500ms");
    }
    #[test]
    fn render_time_test_500ms_no_millis() {
        assert_eq!(render_time(500_u128, false), "0s");
    }
    #[test]
    fn render_time_test_10s() {
        assert_eq!(render_time(10_000_u128, true), "10s0ms");
    }
    #[test]
    fn render_time_test_90s() {
        assert_eq!(render_time(90_000_u128, true), "1m30s0ms");
    }
    #[test]
    fn render_time_test_10110s() {
        assert_eq!(render_time(10_110_000_u128, true), "2h48m30s0ms");
    }
    #[test]
    fn render_time_test_1d() {
        assert_eq!(render_time(86_400_000_u128, false), "1d0h0m0s");
    }

    #[test]
    fn test_humanize_int() {
        assert_eq!(humanize_int(0), "0");
        assert_eq!(humanize_int(999), "999");
        assert_eq!(humanize_int(1000), "1k");
        assert_eq!(humanize_int(1200), "1.2k");
        assert_eq!(humanize_int(10000), "10k");
        assert_eq!(humanize_int(100000), "100k");
        assert_eq!(humanize_int(1000000), "1M");
        assert_eq!(humanize_int(1500000), "1.5M");
    }

    #[test]
    fn exec_mocked_command() {
        let result = exec_cmd(
            "dummy_command",
            &[] as &[&OsStr],
            Duration::from_millis(DEFAULT_COMMAND_TIMEOUT_MS),
        );
        let expected = Some(CommandOutput {
            stdout: String::from("stdout ok!\n"),
            stderr: String::from("stderr ok!\n"),
        });

        assert_eq!(result, expected);
    }

    // While the exec_cmd should work on Windows some of these tests assume a Unix-like
    // environment.

    #[test]
    #[cfg(not(windows))]
    fn exec_no_output() {
        let result = internal_exec_cmd(
            "true",
            &[] as &[&OsStr],
            Duration::from_millis(DEFAULT_COMMAND_TIMEOUT_MS),
        );
        let expected = Some(CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
        });

        assert_eq!(result, expected);
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_output_stdout() {
        let result = internal_exec_cmd(
            "/bin/sh",
            &["-c", "echo hello"],
            Duration::from_millis(DEFAULT_COMMAND_TIMEOUT_MS),
        );
        let expected = Some(CommandOutput {
            stdout: String::from("hello\n"),
            stderr: String::new(),
        });

        assert_eq!(result, expected);
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_output_stderr() {
        let result = internal_exec_cmd(
            "/bin/sh",
            &["-c", "echo hello >&2"],
            Duration::from_millis(DEFAULT_COMMAND_TIMEOUT_MS),
        );
        let expected = Some(CommandOutput {
            stdout: String::new(),
            stderr: String::from("hello\n"),
        });

        assert_eq!(result, expected);
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_output_both() {
        let result = internal_exec_cmd(
            "/bin/sh",
            &["-c", "echo hello; echo world >&2"],
            Duration::from_millis(DEFAULT_COMMAND_TIMEOUT_MS),
        );
        let expected = Some(CommandOutput {
            stdout: String::from("hello\n"),
            stderr: String::from("world\n"),
        });

        assert_eq!(result, expected);
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_with_non_zero_exit_code() {
        let result = internal_exec_cmd(
            "false",
            &[] as &[&OsStr],
            Duration::from_millis(DEFAULT_COMMAND_TIMEOUT_MS),
        );
        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    #[cfg(not(windows))]
    fn exec_slow_command() {
        let result = internal_exec_cmd(
            "sleep",
            &["500"],
            Duration::from_millis(DEFAULT_COMMAND_TIMEOUT_MS),
        );
        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_color_sequence_wrappers() {
        // SGR sequence
        let sgr = "\x1b[31mhello\x1b[0m";
        // Non-SGR CSI sequence (erase line)
        let csi_k = "\x1b[49m\x1b[K";
        // DEC private CSI sequence (cursor show)
        let csi_dec = "\x1b[?25h";
        // Two-char escape
        let two_char = "\x1bM";
        // No escape sequences
        let plain = "herpaderp";
        // Empty string
        let empty = "";

        // OSC 8 hyperlink (ST-terminated)
        let osc8 = "\x1b]8;;https://example.com\x1b\\link\x1b]8;;\x1b\\";
        // OSC 133 semantic prompt (BEL-terminated)
        let osc133 = "\x1b]133;A\x07";
        // OSC window title (BEL-terminated)
        let title = "\x1b]0;my title\x07";

        // DCS sequence (ST-terminated)
        let dcs = "\x1bP1$p\x1b\\";

        // Truncated CSI at end of string
        let truncated_csi = "\x1b[31";
        // Truncated string sequence at end of string
        let truncated_osc = "\x1b]8;;url";

        // --- Zsh wrapping ---

        // CSI sequences
        assert_eq!(
            wrap_colorseq_for_shell(sgr.to_string(), Shell::Zsh),
            "%{\x1b[31m%}hello%{\x1b[0m%}"
        );
        assert_eq!(
            wrap_colorseq_for_shell(csi_k.to_string(), Shell::Zsh),
            "%{\x1b[49m%}%{\x1b[K%}"
        );
        assert_eq!(
            wrap_colorseq_for_shell(csi_dec.to_string(), Shell::Zsh),
            "%{\x1b[?25h%}"
        );
        assert_eq!(
            wrap_colorseq_for_shell(two_char.to_string(), Shell::Zsh),
            "%{\x1bM%}"
        );

        // OSC sequences
        assert_eq!(
            wrap_colorseq_for_shell(osc8.to_string(), Shell::Zsh),
            "%{\x1b]8;;https://example.com\x1b\\\\%}link%{\x1b]8;;\x1b\\\\%}"
        );
        assert_eq!(
            wrap_colorseq_for_shell(osc133.to_string(), Shell::Zsh),
            "%{\x1b]133;A\x07%}"
        );
        assert_eq!(
            wrap_colorseq_for_shell(title.to_string(), Shell::Zsh),
            "%{\x1b]0;my title\x07%}"
        );

        // DCS
        assert_eq!(
            wrap_colorseq_for_shell(dcs.to_string(), Shell::Zsh),
            "%{\x1bP1$p\x1b\\\\%}"
        );

        // --- Bash wrapping ---

        assert_eq!(
            wrap_colorseq_for_shell(sgr.to_string(), Shell::Bash),
            "\\[\x1b[31m\\]hello\\[\x1b[0m\\]"
        );
        assert_eq!(
            wrap_colorseq_for_shell(csi_k.to_string(), Shell::Bash),
            "\\[\x1b[49m\\]\\[\x1b[K\\]"
        );
        assert_eq!(
            wrap_colorseq_for_shell(csi_dec.to_string(), Shell::Bash),
            "\\[\x1b[?25h\\]"
        );
        assert_eq!(
            wrap_colorseq_for_shell(two_char.to_string(), Shell::Bash),
            "\\[\x1bM\\]"
        );

        assert_eq!(
            wrap_colorseq_for_shell(osc8.to_string(), Shell::Bash),
            "\\[\x1b]8;;https://example.com\x1b\\\\\\]link\\[\x1b]8;;\x1b\\\\\\]"
        );
        assert_eq!(
            wrap_colorseq_for_shell(osc133.to_string(), Shell::Bash),
            "\\[\x1b]133;A\x07\\]"
        );
        assert_eq!(
            wrap_colorseq_for_shell(title.to_string(), Shell::Bash),
            "\\[\x1b]0;my title\x07\\]"
        );

        assert_eq!(
            wrap_colorseq_for_shell(dcs.to_string(), Shell::Bash),
            "\\[\x1bP1$p\x1b\\\\\\]"
        );

        // --- Plain / empty ---

        assert_eq!(
            wrap_colorseq_for_shell(plain.to_string(), Shell::Zsh),
            "herpaderp"
        );
        assert_eq!(wrap_colorseq_for_shell(empty.to_string(), Shell::Zsh), "");
        assert_eq!(
            wrap_colorseq_for_shell(plain.to_string(), Shell::Bash),
            "herpaderp"
        );
        assert_eq!(wrap_colorseq_for_shell(empty.to_string(), Shell::Bash), "");

        // --- Truncated sequences (wrapper should still be balanced) ---

        let wrapped = wrap_colorseq_for_shell(truncated_csi.to_string(), Shell::Bash);
        assert!(wrapped.starts_with("\\["));
        assert!(wrapped.ends_with("\\]"));

        let wrapped = wrap_colorseq_for_shell(truncated_osc.to_string(), Shell::Zsh);
        assert!(wrapped.starts_with("%{"));
        assert!(wrapped.ends_with("%}"));
    }

    #[test]
    fn test_get_command_string_output() {
        let case1 = CommandOutput {
            stdout: String::from("stdout"),
            stderr: String::from("stderr"),
        };
        assert_eq!(get_command_string_output(case1), "stdout");
        let case2 = CommandOutput {
            stdout: String::new(),
            stderr: String::from("stderr"),
        };
        assert_eq!(get_command_string_output(case2), "stderr");
    }

    #[test]
    fn sha1_hex() {
        assert_eq!(
            encode_to_hex(&[8, 13, 9, 189, 129, 94]),
            "080d09bd815e".to_string()
        );
    }

    #[test]
    fn test_write_file_atomic() -> Result<()> {
        // Create a temporary file for testing
        let tmp_dir = tempdir()?;
        let path = tmp_dir.path().join("test_config.toml");

        let expected = "test data";
        write_file_atomic(&path, expected, false).unwrap();

        let actual_data = read_file(&path)?;
        assert_eq!(actual_data, expected);

        tmp_dir.close()
    }

    #[test]
    fn test_write_file_atomic_already_exists() -> Result<()> {
        let tmp_dir = tempdir()?;
        let tmp_file_path = tmp_dir.path().join("test_config.toml");

        write_file(&tmp_file_path, "existing data")?;

        let err = write_file_atomic(&tmp_file_path, "should not contain this", false).unwrap_err();
        assert!(err.contains("--force"));

        let actual_data = read_file(&tmp_file_path)?;
        assert_eq!(actual_data, "existing data");

        tmp_dir.close()
    }

    #[test]
    fn test_write_file_atomic_overwrite() -> Result<()> {
        let tmp_dir = tempdir()?;
        let path = tmp_dir.path().join("test_config.toml");

        write_file(&path, "existing data")?;

        let expected = "test data";
        write_file_atomic(&path, expected, true).unwrap();

        let actual = read_file(&path)?;
        assert_eq!(actual, expected);

        tmp_dir.close()
    }
}
