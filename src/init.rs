use std::ffi::OsStr;
use std::path::Path;
use std::{env, io};

/* We use a two-phase init here: the first phase gives a simple command to the
shell. This command evaluates a more complicated script using `source` and
process substitution.

Directly using `eval` on a shell script causes it to be evaluated in
a single line, which sucks because things like comments will comment out the
rest of the script, and you have to spam semicolons everywhere. By using
source and process substitutions, we make it possible to comment and debug
the init scripts. */

fn path_to_starship() -> io::Result<String> {
    let current_exe = env::current_exe()?
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "can't convert to str"))?
        .to_string();
    Ok(current_exe)
}

/* This prints the setup stub, the short piece of code which sets up the main
init code. The stub produces the main init script, then evaluates it with
`source` and process substitution */
pub fn init_stub(shell_name: &str) -> io::Result<()> {
    log::debug!("Shell name: {}", shell_name);

    let shell_basename = Path::new(shell_name).file_stem().and_then(OsStr::to_str);

    let starship = path_to_starship()?.replace("\"", "\"'\"'\"");

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
source <("{}" init bash --print-full-init)
else
source /dev/stdin <<<"$("{}" init bash --print-full-init)"
fi"#,
                    starship, starship
                )
            };

            Some(script)
        }
        Some("zsh") => {
            let script = format!("source <(\"{}\" init zsh --print-full-init)", starship);
            Some(script)
        }
        Some("fish") => {
            // Fish does process substitution with pipes and psub instead of bash syntax
            let script = format!(
                "source (\"{}\" init fish --print-full-init | psub)",
                starship
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
                 For the time being, we support bash, zsh, and fish.\\n\
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
    let starship_path = path_to_starship()?.replace("\"", "\"'\"'\"");

    let setup_script = match shell_name {
        "bash" => Some(BASH_INIT.replace("## STARSHIP ##", &format!("\"{}\"", starship_path))),
        "zsh" => Some(ZSH_INIT.replace("## STARSHIP ##", &format!("\"{}\"", starship_path))),
        "fish" => Some(FISH_INIT.replace("## STARSHIP ##", &format!("\"{}\"", starship_path))),
        _ => {
            println!(
                "printf \"Shell name detection failed on phase two init.\\n\
                 This probably indicates a bug within starship: please open\\n\
                 an issue at https://github.com/starship/starship/issues/new\\n\""
            );
            None
        }
    };
    if let Some(script) = setup_script {
        print!("{}", script);
    };
    Ok(())
}

/* GENERAL INIT SCRIPT NOTES

Each init script will be passed as-is. Global notes for init scripts are in this
comment, with additional per-script comments in the strings themselves.

JOBS: The argument to `--jobs` is quoted because MacOS's `wc` leaves whitespace
in the output. We pass it to starship and do the whitespace removal in Rust,
to avoid the cost of an additional shell fork every shell draw.
*/

/* BASH INIT SCRIPT

We use PROMPT_COMMAND and the DEBUG trap to generate timing information. We try
to avoid clobbering what we can, and try to give the user ways around our
clobbers, if it's unavoidable.

A bash quirk is that the DEBUG trap is fired every time a command runs, even
if it's later on in the pipeline. If uncorrected, this could cause bad timing
data for commands like `slow | slow | fast`, since the timer starts at the start
of the "fast" command.

To solve this, we set a flag `PREEXEC_READY` when the prompt is drawn, and only
start the timer if this flag is present. That way, timing is for the entire command,
and not just a portion of it
*/

const BASH_INIT: &str = r##"
# Will be run before *every* command (even ones in pipes!)
starship_preexec() {
    # Avoid restarting the timer for commands in the same pipeline
    if [ "$PREEXEC_READY" = "true" ]; then
        PREEXEC_READY=false
        STARSHIP_START_TIME=$(date +%s)
    fi
}
# Will be run before the prompt is drawn
starship_precmd() {
    # Save the status, because commands in this pipeline will change $?
    STATUS=$?

    # Run the bash precmd function, if it's set. If not set, evaluates to no-op
    "${starship_precmd_user_func-:}"

    # Prepare the timer data, if needed.
    if [[ $STARSHIP_START_TIME ]]; then
        STARSHIP_END_TIME=$(date +%s)
        STARSHIP_DURATION=$((STARSHIP_END_TIME - STARSHIP_START_TIME))
        PS1="$(## STARSHIP ## prompt --status=$STATUS --jobs="$(jobs -p | wc -l)" --cmd-duration=$STARSHIP_DURATION)"
        unset STARSHIP_START_TIME
    else
        PS1="$(## STARSHIP ## prompt --status=$STATUS --jobs="$(jobs -p | wc -l)")"
    fi
    PREEXEC_READY=true;  # Signal that we can safely restart the timer
}

# If the user appears to be using https://github.com/rcaloras/bash-preexec,
# then hook our functions into their framework.
if [[ $preexec_functions ]]; then
    preexec_functions+=(starship_preexec)
    precmd_functions+=(starship_precmd)
else
# We want to avoid destroying an existing DEBUG hook. If we detect one, create
# a new function that runs both the existing function AND our function, then
# re-trap DEBUG to use this new function. This prevents a trap clobber.
    dbg_trap="$(trap -p DEBUG | cut -d' ' -f3 | tr -d \')"
    if [[ -z "$dbg_trap" ]]; then
        trap starship_preexec DEBUG
    elif [[ "$dbg_trap" != "starship_preexec" && "$dbg_trap" != "starship_preexec_all" ]]; then
        function starship_preexec_all(){
            $dbg_trap; starship_preexec
        }
        trap starship_preexec_all DEBUG
    fi

    # Finally, prepare the precmd function and set up the start time.
    PROMPT_COMMAND="starship_precmd;$PROMPT_COMMAND";
fi

# Set up the start time and STARSHIP_SHELL, which controls shell-specific sequences
STARSHIP_START_TIME=$(date +%s)
export STARSHIP_SHELL="bash"
"##;

/* ZSH INIT SCRIPT

ZSH has a quirk where `preexec` is only run if a command is actually run (i.e
pressing ENTER at an empty command line will not cause preexec to fire). This
can cause timing issues, as a user who presses "ENTER" without running a command
will see the time to the start of the last command, which may be very large.

To fix this, we create STARSHIP_START_TIME upon preexec() firing, and destroy it
after drawing the prompt. This ensures that the timing for one command is only
ever drawn once (for the prompt immediately after it is run).
*/

const ZSH_INIT: &str = r##"
zmodload zsh/parameter  # Needed to access jobstates variable for NUM_JOBS

# Will be run before every prompt draw
starship_precmd() {
    # Save the status, because commands in this pipeline will change $?
    STATUS=$?

    # Use length of jobstates array as number of jobs. Expansion fails inside
    # quotes so we set it here and then use the value later on.
    NUM_JOBS=$#jobstates  
    # Compute cmd_duration, if we have a time to consume
    if [[ ! -z "${STARSHIP_START_TIME+1}" ]]; then
        STARSHIP_END_TIME="$(date +%s)"
        STARSHIP_DURATION=$((STARSHIP_END_TIME - STARSHIP_START_TIME))
        PROMPT="$(## STARSHIP ## prompt --status=$STATUS --cmd-duration=$STARSHIP_DURATION --jobs="$NUM_JOBS")"
        unset STARSHIP_START_TIME
    else
        PROMPT="$(## STARSHIP ## prompt --status=$STATUS --jobs="$NUM_JOBS")"
    fi
}
starship_preexec(){
    STARSHIP_START_TIME="$(date +%s)"
}

# If precmd/preexec arrays are not already set, set them. If we don't do this,
# the code to detect whether starship_precmd is already in precmd_functions will
# fail because the array doesn't exist (and same for starship_preexec)
[[ -z "${precmd_functions+1}" ]] && precmd_functions=()
[[ -z "${preexec_functions+1}" ]] && preexec_functions=()

# If starship precmd/preexec functions are already hooked, don't double-hook them
# to avoid unnecessary performance degradation in nested shells
if [[ ${precmd_functions[(ie)starship_precmd]} -gt ${#precmd_functions} ]]; then
    precmd_functions+=(starship_precmd)
fi
if [[ ${preexec_functions[(ie)starship_preexec]} -gt ${#preexec_functions} ]]; then
    preexec_functions+=(starship_preexec)
fi
# Set up a function to redraw the prompt if the user switches vi modes
function zle-keymap-select
{
    PROMPT=$(## STARSHIP ## prompt --keymap=$KEYMAP --jobs="$(jobs | wc -l)")
    zle reset-prompt
}

STARSHIP_START_TIME="$(date +%s)"
zle -N zle-keymap-select
export STARSHIP_SHELL="zsh"
"##;

const FISH_INIT: &str = r##"
function fish_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set keymap "$fish_bind_mode"
        case '*'
            set keymap insert
    end
    set -l exit_code $status
    # Account for changes in variable name between v2.7 and v3.0
    set -l CMD_DURATION "$CMD_DURATION$cmd_duration"
    set -l starship_duration (math --scale=0 "$CMD_DURATION / 1000")
    ## STARSHIP ## prompt --status=$exit_code --keymap=$keymap --cmd-duration=$starship_duration --jobs=(count (jobs -p))
end
function fish_mode_prompt; end
export STARSHIP_SHELL="fish"
"##;
