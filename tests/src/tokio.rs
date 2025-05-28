#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use tokio::fs::read_to_string;
    use workspace_root::tokio::get_workspace_root_async;

    #[tokio::test]
    async fn test_get_workspace_root() {
        let dir: PathBuf = get_workspace_root_async().await;

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
