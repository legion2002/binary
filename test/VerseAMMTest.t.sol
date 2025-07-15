// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {MultiVerse} from "../src/MultliVerse.sol";
import {Verse} from "../src/Verse.sol";
import {MockOracle} from "./mocks/MockOracle.sol";
import {MockERC20} from "lib/solady/test/utils/mocks/MockERC20.sol";

// UniswapV2 interfaces (compatible with 0.8.x)
interface IUniswapV2Factory {
    function createPair(address tokenA, address tokenB) external returns (address pair);
    function getPair(address tokenA, address tokenB) external view returns (address pair);
}

interface IUniswapV2Pair {
    function mint(address to) external returns (uint256 liquidity);
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external;
    function token0() external view returns (address);
    function token1() external view returns (address);
}

/**
 * @title VerseAMMTest
 * @notice Tests the integration of Binary's Verse tokens with external AMMs (Uniswap V2)
 * @dev Binary creates separate economies ("verses") for each possible outcome of a binary question.
 *      This test demonstrates how these verse tokens can be traded on existing DEXs to create
 *      price discovery mechanisms that reveal the expected value of assets under different outcomes.
 *      
 *      Key concepts from Binary:
 *      1. Markets: Binary questions with oracle resolution (e.g., "Will BTC reach $100k?")
 *      2. Splitting: 1 USDC → 1 YES_USDC + 1 NO_USDC (same for ETH, etc.)
 *      3. Verses: Separate economies for YES and NO outcomes with their own assets
 *      4. Price Discovery: AMM pairs like YES_ETH/YES_USDC reveal ETH's value if YES happens
 *      5. Resolution: When oracle resolves, winning verse tokens redeem 1:1, losing verse tokens → 0
 */
