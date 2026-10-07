pub const NAME: &str = "matcha";

pub const BIO: &[&str] = &[
    "東洋大学 情報連携学部 (INIAD) 2年。",
    "Rust と TypeScript を中心に日々コードを書いています。",
];

pub struct SkillCategory {
    pub name: &'static str,
    pub main: &'static [&'static str],
    pub others: &'static [&'static str],
}

pub const SKILLS: &[SkillCategory] = &[
    SkillCategory {
        name: "Languages",
        main: &["Rust", "TypeScript"],
        others: &["C", "Python"],
    },
    SkillCategory {
        name: "Frameworks / Libraries",
        main: &["React", "Hono", "Tailwind CSS", "Zod"],
        others: &["Next.js"],
    },
    SkillCategory {
        name: "Tools",
        main: &[
            "Vite",
            "Git",
            "pnpm",
            "VS Code",
            "Docker",
            "PostgreSQL",
            "Linux (WSL2)",
        ],
        others: &[],
    },
];

pub struct Work {
    pub name: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    /// (表示名, URL)
    pub links: &'static [(&'static str, &'static str)],
}

pub const WORKS: &[Work] = &[Work {
    name: "portfolio",
    description: "このサイト。Rust と Leptos で書いて、WebAssembly としてブラウザで動かしています。",
    tech: &["Rust", "Leptos", "WebAssembly"],
    links: &[("Source", "https://github.com/harumaki1220/portfolio")],
}];

pub const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/harumaki1220"),
    ("X", "https://x.com/matcha445_dev"),
];

/// GitHub のアイコンをそのまま表示する
pub const ICON_URL: &str = "https://github.com/harumaki1220.png?size=256";

/// アイコンを描いてくれた人（表示名, リンク）
pub const ICON_CREDIT: (&str, &str) = ("@yng_hoti", "https://x.com/yng_hoti");
