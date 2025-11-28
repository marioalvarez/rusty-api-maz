# Makefile for Rust Lambda deployment with cargo-lambda

.PHONY: help build build-arm build-x86 build-zip build-zip-x86 test test-unit test-integration clean deploy deploy-dev deploy-staging deploy-prod local invoke invoke-complete invoke-health invoke-no-payload invoke-aws invoke-http local-verbose install check check-deps format lint logs

# Default target
help:
	@echo "🚀 Rust Lambda Makefile - Available targets:"
	@echo ""
	@echo "Build Commands:"
	@echo "  build           - Build for ARM64 (default, recommended)"
	@echo "  build-arm       - Build for ARM64 (Graviton processors)"
	@echo "  build-x86       - Build for x86_64"
	@echo "  build-zip       - Build and package ARM64 as ZIP in dist/"
	@echo "  build-zip-x86   - Build and package x86_64 as ZIP in dist/"
	@echo ""
	@echo "Deployment Commands:"
	@echo "  deploy          - Deploy to AWS Lambda (dev context)"
	@echo "  deploy-dev      - Deploy to development environment"
	@echo "  deploy-staging  - Deploy to staging environment"
	@echo "  deploy-prod     - Deploy to production environment"
	@echo ""
	@echo "Testing Commands:"
	@echo "  test            - Run all tests"
	@echo "  test-unit       - Run unit tests only"
	@echo "  test-integration- Run integration tests only"
	@echo "  local           - Run locally for testing (watch mode)"
	@echo "  invoke          - Test with basic payload"
	@echo "  invoke-complete - Test with complete payload"
	@echo "  invoke-health   - Test health check"
	@echo "  invoke-no-payload - Test without payload"
	@echo "  invoke-aws      - Test with AWS Console format"
	@echo "  invoke-http     - Test with API Gateway example"
	@echo ""
	@echo "Quality Commands:"
	@echo "  check           - Check code without building"
	@echo "  format          - Format code with rustfmt"
	@echo "  lint            - Run clippy linter"
	@echo "  check-deps      - Check for dependency issues"
	@echo ""
	@echo "Utility Commands:"
	@echo "  clean           - Clean build artifacts"
	@echo "  install         - Install required tools"
	@echo "  logs            - View Lambda logs (requires AWS CLI)"

# Build the Lambda function (ARM64 default)
build: build-arm

# Build for ARM64 (Graviton processors - better cost/performance)
build-arm:
	@echo "🔨 Building for ARM64 (Graviton)..."
	cargo lambda build --release --arm64

# Build for x86_64
build-x86:
	@echo "🔨 Building for x86_64..."
	cargo lambda build --release --x86-64

# Build and package as ZIP (ARM64)
build-zip:
	@echo "📦 Building and packaging as ZIP (ARM64)..."
	cargo lambda build --release --arm64 --output-format zip
	@mkdir -p dist
	@cp target/lambda/bootstrap/bootstrap.zip dist/lambda-arm64.zip
	@echo "✅ ZIP created: dist/lambda-arm64.zip"
	@ls -lh dist/lambda-arm64.zip

# Build and package as ZIP (x86_64)
build-zip-x86:
	@echo "📦 Building and packaging as ZIP (x86_64)..."
	cargo lambda build --release --x86-64 --output-format zip
	@mkdir -p dist
	@cp target/lambda/bootstrap/bootstrap.zip dist/lambda-x86_64.zip
	@echo "✅ ZIP created: dist/lambda-x86_64.zip"
	@ls -lh dist/lambda-x86_64.zip

# Run all tests
test:
	@echo "🧪 Running all tests..."
	cargo test

# Run unit tests only
test-unit:
	@echo "🧪 Running unit tests..."
	cargo test --lib

# Run integration tests only
test-integration:
	@echo "🧪 Running integration tests..."
	cargo test --test integration_test

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Deploy to AWS Lambda (dev context)
deploy: build
	@echo "☁️  Deploying to AWS Lambda (dev)..."
	./deploy.sh --arm64 --context dev

# Deploy to development environment
deploy-dev: build
	@echo "☁️  Deploying to development..."
	./deploy.sh --arm64 --context dev

# Deploy to staging environment
deploy-staging: build
	@echo "☁️  Deploying to staging..."
	./deploy.sh --arm64 --context staging

# Deploy to production environment
deploy-prod: build
	@echo "☁️  Deploying to production..."
	./deploy.sh --arm64 --context production

# Run locally for testing
local:
	@echo "🏃 Starting local Lambda emulator..."
	cargo lambda watch

# Test with basic payload
invoke:
	@echo "📨 Invoking with basic payload..."
	cargo lambda invoke --data-file events/local-test.json

# Test with complete payload
invoke-complete:
	@echo "📦 Testing with complete payload..."
	cargo lambda invoke --data-file events/test-complete.json

# Test health check endpoint
invoke-health:
	@echo "🏥 Testing health check..."
	cargo lambda invoke --data-file events/test-health.json

# Test without payload
invoke-no-payload:
	@echo "📭 Testing without payload..."
	cargo lambda invoke --data-file events/test-no-payload.json

# Test with AWS Console format
invoke-aws:
	@echo "☁️  Testing with AWS format..."
	cargo lambda invoke --data-file events/aws-console-test.json

# Test with API Gateway event example
invoke-http:
	@echo "🌐 Testing with API Gateway event..."
	cargo lambda invoke --data-example apigw-request

# Test with verbose logging
local-verbose:
	@echo "🏃 Starting local Lambda with verbose logging..."
	cargo lambda watch --verbose

# Check code without building
check:
	@echo "🔍 Checking code..."
	cargo check

# Format code with rustfmt
format:
	@echo "✨ Formatting code..."
	cargo fmt

# Run clippy linter
lint:
	@echo "🔎 Running clippy..."
	cargo clippy -- -D warnings

# Check for dependency issues
check-deps:
	@echo "📦 Checking dependencies..."
	cargo tree
	@echo ""
	@echo "Checking for outdated dependencies..."
	cargo outdated || echo "Install cargo-outdated: cargo install cargo-outdated"

# Install required tools
install:
	@echo "📥 Installing required tools..."
	@command -v cargo-lambda >/dev/null 2>&1 || cargo install cargo-lambda
	@command -v zig >/dev/null 2>&1 || echo "⚠️  Consider installing Zig for better cross-compilation: brew install zig"
	@rustup target list --installed | grep -q aarch64-unknown-linux-gnu || rustup target add aarch64-unknown-linux-gnu
	@rustup target list --installed | grep -q x86_64-unknown-linux-gnu || rustup target add x86_64-unknown-linux-gnu
	@echo "✅ Setup complete!"

# View Lambda logs (requires AWS CLI and function name)
logs:
	@echo "📜 Viewing Lambda logs..."
	@aws logs tail /aws/lambda/rusty-api-maz --follow --format short
