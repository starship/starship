# Migrando para v0.45.0

Starship v0.45.0 é o lançamento com grandes mudanças, em preparação para a grande versão v1.0.0. Nos fizemos algumas grandes mudanças em como é feita as configurações do prompt. para aceitar uma customização maior.

Este guia tem a intenção de conduzi-lo através das grandes mudanças.

## `prompt_order` foi substituido por um formato padrão `format`

Antes da v0.45.0, `prompt_order` aceitaria um array de módulos com a ordem em que seriam renderizados pelo Starship.

Starship v0.45.0 aceita um valor `format`, permitindo uma customização do prompt fora dos módulos.

**Exemplo de configuração pre-v0.45.0**

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

**Exemplo de configuração v0.45.0**

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

## Modulo `prefix` e `suffix` foram substituídos por `format`

Antes da v0.45.0, alguns módulos aceitavam `prefix` e/ou `suffix` com o objetivo de estilizar a forma que o modulo seria renderizado.

Starship v0.45.0 aceita um valor `format`, que permite customizar como o modulo será renderizado. Em vez de definir um sufix e um prefix como variáveis de contexto, agora elas são substituídas por uma string que será a representação de como será a renderização do módulo.

**Exemplo de configuração pre-v0.45.0**

```toml
[cmd_duration]
prefix = "took "
```

**Exemplo de configuração v0.45.0**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### Módulos Afetados

#### Caractere

| Propriedades Removidas  | Substituição     |
| ----------------------- | ---------------- |
| `símbolo`               | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Alterações na configuração padrão**

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

Anteriormente, a propriedade `use_symbol_for_status` era usada para configurar o prompt para exibir o `error_symbol` quando o ultimo comando resultou em um status diferente de zero.

Com a versão v0.45.0, nós agora sempre usamos `error_symbol` após status diferentes de zero, unificando as propriedades `use_symbol_for_status` e `error_symbol`.

Para configurar o prompt para usar a configuração antiga `use_symbol_for_status = true`, adicione o seguinte em seu arquivo de configuração:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Nota:_ O elemento `charactere` adiciona automaticamente um espaço depois do mesmo, portanto ao contrario strings `format`, nós não adicionamos o espaço nos exemplos acima.

#### Tempo de execução do comando

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `prefix`               | `format`     |

**Alterações na Configuração Padrão**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Diretório

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `prefix`               | `format`     |

**Alterações na Configuração Padrão**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Variáveis de Ambiente

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `prefix`               | `format`     |
| `suffix`               | `format`     |

**Alterações na Configuração Padrão**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git commit

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `prefix`               | `format`     |
| `suffix`               | `format`     |

**Alterações na Configuração Padrão**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `prefix`               | `format`     |
| `suffix`               | `format`     |
| `show_sync_count`      | `format`     |

**Alterações na Configuração Padrão**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Anteriormente, a propriedade `show_sync_count` era usada para configurar o prompt para exibir o numero de commits que o branch estava a frente ou atrás do branch remoto.

Com a versão v0.45.0, isto foi substituído em propriedades separadas, `ahead`, `behind`, e `diverged`.

Para configurar o prompt para utilizar a configuração antiga, altere a seguinte propriedade no seu arquivo de configuração: `show_sync_count = true`:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `prefix`               | `format`     |
| `suffix`               | `format`     |

**Alterações na Configuração Padrão**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularidade

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `label`                | `format`     |
| `prefix`               | `format`     |
| `suffix`               | `format`     |

**Alterações na Configuração Padrão**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Horário

| Propriedades Removidas | Substituição  |
| ---------------------- | ------------- |
| `format`               | `time_format` |

**Alterações na Configuração Padrão**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Comandos Personalizados

| Propriedades Removidas | Substituição |
| ---------------------- | ------------ |
| `prefix`               | `format`     |
| `suffix`               | `format`     |

**Alterações na Configuração Padrão**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
