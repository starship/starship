import { defineConfig } from "vitepress";
import * as fs from "fs";
import * as path from "path";

const moduleSections = (category: string, shortName: string, key = shortName) => {
    const categoryDir = path.join(__dirname, '..', 'config', 'modules', shortName);
    const files = fs.readdirSync(categoryDir);

    // For each module file, create a sidebar entry, and extract its title
    const items = files.filter((file: string) => file.endsWith('.md')).sort().map((file: string) => {
        const filePath = path.join(categoryDir, file);
        const content = fs.readFileSync(filePath, 'utf-8');

        // Extract title from the first markdown heading
        const match = content.match(/#\s+(.+)$/m);
        const title = match ? match[1].trim() : file.replace('.md', '');

        return {
            text: title,
            page: `config/modules/${shortName}/${file.replace('.md', '')}`
        };
    });

    return {
        text: category,
        collapsed: true,
        items: items,
        section: "configuration",
        key,
    };
}
type SidebarSectionOverride = {
    text?: string;
    items?: Record<string, string>;
};

type SidebarOverride = {
    pages?: Record<string, string>;
    labels?: Record<string, string>;
    configuration?: SidebarSectionOverride;
    [key: string]: string | Record<string, string> | SidebarSectionOverride | undefined;
};

const sidebar = (lang: string | undefined, override: SidebarOverride = {}) => {
    const withLangPrefix = (link: string) => {
        if (!lang || !link.startsWith("/")) {
            return link;
        }

        const normalized = link.endsWith("/") ? link : `${link}/`;
        const prefix = `/${lang}/`;

        if (normalized.startsWith(prefix)) {
            return normalized;
        }

        return `${prefix}${normalized.slice(1)}`;
    };

    const localizeItem = (item: any): any => {
        const pageOverride = item.page
            ? override.pages?.[item.page] ?? (typeof override?.[item.page] === "string" ? override[item.page] : undefined)
            : undefined;
        const configurationOverride =
            item.section === "configuration"
                ? (item.key ? override.configuration?.items?.[item.key] : override.configuration?.text)
                : undefined;
        const localizedText = pageOverride ?? configurationOverride ?? item.text;

        if (item.items) {
            return {
                ...item,
                text: localizedText,
                link: item.link ? withLangPrefix(item.link) : item.link,
                items: item.items.map((child: any) => localizeItem(child)),
            };
        }

        if (item.page) {
            const path = `/${lang ? `${lang}/` : ""}${item.page}/`;
            return { ...item, link: path, text: localizedText };
        }

        if (item.link) {
            return { ...item, link: withLangPrefix(item.link), text: localizedText };
        }

        return { ...item, text: localizedText };
    };

    return [
        { page: "guide", text: "Guide" }, // README, which should always have a override
        // Overrides for any page below is an inconsistency between the sidebar title and page title
        { page: "installing", text: "Installation" },
        {
            text: "Configuration",
            section: "configuration",
            items: [
                {
                    text: "Prompt",
                    page: "config/modules/prompt",
                    section: "configuration",
                    key: "prompt",
                },
                moduleSections("Core & System", "core"),
                moduleSections("Version Control", "vcs"),
                moduleSections("Programming Languages", "languages"),
                moduleSections("Cloud & Container", "cloud"),
                moduleSections("Package & Environment", "environment"),
                moduleSections("Build & Tools", "build_tools"),
                moduleSections("System Info & Status", "info"),
                moduleSections("Custom Commands", "custom"),
            ],
        },
        { page: "advanced-config", text: "Advanced Configuration" },
        { page: "faq", text: "FAQ" },
        { page: "presets", text: "Presets" },
    ].map(localizeItem);
};

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
                    configuration: {
                        text: "Konfiguration",
                        items: {
                            core: "Kern & System",
                            vcs: "Versionskontrolle",
                            languages: "Programmiersprachen",
                            cloud: "Cloud & Container",
                            environment: "Pakete & Umgebung",
                            build_tools: "Build & Werkzeuge",
                            info: "Systeminfo & Status",
                            custom: "Benutzerdefinierte Module",
                        }
                    },
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
                    configuration: {
                        text: "Configuración",
                        items: {
                            core: "Núcleo y sistema",
                            vcs: "Control de versiones",
                            languages: "Lenguajes de programación",
                            cloud: "Nube y contenedores",
                            environment: "Paquetes y entorno",
                            build_tools: "Build y herramientas",
                            info: "Información del sistema y estado",
                            custom: "Módulos personalizados",
                        }
                    },
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
                    configuration: {
                        text: "Configuration",
                        items: {
                            prompt: "Prompt",
                            core: "Noyau et système",
                            vcs: "Contrôle de version",
                            languages: "Langages de programmation",
                            cloud: "Cloud et conteneurs",
                            environment: "Paquets et environnement",
                            build_tools: "Build et outils",
                            info: "Infos système et état",
                            custom: "Modules personnalisés",
                        }
                    },
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
                    configuration: {
                        text: "Konfigurasi",
                        items: {
                            core: "Inti & Sistem",
                            vcs: "Kontrol Versi",
                            languages: "Bahasa Pemrograman",
                            cloud: "Cloud & Kontainer",
                            environment: "Paket & Lingkungan",
                            build_tools: "Build & Alat",
                            info: "Info Sistem & Status",
                            custom: "Perintah Kustom",
                        }
                    },
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
                    configuration: {
                        text: "Configurazione",
                        items: {
                            prompt: "Prompt",
                            core: "Nucleo e sistema",
                            vcs: "Controllo di versione",
                            languages: "Linguaggi di programmazione",
                            cloud: "Cloud e container",
                            environment: "Pacchetti e ambiente",
                            build_tools: "Build e strumenti",
                            info: "Info sistema e stato",
                            custom: "Moduli personalizzati",
                        }
                    },
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
                    installing: "インストール",
                    configuration: {
                        text: "設定",
                        items: {
                            prompt: "プロンプト",
                            core: "コア & システム",
                            vcs: "バージョン管理",
                            languages: "プログラミング言語",
                            cloud: "クラウド & コンテナ",
                            environment: "パッケージ & 環境",
                            build_tools: "ビルド & ツール",
                            info: "システム情報 & ステータス",
                            custom: "カスタムコマンド",
                        }
                    },
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
                    configuration: {
                        text: "Configuração",
                        items: {
                            prompt: "Prompt",
                            core: "Núcleo e sistema",
                            vcs: "Controle de versão",
                            languages: "Linguagens de programação",
                            cloud: "Nuvem e contêineres",
                            environment: "Pacotes e ambiente",
                            build_tools: "Build e ferramentas",
                            info: "Informações do sistema e status",
                            custom: "Módulos personalizados",
                        }
                    },
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
                    configuration: {
                        text: "Настройка",
                        items: {
                            prompt: "Промпт",
                            core: "Ядро и Система",
                            vcs: "Система контроля версий",
                            languages: "Языки программирования",
                            cloud: "Облако и Контейнеры",
                            environment: "Пакеты и Окружение",
                            build_tools: "Сборка и Инструменты",
                            info: "Системная информация и Статус",
                            custom: "Пользовательские команды",
                        }
                    },
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
                    configuration: {
                        text: "Налаштування",
                        items: {
                            prompt: "Запит",
                            core: "Ядро та Система",
                            vcs: "Контроль версій",
                            languages: "Мови програмування",
                            cloud: "Хмара та Контейнери",
                            environment: "Пакети та Середовище",
                            build_tools: "Збірка та Інструменти",
                            info: "Системна інформація та Статус",
                            custom: "Користувацькі команди",
                        }
                    },
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
                    configuration: {
                        text: "Cấu hình",
                        items: {
                            prompt: "Dấu nhắc",
                            core: "Cốt lõi & Hệ thống",
                            vcs: "Kiểm soát phiên bản",
                            languages: "Ngôn ngữ lập trình",
                            cloud: "Đám mây & Container",
                            environment: "Gói & Môi trường",
                            build_tools: "Công cụ & Xây dựng",
                            info: "Thông tin hệ thống & Trạng thái",
                            custom: "Lệnh tùy chỉnh",
                        }
                    },
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
                    configuration: {
                        text: "配置",
                        items: {
                            prompt: "提示符",
                            core: "核心 & 系统",
                            vcs: "版本控制",
                            languages: "编程语言",
                            cloud: "云 & 容器",
                            environment: "包 & 环境",
                            build_tools: "构建 & 工具",
                            info: "系统信息 & 状态",
                            custom: "自定义命令",
                        }
                    },
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
                    configuration: {
                        text: "設定",
                        items: {
                            prompt: "提示字元",
                            core: "核心 & 系統",
                            vcs: "版本控制",
                            languages: "程式語言",
                            cloud: "雲端 & 容器",
                            environment: "套件 & 環境",
                            build_tools: "建置 & 工具",
                            info: "系統資訊 & 狀態",
                            custom: "自定義指令",
                        }
                    },
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
