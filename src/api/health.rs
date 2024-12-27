use actix_web::{get, HttpResponse, web};
use prometheus::{Encoder, TextEncoder, Registry, Gauge};
use std::sync::Mutex;

pub struct HealthState {
    pub registry: Registry,
    pub uptime: Gauge,
}

impl HealthState {
    pub fn new() -> Self {
        let registry = Registry::new();
        let uptime = Gauge::new("uptime_seconds", "Uptime in seconds").unwrap();
        registry.register(Box::new(uptime.clone())).unwrap();
        
        Self {
            registry,
            uptime,
        }
    }
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/metrics")]
async fn metrics(state: web::Data<Mutex<HealthState>>) -> HttpResponse {
    let state = state.lock().unwrap();
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    
    encoder.encode(&state.registry.gather(), &mut buffer).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(metrics);
}
