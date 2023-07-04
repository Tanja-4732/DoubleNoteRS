use chrono::Utc;
use gloo_storage::{Result, Storage};
use leptos::{svg::view, *};

use crate::{
    components::*,
    core::{
        bcp::{self, BCPNotebook},
        common::NotebookType,
    },
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
            <div class="">
                {notebooks
                    .into_iter()
                    .map(|notebook| {
                        let href = format!("/notebooks/bcp/{}", notebook.uuid);
                        view! { cx,
                            <Card buttons=move |_| {
                                view! { cx,
                                    <a href=href.clone() class="dark:bg-green-500 px-2 py-1 max-w-fit rounded">
                                        "Open"
                                    </a>
                                    <a href=href.clone() class="dark:border mr-1 px-2 py-1 max-w-fit rounded">
                                        "Edit"
                                    </a>
                                }
                            }>
                                <h2 class="text-lg">{notebook.name.clone()}</h2>
                            </Card>
                        }
                    })
                    .collect_view(cx)}
            </div>
        },
        Err(err) => view! { cx,
            <div>
                <h1>{"Notebooks"}</h1>
                <p>{"This is the notebooks page."}</p>
                <p>{"Something went wrong here..."} {err.to_string()}</p>
            </div>
        },
    }
}
