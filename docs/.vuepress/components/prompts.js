const prompts = [
    {
        githubId: "starship",
        displayName: "Starship",
        presets: [
            {
                id: "nf-symbols",
                name: "Nerd Font Symbols",
                info: "",
                external: null,
            },
            {
                id: "powerline",
                name: "Powerline style",
                info: "",
                external: null,
            },
        ],
    },
    {
        githubId: "mickimnet",
        displayName: "M.y.t.h.",
        presets: [
            {
                id: "dark-pointed",
                name: "Dark/Pointed",
                info: "The symbols for the modules are all based on the original logos, if available in Nerd Font. Additionally, the color of the modules are based on the colors of the services/languages/tools.",
                external: "https://raw.githubusercontent.com/mickimnet/myth-prompt-themes/main/dark/pointed/starship/starship.toml",
            },
            {
                id: "dark-slanted",
                name: "Dark/Slanted",
                info: "The symbols for the modules are all based on the original logos, if available in Nerd Font. Additionally, the color of the modules are based on the colors of the services/languages/tools.",
                external: "https://raw.githubusercontent.com/mickimnet/myth-prompt-themes/main/dark/slanted/starship/starship.toml",
            },
        ],
    },
    {
        githubId: "nukopy",
        displayName: "nukopy",
        presets: [],
    },
]

export default prompts;
