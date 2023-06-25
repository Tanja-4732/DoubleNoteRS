mod routes;

use leptos::*;

use components::*;
use routes::MainView;

use crate::components;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let version_info = env!("CARGO_PKG_VERSION");

    view! { cx,
        <Sidenav nav_menu=move |_| {
            view! { cx,
                <span>"DoubleNote" <span class="version-info">" v" {version_info}</span></span>
                <a class="list-button" href="/welcome">
                    "Welcome"
                </a>
                <a class="list-button" href="/notebooks">
                    "Notebooks"
                </a>
                <a class="list-button" href="/collaboration">
                    "Collaboration"
                </a>
                <a class="list-button" href="/servers">
                    "Servers"
                </a>
                <a class="list-button" href="/settings">
                    "Settings"
                </a>
                <hr/>
                <a class="list-button" href="https://github.com/Tanja-4732/DoubleNoteRS" target="_blank">
                    "Source Code ↗"
                </a>
            }
        }>
            <MainView/>
        </Sidenav>
    }
}
