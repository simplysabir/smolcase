use crate::config::ConfigManager;
use crate::credential_manager::CredentialManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub async fn execute(format: String, output: Option<PathBuf>) -> Result<()> {
    let cached_creds = CredentialManager::load_credentials()?;

    let user_password = CredentialManager::get_user_password(&cached_creds)?;
    let master_key = CredentialManager::get_master_key(&cached_creds)?;

    let (_, private_config) = ConfigManager::load_full_config(&master_key)?;

    // Find and verify user
    let mut user_found = false;
    let mut username = String::new();

    // Try to use cached username first
    if let Some(cached_username) = &cached_creds.username {
        if let Some(user) = private_config.users.get(cached_username) {
            if CryptoManager::verify_password(&user_password, &user.password_hash)? {
                user_found = true;
                username = cached_username.clone();
            }
        }
    }

    // If cached username didn't work, try all users
    if !user_found {
        for (uname, user) in &private_config.users {
            if CryptoManager::verify_password(&user_password, &user.password_hash)? {
                user_found = true;
                username = uname.clone();
                break;
            }
        }
    }

    if !user_found {
        return Err(anyhow!("Invalid password"));
    }

    let mut accessible_secrets = Vec::new();

    if !private_config.encrypted_secrets.is_empty() {
        let decrypted_data =
            CryptoManager::decrypt_data_with_salt(&private_config.encrypted_secrets, &master_key)?;
        let secrets: EncryptedSecrets = serde_json::from_slice(&decrypted_data)?;

        for secret_value in &secrets.secrets {
            if let Some(secret_meta) = private_config.secrets.get(&secret_value.key) {
                let has_permission = secret_meta.permissions.users.is_empty()
                    && secret_meta.permissions.groups.is_empty()
                    || secret_meta.permissions.users.contains(&username)
                    || secret_meta.permissions.groups.iter().any(|group| {
                        private_config
                            .groups
                            .get(group)
                            .map_or(false, |g| g.members.contains(&username))
                    });

                if has_permission && !secret_value.is_file {
                    accessible_secrets.push((secret_value.key.clone(), secret_value.value.clone()));
                }
            }
        }
    }

    let content = match format.as_str() {
        "env" => accessible_secrets
            .iter()
            .map(|(k, v)| format!("{}={}", k.to_uppercase(), v))
            .collect::<Vec<_>>()
            .join("\n"),
        "json" => {
            let map: HashMap<String, String> = accessible_secrets.into_iter().collect();
            serde_json::to_string_pretty(&map)?
        }
        "yaml" => {
            let map: HashMap<String, String> = accessible_secrets.into_iter().collect();
            serde_yaml::to_string(&map)?
        }
        _ => return Err(anyhow!("Unsupported format: {}", format)),
    };

    if let Some(output_path) = output {
        fs::write(&output_path, content)?;
        UI::success(&format!("Exported to {}", output_path.display()));
    } else {
        println!("{}", content);
    }

    Ok(())
}
