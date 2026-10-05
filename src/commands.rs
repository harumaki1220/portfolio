//! 入力された文字列を解釈して、表示する内容を返す。
//! 画面の描画とは切り離しているので、コマンドの追加はこのファイルだけで完結する。

#[derive(Clone)]
pub enum Line {
    Text(String),
    /// クリックで実行できるコマンド
    Command {
        name: &'static str,
        description: &'static str,
    },
}

pub enum Output {
    Lines(Vec<Line>),
    Clear,
}

const COMMANDS: &[(&str, &str)] = &[
    ("help", "コマンドの一覧を表示"),
    ("whoami", "自己紹介"),
    ("clear", "画面をクリア"),
];

pub fn run(input: &str) -> Output {
    let Some(command) = input.split_whitespace().next() else {
        return Output::Lines(Vec::new());
    };

    match command {
        "help" => Output::Lines(
            COMMANDS
                .iter()
                .map(|&(name, description)| Line::Command { name, description })
                .collect(),
        ),
        "whoami" => text(
            "楠 悠真 (Haruma Kusunoki)\n\
             東洋大学 情報連携学部 (INIAD) 2年。\n\
             Rust と TypeScript を中心に日々コードを書いています。",
        ),
        "clear" => Output::Clear,
        _ => text(format!("command not found: {command}")),
    }
}

fn text(text: impl Into<String>) -> Output {
    Output::Lines(vec![Line::Text(text.into())])
}
