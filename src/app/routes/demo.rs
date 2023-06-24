use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <h1>{"Demo"}</h1>
            <p>{"This is the demo page."}</p>
        </div>
    }
}
