use crate::config::ConfigManager;
use crate::git::GitManager;
use crate::ui::UI;
use anyhow::Result;

pub async fn execute() -> Result<()> {
    if !ConfigManager::is_smolcase_project() {
        return Err(anyhow::anyhow!("Not a smolcase project"));
    }

    let current_dir = std::env::current_dir()?;

    if !GitManager::is_git_repo(&current_dir) {
        UI::warning("Not a Git repository. Initialize with 'git init' first.");
        return Ok(());
    }

    UI::info("Syncing with Git repository...");

    GitManager::add_and_commit(&current_dir, "Update smolcase configuration")?;

    UI::success("Synced successfully!");
    UI::info("Don't forget to push changes to remote repository");

    Ok(())
}
