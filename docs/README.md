---
home: true
heroImage: /logo.svg
actionText: Get Started →
actionLink: /guide/
footer: ISC Licensed | Copyright © 2019-present Matan Kushner
---

<div class="features">
  <div class="feature">
    <h2>Compatibility First</h2>
    <p>Works on the most common shells on the most common operating systems. Use it everywhere!</p>
  </div>
  <div class="feature">
    <h2>Rust-Powered</h2>
    <p>Brings the best-in-class speed and safety of Rust, to make your prompt as quick and reliable as possible.</p>
  </div>
  <div class="feature">
    <h2>Customizable</h2>
    <p>Every little detail is customizable to your liking, to make this prompt as minimal or feature-rich as you'd like it to be.</p>
  </div>
</div>

### Quick Install

1. Install the **starship** binary:

    ```bash
    cargo install starship
    ```

1. Add the init script to your shell's config file:

    #### Bash / Zsh

    Add the following to the end of `~/.bashrc` or `~/.zshrc`:

    ```bash
    # ~/.bashrc or ~/.zshrc
    
    eval "$(starship init $0)"
    ```

    #### Fish

    Add the following to the end of `~/.config/fish/config.fish`:

    ```sh
    # ~/.config/fish/config.fish

    eval (starship init fish)
    ```
