use anyhow::{anyhow, Result};
use std::path::Path;
use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::git::GitManager;
use crate::ui::UI;

pub async fn execute(repo: Option<String>) -> Result<()> {
    let repo_path = if let Some(repo) = repo {
        if repo.starts_with("http") {
            let repo_name = repo.split('/').last().unwrap_or("smolcase-repo").replace(".git", "");
            let target_path = Path::new(&repo_name);
            
            if target_path.exists() {
                return Err(anyhow!("Directory '{}' already exists", repo_name));
            }
            
            UI::info(&format!("Cloning repository: {}", repo));
            GitManager::clone_repo(&repo, target_path)?;
            
            std::env::set_current_dir(target_path)?;
            repo_name
        } else {
            repo
        }
    } else {
        ".".to_string()
    };
    
    if !ConfigManager::is_smolcase_project() {
        return Err(anyhow!("Not a smolcase project"));
    }
    
    let config = ConfigManager::load_config()?;
    
    UI::header(&format!("Setup access for project: {}", config.project_name));
    
    let username = UI::input("Username")?;
    let password = UI::password("Password")?;
    
    if let Some(user) = config.users.get(&username) {
        if CryptoManager::verify_password(&password, &user.password_hash)? {
            UI::success(&format!("Access granted for user: {}", username));
            UI::info("You can now use 'smolcase get' to retrieve secrets");
        } else {
            return Err(anyhow!("Invalid password"));
        }
    } else {
        return Err(anyhow!("User not found"));
    }
    
    Ok(())
}