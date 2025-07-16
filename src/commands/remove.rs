use anyhow::{anyhow, Result};
use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;

pub async fn execute(key: String) -> Result<()> {
    let mut config = ConfigManager::load_config()?;
    
    if !config.secrets.contains_key(&key) {
        return Err(anyhow!("Secret '{}' not found", key));
    }
    
    let admin_password = UI::password("Admin password")?;
    if !CryptoManager::verify_password(&admin_password, &config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin password"));
    }
    
    let master_key = UI::password("Master decryption key")?;
    if !CryptoManager::verify_password(&master_key, &config.master_key_hash)? {
        return Err(anyhow!("Invalid master key"));
    }
    
    if !UI::confirm(&format!("Are you sure you want to remove '{}'?", key))? {
        return Ok(());
    }
    
    if !config.encrypted_data.is_empty() {
        let master_encryption_key = CryptoManager::derive_key_from_password(&master_key)?;
        let decrypted_data = CryptoManager::decrypt_data(&config.encrypted_data, &master_encryption_key)?;
        let mut existing_secrets: EncryptedSecrets = serde_json::from_slice(&decrypted_data)?;
        
        existing_secrets.secrets.retain(|s| s.key != key);
        
        let serialized_secrets = serde_json::to_vec(&existing_secrets)?;
        let encrypted_data = CryptoManager::encrypt_data(&serialized_secrets, &master_encryption_key)?;
        config.encrypted_data = encrypted_data;
    }
    
    config.secrets.remove(&key);
    
    ConfigManager::save_config(&config)?;
    
    UI::success(&format!("Secret '{}' removed successfully!", key));
    
    Ok(())
}
