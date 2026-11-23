//! Literal prompt text.

use std::borrow::Cow;

use crate::context::Shell;

/// Escape literal text for the prompt phase used by `shell`.
///
/// Bash assumes direct insertion into a prompt with `promptvars` enabled:
/// backslash decoding is followed by expansion and quote removal, so both
/// phases must be escaped. Zsh/tcsh escape percent prompt directives; xonsh
/// escapes format braces. Other shells pass text through.
///
/// This is not shell-source quoting. In particular, zsh's optional
/// `PROMPT_SUBST` and history expansion are outside the percent-escape phase.
pub fn shell_prompt_escape<'a>(text: impl Into<Cow<'a, str>>, shell: Shell) -> Cow<'a, str> {
    let text = text.into();
    match shell {
        Shell::Bash => escape_ascii(text, |byte| match byte {
            // Two backslashes survive decoding as one; expansion consumes
            // another layer. Never use `\$`, which is Bash's privilege marker.
            // see #658 for more details
            b'\\' => Some(r"\\\\"),
            b'$' => Some(r"\\$"),
            b'`' => Some(r"\`"),
            _ => None,
        }),
        Shell::Zsh | Shell::Tcsh => escape_ascii(text, |byte| match byte {
            // % is an escape in zsh and tcsh, see PROMPT in `man zshmisc`
            // and prompt in `man tcsh`
            b'%' => Some("%%"),
            _ => None,
        }),
        Shell::Xonsh => escape_ascii(text, |byte| match byte {
            // Xonsh formats a prompt, filling in fields such as `{user}`.
            b'{' => Some("{{"),
            b'}' => Some("}}"),
            _ => None,
        }),
        _ => text,
    }
}

/// The stable mapping matches ASCII only and expands every match. ASCII
/// offsets are UTF-8 boundaries. Count only the changed suffix to avoid
/// output growth, without an edit list or temporary strings.
fn escape_ascii<'a>(
    text: Cow<'a, str>,
    replacement: impl Fn(u8) -> Option<&'static str>,
) -> Cow<'a, str> {
    let Some(first_change) = text.bytes().position(|byte| replacement(byte).is_some()) else {
        return text;
    };
    let changed = &text[first_change..];
    let output_length = changed
        .bytes()
        .filter_map(&replacement)
        .fold(text.len(), |length, escape| {
            length.saturating_add(escape.len() - 1)
        });

    // An overflowing count saturates to an impossible capacity, which String
    // rejects. It must never wrap into a small allocation or a clean result.
    let mut output = String::with_capacity(output_length);
    let mut copied = 0;
    for (offset, byte) in changed.bytes().enumerate() {
        if let Some(escape) = replacement(byte) {
            let position = first_change + offset;
            output.push_str(&text[copied..position]);
            output.push_str(escape);
            copied = position + 1;
        }
    }
    output.push_str(&text[copied..]);
    Cow::Owned(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::exec_cmd;
    use std::time::Duration;

    #[test]
    fn text_is_escaped_for_its_shell_and_borrowed_when_unchanged() {
        for (shell, text, escaped) in [
            (Shell::Bash, "$(echo a)", r"\\$(echo a)"),
            (Shell::Bash, r"a\b", r"a\\\\b"),
            (Shell::Bash, r"\$", r"\\\\\\$"),
            (Shell::Bash, "`id`", r"\`id\`"),
            (Shell::Zsh, "%{x%}$HOME", "%%{x%%}$HOME"),
            (Shell::Tcsh, "50% done", "50%% done"),
            (Shell::Xonsh, "{{x}", "{{{{x}}"),
            // Characters past ASCII sit between and around the escapes.
            (Shell::Bash, "é$ü`目\\🚀", r"é\\$ü\`目\\\\🚀"),
            (Shell::Xonsh, "{é}{🚀}", "{{é}}{{🚀}}"),
            // What a shell does not act on is left as it is.
            (Shell::Bash, "%{}", "%{}"),
            (Shell::Zsh, r"\$`{}", r"\$`{}"),
            (Shell::Fish, r#"\$`%{}"'"#, r#"\$`%{}"'"#),
            (Shell::Bash, "", ""),
        ] {
            let actual = shell_prompt_escape(text, shell);
            assert_eq!(escaped, actual, "{shell:?} {text:?}");
            assert_eq!(
                text == escaped,
                matches!(actual, Cow::Borrowed(_)),
                "{shell:?} {text:?}"
            );
        }
        // Owned text that needs nothing is handed back, not copied.
        let owned = "plain".to_owned();
        let pointer = owned.as_ptr();
        assert_eq!(pointer, shell_prompt_escape(owned, Shell::Bash).as_ptr());
    }

    /// Every text of up to four of the characters a shell may act on, and a
    /// few it does not.
    fn texts_of_shell_characters() -> Vec<String> {
        const CHARACTERS: [char; 11] = ['\\', '$', '`', '"', '%', '{', '}', '(', '\'', 'a', ' '];
        let mut texts = Vec::new();
        let mut last = vec![String::new()];
        for _ in 0..4 {
            last = last
                .iter()
                .flat_map(|text| CHARACTERS.map(|character| format!("{text}{character}")))
                .collect();
            texts.extend(last.iter().cloned());
        }
        texts
    }

    /// Runs `program` with `arguments` and then every text escaped for
    /// `shell`, and checks that it printed each as it was written, ended by a
    /// NUL. A program that is not here, or fails as one too old to expand a
    /// prompt this way does, is skipped.
    fn assert_shell_shows_as_written(shell: Shell, program: &str, arguments: &[&str]) {
        let texts = texts_of_shell_characters();
        let arguments: Vec<String> = arguments
            .iter()
            .map(|argument| (*argument).to_owned())
            .chain(
                texts
                    .iter()
                    .map(|text| shell_prompt_escape(text.as_str(), shell).into_owned()),
            )
            .collect();
        let Some(output) = exec_cmd(program, &arguments, Duration::from_secs(60)) else {
            eprintln!("{program} is not here, or is not a version these tests can ask: skipped");
            return;
        };
        let shown: Vec<&str> = output.stdout.split_terminator('\0').collect();
        assert_eq!(texts.len(), shown.len(), "{program} for {shell:?}");
        let wrong: Vec<_> = texts
            .iter()
            .zip(&shown)
            .filter(|(text, shown)| text != shown)
            .take(5)
            .collect();
        assert!(
            wrong.is_empty(),
            "{program} showed other than was written for {shell:?}: {wrong:?}"
        );
    }

    #[test]
    fn bash_shows_the_text_that_was_escaped_for_it() {
        // Expanding a prompt with `@P` needs bash 4.4.
        assert_shell_shows_as_written(
            Shell::Bash,
            "bash",
            &["-c", r#"printf '%s\0' "${@@P}""#, "bash"],
        );
    }

    #[test]
    fn zsh_shows_the_text_that_was_escaped_for_it() {
        assert_shell_shows_as_written(
            Shell::Zsh,
            "zsh",
            &["-f", "-c", r#"print -rNP -- "$@""#, "zsh"],
        );
    }

    #[test]
    fn xonsh_shows_the_text_that_was_escaped_for_it() {
        assert_shell_shows_as_written(
            Shell::Xonsh,
            "xonsh",
            &[
                "--no-rc",
                "-c",
                "import sys\n\
                 from xonsh.prompt.base import PromptFormatter\n\
                 format = PromptFormatter()\n\
                 for text in sys.argv[sys.argv.index('-c') + 2:]:\n\
                 \x20   print(format(text), end='\\0')",
            ],
        );
    }
}
