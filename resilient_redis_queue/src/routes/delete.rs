use actix_web::{ web, HttpResponse, Responder };
use crate::AppState;
use models::rrq::{status::StatusPayload, queue::QueuePayload, publish::PublishPayload};
use crate::utils::redis::{ get_item_by_id, update_item_by_id, move_item_by_id, publish_to_channel };
use crate::utils::namespace::{ PRIVATE_NAMESPACE, DLQ_NAMESPACE };
use redis::AsyncCommands;

pub async fn delete_all_dlq(
    app_state: web::Data<AppState>
) -> impl Responder {
    let mut conn = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                format!("Failed to get connection from pool: {}", e)
            );
        }
    };

    let pattern = format!("{}:*", DLQ_NAMESPACE);
    let mut cursor: u64 = 0;
    loop {
        let (next_cursor, keys): (u64, Vec<String>) = match conn.scan_match(&pattern).await {
            Ok(result) => result,
            Err(e) => {
                return HttpResponse::InternalServerError().json(
                    format!("Failed to scan keys: {}", e)
                );
            }
        };

        if !keys.is_empty() {
            let _: () = match conn.del(keys).await {
                Ok(_) => (),
                Err(e) => {
                    return HttpResponse::InternalServerError().json(
                        format!("Failed to delete keys: {}", e)
                    );
                }
            };
        }

        cursor = next_cursor;
        if cursor == 0 {
            break;
        }
    }

    HttpResponse::Ok().json("All dlq:* queues deleted")
}