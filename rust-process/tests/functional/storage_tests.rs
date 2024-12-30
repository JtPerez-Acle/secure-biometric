use crate::common::TestContext;
use secure_biometric::storage::TemplateVault;
use secure_biometric::templates::{Template, TemplateMetadata, TemplateType};

#[tokio::test]
async fn test_template_storage_basic() {
    let ctx = TestContext::new();
    let vault = TemplateVault::new(ctx.temp_path())
        .await
        .expect("Failed to create vault");

    // Create test template
    let template = Template::new(
        ctx.create_test_template(),
        TemplateMetadata {
            version: "1.0".to_string(),
            template_type: TemplateType::Face,
            quality_score: 0.95,
            extra: serde_json::json!({}),
        },
    );

    // Store template
    let id = vault
        .store(template.clone())
        .await
        .expect("Failed to store template");

    // Retrieve template
    let retrieved = vault.get(id).await.expect("Failed to retrieve template");

    assert_eq!(retrieved.data, template.data);
    assert_eq!(retrieved.metadata.template_type, TemplateType::Face);
    assert_eq!(retrieved.metadata.quality_score, 0.95);
}

#[tokio::test]
async fn test_template_not_found() {
    let ctx = TestContext::new();
    let vault = TemplateVault::new(ctx.temp_path())
        .await
        .expect("Failed to create vault");

    let result = vault.get(uuid::Uuid::new_v4()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_template_deletion() {
    let ctx = TestContext::new();
    let vault = TemplateVault::new(ctx.temp_path())
        .await
        .expect("Failed to create vault");

    // Create and store template
    let template = Template::new(
        ctx.create_test_template(),
        TemplateMetadata {
            version: "1.0".to_string(),
            template_type: TemplateType::Face,
            quality_score: 0.95,
            extra: serde_json::json!({}),
        },
    );

    let id = vault
        .store(template)
        .await
        .expect("Failed to store template");

    // Delete template
    vault.delete(id).await.expect("Failed to delete template");

    // Verify template is gone
    let result = vault.get(id).await;
    assert!(result.is_err());
}
