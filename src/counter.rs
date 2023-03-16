use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Counter {
    pub count: i32,
}

impl Counter {
    fn get_redis_url() -> String {
        let config = Config::new();
        config
            .map
            .get("REDIS_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned()
    }
    pub fn save(self) -> Result<(), redis::RedisError> {
        let client = match redis::Client::open(Counter::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error),
        };
        let mut con = match client.get_connection() {
            Ok(con) => con,
            Err(error) => return Err(error),
        };
        con.set("COUNTER", self.count)?;
        Ok(())
    }
    pub fn load() -> Result<Counter, redis::RedisError> {
        let client = match redis::Client::open(Counter::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error),
        };
        let mut con = match client.get_connection() {
            Ok(con) => con,
            Err(error) => return Err(error),
        };
        let count = con.get("COUNTER")?;
        Ok(Counter { count })
    }
}
