use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, io};

/* We use a two-phase init here: the first phase gives a simple command to the
shell. This command evaluates a more complicated script using `source` and
process substitution.

Directly using `eval` on a shell script causes it to be evaluated in
a single line, which sucks because things like comments will comment out the
rest of the script, and you have to spam semicolons everywhere. By using
source and process substitutions, we make it possible to comment and debug
the init scripts.

In the future, this may be changed to just directly evaluating the initscript
using whatever mechanism is available in the host shell--this two-phase solution
has been developed as a compatibility measure with `eval $(starship init X)`
*/

struct StarshipPath {
    native_path: PathBuf,
}
impl StarshipPath {
    fn init() -> io::Result<Self> {
        Ok(Self {
            native_path: env::current_exe()?,
        })
    }
    fn str_path(&self) -> io::Result<&str> {
        let current_exe = self
            .native_path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "can't convert to str"))?;
        Ok(current_exe)
    }
    fn sprint(&self) -> io::Result<String> {
        self.str_path().map(|s| s.replace("\"", "\"'\"'\""))
    }
    fn sprint_posix(&self) -> io::Result<String> {
        // On non-Windows platform, return directly.
        if cfg!(not(target_os = "windows")) {
            return self.sprint();
        }
        let str_path = self.str_path()?;
        let res = std::process::Command::new("cygpath.exe")
            .arg(str_path)
            .output();
        let output = match res {
            Ok(output) => output,
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    log::warn!("Failed to convert \"{}\" to unix path:\n{:?}", str_path, e);
                }
                // Failed to execute cygpath.exe means there're not inside cygwin evironment,return directly.
                return self.sprint();
            }
        };
        let res = String::from_utf8(output.stdout);
        let posix_path = match res {
            Ok(ref cygpath_path) if output.status.success() => cygpath_path.trim(),
            Ok(_) => {
                log::warn!(
                    "Failed to convert \"{}\" to unix path:\n{}",
                    str_path,
                    String::from_utf8_lossy(&output.stderr),
                );
                str_path
            }
            Err(e) => {
                log::warn!("Failed to convert \"{}\" to unix path:\n{}", str_path, e);
                str_path
            }
        };
        Ok(posix_path.replace("\"", "\"'\"'\""))
    }
}

