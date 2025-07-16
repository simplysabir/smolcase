use anyhow::Result;
use colored::*;
use crate::config::ConfigManager;
use crate::ui::UI;

pub async fn execute() -> Result<()> {
    let config = ConfigManager::load_config()?;
    
    if config.secrets.is_empty() {
        UI::info("No secrets found");
        return Ok(());
    }
    
    UI::header("Secrets");
    
    for (key, secret) in &config.secrets {
        let type_icon = if secret.is_file { "📄" } else { "🔑" };
        let permissions = if secret.permissions.users.is_empty() && secret.permissions.groups.is_empty() {
            "all users".to_string()
        } else {
            let mut perms = Vec::new();
            if !secret.permissions.users.is_empty() {
                perms.push(format!("users: {}", secret.permissions.users.join(", ")));
            }
            if !secret.permissions.groups.is_empty() {
                perms.push(format!("groups: {}", secret.permissions.groups.join(", ")));
            }
            perms.join(", ")
        };
        
        println!("{} {} ({})", type_icon, key.cyan(), permissions.dimmed());
    }
    
    Ok(())
}