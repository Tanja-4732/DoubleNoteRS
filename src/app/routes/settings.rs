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
        <p>
            "Control the settings for the app, and reset its storage."
        </p>

        <h3>"Settings"</h3>
        <p>"Chose how to display the sidenav"</p>
        <ButtonGroup selection=overlay_type>
            <button class="" on:click=move |_| overlay_type.set(NavType::Overlay)>"Overlay"</button>
            <button class="border-l" on:click=move |_| overlay_type.set(NavType::Push)>"Push"</button>
            <button class="border-l" disabled>"Responsive"</button>
        </ButtonGroup>

        <h3>"Theme"</h3>
        <p>"Use the default theme (via the browser) or chose between light and dark"</p>
        <ButtonGroup selection=overlay_type>
            <button class="">"System"</button>
            <button class="border-l" disabled>"Light"</button>
            <button class="border-l" disabled>"Dark"</button>
        </ButtonGroup>
    }
}
