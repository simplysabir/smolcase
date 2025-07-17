use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmolcaseConfig {
    pub version: String,
    pub project_name: String,
    pub created_at: String,
    pub admin_key_hash: String,        // Only for admin verification
    pub master_key_hash: String,       // Only for master key verification
    pub encrypted_data: EncryptedData, // ALL sensitive data encrypted
}

// PRIVATE config - encrypted with master key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateConfig {
    pub users: HashMap<String, User>,
    pub groups: HashMap<String, Group>,
    pub secrets: HashMap<String, Secret>,
    pub encrypted_secrets: EncryptedData, // Double-encrypted secret values
}

// Local credential cache - stored locally, never committed
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalCredentials {
    pub admin_password: Option<String>,
    pub user_password: Option<String>,
    pub username: Option<String>,
    pub master_key: Option<String>,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub salt: String, // Base64 encoded random salt
    pub data: String, // Base64 encoded encrypted data
}

impl Default for EncryptedData {
    fn default() -> Self {
        Self {
            salt: String::new(),
            data: String::new(),
        }
    }
}

impl EncryptedData {
    pub fn is_empty(&self) -> bool {
        self.salt.is_empty() && self.data.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub salt: String,
    pub created_at: String,
    pub last_access: Option<String>,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub id: Uuid,
    pub key: String,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
    pub permissions: Permissions,
    pub is_file: bool,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions {
    pub users: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretValue {
    pub key: String,
    pub value: String,
    pub is_file: bool,
    pub file_content: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedSecrets {
    pub secrets: Vec<SecretValue>,
}
