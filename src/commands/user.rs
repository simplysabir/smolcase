use crate::UserAction;
use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::User;
use crate::ui::UI;
use anyhow::{Result, anyhow};
use chrono::Utc;
use colored::*;
use uuid::Uuid;

pub async fn execute(action: UserAction) -> Result<()> {
    let public_config = ConfigManager::load_public_config()?;

    let admin_password = UI::password("Admin password")?;
    if !CryptoManager::verify_password(&admin_password, &public_config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin password"));
    }

    let master_key = UI::password("Master decryption key")?;
    let (_, mut private_config) = ConfigManager::load_full_config(&master_key)?;

    match action {
        UserAction::Add { username, email } => {
            if private_config.users.contains_key(&username) {
                return Err(anyhow!("User '{}' already exists", username));
            }

            let password = CryptoManager::generate_password();
            let (password_hash, salt) = CryptoManager::hash_password(&password)?;

            let user = User {
                id: Uuid::new_v4(),
                username: username.clone(),
                email,
                password_hash,
                salt,
                created_at: Utc::now().to_rfc3339(),
                last_access: None,
                is_admin: false,
            };

            private_config.users.insert(username.clone(), user);
            ConfigManager::save_config(&public_config, &private_config, &master_key)?;

            UI::success(&format!("User '{}' created successfully!", username));
            UI::info(&format!("Generated password: {}", password));
            UI::warning("Share this password securely with the user");
        }

        UserAction::Remove { username } => {
            if !private_config.users.contains_key(&username) {
                return Err(anyhow!("User '{}' not found", username));
            }

            if private_config.users[&username].is_admin {
                return Err(anyhow!("Cannot remove admin user"));
            }

            if !UI::confirm(&format!("Remove user '{}'?", username))? {
                return Ok(());
            }

            private_config.users.remove(&username);
            ConfigManager::save_config(&public_config, &private_config, &master_key)?;

            UI::success(&format!("User '{}' removed successfully!", username));
        }

        UserAction::List => {
            UI::header("Users");

            for (username, user) in &private_config.users {
                let role = if user.is_admin { "admin" } else { "user" };
                let last_access = user.last_access.as_deref().unwrap_or("never");

                println!(
                    "{} {} ({})",
                    if user.is_admin { "👑" } else { "👤" },
                    username.cyan(),
                    format!("{}, last access: {}", role, last_access).dimmed()
                );
            }
        }

        UserAction::Reset { username } => {
            if !private_config.users.contains_key(&username) {
                return Err(anyhow!("User '{}' not found", username));
            }

            let new_password = CryptoManager::generate_password();
            let (password_hash, salt) = CryptoManager::hash_password(&new_password)?;

            if let Some(user) = private_config.users.get_mut(&username) {
                user.password_hash = password_hash;
                user.salt = salt;
            }

            ConfigManager::save_config(&public_config, &private_config, &master_key)?;

            UI::success(&format!("Password reset for user '{}'", username));
            UI::info(&format!("New password: {}", new_password));
            UI::warning("Share this password securely with the user");
        }
    }

    Ok(())
}