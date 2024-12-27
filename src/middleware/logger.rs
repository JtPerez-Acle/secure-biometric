use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ready, Ready};
use std::time::Instant;
use log::{info, error};

pub struct RequestLogger;

impl RequestLogger {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware {
            service,
        }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
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
        let start = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let result = fut.await;
            let duration = start.elapsed();
            
            match &result {
                Ok(res) => {
                    info!(
                        "{} {} {} {}ms",
                        method,
                        path,
                        res.status().as_u16(),
                        duration.as_millis()
                    );
                }
                Err(err) => {
                    error!(
                        "{} {} {} {}ms",
                        method,
                        path,
                        err.as_response_error().status_code().as_u16(),
                        duration.as_millis()
                    );
                }
            }
            
            result
        })
    }
}
