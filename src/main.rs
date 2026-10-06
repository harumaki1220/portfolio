mod home;
mod profile;
mod theme;

use leptos::prelude::*;

use home::Home;
use theme::ThemeToggle;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <ThemeToggle />
            <Home />
        }
    });
}
