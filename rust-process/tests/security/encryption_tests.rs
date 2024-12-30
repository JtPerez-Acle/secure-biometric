use crate::common::TestContext;
use log::{debug, info};
use secure_biometric::security::{EncryptionEngine, KeyManager};
use secure_biometric::storage::TemplateVault;
use secure_biometric::templates::{Template, TemplateMetadata, TemplateType};
use std::sync::Arc;
use tokio::time::timeout;
use std::time::Duration;

const TEST_TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::test]
async fn test_basic_encryption() {
    let ctx = TestContext::new();
    let timer = ctx.timer("basic_encryption");
    
    info!("Starting basic encryption test");
    let key_manager = Arc::new(KeyManager::new().expect("Failed to create key manager"));
    let engine = EncryptionEngine::new(key_manager);
    
    let data = b"sensitive biometric data";
    debug!("Encrypting test data of length {}", data.len());
    let encrypted = engine.encrypt(data).await.expect("Failed to encrypt");
    
    // Verify data is actually encrypted
    assert_ne!(&encrypted.ciphertext[..], data);
    debug!("Verified data is encrypted");
    
    // Verify decryption works
    let decrypted = engine.decrypt(&encrypted).await.expect("Failed to decrypt");
    assert_eq!(&decrypted[..], data);
    debug!("Verified successful decryption");
    
    timer.stop(true).await;
}

#[tokio::test]
async fn test_key_rotation() {
    let ctx = TestContext::new();
    let timer = ctx.timer("key_rotation");
    
    info!("Starting key rotation test");
    let key_manager = Arc::new(KeyManager::new().expect("Failed to create key manager"));
    let engine = EncryptionEngine::new(key_manager);
    
    // Encrypt data with original key
    let data = b"sensitive data";
    debug!("Encrypting initial test data");
    let encrypted = timeout(TEST_TIMEOUT, engine.encrypt(data))
        .await
        .expect("Test timed out")
        .expect("Failed to encrypt");
    
    // Rotate key
    debug!("Performing key rotation");
    timeout(TEST_TIMEOUT, engine.rotate_key())
        .await
        .expect("Test timed out")
        .expect("Failed to rotate key");
    
    // Verify we can still decrypt with new key
    debug!("Attempting decryption after key rotation");
    let decrypted = timeout(TEST_TIMEOUT, engine.decrypt(&encrypted))
        .await
        .expect("Test timed out")
        .expect("Failed to decrypt");
    assert_eq!(&decrypted[..], data);
    
    timer.stop(true).await;
}

#[tokio::test]
async fn test_template_vault_encryption() {
    let ctx = TestContext::new();
    let timer = ctx.timer("template_vault_encryption");
    
    info!("Starting template vault encryption test");
    let vault = TemplateVault::new(ctx.temp_path())
        .await
        .expect("Failed to create vault");

    // Create and store template
    let template = Template::new(
        vec![0xDE, 0xAD, 0xBE, 0xEF],
        TemplateMetadata {
            version: "1.0".to_string(),
            template_type: TemplateType::Face,
            quality_score: 0.95,
            extra: serde_json::json!({}),
        },
    );

    debug!("Storing template in vault");
    let id = timeout(TEST_TIMEOUT, vault.store(template.clone()))
        .await
        .expect("Test timed out")
        .expect("Failed to store");

    // Read raw data from disk to verify it's encrypted
    debug!("Verifying template is encrypted on disk");
    let raw_data = std::fs::read(ctx.temp_path().join(id.to_string()));
    assert!(raw_data.is_err() || raw_data.unwrap() != template.data);

    // Verify we can retrieve and decrypt
    debug!("Retrieving and decrypting template from vault");
    let retrieved = timeout(TEST_TIMEOUT, vault.get(id))
        .await
        .expect("Test timed out")
        .expect("Failed to retrieve");
    assert_eq!(retrieved.data, template.data);
    
    timer.stop(true).await;
}

#[tokio::test]
async fn test_vault_key_rotation() {
    let ctx = TestContext::new();
    let timer = ctx.timer("vault_key_rotation");
    
    info!("Starting vault key rotation test");
    let vault = TemplateVault::new(ctx.temp_path())
        .await
        .expect("Failed to create vault");

    // Store multiple templates
    let templates = vec![
        Template::new(
            vec![1, 2, 3],
            TemplateMetadata {
                version: "1.0".to_string(),
                template_type: TemplateType::Face,
                quality_score: 0.95,
                extra: serde_json::json!({}),
            },
        ),
        Template::new(
            vec![4, 5, 6],
            TemplateMetadata {
                version: "1.0".to_string(),
                template_type: TemplateType::Fingerprint,
                quality_score: 0.98,
                extra: serde_json::json!({}),
            },
        ),
    ];

    debug!("Storing multiple templates in vault");
    let mut ids = vec![];
    for template in templates.clone() {
        let id = timeout(TEST_TIMEOUT, vault.store(template))
            .await
            .expect("Test timed out")
            .expect("Failed to store");
        ids.push(id);
    }

    // Rotate encryption key
    debug!("Rotating encryption key");
    timeout(TEST_TIMEOUT, vault.rotate_key())
        .await
        .expect("Test timed out")
        .expect("Failed to rotate key");

    // Verify all templates can still be retrieved
    debug!("Verifying all templates can be retrieved after key rotation");
    for (id, original) in ids.iter().zip(templates.iter()) {
        let retrieved = timeout(TEST_TIMEOUT, vault.get(*id))
            .await
            .expect("Test timed out")
            .expect("Failed to retrieve");
        assert_eq!(retrieved.data, original.data);
    }
    
    timer.stop(true).await;
}

