#!/bin/bash
# Deploy Binary contracts to Tempo Testnet (Andantino)
#
# Prerequisites:
# 1. Set PRIVATE_KEY in .env or export it
# 2. Ensure deployer address has PATH_USD balance for gas fees
#    Get testnet funds: https://docs.tempo.xyz/quickstart/faucet
#
# Usage:
#   ./script/DeployTempo.sh
#
# Network Details:
#   Chain ID: 42429
#   RPC: https://rpc.testnet.tempo.xyz
#   Explorer: https://explore.tempo.xyz

set -e

# Load environment variables
if [ -f .env ]; then
    source .env
fi

if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: PRIVATE_KEY not set"
    echo "Set it in .env or export PRIVATE_KEY=..."
    exit 1
fi

echo "Deploying to Tempo Testnet (Andantino)..."
echo "Chain ID: 42429"
echo "RPC: https://rpc.testnet.tempo.xyz"
echo ""

forge script script/DeployAll.s.sol:DeployAll \
    --rpc-url tempo_testnet \
    --broadcast \
    --private-key "$PRIVATE_KEY" \
    -vvvv

echo ""
echo "Deployment complete!"
echo "View on explorer: https://explore.tempo.xyz"
