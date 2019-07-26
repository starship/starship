use std::ffi::OsStr;
use std::path::Path;

pub fn init(shell_name: &str) {
    log::debug!("Shell name: {}", shell_name);

    let shell_basename = Path::new(shell_name).file_stem().and_then(OsStr::to_str);

    let setup_script = match shell_basename {
        // The contents of `PROMPT_COMMAND` are executed as a regular Bash command
        // just before Bash displays a prompt.
        Some("bash") => {
            let script = r##"
            starship_preexec() {
                if [ "$PREEXEC_READY" = "true" ]; then
                    PREEXEC_READY=false;
                    STARSHIP_START_TIME=$(date +%s);
                fi
            };

            starship_prompt() {
                PREEXEC_READY=true;
                STARSHIP_END_TIME=$(date +%s);
                ELAPSED=$((STARSHIP_END_TIME - STARSHIP_START_TIME));
                PS1="$(starship prompt --status=$? --elapsed=$ELAPSED)";
            };
 
            trap starship_preexec DEBUG;
            PROMPT_COMMAND=starship_prompt;
            "##;
            Some(script)
        }
        /* `precmd` executes a command before the zsh prompt is displayed.
        We need to set STARSHIP_START_TIME as part of the init commmand or
        else the first prompt will display a very large time */
        Some("zsh") => {
            let script = r##"
            precmd() {
                STARSHIP_END_TIME="$(date +%s)"
                ELAPSED=$((STARSHIP_END_TIME - STARSHIP_START_TIME))
                PROMPT="$(starship prompt --status=$? --elapsed=$ELAPSED)"
            };
            preexec(){
                STARSHIP_START_TIME="$(date +%s)"
            };
            STARSHIP_START_TIME="$(date +%s)"
            "##;
            Some(script)
        }
        Some("fish") => {
            let script = r##"function fish_prompt;
            set -l elapsed (math --scale=0 "$CMD_DURATION / 1000");
            starship prompt --status=$status --elapsed=$elapsed;
            end;"##;
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
