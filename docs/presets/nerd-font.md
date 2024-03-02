# Nerd Font Symbols Preset

This preset changes the symbols for each module to use Nerd Font symbols.

![Screenshot of Nerd Font Symbols preset](/presets/img/nerd-font-symbols.png)

### Prerequisites

Before using this preset, ensure you have a Nerd Font installed and enabled in your terminal. Follow the steps below to install Nerd Fonts on your system:

#### Linux (Ubuntu)

1. Download your preferred Nerd Font from the [Nerd Fonts website](https://www.nerdfonts.com/).
2. Extract the font archive.
3. Copy the extracted font files to the appropriate system font directory. You can usually find this directory at `/usr/share/fonts` or `~/.local/share/fonts`. Alternatively, you can create a new directory in either location specifically for custom fonts.
4. Update the font cache by running the following command in your terminal:
    ```sh
    fc-cache -fv
    ```
5. Enable the Nerd Font in your terminal emulator. This process varies depending on the terminal emulator you are using. Typically, you can find this setting in the terminal's preferences or settings menu.

#### macOS

1. Download your preferred Nerd Font from the [Nerd Fonts website](https://www.nerdfonts.com/).
2. Open the downloaded font file (usually a `.zip` file).
3. Double-click on the font file(s) to install them on your system.
4. Restart your terminal emulator to apply the changes.

#### Windows

1. Download your preferred Nerd Font from the [Nerd Fonts website](https://www.nerdfonts.com/).
2. Open the downloaded font file (usually a `.zip` file).
3. Extract the font file(s).
4. Right-click on each font file and select "Install" from the context menu.
5. Restart your terminal emulator to apply the changes.

### Configuration

Once you have installed and enabled a Nerd Font in your terminal, you can use the preset by following these steps:

1. Run the following command in your terminal:

    ```sh
    starship preset nerd-font-symbols -o ~/.config/starship.toml
    ```

2. Download the TOML file for the preset configuration by clicking the link below:

    [Click to download TOML](/presets/toml/nerd-font-symbols.toml)
