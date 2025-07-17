use crate::credential_manager::CredentialManager;
use crate::ui::UI;
use anyhow::Result;

pub async fn execute() -> Result<()> {
    UI::info("Clearing stored credentials...");

    CredentialManager::clear_credentials()?;

    UI::success("Credentials cleared successfully!");
    UI::info("You will be prompted for passwords on next command.");

    Ok(())
}
