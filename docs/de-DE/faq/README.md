# FAQ

## Wie ist die Konfiguration im Demo GIF?

- **Terminal Emulator**: [iTerm2](https://iterm2.com/)
  - **Theme**: Minimal
  - **Farbschema**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Font**: [Fira Code](https://github.com/tonsky/FiraCode)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Konfiguration**: [Matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/master/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## Tun `prompt_order` und `<module>.disabled` dasselbe?

Ja, beide können benutzt werden, um Module in der Prompt zu deaktivieren. Wenn nur Module deaktiviert werden wollen, sollte `<module>.disabled` benutzt werden, aus den folgenden Gründen:

- Das Deaktivieren von Modulen ist expliziter als das Auslassen von Modulen in der prompt_order
- Mit der Aktualisierung von Starship werden neu erstellte Module an die Eingabezeile angefügt

## Laut Dokumentation ist Starship cross-shell, aber es läuft nicht auf shell X. Warum?

Starship ist auf so eine Weise gebaut, das die Unterstützung so gut wie jeder Shell möglch sein sollte. Die Starship Binärdatei läuft völlig unabhängig von der Shell, und sollte auf jeder benutzt werden können, die eine Anpassung des Stils erlaubt.

Hier ist ein kleines Beispiel, wie man Starship auf bash zum Laufen bringt:

```sh
# Den Statuscode des zuletzt ausgeführten Befehls abrufen
STATUS=$?

# Gibt die Anzahl der laufenden Jobs an.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

Die [Bash Implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) ist etwas komplexer, um erweiterte Funktionen wie das [Befehlsdauer-Modul](https://starship.rs/config/#Command-Duration) zu ermöglichen und um sicherzustellen, dass Starship mit vorinstallierten Bash Konfigurationen kompatibel ist.

Für eine Liste aller Flaggen, die von `Starship-Eingabeaufforderung` akzeptiert wird, verwenden Sie den folgenden Befehl:

```sh
starship prompt --help
```

Die Eingabeaufforderung verwendet so viel Kontext wie möglich, aber keine Flagge ist "notwendig".

## How do I run Starship on Linux distributions with older versions of glibc?

If you get an error like "*version 'GLIBC_2.18' not found (required by starship)*" when using the prebuilt binary (for example, on CentOS 6 or 7), you can use a binary compiled with `musl` instead of `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

  - Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
  - You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
  - You are using a [powerline-patched font](https://github.com/powerline/fonts).

To test your system, run the following commands in a terminal:

```
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)
