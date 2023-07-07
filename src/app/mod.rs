mod routes;

use leptos::*;

use components::*;
use routes::MainView;

use crate::{
    components,
    core::common::{get_sidenav_state, set_sidenav_state},
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

    let class = "px-2 py-2";

    view! { cx,
        <Sidenav nav_state=Some(toggle) nav_menu=move |_| {
            view! { cx,
                <span class=class>
                    <span class="text-xl">"DoubleNote"</span>
                    <span class="text-sm ml-auto">" v" {version_info}</span>
                </span>
                <a on:click=move|_|set_toggle(NavState::Close) class=class href="/welcome">
                    "Welcome"
                </a>
                <a on:click=move|_|set_toggle(NavState::Close) class=class href="/notebooks">
                    "Notebooks"
                </a>
                <a on:click=move|_|set_toggle(NavState::Close) class=class href="/collaboration">
                    "Collaboration"
                </a>
                <a on:click=move|_|set_toggle(NavState::Close) class=class href="/servers">
                    "Servers"
                </a>
                <a on:click=move|_|set_toggle(NavState::Close) class=class href="/settings">
                    "Settings"
                </a>
                <hr/>
                <a on:click=move|_|set_toggle(NavState::Close) class=class href="https://github.com/Tanja-4732/DoubleNoteRS" target="_blank">
                    "Source Code ↗"
                </a>
            }
        }>
            <div id="sidenav-children" class="w-full min-w-fit dark:bg-slate-800 dark:text-white">
                <Toolbar nav_toggle=set_toggle/>
                <div id="sidenav-selected-route" class="flex flex-col p-2">
                    <MainView/>
                </div>
            </div>
        </Sidenav>
    }
}
