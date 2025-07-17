use crate::config::ConfigManager;
use crate::credential_manager::CredentialManager;
use crate::crypto::CryptoManager;
use crate::types::LocalCredentials;
use crate::ui::UI;
use anyhow::{Result, anyhow};

pub async fn execute() -> Result<()> {
    if !ConfigManager::is_smolcase_project() {
        return Err(anyhow!(
            "Not a smolcase project. Run 'smolcase init' first."
        ));
    }

    UI::header("Configure Local Credentials");
    UI::info("This will store your credentials locally to avoid repeated password prompts.");
    UI::warning("Credentials are encrypted and stored only on this machine.");

    let public_config = ConfigManager::load_public_config()?;
    let mut credentials = LocalCredentials::default();

    // Ask if user is admin
    let is_admin = UI::confirm("Are you an admin for this project?")?;
    credentials.is_admin = is_admin;

    if is_admin {
        UI::info("Setting up admin credentials...");

        // Get and verify admin password
        let admin_password = UI::password("Admin password")?;
        if !CryptoManager::verify_password(&admin_password, &public_config.admin_key_hash)? {
            return Err(anyhow!("Invalid admin password"));
        }
        credentials.admin_password = Some(admin_password);

        // Get and verify master key
        let master_key = UI::password("Master decryption key")?;
        if !CryptoManager::verify_password(&master_key, &public_config.master_key_hash)? {
            return Err(anyhow!("Invalid master key"));
        }
        credentials.master_key = Some(master_key.clone());

        // For admin, we can also set up user credentials if they want
        if UI::confirm("Do you also want to set up user credentials for day-to-day operations?")? {
            let username = UI::input("Your username")?;
            let user_password = UI::password("Your user password")?;

            // Verify user exists and password is correct
            let (_, private_config) = ConfigManager::load_full_config(&master_key)?;
            if let Some(user) = private_config.users.get(&username) {
                if CryptoManager::verify_password(&user_password, &user.password_hash)? {
                    credentials.username = Some(username);
                    credentials.user_password = Some(user_password);
                } else {
                    UI::warning("Invalid user password, skipping user credentials");
                }
            } else {
                UI::warning("User not found, skipping user credentials");
            }
        }
    } else {
        UI::info("Setting up user credentials...");

        // Get username
        let username = UI::input("Username")?;
        credentials.username = Some(username.clone());

        // Get user password
        let user_password = UI::password("Your password")?;
        credentials.user_password = Some(user_password.clone());

        // Get and verify master key
        let master_key = UI::password("Master decryption key")?;
        if !CryptoManager::verify_password(&master_key, &public_config.master_key_hash)? {
            return Err(anyhow!("Invalid master key"));
        }
        credentials.master_key = Some(master_key.clone());

        // Verify user exists and password is correct
        let (_, private_config) = ConfigManager::load_full_config(&master_key)?;
        if let Some(user) = private_config.users.get(&username) {
            if !CryptoManager::verify_password(&user_password, &user.password_hash)? {
                return Err(anyhow!("Invalid user password"));
            }
        } else {
            return Err(anyhow!("User '{}' not found", username));
        }
    }

    // Save credentials
    CredentialManager::save_credentials(&credentials)?;

    UI::success("Credentials configured successfully!");
    UI::info("You can now run commands without repeated password prompts.");
    UI::info("Use 'smolcase logout' to clear stored credentials.");

    Ok(())
}
