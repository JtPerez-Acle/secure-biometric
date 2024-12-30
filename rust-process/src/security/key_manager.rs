use super::error::SecurityError;
use super::Result;
use ring::aead::{LessSafeKey, UnboundKey, CHACHA20_POLY1305};
use ring::rand::{SecureRandom, SystemRandom};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages encryption keys and provides secure key rotation
pub struct KeyManager {
    current_key: Arc<RwLock<LessSafeKey>>,
    old_key: Arc<RwLock<Option<LessSafeKey>>>,
    rng: SystemRandom,
}

impl Clone for KeyManager {
    fn clone(&self) -> Self {
        Self {
            current_key: self.current_key.clone(),
            old_key: self.old_key.clone(),
            rng: SystemRandom::new(),
        }
    }
}

impl KeyManager {
    /// Create a new key manager with a fresh encryption key
    pub fn new() -> Result<Self> {
        let rng = SystemRandom::new();
        let mut key_bytes = [0u8; 32];
        rng.fill(&mut key_bytes)
            .map_err(|e| SecurityError::KeyGeneration(e.to_string()))?;

        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &key_bytes)
            .map_err(|e| SecurityError::KeyGeneration(e.to_string()))?;
        let key = LessSafeKey::new(unbound_key);

        Ok(Self {
            current_key: Arc::new(RwLock::new(key)),
            old_key: Arc::new(RwLock::new(None)),
            rng,
        })
    }
    
    /// Start key rotation by generating a new key and preserving the old one
    pub async fn start_rotation(&self) -> Result<()> {
        let mut old_key = self.old_key.write().await;
        let current_key = self.current_key.read().await;
        *old_key = Some(current_key.clone());

        // Generate new key
        let mut key_bytes = [0u8; 32];
        self.rng
            .fill(&mut key_bytes)
            .map_err(|e| SecurityError::KeyGeneration(e.to_string()))?;

        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &key_bytes)
            .map_err(|e| SecurityError::KeyGeneration(e.to_string()))?;
        let new_key = LessSafeKey::new(unbound_key);

        // Update current key
        drop(current_key);
        let mut current = self.current_key.write().await;
        *current = new_key;

        Ok(())
    }

    /// Finish key rotation by clearing the old key
    pub async fn finish_rotation(&self) -> Result<()> {
        let mut old_key = self.old_key.write().await;
        *old_key = None;
        Ok(())
    }
    
    /// Get the current encryption key
    pub async fn current_key(&self) -> Result<tokio::sync::RwLockReadGuard<'_, LessSafeKey>> {
        Ok(self.current_key.read().await)
    }

    /// Get the old encryption key if it exists
    pub async fn old_key(
        &self,
    ) -> Result<tokio::sync::RwLockReadGuard<'_, Option<LessSafeKey>>> {
        Ok(self.old_key.read().await)
    }
    
    /// Generate a random nonce for encryption
    pub fn generate_nonce(&self) -> Result<[u8; 12]> {
        let mut nonce = [0u8; 12];
        self.rng
            .fill(&mut nonce)
            .map_err(|e| SecurityError::KeyGeneration(e.to_string()))?;
        Ok(nonce)
    }
}
