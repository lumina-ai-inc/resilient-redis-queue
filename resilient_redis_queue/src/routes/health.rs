use actix_web::HttpResponse;
use std::env;

pub async fn health_check() -> HttpResponse {
    let version = env::var("APP_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let message = format!("OK - Version {}", version);
    println!("{}", message);
    HttpResponse::Ok().body(message)
}