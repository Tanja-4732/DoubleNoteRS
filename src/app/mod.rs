mod routes;

use leptos::*;
use leptos_router::*;

use components::*;
use routes::*;

use crate::components;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Sidenav nav_menu=move |_| {
            view! { cx,
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

#[component]
fn MainView(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <nav></nav>
            <main>
                <Routes>
                    <Route
                        path="/welcome"
                        view=|cx| {
                            view! { cx, <Welcome/> }
                        }
                    />
                    <Route
                        path="/notebooks"
                        view=|cx| {
                            view! { cx, <Notebooks/> }
                        }
                    />
                    <Route
                        path="/collaboration"
                        view=|cx| {
                            view! { cx, <Collaboration/> }
                        }
                    />
                    <Route
                        path="/servers"
                        view=|cx| {
                            view! { cx, <Servers/> }
                        }
                    />
                    <Route
                        path="/settings"
                        view=|cx| {
                            view! { cx, <Settings/> }
                        }
                    />
                    <Route
                        path="/demo"
                        view=|cx| {
                            view! { cx, <Demo/> }
                        }
                    />
                    <Route
                        path="/*any"
                        view=|cx| {
                            view! { cx, <p>{"I'm a catch-all page"}</p> }
                        }
                    />
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx, <a href="welcome">{"Go to the welcome page"}</a> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
