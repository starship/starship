use std::ffi::OsStr;
use std::path::Path;

/* We need to send execution time to the prompt for the timer module. For fish,
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

/* For bash: we need to manually hook functions ourself: PROMPT_COMMAND will exec
   right before the prompt is drawn, and any function trapped by DEBUG will exec
   before a command is run. There is no way to avoid clobbering these functions,
   so we will not hook them if they are already defined.

   There is a preexec/precmd framework for bash out there: if we find the
   appropriate variables set, assume we are using that framework:
   https://github.com/rcaloras/bash-preexec

   Bash quirk: DEBUG is triggered whenever a command is executed, even if that
   command is part of a pipeline. To avoid only timing the last part of a pipeline,
   we only start the timer if no timer has been started since the last prompt draw,
   tracked by the variable PREEXEC_READY. Similarly, only draw timing info if
   STARSHIP_START_TIME is defined, in case preexec was interrupted.

   Finally, note that `eval` will evaluate this thing as a single line, so things
   like comments (#) should be avoided and semicolons should be used on most 
   statements.
*/

const BASH_INIT: &str = r##"
starship_preexec() {
    if [ "$PREEXEC_READY" = "true" ]; then
        PREEXEC_READY=false;
        STARSHIP_START_TIME=$(date +%s);
    fi
};
starship_precmd() {
    if [[ $STARSHIP_START_TIME ]]; then
        STARSHIP_END_TIME=$(date +%s);
        STARSHIP_ELAPSED=$((STARSHIP_END_TIME - STARSHIP_START_TIME));
        PS1="$(starship prompt --status=$? --elapsed=$STARSHIP_ELAPSED)";
        unset STARSHIP_START_TIME;
    else
        PS1="$(starship prompt --status=$?)";
    fi;
    PREEXEC_READY=true;
};
if [[ $preexec_functions ]]; then
    preexec_functions+=(starship_preexec);
    precmd_functions+=(starship_precmd);
    STARSHIP_START_TIME=$(date +%s);
fi;
dbg_trap="$(trap -p DEBUG | cut -d' ' -f3 | tr -d \')";
if [[ -z $dbg_trap || "$dbg_trap" = "starship_preexec" ]]; then
    trap starship_preexec DEBUG;
    PROMPT_COMMAND=starship_precmd;
    STARSHIP_START_TIME=$(date +%s);
else
    PROMPT_COMMAND=starship_precmd;
fi
"##;
//TODO: How to warn the user?

/* For zsh: preexec_functions and precmd_functions provide preexec/precmd in a
   way that lets us avoid clobbering them.

   Zsh quirk: preexec() is only fired if a command is actually run (unlike in
   bash, where spamming empty commands still triggers DEBUG). This means a user
   spamming ENTER at an empty command line will see increasing runtime (since
   preexec never actually fires to reset the start time).

   To fix this, only pass the time if STARSHIP_START_TIME is defined, and unset
   it after passing the time, so that we only measure actual commands.
*/

const ZSH_INIT: &str = r##"
starship_precmd() {
    if [[ $STARSHIP_START_TIME ]]; then
        STARSHIP_END_TIME="$(date +%s)";
        ELAPSED=$((STARSHIP_END_TIME - STARSHIP_START_TIME));
        PROMPT="$(starship prompt --status=$? --elapsed=$ELAPSED)";
        unset STARSHIP_START_TIME;
    else
        PROMPT="$(starship prompt --status=$?)";
    fi
};
starship_preexec(){
    STARSHIP_START_TIME="$(date +%s)"
};
precmd_functions+=(starship_precmd);
preexec_functions+=(starship_preexec);
STARSHIP_START_TIME="$(date +%s)";
"##;

/* Fish setup is simple because they give us CMD_DURATION. Hooray! */
const FISH_INIT: &str = r##"
function fish_prompt;
    set -l elapsed (math --scale=0 "$CMD_DURATION / 1000");
    starship prompt --status=$status --elapsed=$elapsed;
end;
"##;
