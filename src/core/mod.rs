//! This module implements the core functionality of the library.
//!
//! It handles the following:
//!
//! - Reading and writing to data storage
//! - Handling the application state
//! - Working with the git-like data structures
//!
//! It is structured into two submodules:  
//!
//! - [`bcp`](bcp/index.html) - The Box Canvas Page-based (BCP) notebook module
//! - [`sbp`](sbp/index.html) - The Sequential Block Page-based (SBP) notebook module
//! - [`common`](common/index.html) - The common module for both BCP and SBP notebooks

pub mod bcp;
pub mod common;
pub mod sbp;
