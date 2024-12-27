use actix_web::{web, HttpResponse};
use crate::models::Task;
use uuid::Uuid;

pub async fn create_task(task: web::Json<Task>) -> HttpResponse {
    // TODO: Implement task creation
    HttpResponse::Ok().finish()
}

pub async fn get_task(task_id: web::Path<Uuid>) -> HttpResponse {
    // TODO: Implement task retrieval
    HttpResponse::Ok().finish()
}

pub async fn update_task(
    task_id: web::Path<Uuid>,
    task: web::Json<Task>,
) -> HttpResponse {
    // TODO: Implement task update
    HttpResponse::Ok().finish()
}

pub async fn delete_task(task_id: web::Path<Uuid>) -> HttpResponse {
    // TODO: Implement task deletion
    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tasks")
            .route(web::post().to(create_task))
    )
    .service(
        web::resource("/tasks/{id}")
            .route(web::get().to(get_task))
            .route(web::put().to(update_task))
            .route(web::delete().to(delete_task))
    );
}
