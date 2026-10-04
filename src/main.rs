use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main>
            <h1>"Hello, World!"</h1>
            <button on:click=move |_| *set_count.write() += 1>
                "Clicked: " {count}
            </button>
        </main>
    }
}
