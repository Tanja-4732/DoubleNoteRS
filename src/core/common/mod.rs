#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NotebookType {
    #[default]
    BoxCanvasPage,
    SequentialBlockPage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextBoxState {
    // ProseMirror,
    // Markdown,
    // LaTeX,
    // Code,
    WYSIWYG,
    Markdown,
    Both,
}
