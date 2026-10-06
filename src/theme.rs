//! ダーク / ライトの切り替え。
//! 初期値は index.html のスクリプトが <html data-theme="..."> に設定済みなので、それを読む。

use leptos::prelude::*;

const STORAGE_KEY: &str = "theme";

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn as_str(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    fn toggled(self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

fn current() -> Theme {
    let theme = document()
        .document_element()
        .and_then(|el| el.get_attribute("data-theme"));
    match theme.as_deref() {
        Some("light") => Theme::Light,
        _ => Theme::Dark,
    }
}

fn apply(theme: Theme) {
    if let Some(el) = document().document_element() {
        let _ = el.set_attribute("data-theme", theme.as_str());
    }
    // プライベートモードなどで保存できなくても、切り替え自体はできるので無視する
    if let Ok(Some(storage)) = window().local_storage() {
        let _ = storage.set_item(STORAGE_KEY, theme.as_str());
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = RwSignal::new(current());

    let toggle = move |_| {
        let next = theme.get().toggled();
        theme.set(next);
        apply(next);
    };

    view! {
        <button
            class="theme-toggle"
            type="button"
            on:click=toggle
            aria-label=move || match theme.get() {
                Theme::Dark => "ライトモードに切り替え",
                Theme::Light => "ダークモードに切り替え",
            }
        >
            {move || match theme.get() {
                Theme::Dark => "☀",
                Theme::Light => "☾",
            }}
        </button>
    }
}
