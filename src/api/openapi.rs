use utoipa::OpenApi;
use crate::models::{User, Task, Project, Session, ApiKey};
use crate::api::{auth, projects, tasks, users, health};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login,
        auth::register,
        projects::create_project,
        projects::get_project,
        projects::update_project,
        projects::delete_project,
        tasks::create_task,
        tasks::get_task,
        tasks::update_task,
        tasks::delete_task,
        users::get_user,
        users::update_user,
        health::health_check,
        health::metrics
    ),
    components(
        schemas(User, Task, Project, Session, ApiKey)
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
