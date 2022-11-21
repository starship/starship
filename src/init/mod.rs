use crate::utils::create_command;
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

    /// Returns POSIX quoted path to starship binary
    fn sprint(&self) -> io::Result<String> {
        self.str_path().map(|p| shell_words::quote(p).into_owned())
    }

    /// `PowerShell` specific path escaping
    fn sprint_pwsh(&self) -> io::Result<String> {
        self.str_path()
            .map(|s| s.replace('\'', "''"))
            .map(|s| format!("'{}'", s))
    }
    /// Command Shell specific path escaping
    fn sprint_cmdexe(&self) -> io::Result<String> {
        self.str_path().map(|s| format!("\"{}\"", s))
    }
    fn sprint_posix(&self) -> io::Result<String> {
        // On non-Windows platform, return directly.
        if cfg!(not(target_os = "windows")) {
            return self.sprint();
        }
        let str_path = self.str_path()?;
        let res = create_command("cygpath").and_then(|mut cmd| cmd.arg(str_path).output());
        let output = match res {
            Ok(output) => output,
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    log::warn!("Failed to convert \"{}\" to unix path:\n{:?}", str_path, e);
                }
                // Failed to execute cygpath.exe means there're not inside cygwin environment,return directly.
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
        Ok(shell_words::quote(posix_path).into_owned())
    }
}

/* This prints the setup stub, the short piece of code which sets up the main
init code. The stub produces the main init script, then evaluates it with
`source` and process substitution */
pub fn init_stub(shell_name: &str) -> io::Result<()> {
    log::debug!("Shell name: {}", shell_name);

    let shell_basename = Path::new(shell_name)
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or(shell_name);

    let starship = StarshipPath::init()?;

    match shell_basename {
        "bash" => print!(
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
            r#"
__main() {{
    local major="${{BASH_VERSINFO[0]}}"
    local minor="${{BASH_VERSINFO[1]}}"

    if ((major > 4)) || {{ ((major == 4)) && ((minor >= 1)); }}; then
        source <({0} init bash --print-full-init)
    else
        source /dev/stdin <<<"$({0} init bash --print-full-init)"
    fi
}}
__main
unset -f __main
            "#,
            starship.sprint_posix()?
        ),
        "zsh" => print!(
            r#"source <({} init zsh --print-full-init)"#,
            starship.sprint_posix()?
        ),
        "fish" => print!(
            // Fish does process substitution with pipes and psub instead of bash syntax
            r#"source ({} init fish --print-full-init | psub)"#,
            starship.sprint_posix()?
        ),
        "powershell" => print!(
            r#"Invoke-Expression (& {} init powershell --print-full-init | Out-String)"#,
            starship.sprint_pwsh()?
        ),
        "ion" => print!("eval $({} init ion --print-full-init)", starship.sprint()?),
        "elvish" => print!(
            r#"eval ({} init elvish --print-full-init | slurp)"#,
            starship.sprint_posix()?
        ),
        "tcsh" => print!(
            r#"eval `({} init tcsh --print-full-init)`"#,
            starship.sprint_posix()?
        ),
        "nu" => print_script(NU_INIT, &StarshipPath::init()?.sprint()?),
        "xonsh" => print!(
            r#"execx($({} init xonsh --print-full-init))"#,
            starship.sprint_posix()?
        ),
        "cmd" => print_script(CMDEXE_INIT, &StarshipPath::init()?.sprint_cmdexe()?),
        _ => {
            eprint!(
                "{0} is not yet supported by starship.\n\
                 For the time being, we support the following shells:\n\
                 * bash\n\
                 * elvish\n\
                 * fish\n\
                 * ion\n\
                 * powershell\n\
                 * tcsh\n\
                 * zsh\n\
                 * nu\n\
                 * xonsh\n\
                 * cmd\n\
                 \n\
                 Please open an issue in the starship repo if you would like to \
                 see support for {0}:\n\
                 https://github.com/starship/starship/issues/new\n",
                shell_basename
            )
        }
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
        "powershell" => print_script(PWSH_INIT, &starship_path.sprint_pwsh()?),
        "ion" => print_script(ION_INIT, &starship_path.sprint()?),
        "elvish" => print_script(ELVISH_INIT, &starship_path.sprint_posix()?),
        "tcsh" => print_script(TCSH_INIT, &starship_path.sprint_posix()?),
        "xonsh" => print_script(XONSH_INIT, &starship_path.sprint_posix()?),
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
    let script = script.replace("::STARSHIP::", path);
    println!("{}", script);
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

const TCSH_INIT: &str = include_str!("starship.tcsh");

const NU_INIT: &str = include_str!("starship.nu");

const XONSH_INIT: &str = include_str!("starship.xsh");

const CMDEXE_INIT: &str = include_str!("starship.lua");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escape_pwsh() -> io::Result<()> {
        let starship_path = StarshipPath {
            native_path: PathBuf::from(r"C:\starship.exe"),
        };
        assert_eq!(starship_path.sprint_pwsh()?, r"'C:\starship.exe'");
        Ok(())
    }

    #[test]
    fn escape_tick_pwsh() -> io::Result<()> {
        let starship_path = StarshipPath {
            native_path: PathBuf::from(r"C:\'starship.exe"),
        };
        assert_eq!(starship_path.sprint_pwsh()?, r"'C:\''starship.exe'");
        Ok(())
    }

    #[test]
    fn escape_cmdexe() -> io::Result<()> {
        let starship_path = StarshipPath {
            native_path: PathBuf::from(r"C:\starship.exe"),
        };
        assert_eq!(starship_path.sprint_cmdexe()?, r#""C:\starship.exe""#);
        Ok(())
    }

    #[test]
    fn escape_space_cmdexe() -> io::Result<()> {
        let starship_path = StarshipPath {
            native_path: PathBuf::from(r"C:\Cool Tools\starship.exe"),
        };
        assert_eq!(
            starship_path.sprint_cmdexe()?,
            r#""C:\Cool Tools\starship.exe""#
        );
        Ok(())
    }
}
