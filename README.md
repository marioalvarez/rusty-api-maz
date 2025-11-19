# mk-test-lambda

A test AWS Lambda function written in Rust for the MiMarket backend - Development/Testing purposes.

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
curl -X POST http://localhost:9000/lambda-url/mk-test-lambda \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from test!"}'
```

## Deployment

### Deploy to AWS
```bash
cargo lambda deploy
```

### Deploy with specific configuration
```bash
cargo lambda deploy --iam-role arn:aws:iam::ACCOUNT:role/lambda-execution-role
```

## Project Structure

```
mk-sample-rust-lambda/
├── Cargo.toml          # Rust dependencies and configuration
├── src/
│   └── main.rs         # Main Lambda handler
├── README.md           # This file
└── .cargo/
    └── config.toml     # Cargo configuration for cross-compilation
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

Run tests with:
```bash
cargo test
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
