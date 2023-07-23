use std::rc::Rc;

use chrono::Utc;
use gloo_storage::{Result, Storage};
use leptos::{html::Dialog, svg::view, *};
use leptos_router::{use_params_map, use_query, use_query_map};

use crate::{
    components::*,
    core::{
        bcp::{self, BCPNotebook},
        common::{set_title, NotebookType},
    },
};

#[component]
pub fn Notebooks(cx: Scope) -> impl IntoView {
    set_title(cx, "Notebooks");

    let params = use_params_map(cx);
    let query = use_query_map(cx);

    // id: || -> Option<String>
    // let id = move || params.with(|params| params.get("id").cloned());

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

    // let edit_dialog = Rc::new(Box::leak(Box::new(edit_notebook_dialog(cx))));
    // let edit_dialog_2 = edit_dialog.clone();

    // let open_edit = || edit_dialog.show_modal();

    // edit_dialog.show_modal().unwrap();

    let (open_edit_dialog, set_open_edit_dialog) = create_signal(cx, false);

    match notebooks {
        Ok(notebooks) => view! { cx,
            <div class="dn2-card-grid">
            // <div class="grid grid-cols-2 gap-1 sm:grid-cols-3 md:grid-cols-4 md:gap-2 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-7">
                {notebooks
                    .into_iter()
                    .map(|notebook| {
                        let href = format!("/notebooks/bcp/{}", notebook.uuid);
                        view! { cx,
                            <Card buttons=move |_| {
                                view! { cx,
                                    <a href=href.clone() class="bg-green-400 dark:bg-green-500 px-2 py-1 max-w-fit rounded">
                                        "Open"
                                    </a>
                                    <a on:click=move|_| { log::debug!("set edit dialog open"); set_open_edit_dialog(true); } class="border border-gray-500 dark:border-[#e5e7eb] mr-1 px-2 py-1 max-w-fit rounded">
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
            <button on:click=make_notebook class="dark:border mr-1 px-2 py-1 max-w-fit rounded">
                "Make Notebook"
            </button>
            <button on:click=make_notebook class="dark:border mt-2 px-2 py-1 max-w-fit rounded" disabled>
                "Import"
            </button>
            <EditNotebookDialog opened=open_edit_dialog/>

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

#[component]
fn edit_notebook_dialog(cx: Scope, opened: ReadSignal<bool>) -> impl IntoView {
    let dialog = view! {cx,
        <dialog>
            "Hi there"
        </dialog>
    };

    let dialog = store_value(cx, dialog);

    create_effect(cx, move |_| {
        if opened() {
            dialog.with_value(|d| d.show_modal()).unwrap();
        }
    });

    dialog
}
