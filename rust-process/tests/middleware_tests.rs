use actix_web::test::TestRequest;
use actix_web::{web, App, HttpResponse};
use crate::middleware::{rate_limiter::RateLimiter, logger::RequestLogger};
use std::sync::Arc;

#[tokio::test]
async fn test_rate_limiter() {
    let rate_limiter = RateLimiter::new(2, 60); // 2 requests per minute
    
    let app = App::new()
        .wrap(rate_limiter)
        .route("/", web::get().to(HttpResponse::Ok));
    
    let mut app = actix_web::test::init_service(app).await;
    
    // First request should succeed
    let req = TestRequest::get().uri("/").to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    
    // Second request should succeed
    let req = TestRequest::get().uri("/").to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    
    // Third request should fail
    let req = TestRequest::get().uri("/").to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 429);
}

#[tokio::test]
async fn test_logger() {
    let logger = RequestLogger::new();
    
    let app = App::new()
        .wrap(logger)
        .route("/", web::get().to(HttpResponse::Ok));
    
    let mut app = actix_web::test::init_service(app).await;
    
    let req = TestRequest::get().uri("/").to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
}
