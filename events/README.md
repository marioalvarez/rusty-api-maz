# Test Events

This directory contains test event files for testing the Lambda function locally and remotely.

## Available Events

### `local-test.json`
Simple POST request for local development testing.
- **Use case**: Basic local development
- **Method**: POST
- **Payload**: Simple message
- **Command**: `make invoke` or `cargo lambda invoke --data-file events/local-test.json`

### `test-complete.json`
Complete POST request with structured data.
- **Use case**: Full feature testing with complex payload
- **Method**: POST
- **Payload**: Message with nested data structure
- **Command**: `make invoke-complete`

### `test-health.json`
Health check endpoint test.
- **Use case**: Testing health/status endpoints
- **Method**: GET
- **Query params**: `health=true`
- **Payload**: None
- **Command**: `make invoke-health`

### `test-no-payload.json`
POST request without body payload.
- **Use case**: Testing empty request handling
- **Method**: POST
- **Payload**: Empty
- **Command**: `make invoke-no-payload`

### `aws-console-test.json`
Simulates a real AWS Lambda Function URL invocation.
- **Use case**: Production-like testing
- **Method**: POST
- **Headers**: Includes AWS-specific headers (X-Ray, forwarding)
- **Command**: `make invoke-aws`

## Using Test Events

### With Make commands
```bash
make invoke              # Basic test
make invoke-complete     # Complete payload
make invoke-health       # Health check
make invoke-no-payload   # Empty payload
make invoke-aws          # AWS format
```

### With cargo-lambda directly
```bash
cargo lambda invoke --data-file events/local-test.json
cargo lambda invoke --data-file events/test-complete.json
```

### With API Gateway example
```bash
make invoke-http
# or
cargo lambda invoke --data-example apigw-request
```

## Creating Custom Events

You can create custom event files following the Lambda Function URL event format:

```json
{
  "version": "2.0",
  "routeKey": "POST /",
  "rawPath": "/",
  "rawQueryString": "",
  "headers": {
    "content-type": "application/json"
  },
  "requestContext": {
    "http": {
      "method": "POST",
      "path": "/"
    }
  },
  "body": "{\"your\":\"payload\"}",
  "isBase64Encoded": false
}
```

## References

- [Lambda Function URLs Event Format](https://docs.aws.amazon.com/lambda/latest/dg/urls-invocation.html)
- [API Gateway Event Format](https://docs.aws.amazon.com/lambda/latest/dg/services-apigateway.html)
- [cargo-lambda invoke documentation](https://www.cargo-lambda.info/commands/invoke.html)
