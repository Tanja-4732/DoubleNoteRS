use chrono::Utc;
use gloo_storage::{Result, Storage};
use leptos::{svg::view, *};

use crate::core::{
    bcp::{self, BCPNotebook},
    common::NotebookType,
};

#[component]
pub fn Notebooks(cx: Scope) -> impl IntoView {
    // let href = format!("/notebooks/bcp/{}", my_notebook.uuid);

    let notebooks: Result<Vec<BCPNotebook>> = gloo_storage::LocalStorage::get("dn2.bcp.notebooks");
    if notebooks.is_err() {
        let my_notebook = bcp::BCPNotebook::new("My Notebook");
        let mut notebooks = Vec::new();
        notebooks.push(my_notebook);
        gloo_storage::LocalStorage::set("dn2.bcp.notebooks", &notebooks).unwrap();
    }

    match notebooks {
        Ok(notebooks) => view! { cx,
            <div>
                {notebooks
                    .into_iter()
                    .map(|notebook| {
                        let href = format!("/notebooks/bcp/{}", notebook.uuid);
                        view! { cx,
                            <div>
                                <h2>{notebook.name.clone()}</h2>
                                <a href=href>"Open"</a>
                            </div>
                        }
                    })
                    .collect_view(cx)}
            </div>
        },
        Err(_) => view! { cx,
            <div>
                <h1>{"Notebooks"}</h1>
                <p>{"This is the notebooks page."}</p>
                <h2>{"My Notebook"}</h2>
            </div>
        },
    }
}
