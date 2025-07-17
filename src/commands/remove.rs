use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::EncryptedSecrets;
use crate::ui::UI;
use anyhow::{Result, anyhow};

pub async fn execute(key: String) -> Result<()> {
    let public_config = ConfigManager::load_public_config()?;

    let admin_password = UI::password("Admin password")?;
    if !CryptoManager::verify_password(&admin_password, &public_config.admin_key_hash)? {
        return Err(anyhow!("Invalid admin password"));
    }

    let master_key = UI::password("Master decryption key")?;
    let (_, mut private_config) = ConfigManager::load_full_config(&master_key)?;

    if !private_config.secrets.contains_key(&key) {
        return Err(anyhow!("Secret '{}' not found", key));
    }

    if !UI::confirm(&format!("Are you sure you want to remove '{}'?", key))? {
        return Ok(());
    }

    // Remove from encrypted secrets
    if !private_config.encrypted_secrets.is_empty() {
        let decrypted_data = CryptoManager::decrypt_data_with_salt(&private_config.encrypted_secrets, &master_key)?;
        let mut existing_secrets: EncryptedSecrets = serde_json::from_slice(&decrypted_data)?;

        existing_secrets.secrets.retain(|s| s.key != key);

        let serialized_secrets = serde_json::to_vec(&existing_secrets)?;
        private_config.encrypted_secrets = CryptoManager::encrypt_data_with_salt(&serialized_secrets, &master_key)?;
    }

    // Remove from metadata
    private_config.secrets.remove(&key);

    ConfigManager::save_config(&public_config, &private_config, &master_key)?;

    UI::success(&format!("Secret '{}' removed successfully!", key));

    Ok(())
}
