use leptos::prelude::*;

const SKILLS: &[&str] = &["Rust", "TypeScript"];

const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/harumaki1220"),
    ("X", "https://x.com/matcha445_dev"),
];

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="container">
            <Profile />
            <Skills />
            <Links />
        </main>
        <footer class="footer">"© Haruma Kusunoki"</footer>
    }
}

#[component]
fn Profile() -> impl IntoView {
    view! {
        <section class="profile">
            <h1 class="name">"Haruma Kusunoki"</h1>
            <p class="name-ja">"楠 悠真 " <span class="handle">"@harumaki1220"</span></p>
            <p class="bio">
                "東洋大学 情報連携学部 (INIAD) 2年。"
                <br />
                "Rust と TypeScript を中心に日々コードを書いています。"
            </p>
        </section>
    }
}

#[component]
fn Skills() -> impl IntoView {
    view! {
        <section>
            <h2 class="heading">"Skills"</h2>
            <ul class="skills">
                {SKILLS.iter().map(|skill| view! { <li class="skill">{*skill}</li> }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn Links() -> impl IntoView {
    view! {
        <section>
            <h2 class="heading">"Links"</h2>
            <ul class="links">
                {LINKS
                    .iter()
                    .map(|(label, href)| {
                        view! {
                            <li>
                                <a class="link" href=*href target="_blank" rel="noopener noreferrer">
                                    {*label} " ↗"
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}
