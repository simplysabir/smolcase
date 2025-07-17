use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;
use anyhow::{Result, anyhow};

pub async fn execute(key: String) -> Result<()> {
    let user_password = UI::password("Your password")?;
    let master_key = UI::password("Master decryption key")?;
    
    let (public_config, private_config) = ConfigManager::load_full_config(&master_key)?;

    if !private_config.secrets.contains_key(&key) {
        return Err(anyhow!("Secret '{}' not found", key));
    }

    // Find and verify user
    let mut user_found = false;
    let mut username = String::new();

    for (uname, user) in &private_config.users {
        if CryptoManager::verify_password(&user_password, &user.password_hash)? {
            user_found = true;
            username = uname.clone();
            break;
        }
    }

    if !user_found {
        return Err(anyhow!("Invalid password"));
    }

    // Check permissions
    let secret = &private_config.secrets[&key];
    let has_permission = secret.permissions.users.is_empty()
        && secret.permissions.groups.is_empty()
        || secret.permissions.users.contains(&username)
        || secret.permissions.groups.iter().any(|group| {
            private_config
                .groups
                .get(group)
                .map_or(false, |g| g.members.contains(&username))
        });

    if !has_permission {
        return Err(anyhow!("Access denied"));
    }

    // Decrypt and get secret value
    let decrypted_data = CryptoManager::decrypt_data_with_salt(&private_config.encrypted_secrets, &master_key)?;
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
