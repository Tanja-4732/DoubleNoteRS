use leptos::*;

use crate::core::common::set_title;

#[component]
pub fn Servers(cx: Scope) -> impl IntoView {
    set_title(cx, "Servers".into());

    view! { cx,
        <p>{"This is the servers page."}</p>
    }
}
