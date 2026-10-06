mod home;
mod profile;

use leptos::prelude::*;

use home::Home;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Home);
}
