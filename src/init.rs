use std::ffi::OsStr;
use std::path::Path;

/* We need to send execution time to the prompt for the cmd_duration module. For fish,
this is fairly straightforward. For bash and zsh, we'll need to use several
shell utilities to get the time, as well as render the prompt */

pub fn init(shell_name: &str) {
    log::debug!("Shell name: {}", shell_name);

    let shell_basename = Path::new(shell_name).file_stem().and_then(OsStr::to_str);

    let setup_script = match shell_basename {
        Some("bash") => {
            let script = BASH_INIT;
            Some(script)
        }
        Some("zsh") => {
            let script = ZSH_INIT;
            Some(script)
        }
        Some("fish") => {
            let script = FISH_INIT;
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
        _ => {
            /* Calling unwrap() here is fine because the None case will have
            already matched on the previous arm */
            println!(
                "printf \"\\n{0} is not yet supported by starship.\\n\
                 For the time being, we support bash, zsh, and fish.\\n\
                 Please open an issue in the starship repo if you would like to \
                 see support for {0}:\\nhttps://github.com/starship/starship/issues/new\"\\n\\n",
                shell_basename.unwrap()
            );
            None
        }
    };

    if let Some(script) = setup_script {
        print!("{}", script);
    }
}

/*
 For bash: we need to manually hook functions ourself: PROMPT_COMMAND will exec
   right before the prompt is drawn, and any function trapped by DEBUG will exec
   before a command is run.

   There is a preexec/precmd framework for bash out there: if we find the
   appropriate variables set, assume we are using that framework:
   https://github.com/rcaloras/bash-preexec

   Bash quirk: DEBUG is triggered whenever a command is executed, even if that
   command is part of a pipeline. To avoid only timing the last part of a pipeline,
   we only start the timer if no timer has been started since the last prompt draw,
   tracked by the variable PREEXEC_READY. Similarly, only draw timing info if
   STARSHIP_START_TIME is defined, in case preexec was interrupted.

   Finally, to work around existing DEBUG traps in the absence of a preexec-like,
   we parse out the name of the old DEBUG hook, then make a new function which
   calls both that function and our starship hooks. We don't do this for
   PROMPT_COMMAND because that would probably result in two prompts.

   We need to quote the output of `$(jobs -p | wc -l)` since MacOS `wc` leaves
   giant spaces in front of the number (e.g. "    3"), which messes up the
   word-splitting. Instead, quote the whole thing, then let Rust do the whitespace
   trimming within the jobs module
*/

/*
Note to programmers: this and the zsh init will be evaluated on a single line.
Use semicolons, avoid comments, and generally think like all newlines will be
deleted.
*/
const BASH_INIT: &str = r##"
starship_preexec() {
    if [ "$PREEXEC_READY" = "true" ]; then
        PREEXEC_READY=false;
        STARSHIP_START_TIME=$(date +%s);
    fi
};
starship_precmd() {
    STATUS=$?;
    "${starship_precmd_user_func-:}";
    if [[ $STARSHIP_START_TIME ]]; then
        STARSHIP_END_TIME=$(date +%s);
        STARSHIP_DURATION=$((STARSHIP_END_TIME - STARSHIP_START_TIME));
        PS1="$(starship prompt --status=$STATUS --jobs="$(jobs -p | wc -l)" --cmd-duration=$STARSHIP_DURATION)";
        unset STARSHIP_START_TIME;
    else
        PS1="$(starship prompt --status=$STATUS --jobs="$(jobs -p | wc -l)")";
    fi;
    PREEXEC_READY=true;
};
if [[ $preexec_functions ]]; then
    preexec_functions+=(starship_preexec);
    precmd_functions+=(starship_precmd);
    STARSHIP_START_TIME=$(date +%s);
else
    dbg_trap="$(trap -p DEBUG | cut -d' ' -f3 | tr -d \')";
    if [[ -z "$dbg_trap" ]]; then
        trap starship_preexec DEBUG;
    elif [[ "$dbg_trap" != "starship_preexec" && "$dbg_trap" != "starship_preexec_all" ]]; then
        function starship_preexec_all(){
            $dbg_trap; starship_preexec;
        };
        trap starship_preexec_all DEBUG;
    fi;
    PROMPT_COMMAND=starship_precmd;
    STARSHIP_START_TIME=$(date +%s);
fi;
"##;

/* For zsh: preexec_functions and precmd_functions provide preexec/precmd in a
   way that lets us avoid clobbering them.

   Zsh quirk: preexec() is only fired if a command is actually run (unlike in
   bash, where spamming empty commands still triggers DEBUG). This means a user
   spamming ENTER at an empty command line will see increasing runtime (since
   preexec never actually fires to reset the start time).

   To fix this, only pass the time if STARSHIP_START_TIME is defined, and unset
   it after passing the time, so that we only measure actual commands.

   We need to quote the output of the jobs command for the same reason as
   bash.
*/

const ZSH_INIT: &str = r##"
starship_precmd() {
    STATUS=$?;
    if [[ $STARSHIP_START_TIME ]]; then
        STARSHIP_END_TIME="$(date +%s)";
        STARSHIP_DURATION=$((STARSHIP_END_TIME - STARSHIP_START_TIME));
        PROMPT="$(starship prompt --status=$STATUS --cmd-duration=$STARSHIP_DURATION --jobs="$(jobs | wc -l)")";
        unset STARSHIP_START_TIME;
    else
        PROMPT="$(starship prompt --status=$STATUS --jobs="$(jobs | wc -l)")";
    fi
};
starship_preexec(){
    STARSHIP_START_TIME="$(date +%s)"
};
if [[ ${precmd_functions[(ie)starship_precmd]} -gt ${#precmd_functions} ]]; then
    precmd_functions+=(starship_precmd);
fi;
if [[ ${preexec_functions[(ie)starship_preexec]} -gt ${#preexec_functions} ]]; then
    preexec_functions+=(starship_preexec);
fi;
STARSHIP_START_TIME="$(date +%s)";
"##;

/* Fish setup is simple because they give us CMD_DURATION. Just account for name
changes between 2.7/3.0 and do some math to convert ms->s and we can use it */
const FISH_INIT: &str = r##"
function fish_prompt;
    set -l exit_code $status;
    set -l CMD_DURATION "$CMD_DURATION$cmd_duration";
    set -l starship_duration (math --scale=0 "$CMD_DURATION / 1000");
    starship prompt --status=$exit_code --cmd-duration=$starship_duration --jobs=(count (jobs -p));
end;
"##;
