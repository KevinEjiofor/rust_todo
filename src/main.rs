use actix_web::{App, HttpServer, web, middleware};
use dotenv::dotenv;
use std::env;

mod config;
mod db;
pub mod controll;
mod domain;
mod data;
mod middleware;
mod utils;
// 
// use data::dummy_user_repo::DummyUserRepo;
use domain::services::auth_service_impl::AuthServiceImpl;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // let user_repo = DummyUserRepo::new();
    let auth_service = AuthServiceImpl::new(user_repo);
    
    let auth_service_data = web::Data::new(tokio::sync::Mutex::new(auth_service));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(auth_service_data.clone())
            .configure(controller::routes::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
