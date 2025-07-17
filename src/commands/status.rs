use crate::config::ConfigManager;
use crate::git::GitManager;
use crate::ui::UI;
use anyhow::Result;
use colored::*;

pub async fn execute() -> Result<()> {
    if !ConfigManager::is_smolcase_project() {
        return Err(anyhow::anyhow!("Not a smolcase project"));
    }

    let public_config = ConfigManager::load_public_config()?;
    let current_dir = std::env::current_dir()?;

    UI::header(&format!("Project: {}", public_config.project_name));

    UI::table_row("Version", &public_config.version);
    UI::table_row("Created", &public_config.created_at);

    // Try to get private info if master key is available
    let master_key_result = UI::password("Master decryption key (optional for full status)");
    
    if let Ok(master_key) = master_key_result {
        if let Ok((_, private_config)) = ConfigManager::load_full_config(&master_key) {
            UI::table_row("Users", &private_config.users.len().to_string());
            UI::table_row("Groups", &private_config.groups.len().to_string());
            UI::table_row("Secrets", &private_config.secrets.len().to_string());

            if !private_config.secrets.is_empty() {
                println!();
                UI::header("Recent Secrets");

                let mut secrets: Vec<_> = private_config.secrets.values().collect();
                secrets.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

                for secret in secrets.iter().take(5) {
                    let type_icon = if secret.is_file { "📄" } else { "🔑" };
                    println!(
                        "{} {} (updated: {})",
                        type_icon,
                        secret.key.cyan(),
                        secret.updated_at.dimmed()
                    );
                }
            }

            if !private_config.users.is_empty() {
                println!();
                UI::header("Team Overview");

                let admin_count = private_config.users.values().filter(|u| u.is_admin).count();
                let user_count = private_config.users.len() - admin_count;

                UI::table_row("Admins", &admin_count.to_string());
                UI::table_row("Users", &user_count.to_string());
                UI::table_row("Groups", &private_config.groups.len().to_string());
            }
        } else {
            UI::warning("Invalid master key - showing limited status");
            UI::table_row("Users", "encrypted");
            UI::table_row("Groups", "encrypted");
            UI::table_row("Secrets", "encrypted");
        }
    } else {
        UI::info("Master key not provided - showing limited status");
        UI::table_row("Users", "encrypted");
        UI::table_row("Groups", "encrypted");
        UI::table_row("Secrets", "encrypted");
    }

    UI::table_row(
        "Git Repository",
        if GitManager::is_git_repo(&current_dir) {
            "Yes"
        } else {
            "No"
        },
    );

    Ok(())
}
