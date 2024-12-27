mod auth;
mod projects;
mod tasks;
mod users;
mod health;

use actix_web::web;
use std::sync::Mutex;
use crate::api::health::HealthState;

pub fn config(cfg: &mut web::ServiceConfig) {
    let health_state = Mutex::new(HealthState::new());
    
    cfg.app_data(web::Data::new(health_state))
        .service(
            web::scope("/api")
                .configure(auth::config)
                .configure(projects::config)
                .configure(tasks::config)
                .configure(users::config)
        )
        .configure(health::config);
}
