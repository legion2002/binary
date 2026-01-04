#!/bin/bash
# Deploy Binary contracts to local Tempo node
#
# Usage:
#   ./script/deploy-local.sh
#
# Prerequisites:
#   - Local Tempo node running (docker-compose up -d)
#   - forge installed

set -e

LOCAL_RPC="http://localhost:9545"
# Test private key from standard mnemonic (DO NOT use in production)
TEST_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

echo "=========================================="
echo "Binary Protocol - Local Deployment"
echo "=========================================="
echo ""
echo "RPC URL: $LOCAL_RPC"
echo ""

# Check if Tempo node is running
echo "Checking Tempo node..."
if ! curl -s "$LOCAL_RPC" > /dev/null 2>&1; then
    echo "Error: Tempo node not running!"
    echo ""
    echo "Start it with:"
    echo "  docker-compose up -d"
    echo ""
    echo "Or use the TypeScript script:"
    echo "  cd frontend && bun run ../script/test-local.ts"
    exit 1
fi
echo "✓ Tempo node is running"
echo ""

# Deploy contracts
echo "Deploying contracts..."
echo ""

forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
    --rpc-url "$LOCAL_RPC" \
    --broadcast \
    --private-key "$TEST_PRIVATE_KEY" \
    -vvv

echo ""
echo "=========================================="
echo "Local Deployment Complete!"
echo "=========================================="
