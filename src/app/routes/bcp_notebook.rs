use chrono::Utc;
use leptos::*;
use leptos_router::IntoParam;
use leptos_router::*;
use uuid::Uuid;

use crate::core::{
    bcp::{self},
    common::{set_title, NotebookType},
};

#[derive(Params, Debug, PartialEq, Clone)]
pub struct BCPNotebookParams {
    notebook_uuid: Option<Uuid>,
}
use crate::components::Card;

#[component]
pub fn BCPNotebook(cx: Scope) -> impl IntoView {
    let params = use_params::<BCPNotebookParams>(cx);

    let uuid = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.notebook_uuid)
                .unwrap_or_default()
                .unwrap_or_default()
        })
    };
    let uuid_string = move || uuid().to_string();

    view! { cx,
        <div>
            <h1>{"Notebooks"}</h1>
            <p>{"This is the notebooks page."}</p>
            <h2>{"My Notebook"}</h2>
            <p>{uuid_string}</p>
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
