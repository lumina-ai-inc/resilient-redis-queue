use actix_web::{ web, App, HttpServer };
use actix_web::rt::time;
use actix_web::middleware::Logger;
use env_logger::Env;
use std::time::Duration;
use std::sync::Arc;

pub mod routes;
pub mod utils;
pub mod services;
pub mod models;

use crate::utils::config_redis::{create_pool, Pool};
use crate::utils::redis::ping;

pub struct AppState {
    redis_pool: Arc<Pool>,
}

async fn test_redis_connection(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get().await?;
    ping(&mut conn).await?;
    Ok(())
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    
    let redis_pool = Arc::new(create_pool());

    // Test Redis connection before starting the server
    if let Err(e) = test_redis_connection(&redis_pool).await {
        println!("Failed to connect to Redis: {}. Exiting...", e);
        std::process::exit(1);
    }

    let cleanup_pool = redis_pool.clone();

    actix_web::rt::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let _ = services::cleanup::cleanup_orphaned_queues(cleanup_pool.clone()).await;
        }
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let redis_pool = redis_pool.clone();

        App::new()
            .app_data(web::Data::new(AppState { redis_pool }))
            .app_data(web::JsonConfig::default().limit(25_165_824)) 
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(routes::health::health_check))
            .route("/information", web::get().to(routes::information::get_information))
            .route("/produce", web::post().to(routes::produce::produce_data))
            .route("/consume", web::post().to(routes::consume::consume_data))
            .route("/complete", web::post().to(routes::complete::complete_data))
    })
        .bind("0.0.0.0:8000")?
        .run().await
}