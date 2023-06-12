import { defineConfig, SidebarConfigArray } from "vuepress/config";

const sidebar = (lang, override = {}): SidebarConfigArray =>
    [
        "", // "Home", which should always have a override
        "guide", // README, which should always have a override
        // Overrides for any page below is an inconsistency between the sidebar title and page title
        "installing",
        "config",
        "advanced-config",
        "faq",
        "presets",
    ].map(page => {
        let path = "/";

        if (lang) {
            path += `${lang}/`;
        }

        if (page) {
            path += `${page}/`;
        }

        // If no override is set for current page, let VuePress fallback to page title
        return page in override ? [path, override[page]] : path;
    });

module.exports = defineConfig({
    locales: {
        "/": {
            lang: "en-US",
            title: "Starship",
            description: "The minimal, blazing-fast, and infinitely customizable prompt for any shell!",
        },
        "/de-DE/": {
            lang: "de-DE",
            title: "Starship",
            description: "Minimale, super schnelle und unendlich anpassbare Prompt für jede Shell!",
        },
        "/es-ES/": {
            lang: "es-ES",
            title: "Starship",
            description:
                "¡El prompt minimalista, ultrarápido e infinitamente personalizable para cualquier intérprete de comandos!",
        },
        "/fr-FR/": {
            lang: "fr-FR",
            title: "Starship",
            description: "L'invite minimaliste, ultra-rapide et personnalisable à l'infini pour n'importe quel shell !",
        },
        "/id-ID/": {
            lang: "id-ID",
            title: "Starship",
            description: "Prompt yang minimal, super cepat, dan dapat disesuaikan tanpa batas untuk shell apa pun!",
        },
        "/it-IT/": {
            lang: "it-IT",
            title: "Starship",
            description: "Il prompt minimalista, super veloce e infinitamente personalizzabile per qualsiasi shell!",
        },
        "/ja-JP/": {
            lang: "ja-JP",
            title: "Starship",
            description: "シェル用の最小限の、非常に高速で、無限にカスタマイズ可能なプロンプトです！",
        },
        "/pt-BR/": {
            lang: "pt-BR",
            title: "Starship",
            description:
                "O prompt minimalista, extremamente rápido e infinitamente personalizável para qualquer shell!",
        },
        "/ru-RU/": {
            lang: "ru-RU",
            title: "Starship",
            description: "Минималистичная, быстрая и бесконечно настраиваемая командная строка для любой оболочки!",
        },
        "/uk-UA/": {
            lang: "uk-UA",
            title: "Starship",
            description: "Простий, супер швидкий та безмежно адаптивний командний рядок для будь-якої оболонки!",
        },
        "/vi-VN/": {
            lang: "vi-VN",
            title: "Starship",
            description: "Nhỏ gọn, cực nhanh, và khả năng tuỳ chỉnh vô hạn prompt cho bất kì shell nào!",
        },
        "/zh-CN/": {
            lang: "zh-CN",
            title: "Starship",
            description: "轻量级、反应迅速，可定制的高颜值终端！",
        },
        "/zh-TW/": {
            lang: "zh-TW",
            title: "Starship",
            description: "適合任何 shell 的最小、極速、無限客製化的提示字元！",
        },
    },
    // prettier-ignore
    head: [
        ["link", { rel: "icon", href: "/icon.png" }],
        ["meta", { property: "og:title", content: "Starship: Cross-Shell Prompt" }],
        ["meta", {
            property: "og:description",
            content:
                "Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and Powershell.",
        }],
        ["meta", { property: "og:type", content: "website" }],
        ["meta", { property: "og:url", content: "https://starship.rs/" }],
        ["meta", { property: "og:image", content: "https://starship.rs/icon.png" }],
        ["meta", { name: "twitter:card", content: "summary" }],
        ["meta", { name: "twitter:title", content: "Starship: Cross-Shell Prompt" }],
        ["meta", {
            name: "twitter:description",
            content:
                "Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and Powershell.",
        }],
        ["meta", { name: "twitter:image", content: "https://starship.rs/icon.png" }],
        ["meta", { name: "twitter:alt", content: "Starship: Cross-Shell Prompt" }],
        // Google Analytics
        [
            "script",
            {
                async: true,
                src: "https://www.googletagmanager.com/gtag/js?id=G-N3M0VJ9NL6",
            },
        ],
        [
            "script",
            {},
            "window.dataLayer = window.dataLayer || [];\nfunction gtag(){dataLayer.push(arguments);}\ngtag('js', new Date());\ngtag('config', 'G-N3M0VJ9NL6');",
        ],
    ],
    evergreen: true,
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
            apiKey: "44118471f56286dcda7db941a043370d",
            indexName: "starship",
            appId: "M3XUO3SQOR",
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
                sidebar: sidebar("", {
                    guide: "Guide",
                }),
            },
            "/de-DE/": {
                // text for the language dropdown
                selectText: "Sprachen",
                // label for this locale in the language dropdown
                label: "Deutsch",
                // Custom text for edit link. Defaults to "Edit this page"
                editLinkText: "Bearbeite diese Seite auf GitHub",
                // Custom navbar values
                nav: [{ text: "Konfiguration", link: "/de-DE/config/" }],
                // Custom sidebar values
                sidebar: sidebar("de-DE", {
                    guide: "Anleitung",
                    installing: "Erweiterte Installation",
                    faq: "Häufig gestellte Fragen",
                    presets: "Konfigurations-Beispiele",
                }),
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
                sidebar: sidebar("es-ES", {
                    guide: "Guía",
                    installing: "Instalación avanzada",
                    faq: "Preguntas frecuentes",
                    presets: "Ajustes predeterminados",
                }),
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
                sidebar: sidebar("fr-FR", {
                    guide: "Guide",
                    installing: "Installation avancée",
                }),
            },
            "/id-ID/": {
                // text for the language dropdown
                selectText: "Languages",
                // label for this locale in the language dropdown
                label: "Bahasa Indonesia",
                // Custom text for edit link. Defaults to "Edit this page"
                editLinkText: "Sunting halaman ini di Github",
                // Custom navbar values
                nav: [{ text: "Konfigurasi", link: "/id-ID/config/" }],
                // Custom sidebar values
                sidebar: sidebar("id-ID", {
                    guide: "Petunjuk",
                    installing: "Advanced Installation",
                    faq: "Pertanyaan Umum",
                    presets: "Prasetel",
                }),
            },
            "/it-IT/": {
                // text for the language dropdown
                selectText: "Languages",
                // label for this locale in the language dropdown
                label: "Italiano",
                // Custom text for edit link. Defaults to "Edit this page"
                editLinkText: "Modifica questa pagina in Github",
                // Custom navbar values
                nav: [{ text: "Configuration", link: "/it-IT/config/" }],
                // Custom sidebar values
                sidebar: sidebar("it-IT", {
                    guide: "Guide",
                    installing: "Installazione Avanzata",
                }),
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
                sidebar: sidebar("ja-JP", {
                    guide: "ガイド",
                    installing: "高度なインストール",
                }),
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
                sidebar: sidebar("pt-BR", {
                    guide: "Guia",
                    installing: "Instalação avançada",
                    faq: "Perguntas frequentes",
                    presets: "Predefinições",
                }),
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
                sidebar: sidebar("ru-RU", {
                    guide: "Руководство",
                    installing: "Advanced Installation",
                    config: "Настройка",
                    "advanced-config": "Расширенная Настройка",
                    faq: "Часто Задаваемые Вопросы",
                }),
            },
            "/uk-UA/": {
                // text for the language dropdown
                selectText: "Мови",
                // label for this locale in the language dropdown
                label: "Українська",
                // Custom text for edit link. Defaults to "Edit this page"
                editLinkText: "Редагувати цю сторінку на GitHub",
                // Custom navbar values
                nav: [{ text: "Налаштування", link: "/uk-UA/config/" }],
                // Custom sidebar values
                sidebar: sidebar("uk-UA", {
                    guide: "Керівництво",
                    installing: "Розширене встановлення",
                    config: "Налаштування",
                    "advanced-config": "Розширені налаштування",
                    faq: "Часті питання",
                    presets: "Шаблони",
                }),
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
                sidebar: sidebar("vi-VN", {
                    guide: "Hướng dẫn",
                    installing: "Cài đặt nâng cao",
                    faq: "Các hỏi thường gặp",
                }),
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
                sidebar: sidebar("zh-TW", {
                    guide: "指引",
                    installing: "進階安裝",
                }),
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
                sidebar: sidebar("zh-CN", {
                    guide: "指南",
                    installing: "高级安装",
                    presets: "社区配置分享",
                }),
            },
        },
    },
    plugins: [
        [
            "vuepress-plugin-sitemap",
            {
                hostname: "https://starship.rs",
            },
        ],
        ["vuepress-plugin-code-copy", true],
    ],
});
