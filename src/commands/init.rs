use crate::config::ConfigManager;
use crate::crypto::CryptoManager;
use crate::git::GitManager;
use crate::types::{EncryptedData, PrivateConfig, SmolcaseConfig, User};
use crate::ui::UI;
use anyhow::{Result, anyhow};
use chrono::Utc;
use colored::*;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn execute(name: Option<String>, git: bool, non_interactive: bool) -> Result<()> {
    if !non_interactive {
        UI::print_banner();
    }

    if ConfigManager::is_smolcase_project() {
        return Err(anyhow!("Already a smolcase project"));
    }

    if non_interactive {
        // Original simple flow for scripts/CI
        return execute_simple(name, git).await;
    }

    // Interactive flow
    UI::header("🚀 Initialize Smolcase Project");
    UI::info("Let's set up your secure secret management in a few steps!");
    println!();

    // Step 1: Project name
    let project_name = if let Some(name) = name {
        name
    } else {
        let default_name = std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "My Secrets".to_string());

        let input = UI::input(&format!("Project name [{}]", default_name))?;
        if input.is_empty() {
            default_name
        } else {
            input
        }
    };

    // Step 2: Git repository
    let use_git = if git {
        true
    } else {
        UI::confirm("Initialize Git repository? (Recommended for team collaboration)")?
    };

    let mut git_remote = None;
    if use_git {
        if UI::confirm("Add GitHub/GitLab remote now?")? {
            let remote = UI::input("Repository URL (e.g., https://github.com/user/secrets)")?;
            if !remote.is_empty() {
                git_remote = Some(remote);
            }
        }
    }

    // Step 3: Admin credentials
    UI::header("🔐 Admin Setup");
    UI::info("Create an admin account to manage users and secrets.");

    let admin_username = UI::input("Admin username")?;
    let admin_email = UI::input_optional("Admin email")?;

    let admin_password = loop {
        let password = UI::password("Admin password (12+ characters)")?;
        if password.len() < 12 {
            UI::error("Password must be at least 12 characters long");
            continue;
        }
        if UI::confirm("Confirm this password?")? {
            break password;
        }
    };

    // Step 4: Master key
    UI::header("🗝️  Master Encryption Key");
    UI::info("This key encrypts all secrets. Share it securely with your team!");

    let master_key = if UI::confirm("Generate a secure master key automatically?")? {
        let generated = CryptoManager::generate_password() + &CryptoManager::generate_password();
        UI::success("Generated master key:");
        println!("{}", generated.bold().yellow());
        UI::warning("⚠️  SAVE THIS KEY! You'll need it to decrypt secrets.");

        if !UI::confirm("Continue with this master key?")? {
            loop {
                let key = UI::password("Enter your own master key (12+ characters)")?;
                if key.len() < 12 {
                    UI::error("Master key must be at least 12 characters long");
                    continue;
                }
                break key;
            }
        } else {
            generated
        }
    } else {
        loop {
            let key = UI::password("Master encryption key (12+ characters)")?;
            if key.len() < 12 {
                UI::error("Master key must be at least 12 characters long");
                continue;
            }
            break key;
        }
    };

    // Create configuration
    UI::info("Creating encrypted configuration...");

    let (password_hash, salt) = CryptoManager::hash_password(&admin_password)?;
    let (admin_key_hash, _) = CryptoManager::hash_password(&admin_password)?;
    let (master_key_hash, _) = CryptoManager::hash_password(&master_key)?;

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

    let public_config = SmolcaseConfig {
        version: "1.0.0".to_string(),
        project_name: project_name.clone(),
        created_at: Utc::now().to_rfc3339(),
        admin_key_hash,
        master_key_hash,
        encrypted_data: EncryptedData::default(),
    };

    let mut users = HashMap::new();
    users.insert(admin_username.clone(), admin_user);

    let private_config = PrivateConfig {
        users,
        groups: HashMap::new(),
        secrets: HashMap::new(),
        encrypted_secrets: EncryptedData::default(),
    };

    ConfigManager::create_config_dir()?;
    ConfigManager::save_config(&public_config, &private_config, &master_key)?;

    // Git setup
    if use_git {
        UI::info("Setting up Git repository...");
        let current_dir = std::env::current_dir()?;

        if !GitManager::is_git_repo(&current_dir) {
            GitManager::init_repo(&current_dir)?;
        }

        // Create .gitignore
        let gitignore_content = ".smolcase/credentials.json\n.env\n.env.local\n*.log\n";
        std::fs::write(".gitignore", gitignore_content)?;

        GitManager::add_and_commit(&current_dir, "Initial smolcase setup")?;

        if let Some(remote_url) = git_remote {
            if UI::confirm(&format!("Add remote origin: {}?", remote_url))? {
                std::process::Command::new("git")
                    .args(&["remote", "add", "origin", &remote_url])
                    .output()
                    .map_err(|e| anyhow!("Failed to add remote: {}", e))?;

                UI::success("Git remote added!");
                UI::info("Run 'git push -u origin main' to push to remote");
            }
        }
    }

    // Step 5: First secret (optional)
    if UI::confirm("🎯 Add your first secret now?")? {
        UI::info("Let's add a sample secret to get you started!");

        let secret_key = UI::input("Secret name (e.g., DATABASE_URL, API_KEY)")?;
        let secret_value = UI::password("Secret value")?;

        if !secret_key.is_empty() && !secret_value.is_empty() {
            // Add the secret
            crate::commands::add::execute(secret_key.clone(), Some(secret_value), None, None)
                .await?;
            UI::success(&format!("Added secret: {}", secret_key));
        }
    }

    // Final success message
    println!("\n{}", "🎉 Project Setup Complete!".bold().green());
    println!();

    UI::table_row("Project", &project_name);
    UI::table_row("Admin", &admin_username);
    UI::table_row("Git", if use_git { "Enabled" } else { "Disabled" });

    println!();
    println!(
        "{}",
        "🔑 IMPORTANT - Save these credentials:".bold().yellow()
    );
    println!("   Master key: {}", master_key.bold());
    println!("   Admin password: [as entered]");
    println!();

    println!("{}", "🚀 Next steps:".bold().cyan());
    println!("   • Run 'smolcase configure' to cache credentials");
    println!("   • Run 'smolcase add KEY value' to add secrets");
    println!("   • Run 'smolcase tutorial' for guided walkthrough");
    if use_git {
        println!("   • Push to remote: 'git push -u origin main'");
    }
    println!("   • Share master key securely with your team");

    Ok(())
}

