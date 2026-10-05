use crate::profile::{BIO, LINKS, NAME, NAME_JA, SKILLS};

pub struct File {
    pub name: &'static str,
    render: fn() -> String,
}

impl File {
    pub fn content(&self) -> String {
        (self.render)()
    }

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
        render: about_md,
    },
    File {
        name: "skills.toml",
        render: skills_toml,
    },
    File {
        name: "links.json",
        render: links_json,
    },
];

fn about_md() -> String {
    format!("# {NAME_JA} ({NAME})\n\n{}\n", BIO.join("\n"))
}

fn skills_toml() -> String {
    let skills = SKILLS
        .iter()
        .map(|skill| format!("\"{skill}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[languages]\nmain = [{skills}]\n")
}

fn links_json() -> String {
    let entries = LINKS
        .iter()
        .map(|(label, href)| format!("  \"{label}\": \"{href}\""))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("{{\n{entries}\n}}\n")
}
