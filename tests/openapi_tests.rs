use actix_web::test;
use crate::api::openapi::ApiDoc;

#[actix_web::test]
async fn test_openapi_docs() {
    let app = test::init_service(
        actix_web::App::new()
            .configure(crate::api::config)
    ).await;

    // Test OpenAPI JSON endpoint
    let req = test::TestRequest::get()
        .uri("/api-docs/openapi.json")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test Swagger UI endpoint
    let req = test::TestRequest::get()
        .uri("/swagger-ui/")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[test]
fn test_openapi_schema() {
    let doc = ApiDoc::openapi();
    assert!(!doc.paths.paths.is_empty());
    assert!(!doc.components.unwrap().schemas.is_empty());
    assert!(!doc.tags.unwrap().is_empty());
}
