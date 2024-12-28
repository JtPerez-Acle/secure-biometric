use actix_web::{test, web, App};
use prometheus::Encoder;
use crate::api::health::{HealthState, health_check, metrics};

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_metrics() {
    let state = web::Data::new(std::sync::Mutex::new(HealthState::new()));
    
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(metrics)
    ).await;

    let req = test::TestRequest::get()
        .uri("/metrics")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("uptime_seconds"));
}
