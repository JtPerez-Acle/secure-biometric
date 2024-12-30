use crate::common::TestContext;
use actix_web::{test, web, App};
use secure_biometric::storage::TemplateVault;

#[actix_web::test]
async fn test_template_upload() {
    let ctx = TestContext::new();
    let vault = TemplateVault::new(ctx.temp_path())
        .await
        .expect("Failed to create vault");
    let vault = web::Data::new(vault);

    // TODO: Implement API endpoint tests
    let app = test::init_service(
        App::new()
            .app_data(vault.clone())
            // Add API routes here
    )
    .await;

    // TODO: Test template upload endpoint
}

#[actix_web::test]
async fn test_template_retrieval() {
    // TODO: Implement template retrieval API test
}

#[actix_web::test]
async fn test_template_matching() {
    // TODO: Implement template matching API test
}

#[actix_web::test]
async fn test_error_handling() {
    // TODO: Implement API error handling test
}
