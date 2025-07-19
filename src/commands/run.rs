use crate::config::ConfigManager;
use crate::credential_manager::CredentialManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::process::Command;

pub async fn execute(env: Option<String>, command: Vec<String>) -> Result<()> {
    if command.is_empty() {
        return Err(anyhow!(
            "No command specified. Use: smolcase run -- <command>"
        ));
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

    let mut env_vars = HashMap::new();

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
                    env_vars.insert(secret_value.key.clone(), secret_value.value.clone());
                }
            }
        }
    }

    if env_vars.is_empty() {
        UI::warning("No accessible secrets found");
    } else {
        UI::info(&format!("Running command with {} secrets", env_vars.len()));
    }

    let program = &command[0];
    let args = if command.len() > 1 {
        &command[1..]
    } else {
        &[]
    };

    let mut cmd = Command::new(program);
    cmd.args(args);

    // Add secrets as environment variables
    for (key, value) in &env_vars {
        cmd.env(key, value);
    }

    // Preserve existing environment
    cmd.envs(std::env::vars());

    let status = cmd
        .status()
        .map_err(|e| anyhow!("Failed to execute command '{}': {}", program, e))?;

    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        } else {
            std::process::exit(1);
        }
    }

    Ok(())
}
