# Frequently Asked Questions

## Qual è la configurazione utilizzata nella GIF demo?

- **Terminale Emulato**: [iTerm2](https://iterm2.com/)
  - **Tema**: Minimale
  - **Schema Colore**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Font**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Configurazione**: [matchai Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Chiedi**: [Starship](https://starship.rs/)

## Come posso ottenere il completamento dei comandi come mostrato nella GIF demo?

Il supporto di completamento, o autocompletamento, è fornito dalla vostra shell di scelta. Nel caso della demo, la demo è stata fatta con [Fish Shell](https://fishshell.com/), che fornisce i completamenti per impostazione predefinita. Se usi Z Shell (zsh), ti suggerirei di dare un'occhiata a [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Il formato di primo livello `` e `<module>.disabled` fanno la stessa cosa?

Sì, entrambi possono essere utilizzati per disabilitare i moduli nel prompt. Se tutto quello che pensi di fare è disabilitare i moduli, `<module>.disabled` è il modo preferito per per queste ragioni:

- Disabilitare i moduli è più esplicito che ometterli dal primo `formato` di livello
- I nuovi moduli creati saranno aggiunti al prompt come Starship viene aggiornato

## I docs dicono Starship è cross-shell. Perché la mia shell preferita non è supportata?

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

L'implementazione [Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) integrata in Starship è leggermente più complessa per consentire funzionalità avanzate come il [modulo di durata dei comandi](https://starship.rs/config/#command-duration) e per garantire che Starship sia compatibile con le configurazioni Bash preinstallate.

Per un elenco di tutti i flag accettati da `starship prompt`, utilizzare il seguente comando:

```sh
starship prompt --help
```

Il prompt userà tanto contesto quanto è fornito, ma nessun flag è "richiesto".

## Come faccio a eseguire le distribuzioni Starship su Linux con vecchie versioni di glibc?

Se si ottiene un errore come "_versione 'GLIBC_2. 8' non trovato (richiesta da Starship)_" quando si utilizza il binario precostruito (per esempio, su CentOS 6 o 7), puoi usare un binario compilato con `musl` invece di `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Why do I see `Executing command "..." timed out.` warnings?

Starship executes different commands to get information to display in the prompt, for example the version of a program or the current git status. To make sure starship doesn't hang while trying to execute these commands we set a time limit, if a command takes longer than this limit starship will stop the execution of the command and output the above warning, this is expected behaviour. This time limit is configurable using the [`command_timeout`key](/config/#prompt) so if you want you can increase the time limit. You can also follow the debugging steps below to see which command is being slow and see if you can optimise it. Finally you can set the `STARSHIP_LOG` env var to `error` to hide these warnings.

## Vedo simboli che non capisco o mi aspetto, cosa significano?

Se vedi simboli che non riconosci puoi usare `starship explain` per la spiegazione dei moduli correntemente mostrato.

## Starship is doing something unexpected, how can I debug it?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that to blame.

```sh
env STARSHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a GitHub issue.

```sh
starship bug-report
```

## Perché non vedo un simbolo di glifo nel mio prompt?

La causa più comune è la configurazione errata del sistema. Alcune distribuzioni Linux in particolare non vengono fornite con il supporto dei font come impostazione predefinita. È necessario assicurarsi che:

- In locale sia impostato un valore UTF-8, come `de_DE.UTF-8` o `ja_JP.UTF-8`. Se `LC_ALL` non è un valore UTF-8, [dovrai cambiarlo](https://www.tecmint.com/set-system-locales-in-linux/).
- Hai un font emoji installato. La maggior parte dei sistemi ha un font emoji per impostazione predefinita, ma alcuni (in particolare Arch Linux) non lo fanno. Di solito puoi installarne uno attraverso il gestore dei pacchetti del tuo sistema-[noto emoji](https://www.google.com/get/noto/help/emoji/) è uno dei popolari.
- Stai usando un [font Nerd](https://www.nerdfonts.com/).

Per testare il sistema, eseguire i seguenti comandi in un terminale:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

La prima riga dovrebbe riprodurre una [emoji di un serpente](https://emojipedia.org/snake/), mentre la seconda dovrebbe riprodurre il [simbolo powerline di ramo (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Se uno dei due simboli non viene visualizzato correttamente, il sistema è ancora mal configurato. Sfortunatamente, ottenere la configurazione dei caratteri corretta a volte è difficile. Gli utenti su Discord potrebbero essere in grado di aiutarti. Se entrambi i simboli vengono visualizzati correttamente, ma non li vedi ancora in starship, [segnala un bug!](https://github.com/starship/starship/issues/new/choose)

## Come posso disinstallare Starship?

Starship è altrettanto facile da disinstallare come lo è da installare.

1. Rimuovi qualsiasi riga utilizzata per inizializzare Starship nella configurazione della tua shell (ad es. `~/.bashrc`).
1. Elimina il binario di Starship.

Se Starship è stato installato utilizzando un gestore di pacchetti, fai riferimento alla documentazione per le istruzioni di disinstallazione.

Se Starship è stato installato utilizzando lo script di installazione, il seguente comando eliminerà il file binario:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(command -v 'starship')"'
```
