mod routes;

use leptos::*;
use leptos_router::*;

use routes::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <MainView/>
        </div>
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
                            view! { cx, <a href="welcome">{"welcome component"}</a> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
