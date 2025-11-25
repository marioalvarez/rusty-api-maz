use crate::domain::ports::DatabasePort;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use std::collections::HashMap;
use std::error::Error as StdError;

pub struct DynamoDbAdapter {
    client: Client,
}

impl DynamoDbAdapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl DatabasePort for DynamoDbAdapter {
    async fn get_item(
        &self,
        table_name: &str,
        key: HashMap<String, String>,
    ) -> Result<Option<HashMap<String, String>>, Box<dyn StdError + Send + Sync>> {
        let mut dynamo_key = HashMap::new();
        for (k, v) in key {
            dynamo_key.insert(k, AttributeValue::S(v));
        }

        let response = self.client
            .get_item()
            .table_name(table_name)
            .set_key(Some(dynamo_key))
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        if let Some(item) = response.item {
            let mut result = HashMap::new();
            for (k, v) in item {
                if let AttributeValue::S(s) = v {
                    result.insert(k, s);
                }
                // Note: Ignoring non-string values for this simple port implementation
            }
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    async fn put_item(
        &self,
        table_name: &str,
        item: HashMap<String, String>,
    ) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let mut dynamo_item = HashMap::new();
        for (k, v) in item {
            dynamo_item.insert(k, AttributeValue::S(v));
        }

        self.client
            .put_item()
            .table_name(table_name)
            .set_item(Some(dynamo_item))
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        Ok(())
    }
}
