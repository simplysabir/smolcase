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
    
    let user_password = UI::password("Your password")?;
    
    let mut user_found = false;
    let mut username = String::new();
    
    for (uname, user) in &config.users {
        if CryptoManager::verify_password(&user_password, &user.password_hash)? {
            user_found = true;
            username = uname.clone();
            break;
        }
    }
    
    if !user_found {
        return Err(anyhow!("Invalid password"));
    }
    
    let secret = &config.secrets[&key];
    let has_permission = secret.permissions.users.is_empty() && secret.permissions.groups.is_empty() ||
        secret.permissions.users.contains(&username) ||
        secret.permissions.groups.iter().any(|group| {
            config.groups.get(group).map_or(false, |g| g.members.contains(&username))
        });
    
    if !has_permission {
        return Err(anyhow!("Access denied"));
    }
    
    let master_key = UI::password("Master decryption key")?;
    if !CryptoManager::verify_password(&master_key, &config.master_key_hash)? {
        return Err(anyhow!("Invalid master key"));
    }
    
    let master_encryption_key = CryptoManager::derive_key_from_password(&master_key)?;
    let decrypted_data = CryptoManager::decrypt_data(&config.encrypted_data, &master_encryption_key)?;
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