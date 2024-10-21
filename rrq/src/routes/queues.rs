use crate::utils::redis;
use crate::AppState;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn list_queues(app_state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let search_str = path.into_inner();
    println!("Search string: {}", search_str);
    let mut conn = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": format!("Failed to get connection from pool: {}", e)}));
        }
    };

    let queue_keys = match redis::keys(&mut conn, &search_str).await {
        Ok(keys) => keys,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": format!("Failed to fetch queue keys: {}", e)}));
        }
    };

    let mut queue_info = Vec::new();
    for key in queue_keys {
        let queue_name = key.to_string();
        match redis::llen(&mut conn, &key).await {
            Ok(length) => {
                queue_info.push(json!({
                    "name": queue_name,
                    "length": length
                }));
            }
            Err(e) => {
                eprintln!("Failed to get length for queue {}: {}", queue_name, e);
            }
        }
    }

    HttpResponse::Ok().json(queue_info)
}
