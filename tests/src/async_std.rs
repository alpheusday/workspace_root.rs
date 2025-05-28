#[cfg(test)]
mod tests {

    use async_std::{fs::read_to_string, path::PathBuf};

    use workspace_root::async_std::get_workspace_root_async;

    #[tokio::test]
    async fn test_get_workspace_root() {
        let dir: PathBuf = get_workspace_root_async().await;

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
