use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error};

mod services;
use services::AwsServices;

/// Request payload structure
#[derive(Deserialize)]
struct RequestPayload {
    message: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
}

/// Response payload structure
#[derive(Serialize)]
struct ResponsePayload {
    status: String,
    message: String,
    data: Option<HashMap<String, serde_json::Value>>,
    timestamp: String,
}

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

    // Process the request
    let response = process_request(request_payload, query_params, path_params).await;

    // Create response
    let response_payload = ResponsePayload {
        status: "success".to_string(),
        message: response,
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

/// Process the incoming request
async fn process_request(
    payload: Option<RequestPayload>,
    query_params: &HashMap<String, String>,
    path_params: &HashMap<String, String>,
) -> String {
    info!("Processing request with payload: {:?}", payload);
    info!("Query parameters: {:?}", query_params);
    info!("Path parameters: {:?}", path_params);

    // Example business logic
    let message = match payload {
        Some(p) => p.message.unwrap_or_else(|| "No message provided".to_string()),
        None => "No payload provided".to_string(),
    };

    // Initialize AWS services (in a real application, you might want to cache this)
    let aws_services = match AwsServices::new().await {
        Ok(services) => {
            info!("AWS services initialized successfully");
            Some(services)
        }
        Err(e) => {
            error!("Failed to initialize AWS services: {}", e);
            None
        }
    };

    // Example: Use AWS services if available
    if let Some(services) = aws_services {
        // Example: You could interact with DynamoDB or S3 here
        info!("AWS services are available for use");
        
        // Example DynamoDB operation (commented out to avoid actual AWS calls in demo)
        // let result = services.get_dynamo_item("your-table", HashMap::new()).await;
        // info!("DynamoDB result: {:?}", result);
    }

    // Simulate some processing
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    format!("Hello from Rust Lambda! Received message: {}. AWS services: {}", 
            message, 
            if aws_services.is_some() { "available" } else { "unavailable" })
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
