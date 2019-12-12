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
PS1="$(starship prompt --status=$STATUS --jobs=NUM_JOBS)"
```

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#Command-Duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

For a list of all flags accepted by `starship prompt`, use the following command:

```sh
starship prompt --version
```

The prompt will use as much context as is provided, but no flags are "required".
