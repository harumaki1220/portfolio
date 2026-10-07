use leptos::prelude::*;

use crate::home::Home;
use crate::route::{Page, use_page};
use crate::skills::SkillsPage;
use crate::theme::ThemeToggle;
use crate::works::WorksPage;

#[component]
pub fn App() -> impl IntoView {
    let page = use_page();

    // ページが変わったら、タブのタイトルを変えて一番上までスクロールする
    Effect::new(move || {
        let page = page.get();
        document().set_title(page.title());
        window().scroll_to_with_x_and_y(0.0, 0.0);
    });

    view! {
        <ThemeToggle />
        {move || match page.get() {
            Page::Home => view! { <Home /> }.into_any(),
            Page::Skills => view! { <SkillsPage /> }.into_any(),
            Page::Works => view! { <WorksPage /> }.into_any(),
        }}
    }
}

/// サブページ共通の枠。トップに戻るリンクと見出しを付ける。
#[component]
pub fn SubPage(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <main class="home">
            <header>
                <a class="back-link" href="#/">
                    "← Home"
                </a>
                <h1 class="page-title">{title}</h1>
            </header>
            {children()}
        </main>
    }
}
