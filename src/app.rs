use leptos::prelude::*;

use crate::files::{FILES, File};
use crate::terminal::Terminal;

#[component]
pub fn App() -> impl IntoView {
    // TODO: クリックでファイルを切り替えられるようにする
    let active = &FILES[0];

    view! {
        <div class="app">
            <ActivityBar />
            <Sidebar active=active.name />
            <div class="main">
                <Tabs active=active.name />
                <Editor file=active />
                <Panel />
            </div>
            <StatusBar file=active />
        </div>
    }
}

#[component]
fn ActivityBar() -> impl IntoView {
    view! {
        <nav class="activity-bar">
            <button class="activity-item active" type="button" title="Explorer">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M15 3H6a1 1 0 0 0-1 1v13" />
                    <path d="M9 7h6l4 4v9a1 1 0 0 1-1 1H9a1 1 0 0 1-1-1V8a1 1 0 0 1 1-1z" />
                </svg>
            </button>
            <button class="activity-item" type="button" title="Search">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <circle cx="10" cy="10" r="6" />
                    <path d="M14.5 14.5 20 20" />
                </svg>
            </button>
        </nav>
    }
}

#[component]
fn Sidebar(active: &'static str) -> impl IntoView {
    view! {
        <aside class="sidebar">
            <h2 class="sidebar-title">"Explorer"</h2>
            <p class="folder">"▾ portfolio"</p>
            <ul class="file-list">
                {FILES
                    .iter()
                    .map(|file| {
                        view! {
                            <li class="file" class:active=file.name == active>
                                {file.name}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </aside>
    }
}

#[component]
fn Tabs(active: &'static str) -> impl IntoView {
    view! {
        <div class="tabs">
            {FILES
                .iter()
                .map(|file| {
                    view! {
                        <div class="tab" class:active=file.name == active>
                            {file.name}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Editor(file: &'static File) -> impl IntoView {
    view! {
        <div class="editor">
            {file
                .content
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    view! {
                        <div class="code-line">
                            <span class="line-number">{i + 1}</span>
                            <span class="code">{line}</span>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Panel() -> impl IntoView {
    view! {
        <section class="panel">
            <div class="panel-header">
                <span class="panel-tab">"Terminal"</span>
            </div>
            <Terminal />
        </section>
    }
}

#[component]
fn StatusBar(file: &'static File) -> impl IntoView {
    view! {
        <footer class="status-bar">
            <div class="status-group">
                <span class="mode">"-- NORMAL --"</span>
                <span>"main"</span>
            </div>
            <div class="status-group">
                <span>"Ln 1, Col 1"</span>
                <span>"UTF-8"</span>
                <span>{file.language()}</span>
            </div>
        </footer>
    }
}
