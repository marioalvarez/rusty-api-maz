use crate::domain::ports::StoragePort;
use async_trait::async_trait;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use std::error::Error as StdError;

pub struct S3Adapter {
    client: Client,
}

impl S3Adapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl StoragePort for S3Adapter {
    async fn get_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<Vec<u8>, Box<dyn StdError + Send + Sync>> {
        let response = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        let data = response.body.collect().await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
            
        Ok(data.into_bytes().to_vec())
    }

    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        body: Vec<u8>,
    ) -> Result<(), Box<dyn StdError + Send + Sync>> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        Ok(())
    }
}
