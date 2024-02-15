use redis::AsyncCommands;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define the struct wrapper around raw Redis client
struct StorageService {
    redis_client: redis::Client,
}

impl StorageService {
    // Create a new instance of StorageService
    pub fn new(redis_client: redis::Client) -> Self {
        StorageService { redis_client }
    }
}

pub fn initialize_store() -> StorageService {
    let redis_client =
        Client::open("redis://:password@localhost:6379/0").expect("Failed to connect to Redis");
    let connection = redis_client.get_connection();
    if let Err(err) = connection {
        panic!("Error initializing Redis: {}", err);
    }

    println!("Redis started successfully");
    StorageService::new(redis_client)
}

pub fn save_url_mapping(short_url: &str, original_url: &str) {
    StorageService
        .redis_client
        .set_ex(short_url, original_url, None::<Duration>)
        .await?;

    println!(
        "Saved Short URL: {:?} - Original URL: {:?}",
        short_url, original_url
    );
}

pub fn initial_url(short_url: &str) -> String {
    let original_url = StorageService.redis_client.get(short_url)?;
    original_url
}
