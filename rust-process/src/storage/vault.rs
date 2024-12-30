use super::error::StorageError;
use super::Result;
use crate::security::{EncryptedData, EncryptionEngine, KeyManager};
use crate::templates::Template;
use sled::Db;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde_json;

/// Secure storage for biometric templates
#[derive(Clone)]
pub struct TemplateVault {
    db: Arc<RwLock<Db>>,
    encryption: Arc<EncryptionEngine>,
}

impl Drop for TemplateVault {
    fn drop(&mut self) {
        // Attempt to get a write lock and flush the database
        if let Ok(db) = self.db.try_write() {
            let _ = db.flush();
            let _ = db.flush_async(); // Ensure all async operations are flushed
        }
    }
}

impl TemplateVault {
    /// Create a new template vault at the specified path
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db_config = sled::Config::new()
            .mode(sled::Mode::HighThroughput)
            .flush_every_ms(Some(1000))
            .cache_capacity(1024 * 1024 * 128) // 128MB cache
            .path(path);

        let db = db_config.open()?;
        let key_manager = Arc::new(KeyManager::new().map_err(|e| StorageError::Encryption(e))?);
        let encryption = Arc::new(EncryptionEngine::new(key_manager));

        Ok(Self {
            db: Arc::new(RwLock::new(db)),
            encryption,
        })
    }

    /// Store a template securely
    pub async fn store(&self, template: Template) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let template_bytes = serde_json::to_vec(&template)
            .map_err(|e| StorageError::Serialization(Box::new(bincode::ErrorKind::Custom(e.to_string()))))?;
        
        // Encrypt template data
        let encrypted = self.encryption.encrypt(&template_bytes).await
            .map_err(|e| StorageError::Encryption(e))?;
        let storage_data = serde_json::to_vec(&encrypted)
            .map_err(|e| StorageError::Serialization(Box::new(bincode::ErrorKind::Custom(e.to_string()))))?;
        
        // Use batch operation for atomic writes
        let mut batch = sled::Batch::default();
        batch.insert(id.as_bytes(), storage_data);
        self.db.write().await.apply_batch(batch)?;

        Ok(id)
    }

    /// Retrieve a template by ID
    pub async fn get(&self, id: Uuid) -> Result<Template> {
        let encrypted_data = self.db
            .read().await
            .get(id.as_bytes())?
            .ok_or_else(|| StorageError::NotFound(id))?;

        let encrypted: EncryptedData = serde_json::from_slice(&encrypted_data)
            .map_err(|e| StorageError::Serialization(Box::new(bincode::ErrorKind::Custom(e.to_string()))))?;
        let template_bytes = self.encryption.decrypt(&encrypted).await
            .map_err(|e| StorageError::Encryption(e))?;
        let template: Template = serde_json::from_slice(&template_bytes)
            .map_err(|e| StorageError::Serialization(Box::new(bincode::ErrorKind::Custom(e.to_string()))))?;
        
        Ok(template)
    }

    /// Delete a template by ID
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let mut batch = sled::Batch::default();
        batch.remove(id.as_bytes());
        self.db.write().await.apply_batch(batch)?;
        Ok(())
    }

    /// List all template IDs
    pub async fn list_ids(&self) -> Result<Vec<Uuid>> {
        let db = self.db.read().await;
        let mut ids = Vec::new();
        
        for item in db.iter() {
            let (key, _) = item?;
            if let Ok(id) = Uuid::from_slice(&key) {
                ids.push(id);
            }
        }
        
        Ok(ids)
    }

    /// Rotate encryption key and re-encrypt all templates
    pub async fn rotate_key(&self) -> Result<()> {
        // Start key rotation
        self.encryption.rotate_key().await
            .map_err(|e| StorageError::Encryption(e))?;

        // Re-encrypt all templates with new key
        let mut batch = sled::Batch::default();
        let db = self.db.read().await;

        // First collect all the data we need to re-encrypt
        let mut items = Vec::new();
        for item in db.iter() {
            let (key, value) = item?;
            items.push((key.to_vec(), value.to_vec()));
        }

        // Drop the read lock before processing
        drop(db);

        // Process each item
        for (key, value) in items {
            // Decrypt with old key
            let encrypted: EncryptedData = serde_json::from_slice(&value)
                .map_err(|e| StorageError::Serialization(Box::new(bincode::ErrorKind::Custom(e.to_string()))))?;
            let template_bytes = self.encryption.decrypt(&encrypted).await
                .map_err(|e| StorageError::Encryption(e))?;
            
            // Re-encrypt with new key
            let reencrypted = self.encryption.encrypt(&template_bytes).await
                .map_err(|e| StorageError::Encryption(e))?;
            let storage_data = serde_json::to_vec(&reencrypted)
                .map_err(|e| StorageError::Serialization(Box::new(bincode::ErrorKind::Custom(e.to_string()))))?;
            
            batch.insert(key, storage_data);
        }

        // Apply all re-encrypted data
        let db = self.db.write().await;
        db.apply_batch(batch)?;
        db.flush()?;
        drop(db);

        // Finish key rotation
        self.encryption.finish_rotation().await
            .map_err(|e| StorageError::Encryption(e))?;
        
        Ok(())
    }

    /// Flush all pending writes to disk
    pub async fn flush(&self) -> Result<()> {
        let db = self.db.write().await;
        db.flush()?;
        let _ = db.flush_async(); // No need to await this
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{TemplateMetadata, TemplateType};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_template_storage() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let vault = TemplateVault::new(temp_dir.path()).await?;

        // Create test template
        let template = Template::new(
            vec![1, 2, 3, 4],
            TemplateMetadata {
                version: "1.0".to_string(),
                template_type: TemplateType::Face,
                quality_score: 0.95,
                extra: serde_json::json!({}),
            },
        );

        // Store and retrieve
        let id = vault.store(template.clone()).await?;
        let retrieved = vault.get(id).await?;
        assert_eq!(retrieved.data, template.data);

        // Ensure data is flushed
        vault.flush().await?;

        Ok(())
    }
}
