use crate::utils::config_redis::Connection;
use crate::models::{ status::StatusPayload, publish::PublishPayload, queue::QueuePayload };
use crate::utils::errors::ProcessError;
use crate::utils::redis::{remove_item_by_index, publish_to_channel};
use crate::utils::namespace::PRIVATE_NAMESPACE;

pub async fn process_successful_item(
    conn: &mut Connection,
    item: &StatusPayload,
) -> Result<(), ProcessError> {
    let consumer_queue_name = format!(
        "{}:{}:{}",
        PRIVATE_NAMESPACE,
        item.queue_name,
        item.consumer_id
    );

    let removed_item = remove_item_by_index(conn, &consumer_queue_name, item.item_index).await?
        .ok_or_else(|| ProcessError::ItemNotFound { 
            item_id: item.item_id.clone(), 
            index: item.item_index 
        })?;

    let queue_payload: QueuePayload = serde_json::from_str(&removed_item)?;

    if let Some(publish_channel) = queue_payload.publish_channel {
        let publish_payload = PublishPayload {
            item_id: item.item_id.clone(),
            consumer_id: item.consumer_id.clone(),
            queue_name: item.queue_name.clone(),
            payload: queue_payload.payload,
            success: true,
            message: item.message.clone(),
        };

        let payload_json = serde_json::to_string(&publish_payload)?;
        publish_to_channel(conn, &publish_channel, &payload_json).await?;
    }

    Ok(())
}