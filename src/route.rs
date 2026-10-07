//! URL のハッシュ（`#/skills` など）で表示するページを決める。
//! ハッシュはサーバーに送られないので、GitHub Pages でも直接開いたりリロードしたりできる。

use leptos::{ev, prelude::*};

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Skills,
    Works,
}

impl Page {
    fn from_hash(hash: &str) -> Self {
        match hash {
            "#/skills" => Page::Skills,
            "#/works" => Page::Works,
            // 空や知らないハッシュはトップに戻す
            _ => Page::Home,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Page::Home => "matcha",
            Page::Skills => "Skills | matcha",
            Page::Works => "Works | matcha",
        }
    }
}

fn current() -> Page {
    Page::from_hash(&window().location().hash().unwrap_or_default())
}

/// 今のページを返す signal。ハッシュが変わると自動で更新される。
pub fn use_page() -> ReadSignal<Page> {
    let (page, set_page) = signal(current());
    let _ = window_event_listener(ev::hashchange, move |_| set_page.set(current()));
    page
}
