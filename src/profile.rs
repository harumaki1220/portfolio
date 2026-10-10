pub const NAME: &str = "まっちゃ";

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

/// About ページの自己紹介。1要素が1段落になる
pub const ABOUT: &[&str] = &[
    "INIAD（東洋大学 情報連携学部）の2年生です。",
    "初めての開発は、TypeScriptのサークルで作ったオセロでした。今はRustとTypeScriptを中心に、いろいろ作っています。",
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
    description: "このサイト。Rustでのフロントエンド開発を試すためにLeptosで作りました。WebAssemblyとしてブラウザで動いています。",
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
