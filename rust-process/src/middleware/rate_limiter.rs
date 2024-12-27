use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures::future::{ready, Ready};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

pub struct RateLimiter {
    max_requests: u32,
    time_window: u64, // in seconds
    store: Arc<Mutex<HashMap<String, (u32, SystemTime)>>>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, time_window: u64) -> Self {
        Self {
            max_requests,
            time_window,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            max_requests: self.max_requests,
            time_window: self.time_window,
            store: self.store.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    max_requests: u32,
    time_window: u64,
    store: Arc<Mutex<HashMap<String, (u32, SystemTime)>>>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip = req.connection_info().peer_addr().unwrap_or("unknown").to_string();
        
        let mut store = self.store.lock().unwrap();
        let now = SystemTime::now();
        
        if let Some((count, timestamp)) = store.get_mut(&ip) {
            if now.duration_since(*timestamp).unwrap() < Duration::from_secs(self.time_window) {
                if *count >= self.max_requests {
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorTooManyRequests(RateLimitError::RateLimitExceeded))
                    });
                }
                *count += 1;
            } else {
                *count = 1;
                *timestamp = now;
            }
        } else {
            store.insert(ip.clone(), (1, now));
        }

        self.service.call(req)
    }
}
