mod routes;

use leptos::*;

use components::*;
use routes::MainView;

use crate::components;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let version_info = env!("CARGO_PKG_VERSION");

    let class = "px-2 py-2";

    view! { cx,
        <Sidenav nav_menu=move |_| {
            view! { cx,
                <span class=class>
                    <span class="text-xl">"DoubleNote"</span>
                    <span class="text-sm ml-auto">" v" {version_info}</span>
                </span>
                <a class=class href="/welcome">
                    "Welcome"
                </a>
                <a class=class href="/notebooks">
                    "Notebooks"
                </a>
                <a class=class href="/collaboration">
                    "Collaboration"
                </a>
                <a class=class href="/servers">
                    "Servers"
                </a>
                <a class=class href="/settings">
                    "Settings"
                </a>
                <hr/>
                <a class=class href="https://github.com/Tanja-4732/DoubleNoteRS" target="_blank">
                    "Source Code ↗"
                </a>
            }
        }>
            <MainView/>
        </Sidenav>
    }
}
