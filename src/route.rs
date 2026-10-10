//! URL のハッシュ（`#/works` など）で表示するページを決める。
//! ハッシュはサーバーに送られないので、GitHub Pages でも直接開いたりリロードしたりできる。

use leptos::{ev, prelude::*};

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Works,
    Blogs,
}

impl Page {
    /// ナビゲーションに並べる順番
    pub const ALL: [Page; 3] = [Page::Home, Page::Works, Page::Blogs];

    fn from_hash(hash: &str) -> Self {
        match hash {
            "#/works" => Page::Works,
            "#/blogs" => Page::Blogs,
            // 空や知らないハッシュはトップに戻す
            _ => Page::Home,
        }
    }

    pub fn href(self) -> &'static str {
        match self {
            Page::Home => "#/",
            Page::Works => "#/works",
            Page::Blogs => "#/blogs",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Works => "Works",
            Page::Blogs => "Blogs",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Page::Home => "まっちゃ",
            Page::Works => "Works | まっちゃ",
            Page::Blogs => "Blogs | まっちゃ",
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
