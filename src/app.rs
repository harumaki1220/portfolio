use leptos::prelude::*;

use crate::blogs::BlogsPage;
use crate::home::Home;
use crate::route::{Page, use_page};
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
        <SiteHeader page />
        {move || match page.get() {
            Page::Home => view! { <Home /> }.into_any(),
            Page::Works => view! { <WorksPage /> }.into_any(),
            Page::Blogs => view! { <BlogsPage /> }.into_any(),
        }}
    }
}

/// 全ページ共通の上のバー。ページへのリンクとテーマ切り替え。
#[component]
fn SiteHeader(page: ReadSignal<Page>) -> impl IntoView {
    view! {
        <header class="site-header">
            <nav>
                <ul class="site-nav">
                    {Page::ALL
                        .into_iter()
                        .map(|target| {
                            view! {
                                <li>
                                    <a
                                        href=target.href()
                                        aria-current=move || (page.get() == target).then_some("page")
                                    >
                                        {target.label()}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </nav>
            <ThemeToggle />
        </header>
    }
}

/// Home 以外のページ共通の枠。見出しを付ける。
#[component]
pub fn SubPage(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <main class="home">
            <h1 class="page-title">{title}</h1>
            {children()}
        </main>
    }
}
