use leptos::*;

use crate::components::NavType;

#[component]
pub fn ButtonGroup(cx: Scope, selection: RwSignal<NavType>, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row h-full w-fit bg-slate-500 py-1 px-2 rounded gap-2 border">
            {children(cx)}
        </div>
    }
}
