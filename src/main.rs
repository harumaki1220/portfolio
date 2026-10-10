mod app;
mod blogs;
mod chips;
mod home;
mod icons;
mod profile;
mod route;
mod skills;
mod theme;
mod works;

use leptos::prelude::*;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
