use std::sync::Arc;
use crate::utils::namespace::{ MAIN_NAMESPACE, PRIVATE_NAMESPACE, REGISTRY_NAMESPACE };
use crate::utils::redis::{ lmove, keys, hexists };
use crate::utils::config_redis::{ Pool, RedisError };

pub async fn cleanup_orphaned_queues(pool: Arc<Pool>) -> Result<(), RedisError> {
    let mut conn = pool.get().await.expect("Failed to get connection from pool");

    // Pattern to match all consumer queues
    let consumer_queue_pattern = format!("{}:*:*", PRIVATE_NAMESPACE);

    // Get all consumer queues
    let consumer_queues: Vec<String> = keys(&mut conn, &consumer_queue_pattern).await?;

    for consumer_queue in consumer_queues {
        let parts: Vec<&str> = consumer_queue.split(':').collect();
        if parts.len() != 3 {
            println!("Skipping queue with unexpected format: {}", consumer_queue);
            continue; // Skip if the queue name doesn't match the expected format
        }

        let queue_name = parts[1];
        let consumer_id = parts[2];
        let main_queue_name = format!("{}:{}", MAIN_NAMESPACE, queue_name);
        let registry_key = format!("{}:{}:{}", REGISTRY_NAMESPACE, queue_name, consumer_id);

        // Check if the consumer is registered
        let is_registered: bool = hexists(&mut conn, &registry_key, "consumer_id").await?;

        if !is_registered {
            println!("Consumer is not registered, moving items back to main queue.");
            // Move all items from consumer queue to main queue
            let mut moved_count = 0;
            while let Ok(Some(_)) = lmove(
                &mut conn,
                &consumer_queue,
                &main_queue_name,
                "RIGHT",
                "LEFT"
            ).await {
                moved_count += 1;
            }
            println!("Moved {} items from {} to {}", moved_count, consumer_queue, main_queue_name);
        }
    }

    println!("Cleaned up orphaned queues");

    Ok(())
}