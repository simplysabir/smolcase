use crate::config::ConfigManager;
use crate::git::GitManager;
use crate::ui::UI;
use anyhow::Result;
use colored::*;

pub async fn execute() -> Result<()> {
    if !ConfigManager::is_smolcase_project() {
        return Err(anyhow::anyhow!("Not a smolcase project"));
    }

    let config = ConfigManager::load_config()?;
    let current_dir = std::env::current_dir()?;

    UI::header(&format!("Project: {}", config.project_name));

    UI::table_row("Version", &config.version);
    UI::table_row("Created", &config.created_at);
    UI::table_row("Users", &config.users.len().to_string());
    UI::table_row("Groups", &config.groups.len().to_string());
    UI::table_row("Secrets", &config.secrets.len().to_string());
    UI::table_row(
        "Git Repository",
        if GitManager::is_git_repo(&current_dir) {
            "Yes"
        } else {
            "No"
        },
    );

    if !config.secrets.is_empty() {
        println!();
        UI::header("Recent Secrets");

        let mut secrets: Vec<_> = config.secrets.values().collect();
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

    if !config.users.is_empty() {
        println!();
        UI::header("Team Overview");

        let admin_count = config.users.values().filter(|u| u.is_admin).count();
        let user_count = config.users.len() - admin_count;

        UI::table_row("Admins", &admin_count.to_string());
        UI::table_row("Users", &user_count.to_string());
        UI::table_row("Groups", &config.groups.len().to_string());
    }

    Ok(())
}
