use crate::config::ConfigManager;
use crate::credential_manager::CredentialManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;
use anyhow::{Result, anyhow};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub async fn execute(
    template: PathBuf,
    output: Option<PathBuf>,
    env: Option<String>,
) -> Result<()> {
    if !template.exists() {
        return Err(anyhow!("Template file not found: {}", template.display()));
    }

    let cached_creds = CredentialManager::load_credentials()?;
    let user_password = CredentialManager::get_user_password(&cached_creds)?;
    let master_key = CredentialManager::get_master_key(&cached_creds)?;

    let (_, private_config) = ConfigManager::load_full_config(&master_key)?;

    // Find and verify user
    let mut user_found = false;
    let mut username = String::new();

    if let Some(cached_username) = &cached_creds.username {
        if let Some(user) = private_config.users.get(cached_username) {
            if CryptoManager::verify_password(&user_password, &user.password_hash)? {
                user_found = true;
                username = cached_username.clone();
            }
        }
    }

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

    let mut secrets_map = HashMap::new();

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
                    secrets_map.insert(secret_value.key.clone(), secret_value.value.clone());
                }
            }
        }
    }

    let template_content = fs::read_to_string(&template)
        .map_err(|e| anyhow!("Failed to read template file: {}", e))?;

    // Replace {{SECRET_NAME}} with actual secret values
    let re = Regex::new(r"\{\{([A-Za-z0-9_]+)\}\}")
        .map_err(|e| anyhow!("Failed to create regex: {}", e))?;

    let mut missing_secrets = Vec::new();
    let processed_content = re.replace_all(&template_content, |caps: &regex::Captures| {
        let secret_name = &caps[1];
        if let Some(secret_value) = secrets_map.get(secret_name) {
            secret_value.clone()
        } else {
            missing_secrets.push(secret_name.to_string());
            format!("{{{{MISSING:{}}}}}", secret_name)
        }
    });

    if !missing_secrets.is_empty() {
        UI::warning(&format!("Missing secrets: {}", missing_secrets.join(", ")));
        UI::info("These will be left as {{MISSING:SECRET_NAME}} in the output");
    }

    let result = processed_content.to_string();

    if let Some(output_path) = output {
        fs::write(&output_path, &result)
            .map_err(|e| anyhow!("Failed to write output file: {}", e))?;

        UI::success(&format!("Template applied to {}", output_path.display()));

        if !missing_secrets.is_empty() {
            UI::warning(&format!("{} secrets were missing", missing_secrets.len()));
        } else {
            UI::info(&format!("Substituted {} secrets", secrets_map.len()));
        }
    } else {
        println!("{}", result);
    }

    Ok(())
}
