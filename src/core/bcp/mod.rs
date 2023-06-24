//! This is the module for Box Canvas Page-based (BCP) notebooks.

use std::collections::HashMap;

use super::common;

mod vcs;

pub struct BCPNotebook {
    pub name: String,
    pub notebook_type: common::NotebookType,
    pub uuid: uuid::Uuid,

    /// The branches of this notebook
    ///
    /// Every string is a branch name and points to a commit hash
    pub branch_strings: HashMap<String, String>,

    /// The hashes of the tags of this notebook
    pub tag_strings: HashMap<String, String>,

    /// The currently selected object
    ///
    /// This can be:
    ///
    /// 1. A branch name
    /// 2. A commit hash
    ///
    /// They will be checked in this order.
    ///
    // TODO make this an enum
    pub head_string: String,

    /// The hash of the working tree
    ///
    /// This is the hash root tree node of the working tree
    pub working_tree_string: String,

    /// The branches of this notebook
    pub branches: HashMap<String, vcs::BCPCommit>,

    /// The tags of this notebook
    pub tags: Vec<vcs::BCPTag>,

    /// The commit or branch to which the HEAD points to
    pub head: vcs::Head,

    /// The working tree
    ///
    /// This is the root tree node of the working tree
    // TODO create a universal tree node
    pub working_tree: CategoryTree,
}

pub struct CategoryTree {
    /// The name of this category
    pub name: String,

    /// The hashes of the child trees
    pub children_strings: Vec<String>,

    /// The hashes of the direct pages of this category
    pub pages_strings: Vec<String>,

    /// The child trees
    pub children: Vec<CategoryTree>,

    /// The direct pages of this category
    pub pages: Vec<BoxCanvasPage>,
}

pub struct BoxCanvasPage {
    /// The title of this page
    pub title: String,

    /// The UUID of this page which stays the same between edits
    pub uuid: uuid::Uuid,

    /// The hashes of the boxes of this page
    pub box_hash_strings: Vec<String>,

    /// The boxes of this page
    pub boxes: Vec<TextBox>,
}

pub struct TextBox {
    /// The UUID of the text box
    pub uuid: uuid::Uuid,

    /// Which editor(s) to display in the text box
    pub state: common::TextBoxState,

    /// The ProseMirror document contained in this box
    // TODO handle this
    pub pm_doc: String,

    /// The x position of the text box
    pub x: f64,

    /// The y position of the text box
    pub y: f64,

    /// The width of the text box
    pub width: f64,

    /// The height of the text box
    pub height: f64,
}
