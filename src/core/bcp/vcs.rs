//! The version control system (VCS) module.

use std::rc::Rc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct BCPTag {
    /// The timestamp of the tag
    pub timestamp: DateTime<Utc>,

    /// The name of the tag
    pub name: String,

    /// The description of the tag
    pub description: String,

    /// The hash of the commit this tag points to
    pub target_string: String,

    /// The commit this tag points to
    pub target: BCPCommit,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct BCPCommit {
    /// The timestamp of the commit
    pub timestamp: DateTime<Utc>,

    /// The hash of the previous commit
    pub previous_string: String,

    /// The previous commit
    pub previous: Option<Rc<BCPCommit>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum Head {
    Detached(BCPCommit),
    Branch {
        branch_name: String,
        commit: BCPCommit,
    },
}

impl Default for Head {
    fn default() -> Self {
        Self::Detached(BCPCommit {
            timestamp: Utc::now(),
            previous_string: "".to_string(),
            previous: None,
        })
    }
}

impl Head {
    pub fn is_detached(&self) -> bool {
        match self {
            Head::Detached(_) => true,
            Head::Branch { .. } => false,
        }
    }

    pub fn get_commit(&self) -> &BCPCommit {
        match self {
            Head::Detached(commit) => commit,
            Head::Branch { commit, .. } => commit,
        }
    }

    pub fn get_commit_mut(&mut self) -> &mut BCPCommit {
        match self {
            Head::Detached(commit) => commit,
            Head::Branch { commit, .. } => commit,
        }
    }

    pub fn try_get_branch_name(&self) -> Option<&str> {
        match self {
            Head::Detached(_) => None,
            Head::Branch { branch_name, .. } => Some(branch_name),
        }
    }
}
