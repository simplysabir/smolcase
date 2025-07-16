use anyhow::{Result, anyhow};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit, OsRng as ChaChaOsRng},
};
use rand::RngCore;

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

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

    pub fn derive_key_from_password(password: &str) -> Result<[u8; KEY_SIZE]> {
        let fixed_salt = b"smolcase_salt_16"; // 16 bytes exactly

        let argon2 = Argon2::default();
        let mut key = [0u8; KEY_SIZE];

        argon2
            .hash_password_into(password.as_bytes(), fixed_salt, &mut key)
            .map_err(|e| anyhow!("Key derivation failed: {}", e))?;

        Ok(key)
    }

    pub fn encrypt_data(data: &[u8], key: &[u8; KEY_SIZE]) -> Result<String> {
        let cipher = ChaCha20Poly1305::new(key.into());
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        ChaChaOsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(BASE64.encode(result))
    }

    pub fn decrypt_data(encrypted: &str, key: &[u8; KEY_SIZE]) -> Result<Vec<u8>> {
        let data = BASE64
            .decode(encrypted)
            .map_err(|e| anyhow!("Invalid base64: {}", e))?;

        if data.len() < NONCE_SIZE {
            return Err(anyhow!("Invalid encrypted data"));
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = ChaCha20Poly1305::new(key.into());
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
