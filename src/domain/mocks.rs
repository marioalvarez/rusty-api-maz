use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use super::ports::{DatabasePort, StoragePort};

/// Mock implementation of DatabasePort for testing
pub struct MockDatabase {
    pub items: HashMap<String, HashMap<String, String>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn with_item(mut self, table: &str, key: &str, value: HashMap<String, String>) -> Self {
        let full_key = format!("{}::{}", table, key);
        self.items.insert(full_key, value);
        self
    }
}

#[async_trait]
impl DatabasePort for MockDatabase {
    async fn get_item(&self, table_name: &str, key: HashMap<String, String>) -> Result<Option<HashMap<String, String>>, Box<dyn Error + Send + Sync>> {
        let key_str = key.values().next().unwrap_or(&String::new()).clone();
        let full_key = format!("{}::{}", table_name, key_str);
        Ok(self.items.get(&full_key).cloned())
    }

    async fn put_item(&self, _table_name: &str, _item: HashMap<String, String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

/// Mock implementation of StoragePort for testing
pub struct MockStorage {
    pub objects: HashMap<String, Vec<u8>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn with_object(mut self, bucket: &str, key: &str, data: Vec<u8>) -> Self {
        let full_key = format!("{}::{}", bucket, key);
        self.objects.insert(full_key, data);
        self
    }
}

#[async_trait]
impl StoragePort for MockStorage {
    async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let full_key = format!("{}::{}", bucket, key);
        self.objects
            .get(&full_key)
            .cloned()
            .ok_or_else(|| "Object not found".into())
    }

    async fn put_object(&self, _bucket: &str, _key: &str, _body: Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_database() {
        let mut item = HashMap::new();
        item.insert("name".to_string(), "test".to_string());

        let db = MockDatabase::new()
            .with_item("test-table", "test-key", item.clone());

        let mut key = HashMap::new();
        key.insert("id".to_string(), "test-key".to_string());

        let result = db.get_item("test-table", key).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().get("name").unwrap(), "test");
    }

    #[tokio::test]
    async fn test_mock_storage() {
        let data = b"test data".to_vec();
        let storage = MockStorage::new()
            .with_object("test-bucket", "test-key", data.clone());

        let result = storage.get_object("test-bucket", "test-key").await.unwrap();
        assert_eq!(result, data);
    }

    #[tokio::test]
    async fn test_mock_storage_not_found() {
        let storage = MockStorage::new();
        let result = storage.get_object("test-bucket", "missing-key").await;
        assert!(result.is_err());
    }
}
