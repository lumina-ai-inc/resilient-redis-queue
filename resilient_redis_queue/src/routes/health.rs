use actix_web::HttpResponse;
use crate::utils::config_redis::Config;

pub async fn health_check() -> HttpResponse {
    let cfg = Config::from_env().unwrap();
    let version = cfg.version;
    let message = format!("OK - Version {}", version);
    HttpResponse::Ok().body(message)
}