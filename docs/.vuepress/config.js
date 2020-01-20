module.exports = {
  locales: {
    "/": {
      lang: "en-US",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    },
    "/ja-JP/": {
      lang: "ja-JP",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    },
    "/zh-TW/": {
      lang: "zh-TW",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    }
  },
  // prettier-ignore
  head: [
    ["link", { rel: "icon", href: "/icon.png" }],
    ["meta", { name: "description", content: "Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and Powershell." }],
    ["meta", { property: "og:title", content: "Starship: Cross-Shell Prompt" }],
    ["meta", { property: "og:description", content: "Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and Powershell."}],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:url", content: "https://starship.rs/" }],
    ["meta", { property: "og:image", content: "https://starship.rs/icon.png" }],
    ["meta", { name: "twitter:card", content: "summary"}],
    ["meta", { name: "twitter:title", content: "Starship: Cross-Shell Prompt"}],
    ["meta", { name: "twitter:description", content: "Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and Powershell."}],
    ["meta", { name: "twitter:image", content: "https://starship.rs/icon.png"}],
    ["meta", { name: "twitter:alt", content: "Starship: Cross-Shell Prompt"}],
  ],
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
          ["/advanced-config/", "Advanced Configuration"],
          ["/faq/", "Frequently Asked Questions"],
          ["/presets/", "Presets"]
        ]
      },
      "/ja-JP/": {
        // text for the language dropdown
        selectText: "言語",
        // label for this locale in the language dropdown
        label: "日本語",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "GitHub で編集する",
        // Custom navbar values
        nav: [{ text: "設定", link: "/ja-JP/config/" }],
        // Custom sidebar values
        sidebar: [
          "/ja-JP/",
          ["/ja-JP/guide/", "ガイド"],
          ["/ja-JP/config/", "設定"],
          ["/ja-JP/advanced-config/", "高度な設定"]
        ]
      },
      "/zh-TW/": {
        // text for the language dropdown
        selectText: "語言",
        // label for this locale in the language dropdown
        label: "繁體中文",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "在 GitHub 上修改此頁面",
        // Custom navbar values
        nav: [{ text: "設定", link: "/zh-TW/config/" }],
        // Custom sidebar values
        sidebar: [
          "/zh-TW/",
          ["/zh-TW/guide/", "指引"],
          ["/zh-TW/config/", "設定"],
          ["/zh-TW/advanced-config/", "進階設定"]
        ]
      },
      "/ru-RU/": {
        // text for the language dropdown
        selectText: "Языки",
        // label for this locale in the language dropdown
        label: "Русский",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Редактировать эту страницу на GitHub",
        // Custom navbar values
        nav: [{ text: "Настройка", link: "/ru-RU/config/" }],
        // Custom sidebar values
        sidebar: [
          "/",
          ["/guide/", "Руководство"],
          ["/config/", "Настройка"],
          ["/advanced-config/", "Расширенная Настройка"],
          ["/faq/", "Часто Задаваемые Вопросы"],
          ["/presets/", "Предустановки"]
        ]
      },
    }
  },
  plugins: [
    [
      "@vuepress/google-analytics",
      {
        ga: "UA-71160903-4"
      }
    ],
    [
      "sitemap",
      {
        hostname: "https://starship.rs"
      }
    ]
  ]
};
