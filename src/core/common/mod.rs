use gloo_storage::Storage;
use serde::{Deserialize, Serialize};

use crate::components::{NavState, NavType};

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
