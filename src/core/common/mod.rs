use gloo_storage::Storage;
use serde::{Deserialize, Serialize};

use crate::components::Change;

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
    pub sidenav_state: Change,
}

impl Settings {
    fn get() -> Settings {
        gloo_storage::LocalStorage::get("dn2.settings").unwrap_or_default()
    }

    fn write(&self) {
        gloo_storage::LocalStorage::set("dn2.settings", self).unwrap();
    }
}

pub fn set_sidenav_state(state: Change) {
    let mut settings = Settings::get();
    settings.sidenav_state = state;
    settings.write();
}

pub fn get_sidenav_state() -> Change {
    Settings::get().sidenav_state
}
