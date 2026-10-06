use leptos::prelude::*;

use crate::profile::{BIO, HANDLE, LINKS, NAME, NAME_JA, SKILLS};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="home">
            <header>
                <h1 class="home-name">{NAME_JA}</h1>
                <p class="home-sub">{NAME} " · @" {HANDLE}</p>
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
        </main>
    }
}
