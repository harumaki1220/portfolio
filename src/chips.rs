use leptos::prelude::*;

/// 角丸の小さなタグを横に並べる。`main` を付けるとアクセントカラーになる
#[component]
pub fn Chips(items: &'static [&'static str], #[prop(optional)] main: bool) -> impl IntoView {
    view! {
        <ul class="chips" class:chips-main=main>
            {items.iter().map(|item| view! { <li class="chip">{*item}</li> }).collect_view()}
        </ul>
    }
}
