use leptos::*;

use crate::{
    components::*,
    core::common::{
        get_close_sidenav_on_navigation, get_sidenav_type, set_close_sidenav_on_navigation,
        set_sidenav_type, set_title,
    },
};

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    let overlay_type = create_rw_signal(cx, get_sidenav_type());
    create_effect(cx, move |_| set_sidenav_type(overlay_type.get()));

    let close_sidenav_on_navigation = create_rw_signal(cx, get_close_sidenav_on_navigation());
    create_effect(cx, move |_| {
        set_close_sidenav_on_navigation(cx, close_sidenav_on_navigation.get())
    });

    set_title(cx, "Settings".into());

    view! { cx,
        <h1>{"Settings"}</h1>
        <h3>"Settings"</h3>
        <p>
            "Control the settings for the app, and reset its storage."
        </p>

        <p>"Chose how to display the sidenav"</p>
        <ButtonGroup>
            <button class="" on:click=move |_| overlay_type.set(NavType::Overlay)>"Overlay"</button>
            <button class="border-l" on:click=move |_| overlay_type.set(NavType::Push)>"Push"</button>
            <button class="border-l" disabled>"Responsive"</button>
        </ButtonGroup>

        <h3>"Theme"</h3>
        <p>"Use the default theme (via the browser) or chose between light and dark"</p>
        <ButtonGroup>
            <button class="">"System"</button>
            <button class="border-l" disabled>"Light"</button>
            <button class="border-l" disabled>"Dark"</button>
        </ButtonGroup>
        <ButtonGroup>
            <button class="" on:click=move |_| close_sidenav_on_navigation.set(true)>"Close sidenav on navigation"</button>
            <button class="border-l" on:click=move |_| close_sidenav_on_navigation.set(false)>"Keep sidenav open after navigation"</button>
        </ButtonGroup>
    }
}
