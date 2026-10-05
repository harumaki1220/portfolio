//! エディタで開けるファイルの一覧。中身は content/ 以下に置いて、ビルド時に埋め込む。

pub struct File {
    pub name: &'static str,
    pub content: &'static str,
}

impl File {
    pub fn language(&self) -> &'static str {
        match self.name.rsplit_once('.').map(|(_, ext)| ext) {
            Some("md") => "Markdown",
            Some("toml") => "TOML",
            Some("json") => "JSON",
            Some("rs") => "Rust",
            _ => "Plain Text",
        }
    }
}

pub const FILES: &[File] = &[
    File {
        name: "about.md",
        content: include_str!("../content/about.md"),
    },
    File {
        name: "skills.toml",
        content: include_str!("../content/skills.toml"),
    },
    File {
        name: "links.json",
        content: include_str!("../content/links.json"),
    },
];
