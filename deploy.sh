#!/bin/bash

# Deployment script for Rust Lambda function
set -e

# Default values
ARCHITECTURE="arm64"
CONTEXT="dev"
FUNCTION_NAME="rusty-api-maz"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --x86_64)
            ARCHITECTURE="x86_64"
            shift
            ;;
        --arm64)
            ARCHITECTURE="arm64"
            shift
            ;;
        --context)
            CONTEXT="$2"
            shift 2
            ;;
        --name)
            FUNCTION_NAME="$2"
            shift 2
            ;;
        --help)
            echo "Usage: ./deploy.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --arm64         Deploy for ARM64/Graviton (default)"
            echo "  --x86_64        Deploy for x86_64"
            echo "  --context       Deployment context: dev, staging, production (default: dev)"
            echo "  --name          Function name (default: rusty-api-maz)"
            echo "  --help          Show this help message"
            echo ""
            echo "Examples:"
            echo "  ./deploy.sh --arm64 --context production"
            echo "  ./deploy.sh --x86_64 --context staging --name my-lambda"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "🚀 Starting deployment of $FUNCTION_NAME..."
echo "📊 Architecture: $ARCHITECTURE"
echo "🌍 Context: $CONTEXT"

# Check if cargo-lambda is installed
if ! command -v cargo-lambda &> /dev/null; then
    echo "❌ cargo-lambda is not installed. Installing..."
    cargo install cargo-lambda
fi

# Check if Zig is installed (recommended for cross-compilation)
if ! command -v zig &> /dev/null; then
    echo "⚠️  Warning: Zig is not installed. Cross-compilation may not work optimally."
    echo "   Install with: brew install zig"
fi

# Build the function
echo "🔨 Building Lambda function..."
if [ "$ARCHITECTURE" = "arm64" ]; then
    cargo lambda build --release --arm64
else
    cargo lambda build --release --x86-64
fi

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Deploy to AWS
echo "☁️  Deploying to AWS Lambda..."
cargo lambda deploy --context "$CONTEXT" --name "$FUNCTION_NAME"

# Check if deployment was successful
if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Deployment completed successfully!"
    echo ""
    echo "📋 Deployment Summary:"
    echo "   Function: $FUNCTION_NAME"
    echo "   Architecture: $ARCHITECTURE"
    echo "   Context: $CONTEXT"
    echo ""
    echo "🔗 Useful commands:"
    echo "   Test locally: cargo lambda watch"
    echo "   View logs: aws logs tail /aws/lambda/$FUNCTION_NAME --follow"
    echo "   Invoke: cargo lambda invoke $FUNCTION_NAME --data-example apigw-request"
    echo "   Update: ./deploy.sh --context $CONTEXT"
else
    echo "❌ Deployment failed!"
    exit 1
fi
