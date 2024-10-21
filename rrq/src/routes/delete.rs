use crate::utils::redis::delete;
use crate::AppState;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn delete_queue(app_state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let queue_name = path.into_inner();
    let mut conn = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": format!("Failed to get connection from pool: {}", e)}));
        }
    };

    let result = match delete(&mut conn, &queue_name).await {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": format!("Failed to delete queue: {}", e)}));
        }
    };

    HttpResponse::Ok().json(result)
}
