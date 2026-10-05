mod app;
mod commands;
mod files;
mod home;
mod profile;
mod terminal;
mod workbench;

use leptos::prelude::*;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
