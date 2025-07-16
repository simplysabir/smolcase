use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;

pub async fn execute(format: String, output: Option<PathBuf>) -> Result<()> {
    let config = ConfigManager::load_config()?;
    
    let password = UI::password("Password")?;
    
    let mut user_found = false;
    let mut accessible_secrets = Vec::new();
    
    for (username, user) in &config.users {
        if CryptoManager::verify_password(&password, &user.password_hash)? {
            user_found = true;
            
            if !config.encrypted_data.is_empty() {
                let admin_user = config.users.values().find(|u| u.is_admin).unwrap();
                let admin_key = CryptoManager::derive_key(&password, &admin_user.salt)?;
                let decrypted_data = CryptoManager::decrypt_data(&config.encrypted_data, &admin_key)?;
                let secrets: EncryptedSecrets = serde_json::from_slice(&decrypted_data)?;
                
                for secret_value in &secrets.secrets {
                    if let Some(secret_meta) = config.secrets.get(&secret_value.key) {
                        let has_permission = secret_meta.permissions.users.is_empty() && secret_meta.permissions.groups.is_empty() ||
                            secret_meta.permissions.users.contains(username) ||
                            secret_meta.permissions.groups.iter().any(|group| {
                                config.groups.get(group).map_or(false, |g| g.members.contains(username))
                            });
                        
                        if has_permission && !secret_value.is_file {
                            accessible_secrets.push((secret_value.key.clone(), secret_value.value.clone()));
                        }
                    }
                }
            }
            break;
        }
    }
    
    if !user_found {
        return Err(anyhow!("Invalid password"));
    }
    
    let content = match format.as_str() {
        "env" => {
            accessible_secrets.iter()
                .map(|(k, v)| format!("{}={}", k.to_uppercase(), v))
                .collect::<Vec<_>>()
                .join("\n")
        }
        "json" => {
            let map: HashMap<String, String> = accessible_secrets.into_iter().collect();
            serde_json::to_string_pretty(&map)?
        }
        "yaml" => {
            let map: HashMap<String, String> = accessible_secrets.into_iter().collect();
            serde_yaml::to_string(&map)?
        }
        _ => return Err(anyhow!("Unsupported format: {}", format))
    };
    
    if let Some(output_path) = output {
        fs::write(&output_path, content)?;
        UI::success(&format!("Exported to {}", output_path.display()));
    } else {
        println!("{}", content);
    }
    
    Ok(())
}