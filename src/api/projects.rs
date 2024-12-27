use actix_web::{web, HttpResponse};
use crate::models::Project;
use uuid::Uuid;

pub async fn create_project(project: web::Json<Project>) -> HttpResponse {
    // TODO: Implement project creation
    HttpResponse::Ok().finish()
}

pub async fn get_project(project_id: web::Path<Uuid>) -> HttpResponse {
    // TODO: Implement project retrieval
    HttpResponse::Ok().finish()
}

pub async fn update_project(
    project_id: web::Path<Uuid>,
    project: web::Json<Project>,
) -> HttpResponse {
    // TODO: Implement project update
    HttpResponse::Ok().finish()
}

pub async fn delete_project(project_id: web::Path<Uuid>) -> HttpResponse {
    // TODO: Implement project deletion
    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/projects")
            .route(web::post().to(create_project))
    )
    .service(
        web::resource("/projects/{id}")
            .route(web::get().to(get_project))
            .route(web::put().to(update_project))
            .route(web::delete().to(delete_project))
    );
}
