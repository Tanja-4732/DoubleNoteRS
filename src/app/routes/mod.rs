mod bcp_notebook;
mod collaboration;
mod demo;
mod notebooks;
mod servers;
mod settings;
mod welcome;

use bcp_notebook::*;
use collaboration::*;
use demo::*;
use notebooks::*;
use servers::*;
use settings::*;
use uuid::Uuid;
use welcome::*;

use leptos::*;
use leptos_router::*;

#[component]
pub fn MainView(cx: Scope) -> impl IntoView {
    // let route = use_// let route = use_route(cx).path();

    // let (title, set_title) = create_signal(cx, "Welcome");
    // set_title.update(move |_| {
    //     let route = route.as_ref();
    //     return match route {
    //         "/welcome" => "Welcome",
    //         "/notebooks" => "Notebooks",
    //         "/collaboration" => "Collaboration",
    //         "/servers" => "Servers",
    //         "/settings" => "Settings",
    //         "/demo" => "Demo",
    //         _ => "404",
    //     };
    // });
    // provide_context(cx, title);
    view! { cx,
        <Router>
            <main id="router-outlet">
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
                        path="/notebooks/bcp/:notebook_uuid"
                        view=move |cx| {
                            view! { cx,
                                {
                                    let params: Memo<ParamsMap> = use_params_map(cx);
                                    let notebook_uuid = move || {
                                        params.with(|params| params.get("notebook_uuid").cloned())
                                    };
                                    notebook_uuid
                                }
                            }
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
