use std::path::PathBuf;

use smol::io;

use get_dir::{FileTarget, GetDir, Target, smol::GetDirAsyncExt as _};

/// Get the workspace root directory by searching for the `Cargo.lock` file.
///
/// Use [`get_workspace_root_async`] to handle the error automatically.
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use workspace_root::smol::get_workspace_root_directory_async;
///
/// # async fn example() {
/// let root: PathBuf = get_workspace_root_directory_async().await.unwrap();
/// # }
/// ```
pub async fn get_workspace_root_directory_async() -> io::Result<PathBuf> {
    GetDir::new()
        .target(Target::File(FileTarget::new("Cargo.lock")))
        .run_reverse_async()
        .await
}

/// Get the workspace root directory by searching for the `Cargo.lock` file.
///
/// Use [`get_workspace_root_directory_async`] to handle the error manually.
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use workspace_root::smol::get_workspace_root_async;
///
/// # async fn example() {
/// let root: PathBuf = get_workspace_root_async().await;
/// # }
/// ```
pub async fn get_workspace_root_async() -> PathBuf {
    get_workspace_root_directory_async()
        .await
        .expect("Failed to get workspace root")
}
