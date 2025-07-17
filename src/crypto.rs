use crate::types::EncryptedData;
use anyhow::{Result, anyhow};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit, OsRng as ChaChaOsRng},
};
use rand::RngCore;
use serde::{Deserialize, Serialize};

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;
const SALT_SIZE: usize = 32;

pub struct CryptoManager;

impl CryptoManager {
    pub fn hash_password(password: &str) -> Result<(String, String)> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        Ok((password_hash.to_string(), salt.to_string()))
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| anyhow!("Invalid password hash: {}", e))?;

        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    fn derive_key_with_salt(password: &str, salt: &[u8]) -> Result<[u8; KEY_SIZE]> {
        if salt.len() < 16 {
            return Err(anyhow!("Salt must be at least 16 bytes"));
        }

        let argon2 = Argon2::default();
        let mut key = [0u8; KEY_SIZE];

        argon2
            .hash_password_into(password.as_bytes(), &salt[..16], &mut key)
            .map_err(|e| anyhow!("Key derivation failed: {}", e))?;

        Ok(key)
    }

    fn generate_salt() -> [u8; SALT_SIZE] {
        let mut salt = [0u8; SALT_SIZE];
        OsRng.fill_bytes(&mut salt);
        salt
    }

    pub fn encrypt_data_with_salt(data: &[u8], password: &str) -> Result<EncryptedData> {
        let salt = Self::generate_salt();
        let key = Self::derive_key_with_salt(password, &salt)?;

        let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        ChaChaOsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&nonce_bytes);
        encrypted_data.extend_from_slice(&ciphertext);

        Ok(EncryptedData {
            salt: BASE64.encode(salt),
            data: BASE64.encode(encrypted_data),
        })
    }

    pub fn decrypt_data_with_salt(encrypted: &EncryptedData, password: &str) -> Result<Vec<u8>> {
        if encrypted.salt.is_empty() || encrypted.data.is_empty() {
            return Err(anyhow!("No encrypted data found"));
        }

        let salt = BASE64
            .decode(&encrypted.salt)
            .map_err(|e| anyhow!("Invalid salt base64: {}", e))?;

        let key = Self::derive_key_with_salt(password, &salt)?;

        let data = BASE64
            .decode(&encrypted.data)
            .map_err(|e| anyhow!("Invalid data base64: {}", e))?;

        if data.len() < NONCE_SIZE {
            return Err(anyhow!("Invalid encrypted data"));
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }

    pub fn generate_password() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{Rng, thread_rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}
