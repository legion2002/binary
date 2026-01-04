# Binary

**Price feeds from the future.**

- *What will ETH trade at if FOCIL is included in the Glamsterdam hardfork?*  
- *How will GPU prices diverge if Trump is (or isn’t) re-elected?*

Binary is a lean, highly-optimized prediction-market framework that spawns self-contained economies around every yes/no question.


## Market

A **market** is simply an oracle answering a binary question.

- **Will FOCIL be included in the Glamsterdam hardfork?**  
- **Will Trump win the next U.S. presidential election?**


## Splitter

1. Escrow **1 USDC** in a smart contract.  
2. Receive **1 YES_USDC** and **1 NO_USDC**.  
3. **YES_USDC** pays **1 USDC** if the oracle says **YES**, otherwise **0**.  
4. **NO_USDC** does the opposite.

Holding **1 USDC ≡ 1 YES_USDC + 1 NO_USDC**.  
The same mechanic works for any other asset in the same market —e.g., split **1 ETH** into **YES_ETH** and **NO_ETH**.


## Economy

Every economy—onchain or offchain—reduces to three building blocks:

1. **Assets:** goods and services.  
2. **Trade:** venues to exchange them.  
3. **Credit:** mechanisms to lend, borrow, or lever up.

#### YES-Economy

- Assets: **YES_USDC**, **YES_ETH**  
- AMM: **YES_ETH / YES_USDC**  
- Lending: borrow **YES_ETH** against **YES_USDC**

#### NO-Economy

- Assets: **NO_USDC**, **NO_ETH**  
- AMM: **NO_ETH / NO_USDC**  
- Lending: borrow **NO_ETH** against **NO_USDC**

## Price

“**Price**” is distilled collective intelligence of all of humanity - a live forecast of future value.  

Binary markets allow us to use the concept of price, to extract valuable information from the future.

Each possible answer spawns its own self-contained economy. The price feeds inside these separate “verses” reveal what every asset is worth under that specific outcome.

Example:

- **Market**: *Will FOCIL be included in the Glamsterdam hardfork?*  
- **YES_ETH / YES_USDC** quotes ETH’s expected value *if the fix happens*.  
- **NO_ETH / NO_USDC** quotes ETH’s value *if it doesn’t*.

Comparing the two feeds gives the Ethereum community a clear, quantitative signal about which path the future favors—and where to focus effort today.

---

## Deployment

Binary is deployed on **Tempo** - a Layer 1 blockchain optimized for payments with native TIP-20 token support.

### Network: Tempo Testnet (Andantino)

| Property | Value |
|----------|-------|
| Chain ID | `42429` |
| RPC URL | `https://rpc.testnet.tempo.xyz` |
| Explorer | [explore.tempo.xyz](https://explore.tempo.xyz) |

### Prerequisites

1. **Get testnet funds**: Request PATH_USD from the [Tempo Faucet](https://docs.tempo.xyz/quickstart/faucet)
2. **Set private key**: Add `PRIVATE_KEY=...` to `.env`

### Deploy

```bash
./script/DeployTempo.sh
```

Or manually:

```bash
forge script script/DeployAll.s.sol:DeployAll \
    --rpc-url tempo_testnet \
    --broadcast \
    --private-key $PRIVATE_KEY
```

### Contract Addresses

The contracts use Tempo's predeployed system contracts:

| Contract | Address |
|----------|---------|
| TIP-20 Factory | `0x20Fc000000000000000000000000000000000000` |
| PATH_USD | `0x20C0000000000000000000000000000000000000` |

### Tempo-Specific Notes

- **No native gas token** - Fees are paid in TIP-20 tokens (defaults to PATH_USD)
- **Fast finality** - ~0.5 second blocks with deterministic finality
- **TIP-20 tokens** - Binary creates YES/NO verse tokens via the TIP-20 Factory

---

## Local Development

Run a local Tempo node for testing without deploying to testnet.

### Option 1: Using Prool (Recommended)

```bash
cd frontend
bun run tempo:local
```

This starts a Docker-based Tempo node, funds a test account, and deploys contracts.

### Option 2: Using Docker Compose

```bash
# Start local Tempo node
docker-compose up -d

# Deploy contracts
./script/deploy-local.sh

# Stop when done
docker-compose down
```

### Local Node Details

| Property | Value |
|----------|-------|
| RPC URL | `http://localhost:9545` |
| Chain ID | `42429` |
| Test Private Key | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| Test Account | `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` |