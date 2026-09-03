//! Prompt output destinations.

use crate::context::Shell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destination {
    ShellPromptVariable(Shell),
    RawTerminal,
}

impl Destination {
    pub const fn shell_prompt_variable(shell: Shell) -> Self {
        Self::ShellPromptVariable(shell)
    }

    pub fn escape<T>(self, text: T) -> String
    where
        T: Into<String>,
    {
        match self {
            Self::ShellPromptVariable(shell) => shell_prompt_escape(text, shell),
            Self::RawTerminal => text.into(),
        }
    }
}

pub fn shell_prompt_escape<T>(text: T, shell: Shell) -> String
where
    T: Into<String>,
{
    match shell {
        Shell::Bash => {
            let text = text.into();
            let mut escaped = String::with_capacity(text.len());
            for character in text.chars() {
                match character {
                    '\\' => escaped.push_str(r"\\"),
                    '$' => escaped.push_str(r"\$"),
                    '`' => escaped.push_str(r"\`"),
                    character => escaped.push(character),
                }
            }
            escaped
        }
        Shell::Zsh => text.into().replace('%', "%%"),
        _ => text.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_raw_terminal_destination_never_rewrites_anything() {
        for text in ["10% $ `tick` back\\slash", "", "%%%", "$$$"] {
            assert_eq!(text, Destination::RawTerminal.escape(text));
        }
    }

    #[test]
    fn a_zsh_prompt_variable_doubles_every_percent() {
        assert_eq!(
            "10%% off",
            Destination::shell_prompt_variable(Shell::Zsh).escape("10% off")
        );
    }

    #[test]
    fn a_bash_prompt_variable_escapes_what_bash_would_expand() {
        assert_eq!(
            r"\$HOME \`date\` back\\slash",
            Destination::shell_prompt_variable(Shell::Bash).escape(r"$HOME `date` back\slash")
        );
    }

    #[test]
    fn bash_escaping_covers_every_character_bash_would_expand() {
        for (text, expected) in [
            ("$(echo a)", r"\$(echo a)"),
            (r"\$(echo a)", r"\\\$(echo a)"),
            (r"`echo a`", r"\`echo a\`"),
        ] {
            assert_eq!(expected, shell_prompt_escape(text, Shell::Bash));
            assert_eq!(text, shell_prompt_escape(text, Shell::PowerShell));
        }
    }

    #[test]
    fn zsh_escaping_covers_the_one_character_zsh_would_expand() {
        assert_eq!("10%%", shell_prompt_escape("10%", Shell::Zsh));
        assert_eq!("10%", shell_prompt_escape("10%", Shell::PowerShell));
    }

    #[test]
    fn shells_that_expand_nothing_agree_with_the_raw_terminal() {
        let text = "10% $ `tick` back\\slash";
        for shell in [
            Shell::Fish,
            Shell::Ion,
            Shell::Pwsh,
            Shell::PowerShell,
            Shell::Elvish,
            Shell::Tcsh,
            Shell::Nu,
            Shell::Xonsh,
            Shell::Cmd,
            Shell::Unknown,
        ] {
            let destination = Destination::shell_prompt_variable(shell);
            assert_eq!(text, destination.escape(text), "for {shell:?}");
        }
    }
}
