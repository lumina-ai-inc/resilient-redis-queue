use actix_web::{ web, HttpResponse, Responder };
use models::rrq::{consume::{ConsumePayload, ConsumeResponse}, queue::QueuePayload};
use crate::utils::namespace::{ MAIN_NAMESPACE, PRIVATE_NAMESPACE, REGISTRY_NAMESPACE };
use crate::AppState;
use crate::utils::redis::{ blmove, lrange, llen, register_in_hash };

pub async fn consume_data(
    data: web::Json<ConsumePayload>,
    app_state: web::Data<AppState>
) -> impl Responder {

    println!("Consuming {:?} items", data.item_count);

    let mut conn = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                format!("Failed to get connection from pool: {}", e)
            );
        }
    };

    let main_queue_name = format!("{}:{}", MAIN_NAMESPACE, data.queue_name);
    let consumer_queue_name = format!(
        "{}:{}:{}",
        PRIVATE_NAMESPACE,
        data.queue_name,
        data.consumer_id
    );

    let registry_key = format!("{}:{}:{}", REGISTRY_NAMESPACE, data.queue_name, data.consumer_id);

    // Register and reset expiration of consumer
    let _ = register_in_hash(
        &mut conn,
        &registry_key,
        "consumer_id",
        &data.consumer_id,
        Some(data.expiration_seconds.unwrap_or(600)),
        false
    ).await;

    let consumer_queue_length: i64 = match llen(&mut conn, &consumer_queue_name).await {
        Ok(length) => length,
        Err(e) => {
            println!("Failed to get consumer queue length: {}", e);
            return HttpResponse::InternalServerError().json(
                format!("Failed to get consumer queue length: {}", e)
            );
        }
    };

    let main_queue_length: i64 = match llen(&mut conn, &main_queue_name).await {
        Ok(length) => length,
        Err(e) => {
            println!("Failed to get consumer queue length: {}", e);
            return HttpResponse::InternalServerError().json(
                format!("Failed to get consumer queue length: {}", e)
            );
        }
    };

    let items_to_move = if consumer_queue_length < data.item_count {
        (data.item_count - consumer_queue_length).min(main_queue_length)
    } else {
        0
    };

    // Move items from main queue to consumer queue
    let mut moved_items = Vec::new();
    for _ in 0..items_to_move {
        match blmove(&mut conn, &main_queue_name, &consumer_queue_name, 5.0).await {
            Ok(Some(item)) => {
                moved_items.push(item);
            }
            Ok(None) => {
                break;
            }
            Err(e) => {
                println!("Failed to move item: {}", e);
                return HttpResponse::InternalServerError().json(
                    format!("Failed to move item: {}", e)
                );
            }
        }
    }

    let mut items: Vec<ConsumeResponse> = Vec::new();
    let range_end = (data.item_count - 1) as isize;
    
    match lrange(&mut conn, &consumer_queue_name, 0, range_end).await {
        Ok(fetched_items) => {
            for (i, item) in fetched_items.into_iter().enumerate() {
                let queue_item: QueuePayload = match serde_json::from_str(&item) {
                    Ok(payload) => payload,
                    Err(e) => {
                        return HttpResponse::BadRequest().json(format!("Failed to deserialize item: {}", e));
                    }
                };
                let consume_response = ConsumeResponse {
                    queue_item,
                    item_index: i as i64,
                    consumed_at: chrono::Utc::now(),
                };
                items.push(consume_response);
            }
        }
        Err(e) => {
            println!("Failed to get items from consumer queue: {}", e);
            return HttpResponse::InternalServerError().json(
                format!("Failed to get items from consumer queue: {}", e)
            );
        }
    }

    HttpResponse::Ok().json(items)
}
