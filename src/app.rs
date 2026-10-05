use leptos::{ev, prelude::*};

use crate::home::Home;
use crate::workbench::Workbench;

fn is_editor() -> bool {
    window()
        .location()
        .hash()
        .is_ok_and(|hash| hash == "#editor")
}

#[component]
pub fn App() -> impl IntoView {
    let editor = RwSignal::new(is_editor());
    let _ = window_event_listener(ev::hashchange, move |_| editor.set(is_editor()));

    view! {
        <Show when=move || editor.get() fallback=Home>
            <Workbench />
        </Show>
    }
}
