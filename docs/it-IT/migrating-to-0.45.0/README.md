# Migrazione alla versione 0.45.0

Starship v0.45.0 è una versione contenente importanti cambiamenti in preparazione della grande versione 1.0.0. Abbiamo apportato alcuni importanti cambiamenti per come la configurazione viene fatta sul prompt, al fine di consentire un maggior grado di personalizzazione.

Questa guida è destinata ad attraversare questi grandi cambiamenti.

## `prompt_order` è stato sostituito da un formato root-level ``

Prima della v0.45.0, `prompt_order` avrebbe accettato un array di nomi di moduli nell'ordine in cui sarebbero stati renderizzati da Starship.

Starship v0.45.0 invece accetta un valore `format`, consentendo la personalizzazione del prompt al di fuori dei moduli stessi.

**Esempio di configurazione pre-v0.45.0**

```toml
prompt_order = [
  "username",
  "hostname",
  "directory",
  "git_branch",
  "git_commit",
  "git_state",
  "git_status",
  "cmd_duration",
  "custom",
  "line_break",
  "jobs",
  "battery",
  "time",
  "character",
]
```

**Esempio di configurazione v0.45.0**

```toml
format = """\
  $username\
  $hostname\
  $directory\
  $git_branch\
  $git_commit\
  $git_state\
  $git_status\
  $cmd_duration\
  $custom\
  $line_break\
  $jobs\
  $battery\
  $time\
  $character\
  """
```

## Il prefisso `del modulo` e il suffisso `` sono stati sostituiti dal formato ``

Precedentemente la v0.45.0, alcuni moduli accetterebbero `prefisso` e/o `suffisso` per stilare il modo in cui i moduli vengono renderizzati.

Starship v0.45.0 invece accetta un valore `format`, consentendo la personalizzazione del prompt al di fuori dei moduli stessi. Invece di definire un prefisso e un suffisso per le variabili basate sul contesto, le variabili possono ora essere sostituite da una stringa di formato, che rappresenta l'output del modulo.

**Esempio di configurazione pre-v0.45.0**

```toml
[cmd_duration]
prefisso = "tak"
```

**Esempio di configurazione v0.45.0**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### Moduli soggetti

#### Character

| Proprietà rimossa       | Sostituta        |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Modifiche alla configurazione predefinita**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

Precedentemente, la proprietà `use_symbol_for_status` è stata utilizzata per configurare il prompt per mostrare `error_symbol` quando l'ultimo comando ha prodotto un codice di stato diverso da zero.

Con il rilascio di v0.45.0, ora usiamo sempre `error_symbol` dopo codici di stato diversi da zero, unificando le proprietà `use_symbol_for_status` e `error_symbol`.

Per configurare il prompt al fine di usare la vecchia configurazione `use_symbol_for_status = true`, aggiungi quanto segue al tuo file di configurazione:

```toml
[character]
error_symbol = "[✖](bold red)"
```

*Nota:* L'elemento `carattere` aggiunge dopo automaticamente uno spazio, quindi a differenza delle altre stringhe `formato`, non ne aggiungiamo uno specificamente agli esempi di cui sopra.

#### Command Duration

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `prefix`          | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `prefix`          | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Variabili di ambiente

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `prefix`          | `format`  |
| `suffix`          | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `prefix`          | `format`  |
| `suffix`          | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `prefix`          | `format`  |
| `suffix`          | `format`  |
| `show_sync_count` | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Precedentemente, la proprietà `show_sync_count` è stata utilizzata per configurare il prompt per mostrare il numero di commit che il ramo era avanti o dietro il ramo in remoto.

Con il rilascio della v0.45.0, questo è stato sostituito con tre proprietà separate, `ahead`, ` behind` e `diverged`.

Per configurare il prompt al fine di usare la vecchia configurazione `show_sync_count = true`, aggiungi quanto segue al tuo file di configurazione:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `prefix`          | `format`  |
| `suffix`          | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singolarità

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `label`           | `format`  |
| `prefix`          | `format`  |
| `suffix`          | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Time

| Proprietà rimossa | Sostituta     |
| ----------------- | ------------- |
| `format`          | `time_format` |

**Modifiche alla configurazione predefinita**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Comandi Personalizzati

| Proprietà rimossa | Sostituta |
| ----------------- | --------- |
| `prefix`          | `format`  |
| `suffix`          | `format`  |

**Modifiche alla configurazione predefinita**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
