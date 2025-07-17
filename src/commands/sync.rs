use crate::config::ConfigManager;
use crate::credential_manager::CredentialManager;
use crate::crypto::CryptoManager;
use crate::git::GitManager;
use crate::ui::UI;
use anyhow::{Result, anyhow};

pub async fn execute() -> Result<()> {
    if !ConfigManager::is_smolcase_project() {
        return Err(anyhow!("Not a smolcase project"));
    }

    let cached_creds = CredentialManager::load_credentials()?;

    if !cached_creds.is_admin {
        return Err(anyhow!(
            "Only admins can sync with Git repository. Use 'smolcase configure' to set up admin credentials."
        ));
    }

    let public_config = ConfigManager::load_public_config()?;
    let admin_password = CredentialManager::get_admin_password(&cached_creds)?;

    if !CryptoManager::verify_password(&admin_password, &public_config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin credentials"));
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
