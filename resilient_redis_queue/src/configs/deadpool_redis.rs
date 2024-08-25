use deadpool_redis::{ Runtime, Config as RedisConfig };
use dotenvy::dotenv;
use serde::Deserialize;
use config::{ Config as ConfigTrait, ConfigError };
pub use deadpool_redis::{ Pool, Connection, redis::{ cmd, RedisError, RedisResult, Pipeline } };

#[derive(Debug, Deserialize)]
pub struct Config {
    pub redis: RedisConfig,
    pub api_key: String,
    pub version: String
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        ConfigTrait::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

pub fn create_pool() -> Pool {
    dotenv().ok();
    let cfg = Config::from_env().unwrap();
    cfg.redis.create_pool(Some(Runtime::Tokio1)).unwrap()
}
