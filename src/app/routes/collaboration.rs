use leptos::*;

use crate::core::common::set_title;

#[component]
pub fn Collaboration(cx: Scope) -> impl IntoView {
    set_title(cx, "Collaboration".into());

    view! { cx,
        <p>{"This is the collaboration page."}</p>
    }
}
