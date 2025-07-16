use anyhow::{anyhow, Result};
use chrono::Utc;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use base64::Engine;

use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::{EncryptedSecrets, Permissions, Secret, SecretValue};
use crate::ui::UI;

pub async fn execute(key: String, value: Option<String>, users: Option<String>, groups: Option<String>) -> Result<()> {
    let mut config = ConfigManager::load_config()?;
    
    let admin_password = UI::password("Admin password")?;
    if !CryptoManager::verify_password(&admin_password, &config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin password"));
    }
    
    let master_key = UI::password("Master decryption key")?;
    if !CryptoManager::verify_password(&master_key, &config.master_key_hash)? {
        return Err(anyhow!("Invalid master key"));
    }
    
    let is_file = Path::new(&key).exists();
    let secret_key = if is_file {
        Path::new(&key).file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&key)
            .to_string()
    } else {
        key.clone()
    };
    
    let secret_value = if is_file {
        UI::info(&format!("Adding file: {}", key));
        let file_content = fs::read(&key)?;
        base64::engine::general_purpose::STANDARD.encode(&file_content)
    } else {
        value.unwrap_or_else(|| {
            UI::password("Secret value").unwrap_or_default()
        })
    };
    
    let mut permissions = Permissions {
        users: Vec::new(),
        groups: Vec::new(),
    };
    
    if let Some(users_str) = users {
        permissions.users = users_str.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    if let Some(groups_str) = groups {
        permissions.groups = groups_str.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    if permissions.users.is_empty() && permissions.groups.is_empty() {
        UI::info("No permissions specified. This secret will be accessible to all users.");
        if !UI::confirm("Continue?")? {
            return Ok(());
        }
    }
    
    let secret = Secret {
        id: Uuid::new_v4(),
        key: secret_key.clone(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        created_by: "admin".to_string(),
        permissions,
        is_file,
        file_path: if is_file { Some(key.clone()) } else { None },
    };
    
    let mut existing_secrets = if config.encrypted_data.is_empty() {
        EncryptedSecrets { secrets: Vec::new() }
    } else {
        let master_encryption_key = CryptoManager::derive_key_from_password(&master_key)?;
        let decrypted_data = CryptoManager::decrypt_data(&config.encrypted_data, &master_encryption_key)?;
        serde_json::from_slice(&decrypted_data)?
    };
    
    let new_secret_value = SecretValue {
        key: secret_key.clone(),
        value: secret_value,
        is_file,
        file_content: if is_file { Some(fs::read(&key)?) } else { None },
    };
    
    if let Some(pos) = existing_secrets.secrets.iter().position(|s| s.key == secret_key) {
        existing_secrets.secrets[pos] = new_secret_value;
        UI::info(&format!("Updated secret: {}", secret_key));
    } else {
        existing_secrets.secrets.push(new_secret_value);
        UI::info(&format!("Added secret: {}", secret_key));
    }
    
    let master_encryption_key = CryptoManager::derive_key_from_password(&master_key)?;
    let serialized_secrets = serde_json::to_vec(&existing_secrets)?;
    let encrypted_data = CryptoManager::encrypt_data(&serialized_secrets, &master_encryption_key)?;
    
    config.encrypted_data = encrypted_data;
    config.secrets.insert(secret_key.clone(), secret);
    
    ConfigManager::save_config(&config)?;
    
    UI::success(&format!("Secret '{}' added successfully!", secret_key));
    
    Ok(())
}
