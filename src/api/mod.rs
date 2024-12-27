mod auth;
mod projects;
mod tasks;
mod users;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::config)
            .configure(projects::config)
            .configure(tasks::config)
            .configure(users::config)
    );
}