/* This prints the setup stub, the short piece of code which sets up the main
init code. The stub produces the main init script, then evaluates it with
`source` and process substitution */
pub fn init_stub(shell_name: &str) -> io::Result<()> {
    log::debug!("Shell name: {}", shell_name);

    let shell_basename = Path::new(shell_name).file_stem().and_then(OsStr::to_str);

    let starship = StarshipPath::init()?;

    let setup_stub = match shell_basename {
        Some("bash") => {
            /*
             * The standard bash bootstrap is:
             *      `source <(starship init bash --print-full-init)`
             *
             * Unfortunately there is an issue with bash 3.2 (the MacOS
             * default) which prevents this from working. It does not support
             * `source` with process substitution.
             *
             * There are more details here: https://stackoverflow.com/a/32596626
             *
             * The workaround for MacOS is to use the `/dev/stdin` trick you
             * see below. However, there are some systems with emulated POSIX
             * environments which do not support `/dev/stdin`. For example,
             * `Git Bash` within `Git for Windows and `Termux` on Android.
             *
             * Fortunately, these apps ship with recent-ish versions of bash.
             * Git Bash is currently shipping bash 4.4 and Termux is shipping
             * bash 5.0.
             *
             * Some testing has suggested that bash 4.0 is also incompatible
             * with the standard bootstrap, whereas bash 4.1 appears to be
             * consistently compatible.
             *
             * The upshot of all of this, is that we will use the standard
             * bootstrap whenever the bash version is 4.1 or higher. Otherwise,
             * we fall back to the `/dev/stdin` solution.
             *
             * More background can be found in these pull requests:
             * https://github.com/starship/starship/pull/241
             * https://github.com/starship/starship/pull/278
             */
            let script = {
                format!(
                    r#"if [ "${{BASH_VERSINFO[0]}}" -gt 4 ] || ([ "${{BASH_VERSINFO[0]}}" -eq 4 ] && [ "${{BASH_VERSINFO[1]}}" -ge 1 ])
then
source <("{0}" init bash --print-full-init)
else
source /dev/stdin <<<"$("{0}" init bash --print-full-init)"
fi"#,
                    starship.sprint_posix()?
                )
            };

            Some(script)
        }
        Some("zsh") => {
            let script = format!(
                "source <(\"{}\" init zsh --print-full-init)",
                starship.sprint_posix()?
            );
            Some(script)
        }
        Some("fish") => {
            // Fish does process substitution with pipes and psub instead of bash syntax
            let script = format!(
                "source (\"{}\" init fish --print-full-init | psub)",
                starship.sprint_posix()?
            );
            Some(script)
        }
        Some("powershell") => {
            // Explanation of syntax:
            // &: Explicitly tells powershell to execute path with starship executable.
            //
            // @: multi-line stdout is returned as an array, but a single line or no lines
            //    are returned as-is. @ ensures it's always an array.
            //
            // -join "`n": Joins the stdout array together as a string with newlines.
            //             Powershell escapes with ` instead of \ thus `n translates to a newline.
            let script = format!(
                "Invoke-Expression (@(&\"{}\" init powershell --print-full-init) -join \"`n\")",
                starship.sprint()?
            );
            Some(script)
        }
        Some("ion") => {
            let script = format!("eval $({} init ion --print-full-init)", starship.sprint()?);
            Some(script)
        }
        Some("elvish") => {
            let script = format!(
                "eval ({} init elvish --print-full-init | slurp)",
                starship.sprint_posix()?
            );
            Some(script)
        }
        None => {
            println!(
                "Invalid shell name provided: {}\\n\
                 If this issue persists, please open an \
                 issue in the starship repo: \\n\
                 https://github.com/starship/starship/issues/new\\n\"",
                shell_name
            );
            None
        }
        Some(shell_basename) => {
            println!(
                "printf \"\\n{0} is not yet supported by starship.\\n\
                 For the time being, we support bash, zsh, fish, and ion.\\n\
                 Please open an issue in the starship repo if you would like to \
                 see support for {0}:\\nhttps://github.com/starship/starship/issues/new\"\\n\\n",
                shell_basename
            );
            None
        }
    };
    if let Some(script) = setup_stub {
        print!("{}", script);
    };
    Ok(())
}

/* This function (called when `--print-full-init` is passed to `starship init`)
prints out the main initialization script */
pub fn init_main(shell_name: &str) -> io::Result<()> {
    let starship_path = StarshipPath::init()?;

    match shell_name {
        "bash" => print_script(BASH_INIT, &starship_path.sprint_posix()?),
        "zsh" => print_script(ZSH_INIT, &starship_path.sprint_posix()?),
        "fish" => print_script(FISH_INIT, &starship_path.sprint_posix()?),
        "powershell" => print_script(PWSH_INIT, &starship_path.sprint()?),
        "ion" => print_script(ION_INIT, &starship_path.sprint()?),
        "elvish" => print_script(ELVISH_INIT, &starship_path.sprint_posix()?),
        _ => {
            println!(
                "printf \"Shell name detection failed on phase two init.\\n\
                 This probably indicates a bug within starship: please open\\n\
                 an issue at https://github.com/starship/starship/issues/new\\n\""
            );
        }
    }
    Ok(())
}

fn print_script(script: &str, path: &str) {
    let starship_path_string = format!("\"{}\"", path);
    let script = script.replace("::STARSHIP::", &starship_path_string);
    print!("{}", script);
}

/* GENERAL INIT SCRIPT NOTES

Each init script will be passed as-is. Global notes for init scripts are in this
comment, with additional per-script comments in the strings themselves.

JOBS: The argument to `--jobs` is quoted because MacOS's `wc` leaves whitespace
in the output. We pass it to starship and do the whitespace removal in Rust,
to avoid the cost of an additional shell fork every shell draw.

Note that the init scripts are not in their final form--they are processed by
`starship init` prior to emitting the final form. In this processing, some tokens
are replaced, e.g. `::STARSHIP::` is replaced by the full path to the
starship binary.
*/

const BASH_INIT: &str = include_str!("starship.bash");

const ZSH_INIT: &str = include_str!("starship.zsh");

const FISH_INIT: &str = include_str!("starship.fish");

const PWSH_INIT: &str = include_str!("starship.ps1");

const ION_INIT: &str = include_str!("starship.ion");

const ELVISH_INIT: &str = include_str!("starship.elv");
