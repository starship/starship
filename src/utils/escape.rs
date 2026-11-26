//! Literal prompt text and visible terminal controls.
//!
//! Shell escaping and terminal sanitization are separate operations: use
//! `sanitized` or `visible_controls` first when text comes from outside the
//! configuration, then escape it for the shell's documented prompt phase.

use std::borrow::Cow;

use crate::context::Shell;

const ESCAPE: char = '\x1b';
const STYLING_PREFIX: &str = "\x1b[";
const HYPERLINK_PREFIX: &str = "\x1b]8;";
const BELL_TERMINATOR: &str = "\x07";
const STRING_TERMINATOR: &str = "\x1b\\";
const C1_UTF8_LEAD: u8 = 0xc2;
const CARET_NOTATION_BIT: u8 = b'@';

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
            b'`' => Some(r"\\`"),
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

/// Show C0/C1 controls, preserving line feeds, tabs, complete SGR styling,
/// and complete OSC 8 hyperlinks with an empty or lowercase HTTP(S) URI.
///
/// The sequence grammar is deliberately narrow: styling parameters contain
/// only digits, `;`, and `:`; hyperlink fields contain printable ASCII and end
/// in BEL or `ESC \`. URI validation stops at the scheme prefix.
///
/// Rejected sequences retain their payload with each control made visible.
/// Preserved styling and hyperlinks can affect subsequent text; they are not
/// automatically reset. Use [`visible_controls`] when that is undesirable.
pub fn sanitized<'a>(text: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
    show_controls(text.into(), preserved_sequence_length)
}

/// Show C0 controls and DEL in caret notation and C1 controls as `\u{xx}`.
/// Line feeds and tabs remain; CRLF becomes LF. Other Unicode characters,
/// including format and bidirectional controls, are outside this policy.
pub fn visible_controls<'a>(text: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
    show_controls(text.into(), |_| None)
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

fn show_controls<'a>(
    text: Cow<'a, str>,
    preserved_length: impl Fn(&str) -> Option<usize>,
) -> Cow<'a, str> {
    let mut output: Option<String> = None;
    let mut copied = 0;
    let mut searched = 0;
    while let Some((offset, control)) = next_control(&text[searched..]) {
        let start = searched + offset;
        searched = start + control.len_utf8();
        if control == ESCAPE
            && let Some(length) = preserved_length(&text[start..])
        {
            searched = start + length;
            continue;
        }

        let drop_carriage_return = control == '\r' && text[searched..].starts_with('\n');
        let output = output.get_or_insert_with(|| {
            let replacement_length = if drop_carriage_return {
                0
            } else if control.is_ascii() {
                "^@".len()
            } else {
                r"\u{80}".len()
            };
            // Keep space freed by CRLF normalization for later replacements.
            String::with_capacity(
                text.len() + replacement_length.saturating_sub(control.len_utf8()),
            )
        });
        output.push_str(&text[copied..start]);
        if !drop_carriage_return {
            push_visible_control(output, control);
        }
        copied = searched;
    }
    match output {
        None => text,
        Some(mut output) => {
            output.push_str(&text[copied..]);
            Cow::Owned(output)
        }
    }
}

/// Skip ordinary UTF-8 bytes without decoding characters. C1 controls have
/// the unique lead byte 0xc2; its printable Latin-1 neighbors are skipped.
fn next_control(mut text: &str) -> Option<(usize, char)> {
    let mut skipped = 0;
    loop {
        let offset = text
            .bytes()
            .position(|byte| byte.is_ascii_control() || byte == C1_UTF8_LEAD)?;
        let control = text[offset..].chars().next()?;
        if control.is_control() && !matches!(control, '\n' | '\t') {
            return Some((skipped + offset, control));
        }
        let consumed = offset + control.len_utf8();
        skipped += consumed;
        text = &text[consumed..];
    }
}

fn push_visible_control(output: &mut String, control: char) {
    if control.is_ascii() {
        output.push('^');
        output.push(char::from(control as u8 ^ CARET_NOTATION_BIT));
    } else {
        output.extend(control.escape_unicode());
    }
}

fn preserved_sequence_length(text: &str) -> Option<usize> {
    styling_sequence_length(text).or_else(|| hyperlink_sequence_length(text))
}

fn styling_sequence_length(text: &str) -> Option<usize> {
    let parameters = text.strip_prefix(STYLING_PREFIX)?;
    let end = parameters
        .bytes()
        .position(|byte| !byte.is_ascii_digit() && !matches!(byte, b';' | b':'))?;
    (parameters.as_bytes()[end] == b'm').then_some(STYLING_PREFIX.len() + end + "m".len())
}

