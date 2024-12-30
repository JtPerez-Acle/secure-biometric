use super::error::SecurityError;
use super::key_manager::KeyManager;
use super::Result;
use ring::aead::{Aad, Nonce, CHACHA20_POLY1305};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Encryption engine for secure template storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Encrypted data bytes
    pub ciphertext: Vec<u8>,
    /// Nonce used for encryption
    pub nonce: [u8; 12],
}

pub struct EncryptionEngine {
    key_manager: Arc<KeyManager>,
}

impl Clone for EncryptionEngine {
    fn clone(&self) -> Self {
        Self {
            key_manager: self.key_manager.clone(),
        }
    }
}

impl EncryptionEngine {
    /// Create a new encryption engine with a key manager
    pub fn new(key_manager: Arc<KeyManager>) -> Self {
        Self { key_manager }
    }

    /// Encrypt data using ChaCha20-Poly1305
    pub async fn encrypt(&self, data: &[u8]) -> Result<EncryptedData> {
        let nonce_bytes = self.key_manager.generate_nonce()?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let key = self.key_manager.current_key().await?;
        let mut in_out = data.to_vec();
        key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| SecurityError::Encryption(e.to_string()))?;

        Ok(EncryptedData {
            ciphertext: in_out,
            nonce: nonce_bytes,
        })
    }

    /// Decrypt data using ChaCha20-Poly1305
    pub async fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        let key = self.key_manager.current_key().await?;

        // Try with current key first
        let mut in_out = encrypted.ciphertext.clone();
        match key.open_in_place(
            Nonce::assume_unique_for_key(encrypted.nonce),
            Aad::empty(),
            &mut in_out,
        ) {
            Ok(_) => {
                in_out.truncate(in_out.len() - CHACHA20_POLY1305.tag_len());
                return Ok(in_out);
            }
            Err(_) => {
                // Try with old key if available
                if let Some(old_key) = &*self.key_manager.old_key().await? {
                    let mut in_out = encrypted.ciphertext.clone();
                    old_key
                        .open_in_place(
                            Nonce::assume_unique_for_key(encrypted.nonce),
                            Aad::empty(),
                            &mut in_out,
                        )
                        .map_err(|e| SecurityError::Decryption(e.to_string()))?;
                    in_out.truncate(in_out.len() - CHACHA20_POLY1305.tag_len());
                    return Ok(in_out);
                }
            }
        }

        Err(SecurityError::Decryption("Failed to decrypt data".into()))
    }

    /// Start key rotation process
    pub async fn rotate_key(&self) -> Result<()> {
        self.key_manager.start_rotation().await?;
        Ok(())
    }

    /// Re-encrypt data with the current key
    pub async fn reencrypt(&self, data: &[u8]) -> Result<EncryptedData> {
        // Encrypt with new key
        self.encrypt(data).await
    }

    /// Complete key rotation process
    pub async fn finish_rotation(&self) -> Result<()> {
        self.key_manager.finish_rotation().await?;
        Ok(())
    }
}
