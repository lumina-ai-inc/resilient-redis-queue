use actix_web::{web, HttpResponse, Responder};
use models::rrq::{produce::ProducePayload, queue::QueuePayload};
use chrono::Utc;
use std::collections::HashMap;
use crate::utils::{namespace::MAIN_NAMESPACE, redis::multi_push};
use crate::AppState;

pub async fn produce_data(
    data: web::Json<Vec<ProducePayload>>,
    app_state: web::Data<AppState>
) -> impl Responder {

    if data.len() > 120 {
        return HttpResponse::BadRequest().json("Item count cannot be greater than 120");
    }

    let mut conn = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                format!("Failed to get connection from pool: {}", e)
            );
        }
    };

    let mut queue_payloads: HashMap<String, Vec<String>> = HashMap::new();

    for produce_payload in data.iter() {
        let queue_name = format!("{}:{}", MAIN_NAMESPACE, produce_payload.queue_name);
        
        let queue_payload = QueuePayload {
            queue_name: queue_name.clone(),
            publish_channel: produce_payload.publish_channel.clone(),
            attempt: 1,
            max_attempts: produce_payload.max_attempts.unwrap_or(3),
            payload: produce_payload.payload.clone(),
            created_at: Utc::now(),
            item_id: produce_payload.item_id.clone()
        };
    
        let payload_string = serde_json::to_string(&queue_payload)
            .expect("Failed to serialize QueuePayload");
    
        queue_payloads.entry(queue_name).or_default().push(payload_string);
    }
    
    match multi_push(&mut conn, &queue_payloads).await {
        Ok(_) => HttpResponse::Ok().json("Data produced successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to publish items: {}", e)),
    }
}