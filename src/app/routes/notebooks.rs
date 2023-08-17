use std::rc::Rc;

use chrono::Utc;
use gloo_storage::{Result, Storage};
use leptos::{
    html::{Dialog, Input},
    svg::view,
    *,
};
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

    // let params = use_params_map(cx);
    // let query = use_query_map(cx);
    //
    // // id: || -> Option<String>
    // // let id = move || params.with(|params| params.get("id").cloned());

    let (create_notebook_state, set_create_notebook_state) =
        create_signal(cx, CreateNotebookState::default());

    let notebooks: Result<Vec<BCPNotebook>> = gloo_storage::LocalStorage::get("dn2.bcp.notebooks");
    if notebooks.is_err() {
        let my_notebook = bcp::BCPNotebook::new("My Notebook");
        let mut notebooks = Vec::new();
        notebooks.push(my_notebook);
        gloo_storage::LocalStorage::set("dn2.bcp.notebooks", &notebooks).unwrap();
    }

    let make_notebook = move |_| {
        set_create_notebook_state.set(CreateNotebookState::Open);

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

    let edit_notebook_name = create_rw_signal(cx, EditNotebookState::default());

    match notebooks {
        Ok(notebooks) => view! { cx,
            <div class="dn2-card-grid">
            // <div class="grid grid-cols-2 gap-1 sm:grid-cols-3 md:grid-cols-4 md:gap-2 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-7">
                {notebooks
                    .into_iter()
                    .map(|notebook| {
                        let href = format!("/notebooks/bcp/{}", notebook.uuid);
                        let name2 = notebook.name.clone();
                        view! { cx,
                            <Card buttons=move |_| {
                                let name = notebook.name.clone();
                                view! { cx,
                                    <a href=href.clone() class="bg-green-400 dark:bg-green-500 px-2 py-1 max-w-fit rounded">
                                        "Open"
                                    </a>
                                    <a on:click=move|_| edit_notebook_name.set(EditNotebookState::Open{previous_name: name.clone()}) class="border border-gray-500 dark:border-[#e5e7eb] mr-1 px-2 py-1 max-w-fit rounded cursor-pointer">
                                        "Edit"
                                    </a>
                                }
                            }>
                                <h2 class="text-lg">{name2}</h2>
                            </Card>
                        }
                    })
                    .collect_view(cx)}
            </div>
            // <button on:click=make_notebook class="dark:border mr-1 px-2 py-1 max-w-fit rounded">
            <button on:click=move|_| set_create_notebook_state.set(CreateNotebookState::Open) class="dark:border mr-1 px-2 py-1 max-w-fit rounded">
                "Create Notebook"
            </button>
            <button on:click=make_notebook class="dark:border mt-2 px-2 py-1 max-w-fit rounded" disabled>
                "Import"
            </button>
            <EditNotebookDialog edit_state=edit_notebook_name/>
            <CreateNotebookDialog state=create_notebook_state set_state=set_create_notebook_state />
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

#[derive(Default)]
enum CreateNotebookState {
    #[default]
    Initial,
    Open,
    // TODO maybe introduce a `canceled` state
    Confirmed {
        name: String,
    },
}

#[derive(Default, Debug, Clone)]
enum EditNotebookState {
    #[default]
    Initial,
    Open {
        previous_name: String,
    },
    // TODO maybe introduce a `canceled` state
    Confirmed {
        new_name: String,
    },
    Canceled,
}

#[component]
fn CreateNotebookDialog(
    cx: Scope,
    state: ReadSignal<CreateNotebookState>,
    set_state: WriteSignal<CreateNotebookState>,
) -> impl IntoView {
    let name_ref = create_node_ref::<Input>(cx);

    let on_click = move |_| {
        let node = name_ref.get().expect("name_ref should be loaded by now");
        // `node` is strongly typed
        // it is dereferenced to an `HtmlInputElement` automatically
        log!("value is {:?}", node.value());

        set_state.set(CreateNotebookState::Confirmed { name: node.value() })
    };

    let dialog = view! {cx,
        <dialog>
            <form method="dialog" class="grid grid-cols-2">
                <label for="name">"Name"</label>
                <input type="text" name="name" id="name" _ref=name_ref/>

                <button on:click=on_click type="submit" class="cursor-pointer">
                    "Confirm"
                </button>
                <button on:click=move|e| {log::debug!("{:#?}", e)} type="cancel" class="cursor-pointer">
                    "Cancel"
                </button>
            </form>
        </dialog>
    };

    let dialog = store_value(cx, dialog);

    // create_effect(cx, move |_| match state() {
    //     CreateNotebookState::Open => dialog.with_value(|d| d.show_modal()).unwrap(),
    // });

    dialog.get_value()
}

#[component]
fn EditNotebookDialog(cx: Scope, edit_state: RwSignal<EditNotebookState>) -> impl IntoView {
    let opened = move || match edit_state.get() {
        EditNotebookState::Open { .. } => true,
        _ => false,
    };

    let name = move || match edit_state.get() {
        EditNotebookState::Open { previous_name } => previous_name,
        _ => String::new(),
    };

    let name_ref = create_node_ref::<Input>(cx);

    let on_click = move |_| {
        let node = name_ref.get().expect("name_ref should be loaded by now");
        // `node` is strongly typed
        // it is dereferenced to an `HtmlInputElement` automatically
        log!("value is {:?}", node.value());

        edit_state.set(EditNotebookState::Confirmed {
            new_name: node.value(),
        })
    };

    let dialog = view! {cx,
        <dialog class="bg-gray-300 dark:bg-slate-700 dark:text-white p-3">
            <form method="dialog" class="grid grid-cols-2 gap-2">
                <label for="name">"Name"</label>
                <input type="text" name="name" id="name" class="dark:text-black" value=name/>

                <button on:click=move|e| {log::debug!("{:#?}", e)} type="cancel" class="cursor-pointer">
                    "Cancel"
                </button>
                <button on:click=on_click type="submit" class="cursor-pointer">
                    "Confirm"
                </button>
            </form>
        </dialog>
    };

    let dialog = store_value(cx, dialog);

    create_effect(cx, move |_| {
        if opened() {
            dialog.with_value(|d| d.show_modal()).unwrap();
        }
    });

    dialog.get_value()
}
