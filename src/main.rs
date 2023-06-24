mod app;
mod components;
mod core;

use leptos::{mount_to_body, view};

use crate::app::App;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
