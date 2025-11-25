#!/bin/bash

# Deployment script for Rust Lambda function
set -e

echo "🚀 Starting deployment of rusty-api-maz..."

# Check if cargo-lambda is installed
if ! command -v cargo-lambda &> /dev/null; then
    echo "❌ cargo-lambda is not installed. Installing..."
    cargo install cargo-lambda
fi

# Check if required target is installed
if ! rustup target list --installed | grep -q "x86_64-unknown-linux-gnu"; then
    echo "📦 Installing cross-compilation target..."
    rustup target add x86_64-unknown-linux-gnu
fi

# Build the function
echo "🔨 Building Lambda function..."
cargo lambda build --release

# Deploy to AWS
echo "☁️  Deploying to AWS Lambda..."
cargo lambda deploy

echo "✅ Deployment completed successfully!"
echo ""
echo "📋 Next steps:"
echo "1. Configure the Lambda function in AWS Console"
echo "2. Set up API Gateway if needed"
echo "3. Configure environment variables"
echo "4. Test the function"
echo ""
echo "🔗 Useful commands:"
echo "- Test locally: cargo lambda watch"
echo "- View logs: aws logs tail /aws/lambda/rusty-api-maz --follow"
echo "- Update function: cargo lambda deploy"
