use actix_web::{web, HttpResponse};
use serde_json::json;
use std::collections::HashMap;
use crate::AppState;
use crate::utils::redis::info;

fn parse_redis_info(info: &str) -> HashMap<String, HashMap<String, String>> {
    let mut result = HashMap::new();
    let mut current_section = String::new();

    for line in info.lines() {
        if line.starts_with('#') {
            current_section = line.trim_start_matches('#').trim().to_lowercase();
            result.insert(current_section.clone(), HashMap::new());
        } else if !line.is_empty() {
            if let Some((key, value)) = line.split_once(':') {
                if let Some(section) = result.get_mut(&current_section) {
                    section.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    result
}

pub async fn get_information(app_state: web::Data<AppState>) -> HttpResponse {
    println!("Getting information");

    let mut conn = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                json!({"error": format!("Failed to get connection from pool: {}", e)})
            );
        }
    };

    match info(&mut conn).await {
        Ok(info) => {
            let parsed_info = parse_redis_info(&info);
            HttpResponse::Ok().json(parsed_info)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(
                json!({"error": format!("Failed to get Redis info: {}", e)})
            )
        }
    }
}