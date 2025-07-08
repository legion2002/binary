# Binary

**Price feeds from the future.**

- *What will ETH trade at if “stack-too-deep” is finally fixed in Glamsterdam?*  
- *How will GPU prices diverge if Trump is (or isn’t) re-elected?*

Binary is a lean, highly-optimized prediction-market framework that spawns self-contained economies around every yes/no question.


## Market

A **market** is simply an oracle answering a binary question.

- **Will “stack-too-deep” be fixed in Glamsterdam?**  
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

- **Market**: *Will “stack-too-deep” be fixed in Glamsterdam?*  
- **YES_ETH / YES_USDC** quotes ETH’s expected value *if the fix happens*.  
- **NO_ETH / NO_USDC** quotes ETH’s value *if it doesn’t*.

Comparing the two feeds gives the Ethereum community a clear, quantitative signal about which path the future favors—and where to focus effort today.