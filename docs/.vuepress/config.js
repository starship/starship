module.exports = {
  locales: {
    "/": {
      lang: "en-US",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    },
    "/de-DE/": {
      lang: "de-DE",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    },
    "/ru-RU/": {
      lang: "ru-RU",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    },
    "/ja-JP/": {
      lang: "ja-JP",
      title: "Starship",
      description: "The cross-shell prompt for astronauts"
    },
    "/zh-CN/": {
      lang: "zh-CN",
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
  theme: "default-prefers-color-scheme",
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
    // enables Algolia DocSearch
    algolia: {
      apiKey: "107bdc34b894d5d1dd0824b420184c2d",
      indexName: "starship"
    },
    postcss: {
      plugins: [
        require('css-prefers-color-scheme/postcss'),
      ]
    },
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
      "/de-DE/": {
        // text for the language dropdown
        selectText: "Sprachen",
        // label for this locale in the language dropdown
        label: "Deutsch",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Bearbeite diese Seite auf GitHub",
        // Custom navbar values
        nav: [{ text: "Konfiguration", link: "/config/" }],
        // Custom sidebar values
        sidebar: [
          "/de-DE/",
          ["/de-DE/guide/", "Anleitung"],
          ["/de-DE/config/", "Konfiguration"],
          ["/de-DE/advanced-config/", "Erweiterte Konfiguration"],
          ["/de-DE/faq/", "Häufig gestellte Fragen"],
          ["/de-DE/presets/", "Konfigurations-Beispiele"]
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
          "/ru-RU/",
          ["/ru-RU/guide/", "Руководство"],
          ["/ru-RU/config/", "Настройка"],
          ["/ru-RU/advanced-config/", "Расширенная Настройка"],
          ["/ru-RU/faq/", "Часто Задаваемые Вопросы"],
          ["/ru-RU/presets/", "Предустановки"]
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
          ["/ja-JP/advanced-config/", "高度な設定"],
          ["/ja-JP/faq/", "FAQ"],
          ["/ja-JP/presets/", "準備するもの"]
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
      "/zh-CN/": {
        // text for the language dropdown
        selectText: "语言",
        // label for this locale in the language dropdown
        label: "简体中文",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "在 GitHub 上修改此页",
        // Custom navbar values
        nav: [{ text: "配置", link: "/zh-CN/config/" }],
        // Custom sidebar values
        sidebar: [
          "/zh-CN/",
          ["/zh-CN/guide/", "指南"],
          ["/zh-CN/config/", "配置"],
          ["/zh-CN/advanced-config/", "高级配置"],
          ["/zh-CN/faq/", "常见问题"],
          ["/zh-CN/presets/", "社区配置分享"]
        ]
      }
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
