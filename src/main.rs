mod app;
mod blogs;
mod home;
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
