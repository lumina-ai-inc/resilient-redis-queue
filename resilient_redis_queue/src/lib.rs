use actix_web::{ web, App, HttpServer };
use actix_web::rt::time;
use std::time::Duration;
use std::sync::Arc;

pub mod routes;
pub mod utils;
pub mod services;
pub mod models;

use crate::utils::config_redis::{create_pool, Pool};

pub struct AppState {
    redis_pool: Arc<Pool>,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    
    println!("Starting...");

    let redis_pool = Arc::new(create_pool());
    let cleanup_pool = redis_pool.clone();

    actix_web::rt::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let _ = services::cleanup::cleanup_orphaned_queues(cleanup_pool.clone()).await;
        }
    });

    HttpServer::new(move || {
        let redis_pool = redis_pool.clone();

        App::new()
            .app_data(web::Data::new(AppState { redis_pool }))
            .app_data(web::JsonConfig::default().limit(25_165_824)) 
            .route("/", web::get().to(routes::health::health_check))
            .route("/information", web::get().to(routes::information::get_information))
            .route("/produce", web::post().to(routes::produce::produce_data))
            .route("/consume", web::post().to(routes::consume::consume_data))
            .route("/complete", web::post().to(routes::complete::complete_data))
    })
        .bind("0.0.0.0:8000")?
        .run().await
}