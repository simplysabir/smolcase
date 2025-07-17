use crate::types::{SmolcaseConfig, PrivateConfig, EncryptedData};
use crate::crypto::CryptoManager;
use anyhow::{Result, anyhow};
use serde_yaml;
use std::fs;
use std::path::PathBuf;

pub const CONFIG_FILE: &str = ".smolcase.yml";
pub const CONFIG_DIR: &str = ".smolcase";

pub struct ConfigManager;

impl ConfigManager {
    pub fn config_path() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()
            .map_err(|e| anyhow!("Failed to get current directory: {}", e))?;
        Ok(current_dir.join(CONFIG_FILE))
    }

    pub fn config_dir() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()
            .map_err(|e| anyhow!("Failed to get current directory: {}", e))?;
        Ok(current_dir.join(CONFIG_DIR))
    }

    pub fn is_smolcase_project() -> bool {
        Self::config_path().map(|p| p.exists()).unwrap_or(false)
    }

    pub fn load_public_config() -> Result<SmolcaseConfig> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Err(anyhow!(
                "Not a smolcase project. Run 'smolcase init' first."
            ));
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;

        let config: SmolcaseConfig =
            serde_yaml::from_str(&content).map_err(|e| anyhow!("Invalid config file: {}", e))?;

        Ok(config)
    }

    pub fn load_full_config(master_key: &str) -> Result<(SmolcaseConfig, PrivateConfig)> {
        let public_config = Self::load_public_config()?;
        
        if !CryptoManager::verify_password(master_key, &public_config.master_key_hash)? {
            return Err(anyhow!("Invalid master key"));
        }
        
        let private_config = if public_config.encrypted_data.is_empty() {
            PrivateConfig {
                users: std::collections::HashMap::new(),
                groups: std::collections::HashMap::new(),
                secrets: std::collections::HashMap::new(),
                encrypted_secrets: EncryptedData::default(),
            }
        } else {
            let private_data = CryptoManager::decrypt_data_with_salt(&public_config.encrypted_data, master_key)?;
            serde_json::from_slice(&private_data)?
        };
        
        Ok((public_config, private_config))
    }

    pub fn save_config(
        public_config: &SmolcaseConfig, 
        private_config: &PrivateConfig, 
        master_key: &str
    ) -> Result<()> {
        let private_data = serde_json::to_vec(private_config)?;
        let encrypted_data = CryptoManager::encrypt_data_with_salt(&private_data, master_key)?;
        
        let final_config = SmolcaseConfig {
            version: public_config.version.clone(),
            project_name: public_config.project_name.clone(),
            created_at: public_config.created_at.clone(),
            admin_key_hash: public_config.admin_key_hash.clone(),
            master_key_hash: public_config.master_key_hash.clone(),
            encrypted_data,
        };
        
        let config_path = Self::config_path()?;
        let content = serde_yaml::to_string(&final_config)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;

        fs::write(&config_path, content)
            .map_err(|e| anyhow!("Failed to write config file: {}", e))?;

        Ok(())
    }

    pub fn create_config_dir() -> Result<()> {
        let config_dir = Self::config_dir()?;

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| anyhow!("Failed to create config directory: {}", e))?;
        }

        Ok(())
    }
}