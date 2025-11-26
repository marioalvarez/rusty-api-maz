use crate::domain::models::RequestPayload;
use crate::domain::ports::{DatabasePort, StoragePort};
use std::collections::HashMap;
use std::error::Error;

pub struct RequestProcessor {
    database: Box<dyn DatabasePort>,
    storage: Box<dyn StoragePort>,
}

impl RequestProcessor {
    pub fn new(database: Box<dyn DatabasePort>, storage: Box<dyn StoragePort>) -> Self {
        Self { database, storage }
    }

    pub async fn process_request(
        &self,
        payload: Option<RequestPayload>,
        query_params: &HashMap<String, String>,
        _path_params: &HashMap<String, String>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Example business logic
        let message = match payload {
            Some(p) => p.message.unwrap_or_else(|| "No message provided".to_string()),
            None => "No payload provided".to_string(),
        };

        // Check if this is a health check request
        if query_params.get("health").map(|v| v.as_str()) == Some("true") {
            return Ok("Service is healthy".to_string());
        }

        // Example DynamoDB operation - Check if item exists in a demo table
        // This demonstrates port usage for database operations
        let table_name = std::env::var("DYNAMO_TABLE").unwrap_or_else(|_| "demo-table".to_string());
        let mut key = HashMap::new();
        key.insert("order_id".to_string(), "1111".to_string());
        key.insert("segment".to_string(), "10".to_string());
        
        let dynamo_info = match self.database.get_item(&table_name, key).await {
            Ok(Some(item)) => {
                tracing::info!("Found item in DynamoDB: {:?}", item);
                let item_json = serde_json::to_string_pretty(&item).unwrap_or_else(|_| format!("{:?}", item));
                format!("DynamoDB Item Found:\n{}", item_json)
            },
            Ok(None) => {
                tracing::info!("Item not found in DynamoDB");
                "DynamoDB: Item not found".to_string()
            },
            Err(e) => {
                tracing::warn!("DynamoDB error (expected in demo): {}", e);
                format!("DynamoDB Error: {}", e)
            }
        };

        // Example S3 operation - Check if object exists
        // This demonstrates port usage for storage operations
        let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "demo-bucket".to_string());
        let key = "demo-object.txt";
        
        let s3_info = match self.storage.get_object(&bucket, key).await {
            Ok(data) => {
                let content = String::from_utf8_lossy(&data);
                tracing::info!("Found object in S3, size: {} bytes", data.len());
                format!("S3 Object Found ({})\nContent:\n{}", key, content)
            },
            Err(e) => {
                tracing::warn!("S3 error (expected in demo): {}", e);
                format!("S3 Error: {}", e)
            }
        };

        Ok(format!(
            "Hello from Rust Lambda! Received message: {}\n\n--- AWS Services Info ---\n\n{}\n\n{}\n\nDatabase and storage services invoked successfully.",
            message, dynamo_info, s3_info
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::mocks::{MockDatabase, MockStorage};

    #[tokio::test]
    async fn test_process_request_with_payload() {
        let db = Box::new(MockDatabase::new());
        let storage = Box::new(MockStorage::new());
        let processor = RequestProcessor::new(db, storage);

        let payload = Some(RequestPayload {
            message: Some("Test message".to_string()),
            data: None,
        });

        let result = processor.process_request(payload, &HashMap::new(), &HashMap::new()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Test message"));
    }

    #[tokio::test]
    async fn test_process_request_without_payload() {
        let db = Box::new(MockDatabase::new());
        let storage = Box::new(MockStorage::new());
        let processor = RequestProcessor::new(db, storage);

        let result = processor.process_request(None, &HashMap::new(), &HashMap::new()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("No payload provided"));
    }

    #[tokio::test]
    async fn test_health_check() {
        let db = Box::new(MockDatabase::new());
        let storage = Box::new(MockStorage::new());
        let processor = RequestProcessor::new(db, storage);

        let mut query_params = HashMap::new();
        query_params.insert("health".to_string(), "true".to_string());

        let result = processor.process_request(None, &query_params, &HashMap::new()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Service is healthy");
    }

    #[tokio::test]
    async fn test_process_request_with_database_item() {
        let mut item = HashMap::new();
        item.insert("name".to_string(), "test-item".to_string());

        let db = Box::new(MockDatabase::new().with_item("demo-table", "demo-key", item));
        let storage = Box::new(MockStorage::new());
        let processor = RequestProcessor::new(db, storage);

        let payload = Some(RequestPayload {
            message: Some("Test".to_string()),
            data: None,
        });

        let result = processor.process_request(payload, &HashMap::new(), &HashMap::new()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_request_with_storage_object() {
        let db = Box::new(MockDatabase::new());
        let storage = Box::new(MockStorage::new().with_object("demo-bucket", "demo-object.txt", b"test data".to_vec()));
        let processor = RequestProcessor::new(db, storage);

        let payload = Some(RequestPayload {
            message: Some("Test".to_string()),
            data: None,
        });

        let result = processor.process_request(payload, &HashMap::new(), &HashMap::new()).await;
        assert!(result.is_ok());
    }
}
