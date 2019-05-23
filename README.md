<h3 align="center">Starship <img src="https://raw.githubusercontent.com/starship/starship/master/svg/comet.svg?sanitize=true"><img src="https://raw.githubusercontent.com/starship/starship/master/svg/galaxy.svg?sanitize=true"></h3>
<p align="center">The cross-shell prompt for astronauts.</p>
<p align="center">
    <a href="https://crates.io/crates/starship"><img src="https://badgen.net/crates/v/starship" alt="Crates.io version"></a>
    <a href="https://dev.azure.com/starship-control/starship/_build"><img src="https://badgen.net/azure-pipelines/starship-control/starship/Starship%20Test%20Suite" alt="Azure Pipelines build status"></a>
    <a href="https://deps.rs/repo/github/starship/starship"><img src="https://deps.rs/repo/github/starship/starship/status.svg" alt="Crates.io version"></a>
    <br>
    <a href="#contributors"><img src="https://badgen.net/badge/all%20contributors/6/orange" alt="All Contributors"></a>
    <a href="https://discord.gg/8Jzqu3T"><img src="https://badgen.net/badge/chat/on%20discord/7289da" alt="Chat on Discord"></a>
</p>

---

Starship is a Rust port of the minimalistic, powerful, and extremely customizable prompt [Spaceship ZSH](https://github.com/denysdovhan/spaceship-prompt).


## Development plans

The project is beginning as a port of Spaceship ZSH, but will be improved in areas where Spaceship ZSH was previously limited:

- Speed
- Concurrency of segment logic execution
    - Memoization of expensive operations
- Safety and error handling
- Testability of code
- Configuration
    - Cross-shell support with JSON or TOML config files

We will _not_ be aiming to achieve full parity with Spaceship ZSH as a result, so I am very open to discussing and reevaluating new solutions and ideas for the prompt.

I'm very new to Rust, so any help is appreciated when it comes to improving development patterns, writing idiomatic Rust, performance, safety, etc. 😄

### Prompt segments

- [x] Prompt character turns red if the last command exits with non-zero code.
- [x] Current Node.js version(`⬢`).
- [x] Current Rust version (`🦀`).
- [x] Current Python version (`🐍`).
- [x] Current Go version (`🐹`).
- [x] Package version of package in current directory (`📦`).
- [x] Current battery level and status
- [x] Current Git branch and rich repo status:
    - `=` — conflicting changes
    - `⇡` — ahead of remote branch
    - `⇣` — behind of remote branch
    - `⇕` — diverged changes
    - `?` — untracked changes
    - `$` — stashed changes
    - `!` — modified files
    - `+` — added files
    - `»` — renamed files
    - `✘` — deleted files
- [ ] Indicator for jobs in the background (`✦`).
- [ ] Execution time of the last command if it exceeds the set threshold.

### Other features

- [ ] `.starshiprc` configuration (JSON or TOML)
- [ ] Custom sections given commands or binaries
- [ ] Self-updating

### Shell support

- [x] Fish shell
- [x] Z Shell
- [ ] Bash

### Test strategy

- [x] Per-segment benchmarking
- [x] Per-segment unit + integration tests
- [ ] Shell + OS matrix acceptance tests

## Setup

### Prerequisites

- Rust v1.33 or higher

### Getting Started

1. `cd` to the root of the **starship** repo
1. Install the **starship** binary:

    ```bash
    cargo install --path .
    ```

1. Navigate to the adapters directory

    ```bash
    cd adapters
    ```

#### Fish

1. Install the fish shell prompt with fisher:

    ```bash
    fisher $PWD
    ```

#### oh-my-zsh

1. Install the zsh theme:

    ```bash
    cp starship.zsh-theme ~/.oh-my-zsh/themes
    ```

1. In `~/.zshrc`, find the line with `ZSH_THEME=` and set it to

    ```bash
    ZSH_THEME="starship"
    ```

## Contributing

To test locally run the below command:

```bash
cargo run -- $status
```

## Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore -->
<table><tr><td align="center"><a href="https://twitter.com/matchai"><img src="https://avatars0.githubusercontent.com/u/4658208?v=4" width="100px;" alt="Matan Kushner"/><br /><sub><b>Matan Kushner</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=matchai" title="Code">💻</a> <a href="#design-matchai" title="Design">🎨</a> <a href="#ideas-matchai" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-matchai" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-matchai" title="Maintenance">🚧</a> <a href="#review-matchai" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=matchai" title="Tests">⚠️</a></td><td align="center"><a href="https://github.com/johnletey"><img src="https://avatars0.githubusercontent.com/u/30328854?v=4" width="100px;" alt="John Letey"/><br /><sub><b>John Letey</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=johnletey" title="Code">💻</a> <a href="#ideas-johnletey" title="Ideas, Planning, & Feedback">🤔</a> <a href="#review-johnletey" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=johnletey" title="Tests">⚠️</a></td><td align="center"><a href="http://timmulqueen.com"><img src="https://avatars1.githubusercontent.com/u/6132021?v=4" width="100px;" alt="Tim Mulqueen"/><br /><sub><b>Tim Mulqueen</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Multimo" title="Code">💻</a> <a href="#ideas-Multimo" title="Ideas, Planning, & Feedback">🤔</a> <a href="#review-Multimo" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=Multimo" title="Tests">⚠️</a></td><td align="center"><a href="https://github.com/sirMerr"><img src="https://avatars2.githubusercontent.com/u/11183523?v=4" width="100px;" alt="Tiffany Le-Nguyen"/><br /><sub><b>Tiffany Le-Nguyen</b></sub></a><br /><a href="#ideas-sirMerr" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-sirMerr" title="Maintenance">🚧</a> <a href="#review-sirMerr" title="Reviewed Pull Requests">👀</a></td><td align="center"><a href="https://about.snuggi.es"><img src="https://avatars0.githubusercontent.com/u/26250962?v=4" width="100px;" alt="​Snuggle"/><br /><sub><b>​Snuggle</b></sub></a><br /><a href="#design-Snuggle" title="Design">🎨</a> <a href="#ideas-Snuggle" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-Snuggle" title="Maintenance">🚧</a> <a href="#review-Snuggle" title="Reviewed Pull Requests">👀</a></td><td align="center"><a href="https://github.com/mehcode"><img src="https://avatars1.githubusercontent.com/u/753919?v=4" width="100px;" alt="Ryan Leckey"/><br /><sub><b>Ryan Leckey</b></sub></a><br /><a href="#review-mehcode" title="Reviewed Pull Requests">👀</a></td><td align="center"><a href="https://github.com/youssefhabri"><img src="https://avatars3.githubusercontent.com/u/1578005?v=4" width="100px;" alt="Youssef Habri"/><br /><sub><b>Youssef Habri</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=youssefhabri" title="Code">💻</a></td></tr></table>

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
