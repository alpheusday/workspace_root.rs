pub mod async_std;

pub mod smol;

pub mod tokio;

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::PathBuf};

    use workspace_root::get_workspace_root;

    #[test]
    fn test_get_workspace_root() {
        let dir: PathBuf = get_workspace_root();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
