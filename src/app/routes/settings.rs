use leptos::*;

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <h1>{"Settings"}</h1>
            <p>{"This is the settings page."}</p>
        </div>
    }
}
