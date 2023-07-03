use leptos::*;

#[component]
pub fn Welcome(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <h1 class="text-2xl">{"Welcome to DoubleNote!"}</h1>
            <p class="pb-3">
                {"To navigate around the application, use the SideNav (a drawer menu which slides in from the left side of the screen) and the CrumbTrail (an interactive indicator of the current location in the application located directly below the blue title bar).            "}
            </p>
            <p class="pb-3">
                {"This project is still in its infancy, and not yet considered usable."} <br/>
                {"Any data stored may become inaccessible due to changes in the software at any point in time for any reason."}
            </p>
        </div>
    }
}
