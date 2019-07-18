# Usage

## Getting Started

1. Install the **starship** binary:

   ```bash
   cargo install starship
   ```

2. Add the init script to your shell's config file:

   #### Bash / Zsh

   Add the following to the end of `~/.bashrc` or `~/.zshrc`:

   ```bash
   # ~/.bashrc or ~/.zshrc

   eval "$(starship init $0)"
   ```

   #### Fish

   Add the following to the end of `~/.config/fish/config.fish`:

   ```fish
   # ~/.config/config.fish

   eval (starship init fish)
   ```
