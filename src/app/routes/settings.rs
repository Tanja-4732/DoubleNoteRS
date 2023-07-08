use leptos::*;

use crate::{
    components::*,
    core::common::{get_sidenav_type, set_sidenav_type, set_title},
};

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    let overlay_type = create_rw_signal(cx, get_sidenav_type());
    create_effect(cx, move |_| set_sidenav_type(overlay_type()));

    set_title(cx, "Settings".into());

    view! { cx,
        <h1>{"Settings"}</h1>
        <p>{"This is the settings page."}</p>
        <ButtonGroup selection=overlay_type>
            <button class="" on:click=move |_| overlay_type.set(NavType::Overlay)>"Overlay"</button>
            <button class="border-l" on:click=move |_| overlay_type.set(NavType::Push)>"Push"</button>
        </ButtonGroup>
    }
}
