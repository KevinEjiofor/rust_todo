pub mod auth_handler;
pub mod todo_handler;

use actix_web::{web, App};
use crate::middleware_layer::auth_middleware::AuthMiddleware;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(auth_handler::register::<crate::domain::services::auth_service_impl::AuthServiceImpl<crate::data::dummy_user_repo::DummyUserRepo>>))
            .route("/login", web::post().to(auth_handler::login::<crate::domain::services::auth_service_impl::AuthServiceImpl<crate::data::dummy_user_repo::DummyUserRepo>>))
    )
        .service(
            // Protect the /todos endpoints with JWT middleware
            web::scope("/todos")
                .wrap(AuthMiddleware)
                .route("", web::get().to(crate::presentation::todo_handler::get_todos))
            // Add additional routes (POST, PUT, DELETE) here
        );
}
