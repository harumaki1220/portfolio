use leptos::prelude::*;

use crate::profile::{BIO, ICON_CREDIT, ICON_URL, LINKS, NAME, SKILLS};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="home">
            <header class="home-header">
                <img class="home-icon" src=ICON_URL alt="" width="96" height="96" />
                <h1 class="home-name">{NAME}</h1>
            </header>

            <p class="home-bio">
                {BIO.iter().map(|line| view! { <span>{*line}</span> }).collect_view()}
            </p>

            <section>
                <h2 class="home-heading">"Skills"</h2>
                <ul class="home-skills">
                    {SKILLS.iter().map(|skill| view! { <li>{*skill}</li> }).collect_view()}
                </ul>
            </section>

            <section>
                <h2 class="home-heading">"Links"</h2>
                <ul class="home-links">
                    {LINKS
                        .iter()
                        .map(|(label, href)| {
                            view! {
                                <li>
                                    <a href=*href target="_blank" rel="noopener noreferrer">
                                        {*label}
                                        " ↗"
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </section>

            <footer class="home-credit">
                <p>
                    "Icon by "
                    <a href=ICON_CREDIT.1 target="_blank" rel="noopener noreferrer">
                        {ICON_CREDIT.0}
                    </a>
                    " 🎨"
                </p>
            </footer>
        </main>
    }
}
