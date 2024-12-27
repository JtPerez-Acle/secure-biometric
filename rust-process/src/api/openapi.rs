use utoipa::OpenApi;
use crate::models::{User, Task, Project, Session, ApiKey};
use crate::api::{auth, projects, tasks, users, health};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::auth::login,
        crate::api::auth::register,
        crate::api::projects::create_project,
        crate::api::projects::get_project,
        crate::api::projects::update_project,
        crate::api::projects::delete_project,
        crate::api::tasks::create_task,
        crate::api::tasks::get_task,
        crate::api::tasks::update_task,
        crate::api::tasks::delete_task,
        crate::api::users::get_user,
        crate::api::users::update_user,
        crate::api::health::health_check,
        crate::api::health::metrics
    ),
    components(
        schemas(crate::models::User, crate::models::Task, crate::models::Project, crate::models::Session, crate::models::ApiKey)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "projects", description = "Project management endpoints"),
        (name = "tasks", description = "Task management endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "health", description = "Health check and monitoring endpoints")
    )
)]
pub struct ApiDoc;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
