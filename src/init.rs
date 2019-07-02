pub fn init(shell_name: &str) {
    log::debug!("Shell name: {}", shell_name);
    let setup_script = match shell_name {
        "bash" => {
            let script = "PS1=\"$(starship prompt --status=$?)\"";
            Some(script)
        }
        "zsh" => {
            let script = "PROMPT=\"$(starship prompt --status=$?)\"";
            Some(script)
        }
        "fish" => {
            let script = "function fish_prompt; starship prompt --status=$status; end";
            Some(script)
        }
        _ => {
            println!(
                "printf \"\\n{0} is not yet supported by starship.\\n\
                 For the time being, we support bash, zsh, and fish.\\n\
                 Please open an issue in the starship repo if you would like to \
                 see support for {0}:\\nhttps://github.com/starship/starship/issues/new\"",
                shell_name
            );
            None
        }
    };

    if setup_script.is_some() {
        let script = setup_script.unwrap();
        print!("{}", script);
    }
}

#[derive(Debug)]
enum Shell {
    Bash,
    Fish,
    Zsh,
    Unsupported(String),
}
