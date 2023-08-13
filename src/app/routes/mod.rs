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
            <main id="router-outlet">
                <Routes>
                    <Route path="/welcome" view=Welcome/>
                    <Route path="/notebooks" view=Notebooks/>
                    <Route path="/notebooks/bcp/:notebook_uuid" view=BCPNotebook/>
                    <Route path="/collaboration" view=Collaboration/>
                    <Route path="/servers" view=Servers/>
                    <Route path="/settings" view=Settings/>
                    <Route path="/demo" view=Demo/>
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
    }
}
