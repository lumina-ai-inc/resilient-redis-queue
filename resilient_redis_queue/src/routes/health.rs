use actix_web::HttpResponse;
use std::env;
use configs::deadpool_redis::config;

pub async fn health_check() -> HttpResponse {
    let cfg = Config::from_env().unwrap();
    let version = cfg.version.unwrap_or_else(|_| "unknown".to_string());
    let message = format!("OK - Version {}", version);
    println!("{}", message);
    HttpResponse::Ok().body(message)
}