use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;

/// Port for database operations
#[async_trait]
pub trait DatabasePort: Send + Sync {
    async fn get_item(&self, table_name: &str, key: HashMap<String, String>) -> Result<Option<HashMap<String, String>>, Box<dyn Error + Send + Sync>>;
    async fn put_item(&self, table_name: &str, item: HashMap<String, String>) -> Result<(), Box<dyn Error + Send + Sync>>;
}

/// Port for storage operations
#[async_trait]
pub trait StoragePort: Send + Sync {
    async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>;
    async fn put_object(&self, bucket: &str, key: &str, body: Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>>;
}
