use anyhow::{anyhow, Result};
use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;

pub async fn execute(key: String) -> Result<()> {
    let config = ConfigManager::load_config()?;
    
    if !config.secrets.contains_key(&key) {
        return Err(anyhow!("Secret '{}' not found", key));
    }
    
    let password = UI::password("Password")?;
    
    let mut user_found = false;
    for (username, user) in &config.users {
        if CryptoManager::verify_password(&password, &user.password_hash)? {
            user_found = true;
            
            let secret = &config.secrets[&key];
            let has_permission = secret.permissions.users.is_empty() && secret.permissions.groups.is_empty() ||
                secret.permissions.users.contains(username) ||
                secret.permissions.groups.iter().any(|group| {
                    config.groups.get(group).map_or(false, |g| g.members.contains(username))
                });
            
            if !has_permission {
                return Err(anyhow!("Access denied"));
            }
            
            break;
        }
    }
    
    if !user_found {
        return Err(anyhow!("Invalid password"));
    }
    
    let admin_user = config.users.values().find(|u| u.is_admin).unwrap();
    let admin_key = CryptoManager::derive_key(&password, &admin_user.salt)?;
    let decrypted_data = CryptoManager::decrypt_data(&config.encrypted_data, &admin_key)?;
    let secrets: EncryptedSecrets = serde_json::from_slice(&decrypted_data)?;
    
    if let Some(secret_value) = secrets.secrets.iter().find(|s| s.key == key) {
        if secret_value.is_file {
            UI::info(&format!("File: {}", key));
            if let Some(content) = &secret_value.file_content {
                println!("{}", String::from_utf8_lossy(content));
            }
        } else {
            println!("{}", secret_value.value);
        }
    } else {
        return Err(anyhow!("Secret value not found"));
    }
    
    Ok(())
}
