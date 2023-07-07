use std::{cell::RefCell, rc::Rc};

use gloo_storage::Storage;
use leptos::{use_context, Scope, SignalSet, WriteSignal};
use serde::{Deserialize, Serialize};

use crate::components::{NavState, NavType};

// pub fn set_title(cx: Scope, title: Rc<str>) {
pub fn set_title(cx: Scope, title: &'static str) {
    use_context::<WriteSignal<&str>>(cx)
        // we know we just provided this in the parent component
        .expect("there to be a `title` signal provided")
        .set(&title);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum NotebookType {
    #[default]
    BoxCanvasPage,
    SequentialBlockPage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextBoxState {
    // ProseMirror,
    // Markdown,
    // LaTeX,
    // Code,
    WYSIWYG,
    Markdown,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Settings {
    pub sidenav_state: NavState,
    pub sidenav_type: NavType,
}

impl Settings {
    fn get() -> Settings {
        gloo_storage::LocalStorage::get("dn2.settings").unwrap_or_default()
    }

    fn write(&self) {
        gloo_storage::LocalStorage::set("dn2.settings", self).unwrap();
    }
}

pub fn set_sidenav_state(state: NavState) {
    let mut settings = Settings::get();
    settings.sidenav_state = state;
    settings.write();
}

pub fn get_sidenav_state() -> NavState {
    Settings::get().sidenav_state
}

pub fn set_sidenav_type(state: NavType) {
    let mut settings = Settings::get();
    settings.sidenav_type = state;
    settings.write();
}

pub fn get_sidenav_type() -> NavType {
    Settings::get().sidenav_type
}
