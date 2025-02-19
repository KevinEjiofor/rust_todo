use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::domain::services::auth_service::AuthService;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn register<S: AuthService + 'static>(
    auth_service: web::Data<tokio::sync::Mutex<S>>,
    info: web::Json<RegisterRequest>
) -> impl Responder {
    let mut service = auth_service.lock().await;
    match service.register(&info.username, &info.password).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn login<S: AuthService + 'static>(
    auth_service: web::Data<S>,
    info: web::Json<LoginRequest>
) -> impl Responder {
    match auth_service.login(&info.username, &info.password).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
