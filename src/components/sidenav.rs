use leptos::*;

#[component]
pub fn Sidenav<F, IV>(cx: Scope, nav_menu: F, children: Children) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    IV: IntoView,
{
    view! { cx,
        <div class="sidenav-container">
            <div class="sidenav-menu">{nav_menu(cx)}</div>
            <div class="sidenav-children">{children(cx)}</div>
        </div>
    }
}
