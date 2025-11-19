# Makefile for Rust Lambda deployment

.PHONY: help build build-arm test clean deploy local invoke local-verbose

# Default target
help:
	@echo "Available targets:"
	@echo "  build        - Build the Lambda function for release (x86_64)"
	@echo "  build-arm    - Build for ARM64 (Graviton processors)"
	@echo "  test         - Run tests"
	@echo "  clean        - Clean build artifacts"
	@echo "  deploy       - Deploy to AWS Lambda"
	@echo "  local        - Run locally for testing"
	@echo "  local-verbose- Run locally with verbose logging"
	@echo "  invoke       - Test function with sample data"
	@echo "  install      - Install required tools"

# Build the Lambda function
build:
	cargo lambda build --release

# Build for ARM64 (Graviton processors)
build-arm:
	cargo lambda build --release --arm64

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Deploy to AWS Lambda
deploy: build
	cargo lambda deploy

# Run locally for testing
local:
	cargo lambda watch

# Test with sample data
invoke:
	cargo lambda invoke --data-ascii '{"message": "test"}'

# Test with verbose logging
local-verbose:
	cargo lambda watch --verbose

# Install required tools
install:
	@echo "Installing cargo-lambda..."
	cargo install cargo-lambda
	@echo "Installing cross-compilation target..."
	rustup target add x86_64-unknown-linux-gnu
	@echo "Setup complete!"

# Development setup
dev-setup: install
	@echo "Development environment setup complete!"
	@echo "You can now run 'make local' to start local development"
