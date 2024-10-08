use actix_web::{ web, HttpResponse, Responder };
use crate::AppState;
use crate::models::status::{ StatusPayload, StatusResult::{ Success, Failure } };
use crate::utils::{ success::process_successful_item, failure::process_failed_item };
use crate::utils::config_redis::Connection;

pub async fn complete_data(
    data: web::Json<Vec<StatusPayload>>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let mut conn: Connection = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                format!("Failed to get connection from pool: {}", e)
            );
        }
    };

    let mut sorted_data = data.into_inner();
    sorted_data.sort_by(|a, b| b.item_index.cmp(&a.item_index));

    for item in sorted_data {
        let result = match item.result {
            Success => process_successful_item(&mut conn, &item).await,
            Failure => process_failed_item(&mut conn, &item).await,
        };

        if let Err(e) = result {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    } 

    HttpResponse::Ok().json("Items successfully consumed")
}
