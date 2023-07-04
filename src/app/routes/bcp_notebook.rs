use chrono::Utc;
use leptos::*;
use uuid::Uuid;

use crate::core::{
    bcp::{self},
    common::NotebookType,
};

#[component]
pub fn BCPNotebook(cx: Scope, uuid: Uuid) -> impl IntoView {
    let my_notebook = bcp::BCPNotebook::new("My Notebook");

    let href = format!("/notebooks/{}", my_notebook.uuid);

    view! { cx,
        <div>
            <h1>{"Notebooks"}</h1>
            <p>{"This is the notebooks page."}</p>
            <h2>{"My Notebook"}</h2>
        </div>
    }
}
