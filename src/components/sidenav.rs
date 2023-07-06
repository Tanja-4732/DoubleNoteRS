use leptos::*;

#[component]
pub fn Sidenav<F, IV>(
    cx: Scope,
    toggle_signal: Option<WriteSignal<Change>>,
    nav_menu: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    IV: IntoView,
{
    view! { cx,
        // The background color here may be redundant
        <div id="main" class="flex flex-row w-full min-w-fit h-screen dark:bg-slate-800 dark:text-white">
            <div
                id="sidenav-menu"
                class="flex flex-col h-full min-w-fit bg-gray-100 dark:bg-slate-900"
            >
                {nav_menu(cx)}
            </div>
            {children(cx)}
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Change {
    Open,
    Close,
    Toggle,
}
