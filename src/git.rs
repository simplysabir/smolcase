use anyhow::{Result, anyhow};
use git2::{Repository, Signature, Time};
use std::path::Path;

pub struct GitManager;

impl GitManager {
    pub fn init_repo(path: &Path) -> Result<()> {
        Repository::init(path)
            .map_err(|e| anyhow!("Failed to initialize Git repository: {}", e))?;

        Ok(())
    }

    pub fn add_and_commit(path: &Path, message: &str) -> Result<()> {
        let repo =
            Repository::open(path).map_err(|e| anyhow!("Failed to open Git repository: {}", e))?;

        let mut index = repo
            .index()
            .map_err(|e| anyhow!("Failed to get Git index: {}", e))?;

        index
            .add_path(Path::new(".smolcase.yml"))
            .map_err(|e| anyhow!("Failed to add file to Git index: {}", e))?;

        index
            .write()
            .map_err(|e| anyhow!("Failed to write Git index: {}", e))?;

        let tree_id = index
            .write_tree()
            .map_err(|e| anyhow!("Failed to write Git tree: {}", e))?;

        let tree = repo
            .find_tree(tree_id)
            .map_err(|e| anyhow!("Failed to find Git tree: {}", e))?;

        let sig = Signature::now("smolcase", "smolcase@example.com")
            .map_err(|e| anyhow!("Failed to create Git signature: {}", e))?;

        let head = repo.head().ok();
        let parent = head
            .as_ref()
            .and_then(|h| h.target())
            .and_then(|oid| repo.find_commit(oid).ok());

        let parents: Vec<&git2::Commit> = if let Some(ref p) = parent {
            vec![p]
        } else {
            vec![]
        };

        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .map_err(|e| anyhow!("Failed to create Git commit: {}", e))?;

        Ok(())
    }

    pub fn clone_repo(url: &str, path: &Path) -> Result<()> {
        Repository::clone(url, path).map_err(|e| anyhow!("Failed to clone repository: {}", e))?;

        Ok(())
    }

    pub fn is_git_repo(path: &Path) -> bool {
        Repository::open(path).is_ok()
    }
}
