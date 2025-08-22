#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use macro_rules_attribute::apply;
    use smol::fs::read_to_string;
    use smol_macros::test;
    use workspace_root::smol::get_workspace_root_async;

    #[apply(test)]
    async fn test_get_workspace_root() {
        let dir: PathBuf = get_workspace_root_async().await;

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
