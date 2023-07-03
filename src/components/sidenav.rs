use leptos::*;

#[component]
pub fn Sidenav<F, IV>(cx: Scope, nav_menu: F, children: Children) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    IV: IntoView,
{
    view! { cx,
        <div id="main" class="flex flex-row w-full h-screen">
            <div id="sidenav-menu" class="flex flex-col h-full min-w-fit">{nav_menu(cx)}</div>
            <div id="sidenav-children" class="flex flex-col w-full">{children(cx)}</div>
        </div>
    }
}
