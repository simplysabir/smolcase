use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::{EncryptedSecrets, Permissions, Secret, SecretValue};
use crate::ui::UI;
use anyhow::{Result, anyhow};
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub async fn execute(file: PathBuf, format: String) -> Result<()> {
    let mut config = ConfigManager::load_config()?;

    let admin_password = UI::password("Admin password")?;
    if !CryptoManager::verify_password(&admin_password, &config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin password"));
    }

    let master_key = UI::password("Master decryption key")?;
    if !CryptoManager::verify_password(&master_key, &config.master_key_hash)? {
        return Err(anyhow!("Invalid master key"));
    }

    let content = fs::read_to_string(&file)?;

    let secrets_map: HashMap<String, String> = match format.as_str() {
        "env" => {
            let mut map = HashMap::new();
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    map.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            map
        }
        "json" => serde_json::from_str(&content)?,
        "yaml" => serde_yaml::from_str(&content)?,
        _ => return Err(anyhow!("Unsupported format: {}", format)),
    };

    if secrets_map.is_empty() {
        UI::warning("No secrets found in file");
        return Ok(());
    }

    UI::info(&format!("Found {} secrets to import", secrets_map.len()));

    if !UI::confirm("Continue with import?")? {
        return Ok(());
    }

    let mut existing_secrets = if config.encrypted_data.is_empty() {
        EncryptedSecrets {
            secrets: Vec::new(),
        }
    } else {
        let master_encryption_key = CryptoManager::derive_key_from_password(&master_key)?;
        let decrypted_data =
            CryptoManager::decrypt_data(&config.encrypted_data, &master_encryption_key)?;
        serde_json::from_slice(&decrypted_data)?
    };

    let mut imported_count = 0;

    for (key, value) in secrets_map {
        let secret = Secret {
            id: Uuid::new_v4(),
            key: key.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            created_by: "admin".to_string(),
            permissions: Permissions {
                users: Vec::new(),
                groups: Vec::new(),
            },
            is_file: false,
            file_path: None,
        };

        let secret_value = SecretValue {
            key: key.clone(),
            value,
            is_file: false,
            file_content: None,
        };

        if let Some(pos) = existing_secrets.secrets.iter().position(|s| s.key == key) {
            existing_secrets.secrets[pos] = secret_value;
        } else {
            existing_secrets.secrets.push(secret_value);
        }

        config.secrets.insert(key.clone(), secret);
        imported_count += 1;
    }

    let master_encryption_key = CryptoManager::derive_key_from_password(&master_key)?;
    let serialized_secrets = serde_json::to_vec(&existing_secrets)?;
    let encrypted_data = CryptoManager::encrypt_data(&serialized_secrets, &master_encryption_key)?;

    config.encrypted_data = encrypted_data;
    ConfigManager::save_config(&config)?;

    UI::success(&format!(
        "Imported {} secrets successfully!",
        imported_count
    ));

    Ok(())
}
