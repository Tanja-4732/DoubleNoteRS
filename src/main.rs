mod app;
mod components;
mod core;

use leptos::*;

use crate::app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("DoubleNote v{}", env!("CARGO_PKG_VERSION"));
    log!(
        "Licensed under the AGPLv3\n\
        Copyright © 2023 Tanja-473."
    );

    mount_to_body(|cx| view! { cx, <App/> })
}
