use leptos::prelude::*;

use crate::profile::{BIO, ICON_CREDIT, ICON_URL, LINKS, NAME, SKILLS, WORKS};

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
                <div class="skills">
                    {SKILLS
                        .iter()
                        .map(|category| {
                            view! {
                                <div class="skill-category">
                                    <h3 class="skill-category-name">{category.name}</h3>
                                    {if category.others.is_empty() {
                                        // 分ける意味がない分類は、ラベルなしで1行に並べる
                                        view! {
                                            <p class="skill-list">{category.main.join(" / ")}</p>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <dl>
                                                <SkillRow label="Main" skills=category.main />
                                                <SkillRow label="Others" skills=category.others />
                                            </dl>
                                        }
                                            .into_any()
                                    }}
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </section>

            <section>
                <h2 class="home-heading">"Works"</h2>
                <ul class="works">
                    {WORKS
                        .iter()
                        .map(|work| {
                            view! {
                                <li class="work">
                                    <h3 class="work-name">{work.name}</h3>
                                    <p class="work-description">{work.description}</p>
                                    <p class="work-meta">
                                        <span>{work.tech.join(" / ")}</span>
                                        {work
                                            .links
                                            .iter()
                                            .map(|(label, href)| {
                                                view! {
                                                    <a
                                                        href=*href
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                    >
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

/// 「Main」「Others」の1行。該当するスキルがなければ何も表示しない。
#[component]
fn SkillRow(label: &'static str, skills: &'static [&'static str]) -> impl IntoView {
    (!skills.is_empty()).then(|| {
        view! {
            <div class="skill-group">
                <dt>{label}</dt>
                <dd>{skills.join(" / ")}</dd>
            </div>
        }
    })
}
