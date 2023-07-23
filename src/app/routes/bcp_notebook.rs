use chrono::Utc;
use leptos::*;
use uuid::Uuid;

use crate::core::{
    bcp::{self},
    common::{set_title, NotebookType},
};

use crate::components::Card;

#[component]
pub fn BCPNotebook(cx: Scope, uuid: Uuid) -> impl IntoView {
    let my_notebook = bcp::BCPNotebook::new("My Notebook");
    // set_title(cx, my_notebook.name.into());

    let href = format!("/notebooks/{}", my_notebook.uuid);

    view! { cx,
        <div>
            <h1>{"Notebooks"}</h1>
            <p>{"This is the notebooks page."}</p>
            <h2>{"My Notebook"}</h2>
            <Card buttons=move |_|{view! { cx,
                <div>
                    // <a href=href class="btn btn-primary">{"Open"}</a>
                </div>
            }}>
                <p>{"This is my notebook."}</p>
            </Card>
        </div>
    }
}
