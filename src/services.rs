use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client as DynamoClient, Error as DynamoError};
use aws_sdk_s3::{Client as S3Client, Error as S3Error};
use std::collections::HashMap;

/// AWS service clients container
pub struct AwsServices {
    pub dynamodb: DynamoClient,
    pub s3: S3Client,
}

impl AwsServices {
    /// Create new AWS service clients
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;

        let dynamodb = DynamoClient::new(&config);
        let s3 = S3Client::new(&config);

        Ok(AwsServices { dynamodb, s3 })
    }

    /// Example: Get item from DynamoDB
    pub async fn get_dynamo_item(
        &self,
        table_name: &str,
        key: HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    ) -> Result<Option<HashMap<String, aws_sdk_dynamodb::types::AttributeValue>>, DynamoError> {
        let response = self
            .dynamodb
            .get_item()
            .table_name(table_name)
            .set_key(Some(key))
            .send()
            .await?;

        Ok(response.item)
    }

    /// Example: Put item to DynamoDB
    pub async fn put_dynamo_item(
        &self,
        table_name: &str,
        item: HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    ) -> Result<(), DynamoError> {
        self.dynamodb
            .put_item()
            .table_name(table_name)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    /// Example: Get object from S3
    pub async fn get_s3_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<Vec<u8>, S3Error> {
        let response = self
            .s3
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        let data = response.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }

    /// Example: Put object to S3
    pub async fn put_s3_object(
        &self,
        bucket: &str,
        key: &str,
        body: Vec<u8>,
    ) -> Result<(), S3Error> {
        self.s3
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body.into())
            .send()
            .await?;

        Ok(())
    }
}
