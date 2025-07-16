use anyhow::{anyhow, Result};
use serde_yaml;
use std::fs;
use std::path::PathBuf;
use crate::types::SmolcaseConfig;

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
    
    pub fn load_config() -> Result<SmolcaseConfig> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            return Err(anyhow!("Not a smolcase project. Run 'smolcase init' first."));
        }
        
        let content = fs::read_to_string(&config_path)
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
        
        let config: SmolcaseConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow!("Invalid config file: {}", e))?;
        
        Ok(config)
    }
    
    pub fn save_config(config: &SmolcaseConfig) -> Result<()> {
        let config_path = Self::config_path()?;
        
        let content = serde_yaml::to_string(config)
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