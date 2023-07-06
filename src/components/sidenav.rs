use leptos::*;

#[component]
pub fn Sidenav<F, IV>(
    cx: Scope,
    nav_state: Option<ReadSignal<Change>>,
    nav_menu: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    IV: IntoView,
{
    let x = -00;
    let y = 0;
    let z = 0;

    let nav_state = nav_state.unwrap_or_else(|| {
        log::debug!("[Sidenav] No nav_state signal provided, creating one");
        let (r, _) = create_signal(cx, Change::Open);
        r
    });

    let nav_state = move || nav_state() == Change::Open;

    view! { cx,
        // The background color here may be redundant
        <div id="main" class="flex flex-row w-full min-w-fit h-screen dark:bg-slate-800 dark:text-white">
            <div
                id="sidenav-menu"
                class="flex flex-col h-full min-w-fit bg-gray-100 dark:bg-slate-900"
                class:hidden=nav_state
                style=format!("transform: translate3d({x}px, {y}px, {z}px)")
            >
                {nav_menu(cx)}
            </div>
            {children(cx)}
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Change {
    #[default]
    Open,
    Close,
    // Toggle,
}

impl Change {
    pub fn toggle(self) -> Self {
        match self {
            Self::Open => Self::Close,
            Self::Close => Self::Open,
            // Self::Toggle => Self::Toggle,
        }
    }
}

// The following comment is required for tailwind to include the class in the build
// class="hidden"
