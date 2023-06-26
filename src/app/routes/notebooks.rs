use chrono::Utc;
use leptos::*;

use crate::core::{
    bcp::{self, BCPNotebook},
    common::NotebookType,
};

#[component]
pub fn Notebooks(cx: Scope) -> impl IntoView {
    let my_notebook = BCPNotebook::new("My Notebook");

    let href = format!("/notebooks/bcp/{}", my_notebook.uuid);

    view! { cx,
        <div>
            <h1>{"Notebooks"}</h1>
            <p>{"This is the notebooks page."}</p>
            <h2>{"My Notebook"}</h2>
            <a href=href>{my_notebook.name.clone()}</a>
        </div>
    }
}
