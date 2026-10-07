use leptos::prelude::*;

use crate::app::SubPage;
use crate::profile::WORKS;

#[component]
pub fn WorksPage() -> impl IntoView {
    view! {
        <SubPage title="Works">
            <ul class="works">
                {WORKS
                    .iter()
                    .map(|work| {
                        view! {
                            <li class="work">
                                <h2 class="work-name">{work.name}</h2>
                                <p class="work-description">{work.description}</p>
                                <p class="work-meta">
                                    <span>{work.tech.join(" / ")}</span>
                                    {work
                                        .links
                                        .iter()
                                        .map(|(label, href)| {
                                            view! {
                                                <a href=*href target="_blank" rel="noopener noreferrer">
                                                    {*label}
                                                    " ↗"
                                                </a>
                                            }
                                        })
                                        .collect_view()}
                                </p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </SubPage>
    }
}
