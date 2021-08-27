# Najczęściej zadawane pytania (FAQ)

## Jakiej konfiguracji użyto w GIFie demonstracyjnym?

- **Emulator terminala**: [iTerm2](https://iterm2.com/)
  - **Motyw**: Minimal
  - **Schemat kolorów**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Czcionka**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Powłoka**: [Fish Shell](https://fishshell.com/)
  - **Konfiguracja**: [Pliki Dotfile użytkownika matchai](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Wiersz poleceń**: [Starship](https://starship.rs/)

## Jak uzyskać uzupełniania poleceń pokazane na GIFie demonstracyjnym?

Uzupełnianie, znane też jako autouzupełnianie, jest dostarczane przez używaną przez ciebie powłokę. W przypadku demo, jest to powłoka [Fish Shell](https://fishshell.com/), która domyślnie zapewnia uzupełnianie. Jeśli używasz Z Shell (zsh), zalecamy zapoznać się z [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Do top level `format` and `<module>.disabled` do the same thing?

Yes, they can both be used to disable modules in the prompt. If all you plan to do is disable modules, `<module>.disabled` is the preferred way to do so for these reasons:

- Disabling modules is more explicit than omitting them from the top level `format`
- Newly created modules will be added to the prompt as Starship is updated

## Dokumentacja stwierdza że Starship jest wieloplatformowy. Dlaczego moja preferowana powłoka nie jest obsługiwana?

The way Starship is built, it should be possible to add support for virtually any shell. The starship binary is stateless and shell agnostic, so as long as your shell supports prompt customization and shell expansion, Starship can be used.

Here's a small example getting Starship working with bash:

```sh
# Get the status code from the last command executed
STATUS=$?

# Get the number of jobs running.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#command-duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

For a list of all flags accepted by `starship prompt`, use the following command:

```sh
starship prompt --help
```

The prompt will use as much context as is provided, but no flags are "required".

## Jak uruchomić Starship na dystrybucjach systemu Linux ze starszymi wersjami glibc?

Jeżeli podczas używania gotowych instalacji (np. na CentOS 6 lub 7) pojawia się błąd w stylu "_version 'GLIBC_2.18' not found (required by starship)_", możesz użyć instalacji zbudowanej za pomocą `musl` zamiast `glibc`:

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --platform unknown-linux-musl
```

## W wierszu poleceń zauważyłem symbole których się nie spodziewałem bądź nie rozumiem. Co one oznaczają?

Jeśli zauważyłeś symbole których nie rozpoznajesz, możesz użyć `starship explain` aby uzyskać szczegółowy opis obecnie wyświetlanych modułów.

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

## Jak odinstalować Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Remove any lines in your shell config (e.g. `~/.bashrc`) used to initialize Starship.
1. Delete the Starship binary.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the install script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(which starship)"'
```
