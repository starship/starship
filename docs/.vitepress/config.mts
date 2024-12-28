import { defineConfig } from "vitepress";

const sidebar = (lang: string | undefined, override = {}) =>
    [
        { page: "guide", text: "Guide" }, // README, which should always have a override
        // Overrides for any page below is an inconsistency between the sidebar title and page title
        { page: "installing", text: "Installation" },
        { page: "config", text: "Configuration" },
        { page: "advanced-config", text: "Advanced Configuration" },
        { page: "faq", text: "FAQ" },
        { page: "presets", text: "Presets" },
    ].map(item => {
        let path = "/";

        if (lang) {
            path += `${lang}/`;
        }

        if (item.page) {
            path += `${item.page}/`;
        }

        // If no override is set for current page, let VitePress fallback to page title
        return { link: path, text: override?.[item.page] ?? item.text };
    });

const editLinkPattern = 'https://github.com/starship/starship/edit/master/docs/:path';

export default defineConfig({
    locales: {
        root: {
            label: "English",
            lang: "en-US",
            title: "Starship",
            description: "The minimal, blazing-fast, and infinitely customizable prompt for any shell!",

            themeConfig: {
                // Custom navbar values
                nav: [{ text: "Configuration", link: "/config/"}],
                // Custom sidebar values
                sidebar: sidebar("", {
                    guide: "Guide",
                }),
                // Enable edit links
                editLink: {
                    text: "Edit this page on GitHub",
                    pattern: editLinkPattern,
                },
            }
        },
        "de-DE": {
            label: "Deutsch",
            lang: "de-DE",
            title: "Starship",
            description: "Minimale, super schnelle und unendlich anpassbare Prompt für jede Shell!",

            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Sprachen",
                returnToTopLabel: "Zurück zum Seitenanfang",
                sidebarMenuLabel: "Menü",

                nav: [{ text: "Konfiguration", link: "/de-DE/config/" }],
                // Custom sidebar values
                sidebar: sidebar("de-DE", {
                    guide: "Anleitung",
                    installing: "Erweiterte Installation",
                    faq: "Häufig gestellte Fragen",
                    presets: "Konfigurations-Beispiele",
                }),
                editLink: {
                    text: "Bearbeite diese Seite auf GitHub",
                    pattern: editLinkPattern,
                },
            }
        },
        "es-ES": {
            label: "Español",
            lang: "es-ES",
            title: "Starship",
            description:
                "¡El prompt minimalista, ultrarápido e infinitamente personalizable para cualquier intérprete de comandos!",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Idiomas",
                returnToTopLabel: "Volver arriba",
                sidebarMenuLabel: "Menú",
                // Custom navbar values
                nav: [{ text: "Configuración", link: "/es-ES/config/" }],
                // Custom sidebar values
                sidebar: sidebar("es-ES", {
                    guide: "Guía",
                    installing: "Instalación avanzada",
                    faq: "Preguntas frecuentes",
                    presets: "Ajustes predeterminados",
                }),
                editLink: {
                    text: "Edita esta página en GitHub",
                    pattern: editLinkPattern,
                },
            },
        },
        "fr-FR": {
            label: "Français",
            lang: "fr-FR",
            title: "Starship",
            description: "L'invite minimaliste, ultra-rapide et personnalisable à l'infini pour n'importe quel shell !",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Langues",
                returnToTopLabel: "Retour en haut",
                // Custom navbar values
                nav: [{ text: "Configuration", link: "/fr-FR/config/" }],
                // Custom sidebar values
                sidebar: sidebar("fr-FR", {
                    guide: "Guide",
                    installing: "Installation avancée",
                }),
                editLink: {
                    text: "Éditez cette page sur GitHub",
                    pattern: editLinkPattern,
                },
            },
        },
        "id-ID": {
            label: "Bahasa Indonesia",
            lang: "id-ID",
            title: "Starship",
            description: "Prompt yang minimal, super cepat, dan dapat disesuaikan tanpa batas untuk shell apa pun!",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Languages",
                returnToTopLabel: "Kembali ke atas",
                // Custom navbar values
                nav: [{ text: "Konfigurasi", link: "/id-ID/config/" }],
                // Custom sidebar values
                sidebar: sidebar("id-ID", {
                    guide: "Petunjuk",
                    installing: "Advanced Installation",
                    faq: "Pertanyaan Umum",
                    presets: "Prasetel",
                }),
                editLink: {
                    text: "Sunting halaman ini di Github",
                    pattern: editLinkPattern,
                },
            },
        },
        "it-IT": {
            label: "Italiano",
            lang: "it-IT",
            title: "Starship",
            description: "Il prompt minimalista, super veloce e infinitamente personalizzabile per qualsiasi shell!",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Languages",
                returnToTopLabel: "Torna all'inizio",
                // Custom navbar values
                nav: [{ text: "Configuration", link: "/it-IT/config/" }],
                // Custom sidebar values
                sidebar: sidebar("it-IT", {
                    guide: "Guide",
                    installing: "Installazione Avanzata",
                }),
                editLink: {
                    text: "Modifica questa pagina in Github",
                    pattern: editLinkPattern,
                },
            },
        },
        "ja-JP": {
            label: "日本語",
            lang: "ja-JP",
            title: "Starship",
            description: "シェル用の最小限の、非常に高速で、無限にカスタマイズ可能なプロンプトです！",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "言語",
                returnToTopLabel: "ページの先頭へ",
                sidebarMenuLabel: "メニュー",
                // Custom navbar values
                nav: [{ text: "設定", link: "/ja-JP/config/" }],
                // Custom sidebar values
                sidebar: sidebar("ja-JP", {
                    guide: "ガイド",
                    installing: "高度なインストール",
                }),
                editLink: {
                    text: "GitHub で編集する",
                    pattern: editLinkPattern,
                },
            },
        },
        "pt-BR": {
            label: "Português do Brasil",
            lang: "pt-BR",
            title: "Starship",
            description:
                "O prompt minimalista, extremamente rápido e infinitamente personalizável para qualquer shell!",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Languages",
                returnToTopLabel: "Voltar ao topo",
                // Custom navbar values
                nav: [{ text: "Configuração", link: "/pt-BR/config/" }],
                // Custom sidebar values
                sidebar: sidebar("pt-BR", {
                    guide: "Guia",
                    installing: "Instalação avançada",
                    faq: "Perguntas frequentes",
                    presets: "Predefinições",
                }),
                editLink: {
                    text: "Edite esta página no Github",
                    pattern: editLinkPattern,
                },
            },
        },
        "ru-RU": {
            label: "Русский",
            lang: "ru-RU",
            title: "Starship",
            description: "Минималистичная, быстрая и бесконечно настраиваемая командная строка для любой оболочки!",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Языки",
                returnToTopLabel: "Наверх",
                sidebarMenuLabel: "Меню",
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
                editLink: {
                    text: "Редактировать эту страницу на GitHub",
                    pattern: editLinkPattern,
                },
            },
        },
        "uk-UA": {
            label: "Українська",
            lang: "uk-UA",
            title: "Starship",
            description: "Простий, супер швидкий та безмежно адаптивний командний рядок для будь-якої оболонки!",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Мови",
                returnToTopLabel: "Догори",
                sidebarMenuLabel: "Меню",
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
                editLink: {
                    text: "Редагувати цю сторінку на GitHub",
                    pattern: editLinkPattern,
                },
            },
        },
        "vi-VN": {
            label: "Tiếng Việt",
            lang: "vi-VN",
            title: "Starship",
            description: "Nhỏ gọn, cực nhanh, và khả năng tuỳ chỉnh vô hạn prompt cho bất kì shell nào!",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "Ngôn ngữ",
                returnToTopLabel: "Quay lại đầu trang",
                // Custom navbar values
                nav: [{ text: "Cấu hình", link: "/vi-VN/config/" }],
                // Custom sidebar values
                sidebar: sidebar("vi-VN", {
                    guide: "Hướng dẫn",
                    installing: "Cài đặt nâng cao",
                    faq: "Các hỏi thường gặp",
                }),
                editLink: {
                    text: "Chỉnh sửa trang này trên GitHub",
                    pattern: editLinkPattern,
                },
            },
        },
        "zh-CN": {
            label: "简体中文",
            lang: "zh-CN",
            title: "Starship",
            description: "轻量级、反应迅速，可定制的高颜值终端！",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "语言",
                returnToTopLabel: "返回顶部",
                sidebarMenuLabel: "目录",
                // Custom navbar values
                nav: [{ text: "配置", link: "/zh-CN/config/" }],
                // Custom sidebar values
                sidebar: sidebar("zh-CN", {
                    guide: "指南",
                    installing: "高级安装",
                    config: "配置",
                    "advanced-config": "高级配置",
                    faq: "常见问题",
                    presets: "社区配置分享",
                }),
                editLink: {
                    text: "在 GitHub 上修改此页",
                    pattern: editLinkPattern,
                },
            },
        },
        "zh-TW": {
            label: "繁體中文",
            lang: "zh-TW",
            title: "Starship",
            description: "適合任何 shell 的最小、極速、無限客製化的提示字元！",
            themeConfig: {
                // text for the language dropdown
                langMenuLabel: "語言",
                returnToTopLabel: "返回頂部",
                sidebarMenuLabel: "目錄",
                // Custom navbar values
                nav: [{ text: "設定", link: "/zh-TW/config/" }],
                // Custom sidebar values
                sidebar: sidebar("zh-TW", {
                    guide: "指引",
                    installing: "進階安裝",
                }),
                editLink: {
                    text: "在 GitHub 上修改此頁面",
                    pattern: editLinkPattern,
                },
            },
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
                async: '',
                src: "https://www.googletagmanager.com/gtag/js?id=G-N3M0VJ9NL6",
            },
        ],
        [
            "script",
            {},
            "window.dataLayer = window.dataLayer || [];\nfunction gtag(){dataLayer.push(arguments);}\ngtag('js', new Date());\ngtag('config', 'G-N3M0VJ9NL6');",
        ],
    ],
    sitemap: {
        hostname: 'https://starship.rs'
    },
    vite: {
        resolve: {
            preserveSymlinks: true
        }
    },
    cleanUrls: true,
    markdown: {
        theme: "github-dark"
    },
    ignoreDeadLinks: [
        /\/toml\/.*/,
    ],
    // VitePress doesn't support README.md as index files
    // Rewrite README.md to index.md at different levels
    rewrites: {
        "README.md": "index.md",
        ":c0/README.md": ":c0/index.md",
        ":c0/:c1/README.md": ":c0/:c1/index.md",
        ":c0/:c1/:c2/README.md": ":c0/:c1/:c2/index.md",
        ":c0/:c1/:c2/:c3/README.md": ":c0/:c1/:c2/:c3/index.md",
    },
    themeConfig: {
        logo: "/icon.png",
        socialLinks: [
            { icon: 'github', link: 'https://github.com/starship/starship' },
        ],

        // enables Algolia DocSearch
        algolia: {
            apiKey: "44118471f56286dcda7db941a043370d",
            indexName: "starship",
            appId: "M3XUO3SQOR",
        },
    }
});
