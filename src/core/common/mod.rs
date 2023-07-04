use serde::{Deserialize, Serialize};

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
