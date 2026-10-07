use leptos::prelude::*;

use crate::app::SubPage;
use crate::profile::SKILLS;

#[component]
pub fn SkillsPage() -> impl IntoView {
    view! {
        <SubPage title="Skills">
            <div class="skills">
                {SKILLS
                    .iter()
                    .map(|category| {
                        view! {
                            <div class="skill-category">
                                <h2 class="skill-category-name">{category.name}</h2>
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
        </SubPage>
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
