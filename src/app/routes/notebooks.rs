use chrono::Utc;
use gloo_storage::{Result, Storage};
use leptos::{svg::view, *};

use crate::{
    components::*,
    core::{
        bcp::{self, BCPNotebook},
        common::{set_title, NotebookType},
    },
};

#[component]
pub fn Notebooks(cx: Scope) -> impl IntoView {
    set_title(cx, "Notebooks".into());

    let notebooks: Result<Vec<BCPNotebook>> = gloo_storage::LocalStorage::get("dn2.bcp.notebooks");
    if notebooks.is_err() {
        let my_notebook = bcp::BCPNotebook::new("My Notebook");
        let mut notebooks = Vec::new();
        notebooks.push(my_notebook);
        gloo_storage::LocalStorage::set("dn2.bcp.notebooks", &notebooks).unwrap();
    }

    let make_notebook = move |_| {
        let notebooks: Result<Vec<BCPNotebook>> =
            gloo_storage::LocalStorage::get("dn2.bcp.notebooks");

        let mut notebooks = notebooks.unwrap_or_default();
        let my_notebook = bcp::BCPNotebook::new("My Notebook");
        notebooks.push(my_notebook);
        gloo_storage::LocalStorage::set("dn2.bcp.notebooks", &notebooks).unwrap();

        log::debug!("make_notebook")
    };

    match notebooks {
        Ok(notebooks) => view! { cx,
            <button on:click=make_notebook>
                "Make Notebook"
            </button>
            <div class="grid grid-cols-2 gap-1 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-7">
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
            <>
            <div>
                <h1>{"Notebooks"}</h1>
                <p>{"This is the notebooks page."}</p>
                <p>{"Something went wrong here..."} {err.to_string()}</p>
            </div>
            </>
        },
    }
}
