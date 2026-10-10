use leptos::prelude::*;

use crate::app::SubPage;
use crate::chips::Chips;
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
                                <div class="work-header">
                                    <h2 class="work-name">{work.name}</h2>
                                    <time class="work-date">{work.date}</time>
                                </div>
                                <p class="work-description">{work.description}</p>
                                <Chips items=work.tech />
                                <p class="work-links">
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
