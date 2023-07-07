use leptos::*;
use leptos_router::*;

use crate::components::NavState;

#[component]
pub fn Toolbar(cx: Scope, nav_toggle: WriteSignal<NavState>) -> impl IntoView {
    // let params: Memo<ParamsMap> = use_params_map(cx);
    // let notebook_uuid = move || params.with(|params| params.get("notebook_uuid").cloned());

    let title = use_context::<ReadSignal<&str>>(cx)
        // we know we just provided this in the parent component
        .expect("there to be a `title` signal provided");

    view! { cx,
        <div class="w-full dark:bg-slate-700 dark:text-white p-2 text-xl">
            <button
                class="dark:bg-green-500 px-2 max-w-fit rounded-full mr-1"
                on:click=move |_| nav_toggle.update(|state: &mut NavState| *state = state.toggle())
            >
                "☰"
            </button>
            {title()}
        </div>
    }
}
