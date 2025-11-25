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
        _query_params: &HashMap<String, String>,
        _path_params: &HashMap<String, String>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Example business logic
        let message = match payload {
            Some(p) => p.message.unwrap_or_else(|| "No message provided".to_string()),
            None => "No payload provided".to_string(),
        };

        // Example: Use AWS services (via ports)
        // In a real app, you would do something useful here.
        // For now, we just check if we can call them.
        
        // Example DynamoDB operation
        // let _ = self.database.get_item("some-table", HashMap::new()).await?;

        // Example S3 operation
        // let _ = self.storage.get_object("some-bucket", "some-key").await?;

        // Simulate processing
        // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(format!("Hello from Rust Lambda! Received message: {}. Services are available.", message))
    }
}
