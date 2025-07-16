use anyhow::{anyhow, Result};
use chrono::Utc;
use uuid::Uuid;
use colored::*;
use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::Group;
use crate::ui::UI;
use crate::{GroupAction};

pub async fn execute(action: GroupAction) -> Result<()> {
    let mut config = ConfigManager::load_config()?;
    
    let admin_password = UI::password("Admin password")?;
    if !CryptoManager::verify_password(&admin_password, &config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin password"));
    }
    
    match action {
        GroupAction::Create { name, description } => {
            if config.groups.contains_key(&name) {
                return Err(anyhow!("Group '{}' already exists", name));
            }
            
            let group = Group {
                id: Uuid::new_v4(),
                name: name.clone(),
                description,
                members: Vec::new(),
                created_at: Utc::now().to_rfc3339(),
            };
            
            config.groups.insert(name.clone(), group);
            ConfigManager::save_config(&config)?;
            
            UI::success(&format!("Group '{}' created successfully!", name));
        }
        
        GroupAction::Delete { name } => {
            if !config.groups.contains_key(&name) {
                return Err(anyhow!("Group '{}' not found", name));
            }
            
            if !UI::confirm(&format!("Delete group '{}'?", name))? {
                return Ok(());
            }
            
            config.groups.remove(&name);
            ConfigManager::save_config(&config)?;
            
            UI::success(&format!("Group '{}' deleted successfully!", name));
        }
        
        GroupAction::List => {
            UI::header("Groups");
            
            for (name, group) in &config.groups {
                println!("👥 {} ({} members)", 
                    name.cyan(),
                    group.members.len().to_string().dimmed()
                );
                if let Some(desc) = &group.description {
                    println!("   {}", desc.dimmed());
                }
            }
        }
        
        GroupAction::AddUser { group, users } => {
            if !config.groups.contains_key(&group) {
                return Err(anyhow!("Group '{}' not found", group));
            }
            
            let mut added_users = Vec::new();
            
            if let Some(group_obj) = config.groups.get_mut(&group) {
                for username in users {
                    if !config.users.contains_key(&username) {
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
            
            ConfigManager::save_config(&config)?;
            
            if !added_users.is_empty() {
                UI::success(&format!("Added users to group '{}': {}", group, added_users.join(", ")));
            }
        }
        
        GroupAction::RemoveUser { group, users } => {
            if !config.groups.contains_key(&group) {
                return Err(anyhow!("Group '{}' not found", group));
            }
            
            let mut removed_users = Vec::new();
            
            if let Some(group_obj) = config.groups.get_mut(&group) {
                for username in users {
                    if let Some(pos) = group_obj.members.iter().position(|u| u == &username) {
                        group_obj.members.remove(pos);
                        removed_users.push(username);
                    } else {
                        UI::warning(&format!("User '{}' not in group", username));
                    }
                }
            }
            
            ConfigManager::save_config(&config)?;
            
            if !removed_users.is_empty() {
                UI::success(&format!("Removed users from group '{}': {}", group, removed_users.join(", ")));
            }
        }
    }
    
    Ok(())
}