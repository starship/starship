<h3 align="center">Starship ✨🚀</h3>
<p align="center">The cross-shell prompt for astronauts.</p>

---

> ⚠️ This is very much work in progress, so please don't try to use it just yet!

Starship is a Rust port of the minimalistic, powerful, and extremely customizable prompt [Spaceship ZSH](https://github.com/denysdovhan/spaceship-prompt).


## Features

### Prompt segments

- [x] Prompt character turns red if the last command exits with non-zero code.
- [x] Current Node.js version(`⬢`).
- [ ] Current Rust version (`𝗥`).
- [ ] Current battery level and status
- [ ] Current Git branch and rich repo status.
- [ ] Indicator for jobs in the background (`✦`).
- [ ] Execution time of the last command if it exceeds the set threshold.

### Shell support

- [x] Fish shell
- [ ] Z Shell
- [ ] Bash

## Setup

### Prerequisites

- Rust v1.33 or higher
- Fisher v3

### Getting Started

1. `cd` to the root of the **starship** repo
1. Install the **starship** binary:

    ```bash
    cargo install --path .
    ```

1. Install the fish shell prompt with fisher:

    ```bash
    fisher $PWD
    ```
