use leptos::*;

#[component]
pub fn Card<F, IV>(cx: Scope, buttons: F, children: Children) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    IV: IntoView,
{
    view! { cx,
        <div class="flex flex-col bg-gray-200 dark:bg-slate-500 dark:text-white p-2 rounded">
            <div class="mb-2">{children(cx)}</div>
            <div class="dn2-card flex flex-row-reverse h-full w-fit ml-auto">
                {buttons(cx)}
            </div>
        </div>
    }
}