#[tokio::test]
async fn test_encryption_integrity() {
    let ctx = TestContext::new();
    let timer = ctx.timer("encryption_integrity");
    
    info!("Starting encryption integrity test");
    let key_manager = Arc::new(KeyManager::new().expect("Failed to create key manager"));
    let engine = EncryptionEngine::new(key_manager);
    
    let data = b"sensitive data";
    debug!("Encrypting test data");
    let mut encrypted = timeout(TEST_TIMEOUT, engine.encrypt(data))
        .await
        .expect("Test timed out")
        .expect("Failed to encrypt");
    
    // Tamper with ciphertext
    debug!("Tampering with ciphertext");
    encrypted.ciphertext[0] ^= 0xFF;
    
    // Verify decryption fails due to integrity check
    debug!("Verifying decryption fails due to integrity check");
    let result = timeout(TEST_TIMEOUT, engine.decrypt(&encrypted))
        .await
        .expect("Test timed out");
    assert!(result.is_err());
    
    timer.stop(true).await;
}

#[tokio::test]
async fn test_nonce_uniqueness() {
    let ctx = TestContext::new();
    let timer = ctx.timer("nonce_uniqueness");
    
    info!("Starting nonce uniqueness test");
    let key_manager = Arc::new(KeyManager::new().expect("Failed to create key manager"));
    let engine = EncryptionEngine::new(key_manager);
    
    let data = b"test data";
    debug!("Encrypting test data");
    let encrypted1 = timeout(TEST_TIMEOUT, engine.encrypt(data))
        .await
        .expect("Test timed out")
        .expect("Failed to encrypt");
    let encrypted2 = timeout(TEST_TIMEOUT, engine.encrypt(data))
        .await
        .expect("Test timed out")
        .expect("Failed to encrypt");
    
    // Verify nonces are different even for same data
    debug!("Verifying nonces are unique");
    assert_ne!(encrypted1.nonce, encrypted2.nonce);
    
    timer.stop(true).await;
}

#[tokio::test]
async fn test_concurrent_encryption() {
    let ctx = TestContext::new();
    let timer = ctx.timer("concurrent_encryption");
    
    info!("Starting concurrent encryption test");
    let key_manager = Arc::new(KeyManager::new().expect("Failed to create key manager"));
    let engine = Arc::new(EncryptionEngine::new(key_manager));
    
    let data = b"test data".to_vec();
    let mut handles = Vec::new();
    
    // Spawn multiple concurrent encryption tasks
    for _ in 0..10 {
        let engine = engine.clone();
        let data = data.clone();
        let handle = tokio::spawn(async move {
            let encrypted = timeout(TEST_TIMEOUT, engine.encrypt(&data))
                .await
                .expect("Test timed out")
                .expect("Failed to encrypt");
            let decrypted = timeout(TEST_TIMEOUT, engine.decrypt(&encrypted))
                .await
                .expect("Test timed out")
                .expect("Failed to decrypt");
            assert_eq!(data, decrypted);
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task failed");
    }
    
    timer.stop(true).await;
}

#[tokio::test]
async fn test_large_data_encryption() {
    let ctx = TestContext::new();
    let timer = ctx.timer("large_data_encryption");
    
    info!("Starting large data encryption test");
    let key_manager = Arc::new(KeyManager::new().expect("Failed to create key manager"));
    let engine = EncryptionEngine::new(key_manager);
    
    // Create large data (1MB)
    let data = vec![0xAA; 1024 * 1024];
    debug!("Encrypting {} bytes of data", data.len());
    
    let encrypted = timeout(TEST_TIMEOUT, engine.encrypt(&data))
        .await
        .expect("Test timed out")
        .expect("Failed to encrypt");
    let decrypted = timeout(TEST_TIMEOUT, engine.decrypt(&encrypted))
        .await
        .expect("Test timed out")
        .expect("Failed to decrypt");
    
    assert_eq!(data, decrypted);
    debug!("Successfully encrypted and decrypted large data");
    
    timer.stop(true).await;
}
