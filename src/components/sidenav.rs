use leptos::*;

#[component]
pub fn Sidenav<F, IV>(cx: Scope, nav_menu: F, children: Children) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    IV: IntoView,
{
    view! { cx,
        <div id="main" class="flex flex-row w-full h-screen dark:bg-slate-800 dark:text-white">
            <div id="sidenav-menu" class="flex flex-col h-full min-w-fit bg-gray-100 dark:bg-slate-900">{nav_menu(cx)}</div>
            <div id="sidenav-children" class="flex flex-col w-full p-2">{children(cx)}</div>
        </div>
    }
}
