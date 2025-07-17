use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::types::LocalCredentials;
use crate::ui::UI;
use anyhow::{Result, anyhow};
use serde_json;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct CredentialManager;

impl CredentialManager {
    fn credentials_path() -> Result<PathBuf> {
        let config_dir = ConfigManager::config_dir()?;
        Ok(config_dir.join("credentials.json"))
    }

    pub fn save_credentials(credentials: &LocalCredentials) -> Result<()> {
        ConfigManager::create_config_dir()?;
        let creds_path = Self::credentials_path()?;

        // Encrypt the credentials with a simple key derived from system info
        let system_key = Self::get_system_key()?;
        let creds_data = serde_json::to_vec(credentials)?;
        let encrypted_creds = CryptoManager::encrypt_data_with_salt(&creds_data, &system_key)?;

        let encrypted_json = serde_json::to_string_pretty(&encrypted_creds)?;
        fs::write(&creds_path, encrypted_json)?;

        Ok(())
    }

    pub fn load_credentials() -> Result<LocalCredentials> {
        let creds_path = Self::credentials_path()?;

        if !creds_path.exists() {
            return Ok(LocalCredentials::default());
        }

        let encrypted_json = fs::read_to_string(&creds_path)?;
        let encrypted_creds: crate::types::EncryptedData = serde_json::from_str(&encrypted_json)?;

        let system_key = Self::get_system_key()?;
        let creds_data = CryptoManager::decrypt_data_with_salt(&encrypted_creds, &system_key)?;
        let credentials: LocalCredentials = serde_json::from_slice(&creds_data)?;

        Ok(credentials)
    }

    pub fn clear_credentials() -> Result<()> {
        let creds_path = Self::credentials_path()?;
        if creds_path.exists() {
            fs::remove_file(&creds_path)?;
        }
        Ok(())
    }

    fn get_system_key() -> Result<String> {
        // Create a simple key from system information
        // This is not super secure but provides basic protection for local storage
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "default".to_string());

        let hostname = hostname::get()
            .unwrap_or_else(|_| "localhost".into())
            .to_string_lossy()
            .to_string();

        Ok(format!("smolcase-{}-{}", username, hostname))
    }

    pub fn get_admin_password(cached_creds: &LocalCredentials) -> Result<String, io::Error> {
        if let Some(password) = &cached_creds.admin_password {
            if !password.is_empty() {
                return Ok(password.clone());
            }
        }
        UI::password("Admin password")
    }

    pub fn get_user_password(cached_creds: &LocalCredentials) -> Result<String, io::Error> {
        if let Some(password) = &cached_creds.user_password {
            if !password.is_empty() {
                return Ok(password.clone());
            }
        }
        UI::password("Your password")
    }

    pub fn get_master_key(cached_creds: &LocalCredentials) -> Result<String, io::Error> {
        if let Some(key) = &cached_creds.master_key {
            if !key.is_empty() {
                return Ok(key.clone());
            }
        }
        UI::password("Master decryption key")
    }

    pub fn get_username(cached_creds: &LocalCredentials) -> Result<String, io::Error> {
        if let Some(username) = &cached_creds.username {
            if !username.is_empty() {
                return Ok(username.clone());
            }
        }
        UI::input("Username")
    }
}
