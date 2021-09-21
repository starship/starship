module.exports = {
  locales: {
    "/": {
      lang: "en-US",
      title: "Starship",
      description: "The minimal, blazing-fast, and infinitely customizable prompt for any shell!"
    },
    "/de-DE/": {
      lang: "de-DE",
      title: "Starship",
      description: "Minimale, super schnelle und unendlich anpassbare Prompt für jede Shell!"
    },
    "/es-ES/": {
      lang: "es-ES",
      title: "Starship",
      description: "¡El prompt minimalista, ultrarápido e infinitamente personalizable para cualquier intérprete de comandos!"
    },
    "/fr-FR/": {
      lang: "fr-FR",
      title: "Starship",
      description: "L'invite minimaliste, ultra-rapide et personnalisable à l'infini pour n'importe quel shell !"
    },
    "/ja-JP/": {
      lang: "ja-JP",
      title: "Starship",
      description: "シェル用の最小限の、非常に高速で、無限にカスタマイズ可能なプロンプトです！"
    },
    "/pt-BR/": {
      lang: "pt-BR",
      title: "Starship",
      description: "O prompt minimalista, extremamente rápido e infinitamente personalizável para qualquer shell!"
    },
    "/ru-RU/": {
      lang: "ru-RU",
      title: "Starship",
      description: "Минималистичная, быстрая и бесконечно настраиваемая командная строка для любой оболочки!"
    },
    "/vi-VN/": {
      lang: "vi-VN",
      title: "Starship",
      description: "Nhỏ gọn, cực nhanh, và khả năng tuỳ chỉnh vô hạn prompt cho bất kì shell nào!"
    },
    "/zh-CN/": {
      lang: "zh-CN",
      title: "Starship",
      description: "轻量级、反应迅速，可定制的高颜值终端！"
    },
    "/zh-TW/": {
      lang: "zh-TW",
      title: "Starship",
      description: "適合任何 shell 的最小、極速、無限客製化的提示字元！"
    }
  },
  // prettier-ignore
  head: [
    ["link", { rel: "icon", href: "/icon.png" }],
    ["meta", { property: "og:title", content: "Starship: Cross-Shell Prompt" }],
    ["meta", { property: "og:description", content: "Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, and Powershell."}],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:url", content: "https://starship.rs/" }],
    ["meta", { property: "og:image", content: "https://starship.rs/icon.png" }],
    ["meta", { name: "twitter:card", content: "summary"}],
    ["meta", { name: "twitter:title", content: "Starship: Cross-Shell Prompt"}],
    ["meta", { name: "twitter:description", content: "Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, and Powershell."}],
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
          ["/installing/", "Advanced Installation"],
          ["/config/", "Configuration"],
          ["/advanced-config/", "Advanced Configuration"],
          ["/faq/", "Frequently Asked Questions"],
          ["/presets/", "Presets"],
          ["/migrating-to-0.45.0/", "Migrating to v0.45.0"]
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
      "/es-ES/": {
        // text for the language dropdown
        selectText: "Idiomas",
        // label for this locale in the language dropdown
        label: "Español",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Edita esta página en GitHub",
        // Custom navbar values
        nav: [{ text: "Configuración", link: "/es-ES/config/" }],
        // Custom sidebar values
        sidebar: [
          "/es-ES/",
          ["/es-ES/guide/", "Guía"],
          ["/es-ES/config/", "Configuración"],
          ["/es-ES/advanced-config/", "Configuración Avanzada"],
          ["/es-ES/faq/", "Preguntas frecuentes"],
          ["/es-ES/presets/", "Ajustes predeterminados"]
        ]
      },
      "/fr-FR/": {
        // text for the language dropdown
        selectText: "Langues",
        // label for this locale in the language dropdown
        label: "Français",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Éditez cette page sur GitHub",
        // Custom navbar values
        nav: [{ text: "Configuration", link: "/fr-FR/config/" }],
        // Custom sidebar values
        sidebar: [
          "/fr-FR/",
          ["/fr-FR/guide/", "Guide"],
          ["/fr-FR/config/", "Configuration"],
          ["/fr-FR/advanced-config/", "Configuration avancée"],
          ["/fr-FR/faq/", "Foire aux questions"],
          ["/fr-FR/presets/", "Paramètres par défaut"]
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
          ["/ja-JP/presets/", "準備するもの"],
          ["/ja-JP/migrating-to-0.45.0/", "v0.45への移行"]
        ]
      },
      "/pt-BR/": {
        // text for the language dropdown
        selectText: "Languages",
        // label for this locale in the language dropdown
        label: "Português do Brasil",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Edite esta página no Github",
        // Custom navbar values
        nav: [{ text: "Configuração", link: "/pt-BR/config/" }],
        // Custom sidebar values
        sidebar: [
          "/pt-BR/",
          ["/pt-BR/guide/", "Guia"],
          ["/pt-BR/config/", "Configuração"],
          ["/pt-BR/advanced-config/", "Configuração avançada"],
          ["/pt-BR/faq/", "Perguntas frequentes"],
          ["/pt-BR/presets/", "Predefinições"]
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
      "/vi-VN/": {
        // text for the language dropdown
        selectText: "Ngôn ngữ",
        // label for this locale in the language dropdown
        label: "Tiếng Việt",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Chỉnh sửa trang này trên GitHub",
        // Custom navbar values
        nav: [{ text: "Cấu hình", link: "/vi-VN/config/" }],
        // Custom sidebar values
        sidebar: [
          "/vi-VN/",
          ["/vi-VN/guide/", "Hướng dẫn"],
          ["/vi-VN/config/", "Cấu hình"],
          ["/vi-VN/advanced-config/", "Cấu hình nâng cao"],
          ["/vi-VN/faq/", "Các hỏi thường gặp"],
          ["/vi-VN/presets/", "Mẫu thiết lập"]
        ]
      }
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
