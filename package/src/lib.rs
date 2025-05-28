//! # Workspace Root
//!
//! A utility to get workspace root.
//!
//! The functions will try to get the workspace root by searching for the `Cargo.lock` file.
//! Then it will return the directory that contains the file.
//! If the file is not found, an error will be returned instead.
//!
//! ## Usage
//!
//! Get workspace root with the following code:
//!
//! ```no_run
//! use std::path::PathBuf;
//!
//! use workspace_root::get_workspace_root;
//!
//! let root: PathBuf = get_workspace_root();
//! ```
//!
//! Async version also available with `async_std` and `tokio` features:
//!
//! ```no_run
//! // This is a `async_std` example
//!
//! use async_std::path::PathBuf;
//!
//! use workspace_root::async_std::get_workspace_root_async;
//!
//! # async fn example() {
//! let root: PathBuf = get_workspace_root_async().await;
//! # }
//! ```
//!
//! ```no_run
//! // This is a `tokio` example
//!
//! use std::path::PathBuf;
//!
//! use workspace_root::tokio::get_workspace_root_async;
//!
//! # async fn example() {
//! let root: PathBuf = get_workspace_root_async().await;
//! # }
//! ```

/// Async functions available with `async_std` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// workspace_root = { version = "*", features = ["async_std"] }
/// ```
#[cfg(feature = "async_std")]
pub mod async_std;

/// Async functions available with `tokio` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// workspace_root = { version = "*", features = ["tokio"] }
/// ```
#[cfg(feature = "tokio")]
pub mod tokio;

use std::{io, path::PathBuf};

use get_dir::{FileTarget, GetDir, Target};

/// Get the workspace root directory by searching for the `Cargo.lock` file.
///
/// Use [`get_workspace_root`] to handle the error automatically.
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use workspace_root::get_workspace_root_directory;
///
/// let root: PathBuf = get_workspace_root_directory().unwrap();
/// ```
pub fn get_workspace_root_directory() -> io::Result<PathBuf> {
    GetDir::new()
        .targets(vec![Target::File(FileTarget { name: "Cargo.lock" })])
        .run_reverse()
}

/// Get the workspace root directory by searching for the `Cargo.lock` file.
///
/// Use [`get_workspace_root_directory`] to handle the error manually.
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use workspace_root::get_workspace_root;
///
/// let root: PathBuf = get_workspace_root();
/// ```
pub fn get_workspace_root() -> PathBuf {
    get_workspace_root_directory().expect("Failed to get workspace root")
}
