use crate::GroupAction;
use crate::config::ConfigManager;
use crate::credential_manager::CredentialManager;
use crate::crypto::CryptoManager;
use crate::types::Group;
use crate::ui::UI;
use anyhow::{Result, anyhow};
use chrono::Utc;
use colored::*;
use uuid::Uuid;

pub async fn execute(action: GroupAction) -> Result<()> {
    let public_config = ConfigManager::load_public_config()?;
    let cached_creds = CredentialManager::load_credentials()?;

    if !cached_creds.is_admin {
        return Err(anyhow!(
            "Only admins can manage groups. Use 'smolcase configure' to set up admin credentials."
        ));
    }

    let admin_password = CredentialManager::get_admin_password(&cached_creds)?;
    if !CryptoManager::verify_password(&admin_password, &public_config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin password"));
    }

    let master_key = CredentialManager::get_master_key(&cached_creds)?;
    let (_, mut private_config) = ConfigManager::load_full_config(&master_key)?;

    match action {
        GroupAction::Create { name, description } => {
            if private_config.groups.contains_key(&name) {
                return Err(anyhow!("Group '{}' already exists", name));
            }

            let group = Group {
                id: Uuid::new_v4(),
                name: name.clone(),
                description,
                members: Vec::new(),
                created_at: Utc::now().to_rfc3339(),
            };

            private_config.groups.insert(name.clone(), group);
            ConfigManager::save_config(&public_config, &private_config, &master_key)?;

            UI::success(&format!("Group '{}' created successfully!", name));
        }

        GroupAction::Delete { name } => {
            if !private_config.groups.contains_key(&name) {
                return Err(anyhow!("Group '{}' not found", name));
            }

            if !UI::confirm(&format!("Delete group '{}'?", name))? {
                return Ok(());
            }

            private_config.groups.remove(&name);
            ConfigManager::save_config(&public_config, &private_config, &master_key)?;

            UI::success(&format!("Group '{}' deleted successfully!", name));
        }

        GroupAction::List => {
            UI::header("Groups");

            for (name, group) in &private_config.groups {
                println!(
                    "👥 {} ({} members)",
                    name.cyan(),
                    group.members.len().to_string().dimmed()
                );
                if let Some(desc) = &group.description {
                    println!("   {}", desc.dimmed());
                }
            }
        }

        GroupAction::AddUser { group, users } => {
            if !private_config.groups.contains_key(&group) {
                return Err(anyhow!("Group '{}' not found", group));
            }

            let mut added_users = Vec::new();

            if let Some(group_obj) = private_config.groups.get_mut(&group) {
                for username in users {
                    if !private_config.users.contains_key(&username) {
                        UI::warning(&format!("User '{}' not found, skipping", username));
                        continue;
                    }

                    if !group_obj.members.contains(&username) {
                        group_obj.members.push(username.clone());
                        added_users.push(username);
                    } else {
                        UI::warning(&format!("User '{}' already in group", username));
                    }
                }
            }

            ConfigManager::save_config(&public_config, &private_config, &master_key)?;

            if !added_users.is_empty() {
                UI::success(&format!(
                    "Added users to group '{}': {}",
                    group,
                    added_users.join(", ")
                ));
            }
        }

        GroupAction::RemoveUser { group, users } => {
            if !private_config.groups.contains_key(&group) {
                return Err(anyhow!("Group '{}' not found", group));
            }

            let mut removed_users = Vec::new();

            if let Some(group_obj) = private_config.groups.get_mut(&group) {
                for username in users {
                    if let Some(pos) = group_obj.members.iter().position(|u| u == &username) {
                        group_obj.members.remove(pos);
                        removed_users.push(username);
                    } else {
                        UI::warning(&format!("User '{}' not in group", username));
                    }
                }
            }

            ConfigManager::save_config(&public_config, &private_config, &master_key)?;

            if !removed_users.is_empty() {
                UI::success(&format!(
                    "Removed users from group '{}': {}",
                    group,
                    removed_users.join(", ")
                ));
            }
        }
    }

    Ok(())
}
