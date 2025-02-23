# Workspace Root

An utility to get workspace root.

The functions will try to get the workspace root by searching for the `Cargo.lock` file. Then it will return the directory that contains the file. If the file is not found, an error will be returned instead.

## Usage

Get workspace root with the following code:

```rust
use std::path::PathBuf;

use workspace_root::get_workspace_root;

let root: PathBuf = get_workspace_root();
```

Async version also available with `async-std`/`async_std` and `tokio` features:

```rust
// This is a `async-std` example

use async_std::path::PathBuf;

use workspace_root::async_std::get_workspace_root_async;

let root: PathBuf = get_workspace_root_async().await;
```

```rust
// This is a `tokio` example

use std::path::PathBuf;

use workspace_root::tokio::get_workspace_root_async;

let root: PathBuf = get_workspace_root_async().await;
```

## License

This project is licensed under the terms of the MIT license.
