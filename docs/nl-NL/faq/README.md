# Veelgestelde Vragen

## Wat is de configuratie gebruikt in de demo GIF?

- **Terminal Emulator**: [iTerm2](https://iterm2.com/)
  - **Thema**: Minimal
  - **Kleurschema**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Lettertype**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Configuratie**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## Hoe kan ik de opdracht voltooien zoals weergegeven in de demo GIF?

Ondersteuning voor voltooiing, of automatische voltooiing, wordt aangeboden door jouw opdrachtshell van jouw keuze. In het geval van de demo werd de demo gedaan met [Fish Shell](https://fishshell.com/), die standaard automatische voltooiing biedt. Als je Z Shell (zsh) gebruikt, zou ik voorstellen een kijkje te nemen bij [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Doen het hoogste niveau `format` en `<module>.disabled` hetzelfde?

Ja, ze kunnen beide worden gebruikt om modules uit te schakelen in de prompt. Als u alleen modules wilt uitschakelen, is `<module>.disabled` de beste manier om dit te doen om deze redenen:

- Het uitschakelen van modules is uitdrukkelijker dan het weglaten van deze op het bovenste niveau `format`
- Nieuw aangemaakte modules zullen worden toegevoegd aan de prompt als Starship wordt bijgewerkt

## De documentatie zegt dat Starship cross-shell is. Waarom wordt mijn voorkeursopdrachtshell niet ondersteund?

De manier waarop Starship wordt gebouwd, moet het mogelijk zijn om bijna elke opdrachtshell te ondersteunen. The starship binary is stateless and shell agnostic, so as long as your shell supports prompt customization and shell expansion, Starship can be used.

Hier is een klein voorbeeld van het werken met een bash:

```sh
# Haal de statuscode op van het laatst uitgevoerde commando
STATUS=$?

# Haal het aantal draaiende taken op.
NUM_JOBS=$(jobs -p | wc -l)

# Stel het prompt in naar de uitvoer van `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#command-duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

For a list of all flags accepted by `starship prompt`, use the following command:

```sh
starship prompt --help
```

The prompt will use as much context as is provided, but no flags are "required".

## How do I run Starship on Linux distributions with older versions of glibc?

If you get an error like "_version 'GLIBC_2.18' not found (required by starship)_" when using the prebuilt binary (for example, on CentOS 6 or 7), you can use a binary compiled with `musl` instead of `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Why do I see `Executing command "..." timed out.` warnings?

Starship executes different commands to get information to display in the prompt, for example the version of a program or the current git status. To make sure starship doesn't hang while trying to execute these commands we set a time limit, if a command takes longer than this limit starship will stop the execution of the command and output the above warning, this is expected behaviour. This time limit is configurable using the [`command_timeout`key](../config/#prompt) so if you want you can increase the time limit. You can also follow the debugging steps below to see which command is being slow and see if you can optimise it. Finally you can set the `STARSHIP_LOG` env var to `error` to hide these warnings.

## I see symbols I don't understand or expect, what do they mean?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Starship is doing something unexpected, how can I debug it?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that is to blame.

```sh
env STARSHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a GitHub issue.

```sh
starship bug-report
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
- You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
- You are using a [Nerd Font](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## How do I uninstall Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Remove any lines in your shell config (e.g. `~/.bashrc`) used to initialize Starship.
1. Delete the Starship binary.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the install script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(command -v 'starship')"'
```

## How do I install Starship without `sudo`?

The shell install script (`https://starship.rs/install.sh`) only attempts to use `sudo` if the target installation directory is not writable by the current user. The default installation directory is the value of the `$BIN_DIR` environment variable or `/usr/local/bin` if `$BIN_DIR` is not set. If you instead set the installation directory to one that is writable by your user, you should be able to install starship without `sudo`. For example, `curl -sS https://starship.rs/install.sh | sh -s -- -b ~/.local/bin` uses the `-b` command line option of the install script to set the installation directory to `~/.local/bin`.

For a non-interactive installation of Starship, don't forget to add the `-y` option to skip the confirmation. Check the source of the installation script for a list of all supported installation options.

When using a package manager, see the documentation for your package manager about installing with or without `sudo`.
