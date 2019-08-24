use std::ffi::OsStr;
use std::path::Path;

pub fn init(shell_name: &str) {
    log::debug!("Shell name: {}", shell_name);

    let shell_basename = Path::new(shell_name).file_stem().and_then(OsStr::to_str);

    let setup_script = match shell_basename {
        // The contents of `PROMPT_COMMAND` are executed as a regular Bash command
        // just before Bash displays a prompt.
        Some("bash") => {
            let script = "
            PROMPT_COMMAND=starship_prompt
            
            starship_prompt() {
                PS1=\"$(starship prompt --status=$?)\"
            }";
            Some(script)
        }
        // `precmd` executes a command before the zsh prompt is displayed.
        Some("zsh") => {
            let script = "
            precmd() {
                PROMPT=\"$(starship prompt --status=$?)\"
            }";
            Some(script)
        }
        Some("fish") => {
            let script = "function fish_prompt; starship prompt --status=$status; end";
            Some(script)
        }
        Some("powershell") => {
            let script = "function Prompt { starship prompt --status=$LastExitCode }";
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

    if setup_script.is_some() {
        let script = setup_script.unwrap();
        print!("{}", script);
    }
}
