use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use mk_test_lambda::application::service::RequestProcessor;
use mk_test_lambda::domain::models::{RequestPayload, ResponsePayload};
use mk_test_lambda::infrastructure::dynamo::DynamoDbAdapter;
use mk_test_lambda::infrastructure::s3::S3Adapter;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_s3::Client as S3Client;
use tracing::{info, error};

/// Main Lambda handler function
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Processing request: {:?}", event);

    // Extract query parameters
    let query_params = event.query_string_parameters();
    let path_params = event.path_parameters();
    
    // Parse request body if present
    let request_payload: Option<RequestPayload> = match event.body() {
        Body::Empty => None,
        Body::Text(text) => {
            match serde_json::from_str(text) {
                Ok(payload) => Some(payload),
                Err(e) => {
                    error!("Failed to parse request body: {}", e);
                    return Ok(create_error_response("Invalid JSON in request body"));
                }
            }
        }
        Body::Binary(_) => {
            error!("Binary body not supported");
            return Ok(create_error_response("Binary body not supported"));
        }
    };

    // Initialize AWS configuration
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    // Initialize Infrastructure Adapters
    let dynamo_client = DynamoClient::new(&config);
    let s3_client = S3Client::new(&config);

    let database_adapter = Box::new(DynamoDbAdapter::new(dynamo_client));
    let storage_adapter = Box::new(S3Adapter::new(s3_client));

    // Initialize Application Service
    let processor = RequestProcessor::new(database_adapter, storage_adapter);

    // Process the request
    // Note: query_params and path_params need to be converted to HashMap<String, String>
    // The lambda_http types are effectively maps, but we need to convert them to standard HashMaps for our port
    // For simplicity in this demo, we'll just pass empty maps or convert if needed.
    // The RequestProcessor signature expects &HashMap<String, String>.
    // lambda_http::aws_lambda_events::query_map::QueryMap is iterable.
    
    let mut q_params = std::collections::HashMap::new();
    for (k, v) in query_params.iter() {
        q_params.insert(k.to_string(), v.to_string());
    }

    let mut p_params = std::collections::HashMap::new();
    for (k, v) in path_params.iter() {
        p_params.insert(k.to_string(), v.to_string());
    }

    let result = processor.process_request(request_payload, &q_params, &p_params).await;

    match result {
        Ok(message) => {
            // Create response
            let response_payload = ResponsePayload {
                status: "success".to_string(),
                message,
                data: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };

            let response_body = serde_json::to_string(&response_payload)
                .map_err(|e| Error::from(format!("Failed to serialize response: {}", e)))?;

            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
                .body(Body::Text(response_body))
                .map_err(|e| Error::from(format!("Failed to build response: {}", e)))?)
        }
        Err(e) => {
            error!("Processing failed: {}", e);
            Ok(create_error_response(&format!("Processing failed: {}", e)))
        }
    }
}

/// Create an error response
fn create_error_response(message: &str) -> Response<Body> {
    let error_response = ResponsePayload {
        status: "error".to_string(),
        message: message.to_string(),
        data: None,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let error_body = serde_json::to_string(&error_response).unwrap_or_else(|_| {
        r#"{"status":"error","message":"Failed to serialize error response","timestamp":"1970-01-01T00:00:00Z"}"#.to_string()
    });

    Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::Text(error_body))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(500)
                .body(Body::Text("Internal server error".to_string()))
                .unwrap()
        })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    info!("Starting Rust Lambda function");

    // Run the Lambda function
    run(service_fn(function_handler)).await
}