fn hyperlink_sequence_length(text: &str) -> Option<usize> {
    let body = text.strip_prefix(HYPERLINK_PREFIX)?;
    let end = body.bytes().position(|byte| !matches!(byte, b' '..=b'~'))?;
    let (_, destination) = body[..end].split_once(';')?;
    if !is_web_link_or_close(destination) {
        return None;
    }
    let terminator = [BELL_TERMINATOR, STRING_TERMINATOR]
        .into_iter()
        .find(|terminator| body[end..].starts_with(terminator))?;
    Some(HYPERLINK_PREFIX.len() + end + terminator.len())
}

fn is_web_link_or_close(destination: &str) -> bool {
    destination.is_empty()
        || destination.starts_with("http://")
        || destination.starts_with("https://")
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
            (Shell::Bash, "`id`", r"\\`id\\`"),
            (Shell::Zsh, "%{x%}$HOME", "%%{x%%}$HOME"),
            (Shell::Tcsh, "50% done", "50%% done"),
            (Shell::Xonsh, "{{x}", "{{{{x}}"),
            // Characters past ASCII sit between and around the escapes.
            (Shell::Bash, "é$ü`目\\🚀", r"é\\$ü\\`目\\\\🚀"),
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

    #[test]
    fn controls_are_shown_but_for_whole_styling_and_web_links() {
        for (text, shown) in [
            // C0 and DEL in caret notation, C1 as its escape.
            ("\0\x07\x08\x1b\x1f\x7f", "^@^G^H^[^_^?"),
            ("\u{9b}2J\u{9d}52;c;x\u{9c}", r"\u{9b}2J\u{9d}52;c;x\u{9c}"),
            // The characters that share the first byte of C1 are not controls.
            ("\u{a0}\u{bf}é", "\u{a0}\u{bf}é"),
            // A carriage return shows, but for the one a Windows line ends with.
            ("a\r\nb\r\r\nc\r", "a\nb^M\nc^M"),
            ("a\r\x1b[0m\n", "a^M\x1b[0m\n"),
            // Whole styling is kept, and nothing else that begins as it does.
            ("\x1b[1;38:2::1:2:3mx\x1b[m", "\x1b[1;38:2::1:2:3mx\x1b[m"),
            ("\x1b[?25l\x1b[2J\x1b[31", "^[[?25l^[[2J^[[31"),
            ("\x1b[31\x1b[1m", "^[[31\x1b[1m"),
            ("\x1b\x1b[1m", "^[\x1b[1m"),
            // Whole web links are kept, and their ends.
            (
                "\x1b]8;id=1;https://x;y\x1b\\a\x1b]8;;\x07",
                "\x1b]8;id=1;https://x;y\x1b\\a\x1b]8;;\x07",
            ),
            ("\x1b]8;;http://x\x07", "\x1b]8;;http://x\x07"),
            // Any other link, or one that is not whole, shows.
            (
                "\x1b]8;;file:///etc/passwd\x07",
                "^[]8;;file:///etc/passwd^G",
            ),
            ("\x1b]8;;HTTPS://x\x07", "^[]8;;HTTPS://x^G"),
            ("\x1b]8;;https://exämple\x07", "^[]8;;https://exämple^G"),
            ("\x1b]8;;https://x\x1b[2J\x07", "^[]8;;https://x^[[2J^G"),
            ("\x1b]8;https://x\x07", "^[]8;https://x^G"),
            ("\x1b]8;;https://x", "^[]8;;https://x"),
            ("\x1b]52;c;SGk=\x1b\\", "^[]52;c;SGk=^[\\"),
        ] {
            assert_eq!(shown, sanitized(text), "{text:?}");
            // What is shown is shown again as it is.
            assert!(matches!(sanitized(shown), Cow::Borrowed(_)), "{shown:?}");
        }
        assert_eq!("^[[31mred^[[0m", visible_controls("\x1b[31mred\x1b[0m"));
        assert_eq!(
            "^[]8;;https://x^G",
            visible_controls("\x1b]8;;https://x\x07")
        );
    }

    #[test]
    fn a_sequence_is_kept_only_when_whole() {
        for kept in [
            "\x1b[1;38:2::1:2:3m",
            "\x1b]8;id=a;https://x?y=1;z\x07",
            "\x1b]8;;http://x\x1b\\",
        ] {
            assert_eq!(kept, sanitized(kept));
            // Cut short, or with a control before any of its characters.
            for end in (1..kept.len()).filter(|end| kept.is_char_boundary(*end)) {
                for broken in [
                    kept[..end].to_owned(),
                    format!("{}\x08{}", &kept[..end], &kept[end..]),
                ] {
                    let shown = sanitized(broken.as_str());
                    assert!(!shown.contains('\x1b'), "{broken:?} became {shown:?}");
                }
            }
        }
    }
}
