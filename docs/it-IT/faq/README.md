# FAQ

## Qual è la configurazione utilizzata nella GIF demo?

- **Terminale Emulato**: [iTerm2](https://iterm2.com/)
  - **Tema**: Minimale
  - **Schema Colore**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Font**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Configurazione**: [matchai Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Chiedi**: [Starship](https://starship.rs/)

## Come posso ottenere il completamento dei comandi come mostrato nella GIF demo?

Completion support, or autocomplete, is provided by your shell of choice. Nel caso della demo, la demo è stata fatta con [Fish Shell](https://fishshell.com/), che fornisce i completamenti per impostazione predefinita. Se usi Z Shell (zsh), ti suggerirei di dare un'occhiata a [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Il formato di primo livello `` e `<module>.disabled` fanno la stessa cosa?

Sì, entrambi possono essere utilizzati per disabilitare i moduli nel prompt. Se tutto quello che pensi di fare è disabilitare i moduli, `<module>.disabled` è il modo preferito per per queste ragioni:

- Disabilitare i moduli è più esplicito che ometterli dal primo `formato` di livello
- I nuovi moduli creati saranno aggiunti al prompt come Starship viene aggiornato

## The docs say Starship is cross-shell. Why isn't my preferred shell supported?

Il modo in cui Starship è costruito, dovrebbe rendere possibile aggiungere il supporto per qualsiasi shell. Il binario di Starship è apolide e indipendente dalla shell, fino a quando la tua shell supporterà prompt personalizzati, Starship può essere utilizzato.

Ecco un piccolo esempio per avere Starship lavorando con bash:

```sh
# Ottenere lo status code dall'ultimo comando eseguito
STATUS=$?

# Ottieni il numero di processi in esecuzione.
NUM_JOBS=$(jobs -p | wc -l)

# Imposta il prompt come output di `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#command-duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

Per un elenco di tutti i flag accettati da `starship prompt`, utilizzare il seguente comando:

```sh
starship prompt --help
```

The prompt will use as much context as is provided, but no flags are "required".

## Come faccio a eseguire le distribuzioni Starship su Linux con vecchie versioni di glibc?

Se si ottiene un errore come "_versione 'GLIBC_2. 8' non trovato (richiesta da Starship)_" quando si utilizza il binario precostruito (per esempio, su CentOS 6 o 7), puoi usare un binario compilato con `musl` invece di `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```

## I see symbols I don't understand or expect, what do they mean?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- In locale sia impostato un valore UTF-8, come `de_DE.UTF-8` o `ja_JP.UTF-8`. Se `LC_ALL` non è un valore UTF-8, [dovrai cambiarlo](https://www.tecmint.com/set-system-locales-in-linux/).
- Hai un font emoji installato. La maggior parte dei sistemi ha un font emoji per impostazione predefinita, ma alcuni (in particolare Arch Linux) non lo fanno. Di solito puoi installarne uno attraverso il gestore dei pacchetti del tuo sistema-[noto emoji](https://www.google.com/get/noto/help/emoji/) è uno dei popolari.
- Stai usando un [font Nerd](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## How do I uninstall Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Rimuovi qualsiasi riga utilizzata per inizializzare Starship nella configurazione della tua shell (ad es. `~/.bashrc`).
1. Elimina il binario di Starship.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the `curl | bash` script, the following command will delete the binary:

```sh
# Individua ed elimina il binario di Starship
rm "$(che starship)"
```
