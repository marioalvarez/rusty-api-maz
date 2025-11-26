# rusty-api-maz

AWS Lambda API written in Rust with Hexagonal Architecture (Ports & Adapters) - Production ready.

**Author:** malvarezz

## Overview

This Lambda function demonstrates how to create a Rust-based AWS Lambda with:
- HTTP request handling
- JSON serialization/deserialization
- Error handling
- Logging with tracing
- CORS support

## Prerequisites

1. **Install Rust**: Follow the instructions at [rustup.rs](https://rustup.rs/)
2. **Install cargo-lambda**: 
   ```bash
   cargo install cargo-lambda
   ```

## Local Development

### Build the function
```bash
cargo lambda build --release
```

### Test locally
```bash
cargo lambda watch
```

This will start a local server that you can test with:
```bash
curl -X POST http://localhost:9000/lambda-url/rusty-api-maz \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from test!"}'
```

## Deployment

### Option 1: Manual Upload (AWS Console)

1. **Build the deployment package:**
   ```bash
   # For x86_64 architecture (most common)
   cargo lambda build --release --x86-64
   
   # For ARM64 architecture (Graviton2 - more cost-effective)
   cargo lambda build --release --arm64
   ```

2. **Create the ZIP file:**
   ```bash
   # x86_64
   cd target/lambda/bootstrap && zip -j ../../../dist/lambda-x86_64.zip bootstrap && cd ../../..
   
   # ARM64
   cd target/lambda/bootstrap && zip -j ../../../dist/lambda-arm64.zip bootstrap && cd ../../..
   ```

3. **Upload to AWS Lambda:**
   - Go to your Lambda function in AWS Console
   - Click "Upload from" → ".zip file"
   - Select `dist/lambda-x86_64.zip` (or `dist/lambda-arm64.zip` for ARM)
   - Click "Save"
   - Ensure the architecture matches (x86_64 or arm64)

### Option 2: AWS CLI

```bash
# Build and package
cargo lambda build --release --x86-64
cd target/lambda/bootstrap && zip -j ../../../dist/lambda-x86_64.zip bootstrap && cd ../../..

# Update Lambda function
aws lambda update-function-code \
  --function-name your-lambda-name \
  --zip-file fileb://dist/lambda-x86_64.zip
```

### Option 3: Direct Deploy with cargo-lambda

```bash
cargo lambda deploy

# With specific IAM role
cargo lambda deploy --iam-role arn:aws:iam::ACCOUNT:role/lambda-execution-role
```

### Environment Variables (Optional)

Configure these in your Lambda function settings:

- `DYNAMO_TABLE` - DynamoDB table name (default: `demo-table`)
- `S3_BUCKET` - S3 bucket name (default: `demo-bucket`)
- `RUST_LOG` - Logging level (default: `info`, options: `trace`, `debug`, `info`, `warn`, `error`)
- `AWS_REGION` - AWS region (default: `us-east-1`)

## Architecture

This project follows **Hexagonal Architecture (Ports & Adapters)** principles:

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                    │
│              (Business Logic / Use Cases)               │
│                  RequestProcessor                       │
└──────────────────┬───────────────────┬──────────────────┘
                   │                   │
          ┌────────▼────────┐ ┌────────▼────────┐
          │  DatabasePort   │ │  StoragePort    │
          │    (Trait)      │ │    (Trait)      │
          └────────┬────────┘ └────────┬────────┘
                   │                   │
     ┌─────────────▼───────────────────▼─────────────┐
     │         Infrastructure Layer                   │
     │     (Adapters / External Services)             │
     │    DynamoDbAdapter    S3Adapter                │
     └────────────────────────────────────────────────┘
```

### Layers

1. **Domain Layer** (`src/domain/`)
   - `models.rs`: Core data structures (RequestPayload, ResponsePayload)
   - `ports.rs`: Trait definitions for external dependencies (DatabasePort, StoragePort)
   - `mocks.rs`: Mock implementations for testing (test-only)

2. **Application Layer** (`src/application/`)
   - `service.rs`: Business logic (RequestProcessor)
   - Uses ports to interact with external services
   - Independent of infrastructure details

3. **Infrastructure Layer** (`src/infrastructure/`)
   - `dynamo.rs`: DynamoDB adapter implementing DatabasePort
   - `s3.rs`: S3 adapter implementing StoragePort
   - Concrete implementations of domain ports

4. **Main** (`src/main.rs`)
   - Dependency injection and wiring
   - Lambda runtime setup
   - HTTP request/response handling

### Benefits

- **Testability**: Easy to test with mock implementations
- **Flexibility**: Can swap implementations without changing business logic
- **Independence**: Business logic doesn't depend on AWS SDK
- **SOLID Principles**: Dependency Inversion, Single Responsibility

## Project Structure

```
rusty-api-maz/
├── Cargo.toml                  # Rust dependencies
├── src/
│   ├── domain/                 # Domain layer
│   │   ├── mod.rs
│   │   ├── models.rs           # Core data structures
│   │   ├── ports.rs            # Port traits
│   │   └── mocks.rs            # Test mocks
│   ├── application/            # Application layer
│   │   ├── mod.rs
│   │   └── service.rs          # Business logic
│   ├── infrastructure/         # Infrastructure layer
│   │   ├── mod.rs
│   │   ├── dynamo.rs           # DynamoDB adapter
│   │   └── s3.rs               # S3 adapter
│   ├── lib.rs
│   └── main.rs                 # Entry point & DI wiring
├── tests/
│   └── integration_test.rs     # Integration tests
└── .cargo/
    └── config.toml             # Cross-compilation config
```

## API

### Request Format
```json
{
  "message": "Optional message",
  "data": {
    "key": "value"
  }
}
```

### Response Format
```json
{
  "status": "success",
  "message": "Hello from Rust Lambda! Received message: ...",
  "data": null,
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Environment Variables

The function can be configured with the following environment variables:
- `RUST_LOG`: Logging level (default: info)
- `AWS_REGION`: AWS region (default: us-east-1)

## Testing

The project includes comprehensive tests:

### Unit Tests
```bash
# Run all tests
cargo test

# Run only library tests (unit tests)
cargo test --lib

# Run specific test module
cargo test domain::mocks
cargo test application::service
```

### Test Coverage

- **Domain Layer**: Mock implementations with unit tests
- **Application Layer**: Business logic tests using mocks (5 test cases)
- **Integration Tests**: Lambda handler integration tests (2 test cases)

**Total: 10 tests** (8 unit + 2 integration)

### Test Structure

```rust
// Example: Testing with mocks
use crate::domain::mocks::{MockDatabase, MockStorage};

let db = Box::new(MockDatabase::new());
let storage = Box::new(MockStorage::new());
let processor = RequestProcessor::new(db, storage);
```

## Performance

Rust Lambda functions typically offer:
- Faster cold start times compared to Node.js
- Lower memory usage
- Better performance for CPU-intensive tasks
- Type safety and memory safety

## Monitoring

The function includes structured logging that integrates with AWS CloudWatch Logs. Use the following log levels:
- `ERROR`: For errors that need immediate attention
- `WARN`: For warnings
- `INFO`: For general information (default)
- `DEBUG`: For detailed debugging information
