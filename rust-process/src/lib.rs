pub mod logging;
pub mod security;
pub mod storage;
pub mod templates;

#[cfg(test)]
mod tests {
    use crate::security::{EncryptionEngine, KeyManager};
    use crate::storage::TemplateVault;
    use crate::templates::{Template, TemplateMetadata, TemplateType};
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio;
    use serde_json;

    #[tokio::test]
    async fn test_full_template_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
        // Create temporary directory for test
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

        // Test template storage and retrieval
        let id = vault.store(template.clone()).await?;
        let retrieved = vault.get(id).await?;
        assert_eq!(retrieved.data, template.data);

        // Test key rotation
        vault.rotate_key().await?;
        
        // Verify template can still be retrieved after rotation
        let retrieved = vault.get(id).await?;
        assert_eq!(retrieved.data, template.data);

        // Test template deletion
        vault.delete(id).await?;
        assert!(vault.get(id).await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_encryption_engine() -> Result<(), Box<dyn std::error::Error>> {
        let key_manager = Arc::new(KeyManager::new()?);
        let engine = EncryptionEngine::new(key_manager);

        // Test encryption and decryption
        let data = b"test data";
        let encrypted = engine.encrypt(data).await?;
        let decrypted = engine.decrypt(&encrypted).await?;
        assert_eq!(&decrypted[..], data);

        // Test key rotation
        engine.rotate_key().await?;
        let decrypted = engine.decrypt(&encrypted).await?;
        assert_eq!(&decrypted[..], data);

        Ok(())
    }
}