contract VerseAMMTest is Test {
    MultiVerse public multiVerse;
    MockOracle public oracle;
    
    // Mock tokens
    MockERC20 public weth;
    MockERC20 public usdc;
    
    // UniswapV2 contracts on mainnet
    IUniswapV2Factory public constant UNISWAP_V2_FACTORY = IUniswapV2Factory(0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f);
    
    // Test addresses
    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");
    address public lpProvider = makeAddr("lpProvider");
    
    // Market parameters
    bytes32 public constant QUESTION_HASH = keccak256("Will BTC reach $100k in 2024?");
    uint32 public constant RESOLUTION_DEADLINE = 1735689600; // Jan 1, 2025
    
    function setUp() public {
        vm.createSelectFork(vm.envString("ETH_RPC_URL"), 18_000_000);
        
        // Deploy MultiVerse and Oracle
        multiVerse = new MultiVerse();
        oracle = new MockOracle();
        
        // Deploy mock tokens
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        usdc = new MockERC20("USD Coin", "USDC", 6);
        
        // Fund test accounts
        weth.mint(alice, 100 ether);
        weth.mint(bob, 100 ether);
        weth.mint(lpProvider, 1000 ether);
        
        usdc.mint(alice, 100_000e6); // 100k USDC
        usdc.mint(bob, 100_000e6);
        usdc.mint(lpProvider, 5_000_000e6); // 5M USDC
        
        // Approve multiverse for all users
        vm.prank(alice);
        weth.approve(address(multiVerse), type(uint256).max);
        vm.prank(alice);
        usdc.approve(address(multiVerse), type(uint256).max);
        
        vm.prank(bob);
        weth.approve(address(multiVerse), type(uint256).max);
        vm.prank(bob);
        usdc.approve(address(multiVerse), type(uint256).max);
        
        vm.prank(lpProvider);
        weth.approve(address(multiVerse), type(uint256).max);
        vm.prank(lpProvider);
        usdc.approve(address(multiVerse), type(uint256).max);
    }
    
    struct TestVars {
        bytes32 marketHash;
        address yesWeth;
        address noWeth;
        address yesUsdc;
        address noUsdc;
        address yesPair;
        uint256 aliceYesWethBefore;
        uint256 aliceYesUsdcBefore;
    }
    
    /**
     * @notice Tests the complete flow of creating verse tokens and trading them on Uniswap V2
     * @dev This test demonstrates the Binary protocol's core functionality:
     *      1. Opening a prediction market for a binary question
     *      2. Creating verse tokens (YES/NO variants) for multiple assets (WETH, USDC)
     *      3. Using these verse tokens to create AMM pairs that reveal conditional prices
     *      4. Trading between verse tokens to express market views
     *      5. Resolving the market and redeeming winning tokens
     */
    function testCrossAssetVerseTradingOnUniswapV2() public {
        TestVars memory vars;
        
        // Step 1: Open a prediction market
        // This creates a binary market asking "Will BTC reach $100k in 2024?"
        // The market will be resolved by the oracle at the deadline
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        vars.marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Step 2: Create verse tokens for both WETH and USDC
        // This deploys 4 new ERC20 tokens:
        // - YES_WETH: Worth 1 WETH if BTC reaches $100k, 0 otherwise
        // - NO_WETH: Worth 1 WETH if BTC doesn't reach $100k, 0 otherwise
        // - YES_USDC: Worth 1 USDC if BTC reaches $100k, 0 otherwise
        // - NO_USDC: Worth 1 USDC if BTC doesn't reach $100k, 0 otherwise
        (vars.yesWeth, vars.noWeth) = multiVerse.create(address(weth), vars.marketHash);
        (vars.yesUsdc, vars.noUsdc) = multiVerse.create(address(usdc), vars.marketHash);
        
        // Step 3: Setup initial liquidity by splitting tokens
        // LP provider splits 100 WETH → 100 YES_WETH + 100 NO_WETH
        // LP provider splits 500k USDC → 500k YES_USDC + 500k NO_USDC
        _setupLiquidity(vars.marketHash, vars.yesWeth, vars.yesUsdc);
        
        // Step 4: Create Uniswap V2 pair for YES_WETH/YES_USDC
        // This AMM pair will discover the price of ETH in the "YES" universe
        // If traders think ETH will be worth more if BTC hits $100k, YES_WETH will trade at a premium
        vars.yesPair = _createAndSetupPair(vars.yesWeth, vars.yesUsdc);
        
        // Step 5: Alice participates in the market
        // She splits WETH and trades YES_WETH for YES_USDC, expressing a view on relative values
        _aliceSplitsAndTrades(vars);
        
        // Step 6: Market resolves to YES (BTC did reach $100k)
        // Alice redeems her YES_USDC for real USDC at 1:1 ratio
        // All NO tokens become worthless
        _resolveAndRedeem(vars.marketHash, vars.yesUsdc);
    }
    
    /**
     * @notice LP provider splits tokens to create liquidity for the verse economy
     * @dev Splitting is the core mechanism: 1 token → 1 YES_token + 1 NO_token
     *      The LP will provide liquidity to various pairs to enable price discovery
     */
    function _setupLiquidity(bytes32 marketHash, address yesWeth, address yesUsdc) internal {
        vm.startPrank(lpProvider);
        // Split 100 WETH into 100 YES_WETH + 100 NO_WETH
        multiVerse.split(address(weth), 100 ether, marketHash);
        // Split 500k USDC into 500k YES_USDC + 500k NO_USDC
        multiVerse.split(address(usdc), 500_000e6, marketHash);
        vm.stopPrank();
    }
    
    /**
     * @notice Creates and initializes a Uniswap V2 pair for YES verse tokens
     * @dev This pair (YES_WETH/YES_USDC) discovers the ETH/USDC price in the "YES" universe
     *      The initial ratio of 100 ETH : 500k USDC implies ETH = $5000 in the YES outcome
     * @return yesPair Address of the created Uniswap V2 pair
     */
    function _createAndSetupPair(address yesWeth, address yesUsdc) internal returns (address) {
        // Create the pair on Uniswap V2 factory (mainnet fork)
        address yesPair = UNISWAP_V2_FACTORY.createPair(yesWeth, yesUsdc);
        require(yesPair != address(0), "Pair creation failed");
        
        vm.startPrank(lpProvider);
        // Add liquidity: 100 YES_WETH + 500k YES_USDC
        // This sets initial price of YES_WETH at 5000 YES_USDC
        Verse(yesWeth).transfer(yesPair, 100 ether);
        Verse(yesUsdc).transfer(yesPair, 500_000e6);
        // Mint LP tokens to the provider
        IUniswapV2Pair(yesPair).mint(lpProvider);
        vm.stopPrank();
        
        return yesPair;
    }
    
    /**
     * @notice Alice enters the market by splitting tokens and trading on the AMM
     * @dev This demonstrates a user flow:
     *      1. Split regular tokens into verse tokens
     *      2. Trade between different verse tokens based on market views
     *      3. The AMM provides price discovery for conditional asset values
     */
    function _aliceSplitsAndTrades(TestVars memory vars) internal {
        // Alice splits 10 WETH into 10 YES_WETH + 10 NO_WETH
        // She's now exposed to both outcomes
        vm.prank(alice);
        multiVerse.split(address(weth), 10 ether, vars.marketHash);
        
        // Record balances before trade
        vars.aliceYesWethBefore = Verse(vars.yesWeth).balanceOf(alice);
        vars.aliceYesUsdcBefore = Verse(vars.yesUsdc).balanceOf(alice);
        
        // Alice trades 1 YES_WETH for YES_USDC
        // This could represent a view that ETH is overvalued in the YES scenario
        // Or simply a desire to lock in some USDC exposure in that outcome
        _executeSwap(vars.yesPair, vars.yesWeth, vars.yesUsdc);
        
        // Verify the trade executed correctly
        assertEq(Verse(vars.yesWeth).balanceOf(alice), vars.aliceYesWethBefore - 1 ether);
        assertGt(Verse(vars.yesUsdc).balanceOf(alice), vars.aliceYesUsdcBefore);
    }
    
    /**
     * @notice Executes a swap of YES_WETH for YES_USDC on Uniswap V2
     * @dev This shows how verse tokens can be traded on existing DEX infrastructure
     *      The swap follows standard Uniswap V2 mechanics with 0.3% fee
     */
    function _executeSwap(address yesPair, address yesWeth, address yesUsdc) internal {
        // Get current reserves to calculate swap amounts
        (uint112 reserve0, uint112 reserve1,) = IUniswapV2Pair(yesPair).getReserves();
        address token0 = IUniswapV2Pair(yesPair).token0();
        
        // Determine which reserve corresponds to which token
        (uint256 reserveWeth, uint256 reserveUsdc) = token0 == yesWeth 
            ? (uint256(reserve0), uint256(reserve1)) 
            : (uint256(reserve1), uint256(reserve0));
        
        // Alice wants to swap 1 YES_WETH
        uint256 amountIn = 1 ether;
        // Calculate expected output using Uniswap V2 formula (includes 0.3% fee)
        uint256 amountOut = getAmountOut(amountIn, reserveWeth, reserveUsdc);
        
        vm.startPrank(alice);
        // Transfer YES_WETH to the pair
        Verse(yesWeth).transfer(yesPair, amountIn);
        
        // Execute the swap based on token ordering
        if (token0 == yesWeth) {
            IUniswapV2Pair(yesPair).swap(0, amountOut, alice, "");
        } else {
            IUniswapV2Pair(yesPair).swap(amountOut, 0, alice, "");
        }
        vm.stopPrank();
    }
    
    /**
     * @notice Resolves the market and allows redemption of winning tokens
     * @dev When oracle confirms BTC reached $100k (resolution = true):
     *      - All YES tokens can be redeemed 1:1 for underlying assets
     *      - All NO tokens become worthless (cannot be redeemed)
     *      This creates the payoff structure of a binary option
     */
    function _resolveAndRedeem(bytes32 marketHash, address yesUsdc) internal {
        // Oracle resolves: YES, BTC did reach $100k!
        oracle.setResolution(marketHash, true);
        // MultiVerse processes the resolution
        multiVerse.resolve(marketHash);
        
        // Track Alice's USDC balance before redemption
        uint256 aliceUsdcBefore = usdc.balanceOf(alice);
        uint256 yesUsdcBalance = Verse(yesUsdc).balanceOf(alice);
        
        // Alice redeems her YES_USDC for real USDC
        vm.startPrank(alice);
        Verse(yesUsdc).approve(address(multiVerse), yesUsdcBalance);
        multiVerse.redeem(yesUsdc, yesUsdcBalance);
        vm.stopPrank();
        
        // Verify Alice received exactly 1:1 redemption
        assertEq(usdc.balanceOf(alice), aliceUsdcBefore + yesUsdcBalance);
    }
    
    /**
     * @notice Demonstrates the rich ecosystem of trading pairs possible with verse tokens
     * @dev Binary's verse tokens enable multiple types of price discovery:
     *      1. YES/NO pairs: Direct probability trading (e.g., YES_WETH/NO_WETH)
     *      2. Cross-asset pairs within verses: Conditional asset pricing (e.g., YES_WETH/YES_USDC)
     *      3. Cross-verse pairs: Complex conditional strategies
     *      
     *      This creates a complete economy for each outcome where:
     *      - Traders can express nuanced views on conditional asset values
     *      - Market makers can provide liquidity and earn fees
     *      - Arbitrageurs keep prices efficient across pairs
     */
    function testMultipleVersePairsOnUniswap() public {
        // Setup market for the binary question
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Create verse tokens for both assets
        (address yesWeth, address noWeth) = multiVerse.create(address(weth), marketHash);
        (address yesUsdc, address noUsdc) = multiVerse.create(address(usdc), marketHash);
        
        // Create multiple trading pairs to enable different strategies:
        
        // 1. YES_WETH/NO_WETH: Direct probability trading
        //    If this trades at 60/40, market thinks 60% chance of YES
        address yesNoWethPair = UNISWAP_V2_FACTORY.createPair(yesWeth, noWeth);
        
        // 2. YES_USDC/NO_USDC: Probability trading in USDC terms
        //    Should trade at similar ratio to WETH pair (arbitrage opportunity otherwise)
        address yesNoUsdcPair = UNISWAP_V2_FACTORY.createPair(yesUsdc, noUsdc);
        
        // 3. YES_WETH/YES_USDC: ETH price in the YES universe
        //    Reveals market's view of ETH value conditional on YES outcome
        address yesWethYesUsdcPair = UNISWAP_V2_FACTORY.createPair(yesWeth, yesUsdc);
        
        // Verify all pairs were successfully created
        assertTrue(yesNoWethPair != address(0), "YES/NO WETH pair should exist");
        assertTrue(yesNoUsdcPair != address(0), "YES/NO USDC pair should exist");
        assertTrue(yesWethYesUsdcPair != address(0), "YES_WETH/YES_USDC pair should exist");
        
        // Additional pairs could include:
        // - NO_WETH/NO_USDC: ETH price in the NO universe
        // - YES_WETH/NO_USDC: Cross-verse arbitrage plays
        // This demonstrates Binary's vision of "price feeds from the future"
    }
    
    /**
     * @notice Calculates output amount for a Uniswap V2 swap
     * @dev Standard Uniswap V2 formula: xy=k with 0.3% fee
     *      This is used to calculate how much YES_USDC Alice receives for her YES_WETH
     * @param amountIn Amount of input token
     * @param reserveIn Reserve of input token in the pair
     * @param reserveOut Reserve of output token in the pair
     * @return amountOut Amount of output token to be received
     */
    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) 
        internal 
        pure 
        returns (uint256 amountOut) 
    {
        require(amountIn > 0, "INSUFFICIENT_INPUT_AMOUNT");
        require(reserveIn > 0 && reserveOut > 0, "INSUFFICIENT_LIQUIDITY");
        // Apply 0.3% fee (997/1000)
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn * 1000 + amountInWithFee;
        amountOut = numerator / denominator;
    }
}