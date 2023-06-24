mod app;
mod components;

use leptos::*;
use leptos_router::*;

use crate::app::App;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
