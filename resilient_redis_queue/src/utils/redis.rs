use std::collections::HashMap;
use serde::Serialize;
use crate::utils::config_redis::{ cmd, RedisResult, Pipeline, Connection };


// Redis Info Operations
// ---------------------

pub async fn info(conn: &mut Connection) -> RedisResult<String> {
    cmd("INFO").query_async(conn).await
}


// Basic Redis Operations
// ----------------------

pub async fn keys(conn: &mut Connection, pattern: &str) -> RedisResult<Vec<String>> {
    cmd("KEYS").arg(pattern).query_async(conn).await
}

pub async fn hexists(conn: &mut Connection, key: &str, field: &str) -> RedisResult<bool> {
    cmd("HEXISTS").arg(&[key, field]).query_async(conn).await
}

// List Operations
// ---------------

pub async fn llen(conn: &mut Connection, key: &str) -> RedisResult<i64> {
    cmd("LLEN").arg(key).query_async(conn).await
}

pub async fn lmove(
    conn: &mut Connection,
    source: &str,
    destination: &str,
    from: &str,
    to: &str
) -> RedisResult<Option<String>> {
    cmd("LMOVE").arg(&[source, destination, from, to]).query_async(conn).await
}

pub async fn blmove(
    conn: &mut Connection,
    source: &str,
    destination: &str,
    timeout: f64
) -> RedisResult<Option<String>> {
    cmd("BLMOVE")
        .arg(&[source, destination, "LEFT", "RIGHT", &timeout.to_string()])
        .query_async(conn).await
}

pub async fn lrange(conn: &mut Connection, key: &str, start: isize, stop: isize) -> RedisResult<Vec<String>> {
    cmd("LRANGE").arg(&[key, &start.to_string(), &stop.to_string()]).query_async(conn).await
}


// Multi-Queue Operations
// ----------------------

/// Pushes multiple payloads to multiple queues in a single atomic operation.
/// eg. {"queue1": ["payload1", "payload2"], "queue2": ["payload3", "payload4"]}
pub async fn multi_push(
    conn: &mut Connection,
    queue_payloads: &HashMap<String, Vec<String>>
) -> RedisResult<()> {
    let mut pipe = Pipeline::new();

    for (queue_name, payloads) in queue_payloads {
        pipe.rpush(queue_name, payloads);
    }

    pipe.atomic().query_async(conn).await
}

// Hash Operations
// ---------------

pub async fn register_in_hash<T: Serialize>(
    conn: &mut Connection,
    hash_key: &str,
    field: &str,
    value: &T,
    expiration_seconds: Option<u64>,
    reset_expiration: bool
) -> RedisResult<()> {
    let serialized = serde_json::to_string(value).expect("Failed to serialize value");

    // Set the field in the hash
    cmd("HSET").arg(&[hash_key, field, &serialized]).query_async(conn).await?;

    // Set expiration if provided
    if let Some(exp) = expiration_seconds {
        let mut expire_cmd = cmd("EXPIRE");
        expire_cmd.arg(&[hash_key, &exp.to_string()]);

        // Add NX option if reset_expiration is true
        if reset_expiration {
            expire_cmd.arg("NX");
        }

        expire_cmd.query_async(conn).await?;
    }

    Ok(())
}

// Queue Item Operations by Index
// ------------------------------

pub async fn remove_item_by_index(
    conn: &mut Connection,
    queue_name: &str,
    index: i64
) -> RedisResult<Option<String>> {
    // First, get the item at the specified index
    let item: Option<String> = cmd("LINDEX")
        .arg(&[queue_name, &index.to_string()])
        .query_async(conn)
        .await?;

    if let Some(item) = item {
        // If the item exists, remove it
        let removed: i64 = cmd("LREM")
            .arg(&[queue_name, "1", &item])
            .query_async(conn)
            .await?;

        if removed > 0 {
            Ok(Some(item))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub async fn get_item_by_index(
    conn: &mut Connection,
    queue_name: &str,
    index: i64
) -> RedisResult<Option<String>> {
    cmd("LINDEX").arg(&[queue_name, &index.to_string()]).query_async(conn).await
}

pub async fn overwrite_item_by_index(
    conn: &mut Connection,
    queue_name: &str,
    index: i64,
    updated_item: &str
) -> RedisResult<()> {
    cmd("LSET")
        .arg(&[queue_name, &index.to_string(), updated_item])
        .query_async(conn)
        .await
}

pub async fn move_item_by_index(
    conn: &mut Connection,
    source_queue: &str,
    destination_queue: &str,
    index: i64
) -> RedisResult<Option<String>> {
    cmd("LMOVE")
        .arg(&[source_queue, destination_queue, "LEFT", &index.to_string()])
        .query_async(conn)
        .await
}

// Channel Operations
// ------------------

pub async fn publish_to_channel(
    conn: &mut Connection,
    channel: &str,
    message: &str
) -> RedisResult<i64> {
    cmd("PUBLISH").arg(&[channel, message]).query_async(conn).await
}
