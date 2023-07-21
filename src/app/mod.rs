mod routes;

use leptos::{ev::close, *};

use components::*;
use routes::MainView;

use crate::{
    components,
    core::common::{get_close_sidenav_on_navigation, get_sidenav_state, set_sidenav_state},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let version_info = env!("CARGO_PKG_VERSION");

    let (toggle, set_toggle) = create_signal(cx, get_sidenav_state());
    create_effect(cx, move |_| set_sidenav_state(toggle()));

    // TODO replace this with the title of the current route
    let (title, set_title) = create_signal(cx, "DoubleNote");
    provide_context(cx, title);
    provide_context(cx, set_title);

    let (close_sidenav_on_navigation, set_close_sidenav_on_navigation) =
        create_signal(cx, get_close_sidenav_on_navigation());
    provide_context(cx, close_sidenav_on_navigation);
    provide_context(cx, set_close_sidenav_on_navigation);

    let on_navigate = move |_| {
        set_toggle.update(|v| *v = v.auto_close_with_policy(close_sidenav_on_navigation()))
    };

    // let nav_state = move || toggle().with_policy(close_sidenav_on_navigation());
    let nav_state = move || toggle();

    let class = "px-2 py-2";

    view! { cx,
        <Sidenav nav_state=nav_state nav_menu=move |_| {
            view! { cx,
                <span class=class>
                    <span class="text-xl">"DoubleNote"</span>
                    <span class="text-sm ml-auto">" v" {version_info}</span>
                </span>
                <a on:click=on_navigate class=class href="/welcome">
                    "Welcome"
                </a>
                <a on:click=on_navigate class=class href="/notebooks">
                    "Notebooks"
                </a>
                <a on:click=on_navigate class=class href="/collaboration">
                    "Collaboration"
                </a>
                <a on:click=on_navigate class=class href="/servers">
                    "Servers"
                </a>
                <a on:click=on_navigate class=class href="/settings">
                    "Settings"
                </a>
                <hr class="border-gray-500"/>
                <a class=class href="https://github.com/Tanja-4732/DoubleNoteRS" target="_blank">
                    "Source Code ↗"
                </a>
            }
        }>
            <div id="sidenav-children" class="w-full min-w-fit min-h-screen bg-gray-100 dark:bg-slate-800 dark:text-white">
                <Toolbar nav_toggle=set_toggle/>
                <div id="sidenav-selected-route" class="flex flex-col p-2">
                    <MainView/>
                </div>
            </div>
        </Sidenav>
    }
}
