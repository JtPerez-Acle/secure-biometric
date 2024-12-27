use actix_web::{web, HttpResponse};
use crate::models::User;
use uuid::Uuid;

pub async fn get_user(user_id: web::Path<Uuid>) -> HttpResponse {
    // TODO: Implement user retrieval
    HttpResponse::Ok().finish()
}

pub async fn update_user(
    user_id: web::Path<Uuid>,
    user: web::Json<User>,
) -> HttpResponse {
    // TODO: Implement user update
    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}")
            .route(web::get().to(get_user))
            .route(web::put().to(update_user))
    );
}
