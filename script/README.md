# Binary Protocol Deployment Scripts

This directory contains deployment and management scripts for the Binary Protocol on Base mainnet.

## Prerequisites

1. Install Foundry if you haven't already:
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   ```

2. Set up your environment variables:
   ```bash
   export PRIVATE_KEY="your_private_key_here"
   export RPC_URL="https://mainnet.base.org"
   ```

## Deployment Process

### 1. Deploy MultiVerse Contract

Deploy the main MultiVerse contract to Base mainnet:

```bash
forge script script/DeployMultiVerse.s.sol:DeployMultiVerse \
  --rpc-url $RPC_URL \
  --broadcast \
  --verify \
  --etherscan-api-key $BASESCAN_API_KEY \
  -vvvv
```

After deployment, note the deployed MultiVerse address from the logs.

### 2. Update Frontend Configuration

Update the `MULTIVERSE` address in `frontend/src/config/contracts.ts` with the deployed address.

### 3. Create Markets

To create a new prediction market:

```bash
export MULTIVERSE_ADDRESS="0x..." # Your deployed MultiVerse address

forge script script/CreateMarket.s.sol:CreateMarket \
  --rpc-url $RPC_URL \
  --broadcast \
  -vvvv
```

## Base Mainnet Configuration

The scripts are configured for Base mainnet with the following addresses:

- **Chain ID**: 8453
- **WETH**: `0x4200000000000000000000000000000000000006`
- **USDC**: `0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913`
- **Uniswap V2 Factory**: `0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6`
- **Uniswap V2 Router**: `0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24`

## Gas Estimation

Before broadcasting, you can simulate the deployment to estimate gas costs:

```bash
forge script script/DeployMultiVerse.s.sol:DeployMultiVerse \
  --rpc-url $RPC_URL \
  -vvvv
```

## Verification

The deployment script includes automatic contract verification on BaseScan. Make sure you have:

1. A BaseScan API key (get one at https://basescan.org/myapikey)
2. Set it as an environment variable: `export BASESCAN_API_KEY="your_api_key"`

## Security Notes

- Always test deployments on testnet first
- Double-check all addresses and parameters before mainnet deployment
- Keep your private keys secure and never commit them to version control
- Consider using a hardware wallet or multisig for mainnet deployments

## Troubleshooting

If you encounter issues:

1. Ensure you have sufficient ETH on Base for gas fees
2. Check that your RPC URL is correct
3. Verify that your private key has the correct format (with or without 0x prefix)
4. Use `-vvvv` flag for detailed debugging output