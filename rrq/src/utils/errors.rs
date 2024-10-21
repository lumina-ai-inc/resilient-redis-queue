use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Redis error: {0}")]
    Redis(#[from] crate::utils::config_redis::RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Item {item_id} with index {index} not found in the queue")]
    ItemNotFound { item_id: String, index: i64 },

    #[error("Failed to process item {item_id}: {message}")]
    ProcessingError { item_id: String, message: String },
}