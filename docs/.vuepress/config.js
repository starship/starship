module.exports = {
  locales: {
    "/": {
      lang: "en-US",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    },
    "/ja/": {
      lang: "ja-JP",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    }
  },
  head: [["link", { rel: "icon", href: "/icon.png" }]],
  themeConfig: {
    logo: "/icon.png",
    // the GitHub repo path
    repo: "starship/starship",
    // the label linking to the repo
    repoLabel: "GitHub",
    // if your docs are not at the root of the repo:
    docsDir: "docs",
    // defaults to false, set to true to enable
    editLinks: true,
    locales: {
      "/": {
        // text for the language dropdown
        selectText: "Languages",
        // label for this locale in the language dropdown
        label: "English",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Edit this page on GitHub",
        // Custom navbar values
        nav: [{ text: "Configuration", link: "/config/" }],
        // Custom sidebar values
        sidebar: [
          "/",
          ["/guide/", "Guide"],
          ["/config/", "Configuration"],
          ["/advanced-config/", "Advanced Configuration"]
        ],
      },
      "/ja/": {
        // text for the language dropdown
        selectText: "Languages",
        // label for this locale in the language dropdown
        label: "日本語",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Edit this page on GitHub",
        // Custom navbar values
        nav: [{ text: "Configuration", link: "/config/" }],
        // Custom sidebar values
        sidebar: [
          "/",
          ["/guide/", "Guide"],
          ["/config/", "Configuration"],
          ["/advanced-config/", "Advanced Configuration"]
        ],
      },
    }
  },
  plugins: [
    [
      "@vuepress/google-analytics",
      {
        ga: "UA-71160903-4"
      }
    ]
  ]
};
