mod app;
mod components;
mod data;
mod dom;
mod pages;
mod progress;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
