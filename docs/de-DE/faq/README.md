# FAQ

## Wie ist die Konfiguration im Demo GIF?

- **Terminal Emulator**: [iTerm2](https://iterm2.com/)
  - **Theme**: Minimal
  - **Farbschema**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Schriftart**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Konfiguration**: [Matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## How do I get command completion as shown in the demo GIF?

Completion support, or autocomplete, is provided by your shell of choice. In the case of the demo, the demo was done with [Fish Shell](https://fishshell.com/), which provides completions by default. If you use Z Shell (zsh), I'd suggest taking a look at [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Do top level `format` and `<module>.disabled` do the same thing?

Ja, beide können benutzt werden, um Module in der Prompt zu deaktivieren. Wenn nur Module deaktiviert werden wollen, sollte `<module>.disabled` benutzt werden, aus den folgenden Gründen:

- Disabling modules is more explicit than omitting them from the top level `format`
- Mit der Aktualisierung von Starship werden neu erstellte Module an die Eingabezeile angefügt

## Die Dokumentation sagt, dass die Starship interkompatibel ist. Warum wird meine bevorzugte Shell nicht unterstützt?

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

Die [Bash Implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) ist etwas komplexer, um erweiterte Funktionen wie das [Befehlsdauer-Modul](https://starship.rs/config/#command-duration) zu ermöglichen und um sicherzustellen, dass Starship mit vorinstallierten Bash Konfigurationen kompatibel ist.

Für eine Liste aller Flaggen, die von `Starship-Eingabeaufforderung` akzeptiert wird, verwenden Sie den folgenden Befehl:

```sh
starship prompt --help
```

Die Eingabeaufforderung verwendet so viel Kontext wie möglich, aber keine Flagge ist "notwendig".

## How do I run Starship on Linux distributions with older versions of glibc?

If you get an error like "_version 'GLIBC_2.18' not found (required by starship)_" when using the prebuilt binary (for example, on CentOS 6 or 7), you can use a binary compiled with `musl` instead of `glibc`:

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --platform unknown-linux-musl
```

## I see symbols I don't understand or expect, what do they mean?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. Sie müssen sicherstellen, dass:

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. Wenn `LC_ALL` kein UTF-8-Wert ist, [müssen Sie ihn ändern](https://www.tecmint.com/set-system-locales-in-linux/).
- You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
- You are using a [Nerd Font](https://www.nerdfonts.com/).

Um Ihr System zu testen, führen Sie folgende Befehle in einem Terminal aus:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Benutzer auf dem Discord können vielleicht helfen. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## Wie deinstalliere ich Starship?

Starship ist genauso einfach zu deinstallieren wie zu installieren.

1. Remove any lines in your shell config (e.g. `~/.bashrc`) used to initialize Starship.
1. Löschen Sie die Starship-Binary.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

Wenn Starship mit Hilfe des Installationsscripts installiert wurde, entfernt der folgende Befehl Starship:

```sh
# Starship binary suchen und löschen
sh -c 'rm "$(which starship)"'
```
