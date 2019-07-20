<h1 align="center">
	<br>
	<img width="400" src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png" alt="Starship – Cross-shell prompt">
    <p align="center">
        <a href="https://crates.io/crates/starship"><img src="https://badgen.net/crates/v/starship" alt="Crates.io version"></a>
        <a href="https://dev.azure.com/starship-control/starship/_build"><img src="https://badgen.net/azure-pipelines/starship-control/starship/Starship%20Test%20Suite" alt="Azure Pipelines Build Status"></a>
        <a href="#contributors"><img src="https://badgen.net/badge/all%20contributors/7/orange" alt="All Contributors"></a>
        <a href="https://discord.gg/8Jzqu3T"><img src="https://badgen.net/badge/chat/on%20discord/7289da" alt="Chat on Discord"></a>
    </p>
	<br>
</h1>

<h4 align="center"></h4>
<h4 align="center">
    <a href="https://starship.rs">Website</a> · 
    <a href="#installation">Installation</a> · 
    <a href="https://starship.rs/config/">Configuration</a>
</h4>

Starship is the minimal, blazing fast, and extremely customizable prompt for any shell!
The prompt shows information need while you're working, while staying sleek and out of the way.

<p align="center">
  <img alt="Starship with Hyper and One Dark" src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif">
</p>

## Features

- Prompt character turns red if the last command exits with non-zero code.
- Current username if not the same as the logged-in user.
- Current Node.js version(`⬢`).
- Current Rust version (`🦀`).
- Current Python version (`🐍`).
- Current Go version (`🐹`).
- Package version of package in current directory (`📦`).
- Current battery level and status
- Current Git branch and rich repo status:
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
- [PLANNED #80](https://github.com/starship/starship/issues/80) – Indicator for jobs in the background (`✦`).
- [PLANNED #104](https://github.com/starship/starship/issues/104) – Execution time of the last command if it exceeds the set threshold.

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) v1.33 or higher
- A [Powerline font](https://github.com/powerline/fonts)  installed and enabled in your terminal (for example, try [Fira Code](https://github.com/tonsky/FiraCode)).

### Getting Started

1. Install the **starship** binary:

   ```sh
   cargo install starship
   ```

1. Add the init script to your shell's config file:

   #### Bash / Zsh

   Add the following to the end of `~/.bashrc` or `~/.zshrc`:

   ```sh
   # ~/.bashrc or ~/.zshrc

   eval "$(starship init $0)"
   ```

   #### Fish

   Add the following to the end of `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   eval (starship init fish)
   ```

## Configuration

For details on how to configure Starship, check out our [documentation](https://starship.rs/config/).

## Contribution

If you are interested in helping contribute to starship, please take a look at our [Contributing Guide](./CONTRIBUTING.md).

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore -->
<table>
  <tr>
    <td align="center"><a href="https://twitter.com/matchai"><img src="https://avatars0.githubusercontent.com/u/4658208?v=4" width="100px;" alt="Matan Kushner"/><br /><sub><b>Matan Kushner</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=matchai" title="Code">💻</a> <a href="#design-matchai" title="Design">🎨</a> <a href="#ideas-matchai" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-matchai" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-matchai" title="Maintenance">🚧</a> <a href="#review-matchai" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=matchai" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/johnletey"><img src="https://avatars0.githubusercontent.com/u/30328854?v=4" width="100px;" alt="John Letey"/><br /><sub><b>John Letey</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=johnletey" title="Code">💻</a> <a href="#ideas-johnletey" title="Ideas, Planning, & Feedback">🤔</a> <a href="#review-johnletey" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=johnletey" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://timmulqueen.com"><img src="https://avatars1.githubusercontent.com/u/6132021?v=4" width="100px;" alt="Tim Mulqueen"/><br /><sub><b>Tim Mulqueen</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Multimo" title="Code">💻</a> <a href="#ideas-Multimo" title="Ideas, Planning, & Feedback">🤔</a> <a href="#review-Multimo" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=Multimo" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/sirMerr"><img src="https://avatars2.githubusercontent.com/u/11183523?v=4" width="100px;" alt="Tiffany Le-Nguyen"/><br /><sub><b>Tiffany Le-Nguyen</b></sub></a><br /><a href="#ideas-sirMerr" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-sirMerr" title="Maintenance">🚧</a> <a href="#review-sirMerr" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=sirMerr" title="Documentation">📖</a></td>
    <td align="center"><a href="https://about.snuggi.es"><img src="https://avatars0.githubusercontent.com/u/26250962?v=4" width="100px;" alt="​Snuggle"/><br /><sub><b>​Snuggle</b></sub></a><br /><a href="#design-Snuggle" title="Design">🎨</a> <a href="#ideas-Snuggle" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-Snuggle" title="Maintenance">🚧</a> <a href="#review-Snuggle" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/mehcode"><img src="https://avatars1.githubusercontent.com/u/753919?v=4" width="100px;" alt="Ryan Leckey"/><br /><sub><b>Ryan Leckey</b></sub></a><br /><a href="#review-mehcode" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/youssefhabri"><img src="https://avatars3.githubusercontent.com/u/1578005?v=4" width="100px;" alt="Youssef Habri"/><br /><sub><b>Youssef Habri</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=youssefhabri" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/chipbuster"><img src="https://avatars2.githubusercontent.com/u/4605384?v=4" width="100px;" alt="Kevin Song"/><br /><sub><b>Kevin Song</b></sub></a><br /><a href="https://github.com/starship/starship/issues?q=author%3Achipbuster" title="Bug reports">🐛</a> <a href="https://github.com/starship/starship/commits?author=chipbuster" title="Code">💻</a></td>
  </tr>
</table>

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

<p align="center">
    <br>
    <img width="100" src="media/icon.png" alt="Starship rocket icon">
</p>

## License

[ISC Licensed](./LICENSE)

Copyright © 2019-present, Matan Kushner
