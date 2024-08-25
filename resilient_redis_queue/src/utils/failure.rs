use configs::deadpool_redis::Connection;
use models::{ status::StatusPayload, publish::PublishPayload, queue::QueuePayload };
use utils::errors::ProcessError;
use utils::redis::{get_item_by_index, overwrite_item_by_index, move_item_by_index, publish_to_channel};
use utils::namespace::{PRIVATE_NAMESPACE, DLQ_NAMESPACE};

pub async fn process_failed_item(
    mut conn: &mut Connection,
    item: &StatusPayload,
) -> Result<(), ProcessError> {
    let consumer_queue_name = format!(
        "{}:{}:{}",
        PRIVATE_NAMESPACE,
        item.queue_name,
        item.consumer_id
    );

    let dlq_queue_name = format!("{}:{}:{}", DLQ_NAMESPACE, item.queue_name, item.consumer_id);

    let item_str = get_item_by_index(&mut conn, &consumer_queue_name, item.item_index).await?
        .ok_or_else(|| ProcessError::ItemNotFound { 
            item_id: item.item_id.clone(), 
            index: item.item_index 
        })?;

    let mut queue_item: QueuePayload = serde_json::from_str(&item_str)?;

    if queue_item.attempt < queue_item.max_attempts {
        queue_item.attempt += 1;
        let updated_item = serde_json::to_string(&queue_item)?;
        overwrite_item_by_index(&mut conn, &consumer_queue_name, item.item_index, &updated_item).await?;
    } else {
        let moved_item = move_item_by_index(&mut conn, &consumer_queue_name, &dlq_queue_name, item.item_index).await?
            .ok_or_else(|| ProcessError::ProcessingError { 
                item_id: item.item_id.clone(), 
                message: "Failed to move item to the dead letter queue".to_string() 
            })?;

        let queue_payload: QueuePayload = serde_json::from_str(&moved_item)?;

        if let Some(publish_channel) = queue_payload.publish_channel {
            let publish_payload = PublishPayload {
                item_id: item.item_id.clone(),
                consumer_id: item.consumer_id.clone(),
                queue_name: item.queue_name.clone(),
                payload: queue_payload.payload,
                success: false,
                message: item.message.clone(),
            };

            let payload_json = serde_json::to_string(&publish_payload)?;
            publish_to_channel(&mut conn, &publish_channel, &payload_json).await?;
        }
    }

    Ok(())
}