use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Sidenav<F, G, IV>(cx: Scope, nav_state: G, nav_menu: F, children: Children) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    G: Fn() -> NavState + 'static,
    IV: IntoView,
{
    let x = -00;
    let y = 0;
    let z = 0;

    // let nav_state = nav_state.unwrap_or_else(|| {
    //     log::debug!("[Sidenav] No nav_state signal provided, creating one");
    //     let (r, _) = create_signal(cx, NavState::Open);
    //     r
    // });

    let nav_hidden = move || nav_state() == NavState::Close;

    view! { cx,
        // TODO the background color here may be redundant
        <div id="main" class="flex flex-row w-full min-w-fit h-full dark:bg-slate-800 dark:text-white">
            <div
                id="sidenav-menu"
                class="flex flex-col h-screen min-w-fit bg-gray-400 dark:bg-slate-900 sticky top-0"
                // class="flex flex-col h-full min-h-screen min-w-fit bg-gray-100 dark:bg-slate-900 sticky"
                class:hidden=nav_hidden
                style=format!("transform: translate3d({x}px, {y}px, {z}px)")
            >
                {nav_menu(cx)}
                // <span class="pb-[100vh]">  </span>
            </div>
            {children(cx)}
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum NavState {
    #[default]
    Open,
    Close,
    // Toggle,
}

impl NavState {
    pub fn toggle(self) -> Self {
        match self {
            Self::Open => Self::Close,
            Self::Close => Self::Open,
            // Self::Toggle => Self::Toggle,
        }
    }

    pub fn auto_close_with_policy(self, close_sidenav_on_navigation_policy: bool) -> Self {
        if close_sidenav_on_navigation_policy {
            Self::Close
        } else {
            self
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum NavType {
    #[default]
    Push,
    Overlay,
}

// The following comment is required for tailwind to include the class in the build
// class="hidden"
