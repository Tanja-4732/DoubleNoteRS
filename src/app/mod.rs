use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
       <div>
           <h1> {"Hello, world!"} </h1>
           <MainView />
       </div>
    }
}

#[component]
fn MainView(cx: Scope) -> impl IntoView {
    view! { cx,
      <Router>
        <nav>
         <p> {"I'm a nav bar"} </p>
        </nav>
        <main>
            //  <p> {"I'm the main content"} </p>

            // all our routes will appear inside <main>
        <Routes>
            <Route path="/" view=|cx| view! { cx, <p> {"I'm the home page"} </p> } />
            <Route path="/*any" view=|cx| view! { cx, <p> {"I'm a catch-all page"} </p> } />
      </Routes>
        </main>
      </Router>
    }
}
