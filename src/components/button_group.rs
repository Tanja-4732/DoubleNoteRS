use leptos::*;

use crate::components::NavType;

#[component]
pub fn ButtonGroup(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="dn2-btn-grp flex flex-row rounded-md bg-gray-200 dark:bg-slate-500 w-fit">
            {children(cx)}
        </div>
    }
}
