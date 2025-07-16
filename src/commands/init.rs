use anyhow::{anyhow, Result};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;
use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::git::GitManager;
use crate::types::{SmolcaseConfig, User};
use crate::ui::UI;

pub async fn execute(name: Option<String>, git: bool) -> Result<()> {
    UI::print_banner();
    
    if ConfigManager::is_smolcase_project() {
        return Err(anyhow!("Already a smolcase project"));
    }
    
    UI::header("Initialize Smolcase Project");
    
    let project_name = if let Some(name) = name {
        name
    } else {
        UI::input("Project name")?
    };
    
    let admin_username = UI::input("Admin username")?;
    let admin_email = UI::input_optional("Admin email")?;
    let admin_password = UI::password("Admin password")?;
    
    if admin_password.len() < 8 {
        return Err(anyhow!("Admin password must be at least 8 characters long"));
    }
    
    // NEW: Ask for master decryption key
    UI::info("Set a master decryption key that will be shared with your team:");
    let master_key = UI::password("Master decryption key")?;
    
    if master_key.len() < 8 {
        return Err(anyhow!("Master key must be at least 8 characters long"));
    }
    
    UI::info("Creating project configuration...");
    
    let (password_hash, salt) = CryptoManager::hash_password(&admin_password)?;
    let (admin_key_hash, _) = CryptoManager::hash_password(&admin_password)?;
    let (master_key_hash, _) = CryptoManager::hash_password(&master_key)?; // NEW
    
    let admin_user = User {
        id: Uuid::new_v4(),
        username: admin_username.clone(),
        email: admin_email,
        password_hash,
        salt,
        created_at: Utc::now().to_rfc3339(),
        last_access: None,
        is_admin: true,
    };
    
    let mut users = HashMap::new();
    users.insert(admin_username, admin_user);
    
    let config = SmolcaseConfig {
        version: "1.0.0".to_string(),
        project_name: project_name.clone(),
        created_at: Utc::now().to_rfc3339(),
        admin_key_hash,
        master_key_hash, // NEW
        users,
        groups: HashMap::new(),
        secrets: HashMap::new(),
        encrypted_data: String::new(),
    };
    
    ConfigManager::create_config_dir()?;
    ConfigManager::save_config(&config)?;
    
    if git {
        UI::info("Initializing Git repository...");
        let current_dir = std::env::current_dir()?;
        GitManager::init_repo(&current_dir)?;
        GitManager::add_and_commit(&current_dir, "Initial smolcase setup")?;
    }
    
    UI::success(&format!("Project '{}' initialized successfully!", project_name));
    UI::warning("IMPORTANT: Share the master decryption key securely with your team!");
    UI::info(&format!("Master key: {}", master_key));
    UI::info("Run 'smolcase add' to start adding secrets");
    
    Ok(())
}