async fn execute_simple(name: Option<String>, git: bool) -> Result<()> {
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

    UI::info("Set a master decryption key that will be shared with your team:");
    let master_key = UI::password("Master decryption key")?;

    if master_key.len() < 8 {
        return Err(anyhow!("Master key must be at least 8 characters long"));
    }

    UI::info("Creating project configuration...");

    let (password_hash, salt) = CryptoManager::hash_password(&admin_password)?;
    let (admin_key_hash, _) = CryptoManager::hash_password(&admin_password)?;
    let (master_key_hash, _) = CryptoManager::hash_password(&master_key)?;

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

    let public_config = SmolcaseConfig {
        version: "1.0.0".to_string(),
        project_name: project_name.clone(),
        created_at: Utc::now().to_rfc3339(),
        admin_key_hash,
        master_key_hash,
        encrypted_data: EncryptedData::default(),
    };

    let mut users = HashMap::new();
    users.insert(admin_username, admin_user);

    let private_config = PrivateConfig {
        users,
        groups: HashMap::new(),
        secrets: HashMap::new(),
        encrypted_secrets: EncryptedData::default(),
    };

    ConfigManager::create_config_dir()?;
    ConfigManager::save_config(&public_config, &private_config, &master_key)?;

    if git {
        UI::info("Initializing Git repository...");
        let current_dir = std::env::current_dir()?;
        GitManager::init_repo(&current_dir)?;
        GitManager::add_and_commit(&current_dir, "Initial smolcase setup")?;
    }

    UI::success(&format!(
        "Project '{}' initialized successfully!",
        project_name
    ));
    UI::warning("IMPORTANT: Share the master decryption key securely with your team!");
    UI::info(&format!("Master key: {}", master_key));
    UI::info("Run 'smolcase add' to start adding secrets");

    Ok(())
}
