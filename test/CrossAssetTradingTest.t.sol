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

contract CrossAssetTradingTest is Test {
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
    
    function testCrossAssetVerseTradingOnUniswapV2() public {
        TestVars memory vars;
        
        // 1. Setup market
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        vars.marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // 2. Create verses for both WETH and USDC
        (vars.yesWeth, vars.noWeth) = multiVerse.create(address(weth), vars.marketHash);
        (vars.yesUsdc, vars.noUsdc) = multiVerse.create(address(usdc), vars.marketHash);
        
        // 3. Setup liquidity
        _setupLiquidity(vars.marketHash, vars.yesWeth, vars.yesUsdc);
        
        // 4. Create and setup UniswapV2 pair
        vars.yesPair = _createAndSetupPair(vars.yesWeth, vars.yesUsdc);
        
        // 5. Alice splits and trades
        _aliceSplitsAndTrades(vars);
        
        // 6. Resolve and redeem
        _resolveAndRedeem(vars.marketHash, vars.yesUsdc);
    }
    
    function _setupLiquidity(bytes32 marketHash, address yesWeth, address yesUsdc) internal {
        vm.startPrank(lpProvider);
        multiVerse.split(address(weth), 100 ether, marketHash);
        multiVerse.split(address(usdc), 500_000e6, marketHash);
        vm.stopPrank();
    }
    
    function _createAndSetupPair(address yesWeth, address yesUsdc) internal returns (address) {
        address yesPair = UNISWAP_V2_FACTORY.createPair(yesWeth, yesUsdc);
        require(yesPair != address(0), "Pair creation failed");
        
        vm.startPrank(lpProvider);
        Verse(yesWeth).transfer(yesPair, 100 ether);
        Verse(yesUsdc).transfer(yesPair, 500_000e6);
        IUniswapV2Pair(yesPair).mint(lpProvider);
        vm.stopPrank();
        
        return yesPair;
    }
    
    function _aliceSplitsAndTrades(TestVars memory vars) internal {
        // Alice splits WETH
        vm.prank(alice);
        multiVerse.split(address(weth), 10 ether, vars.marketHash);
        
        vars.aliceYesWethBefore = Verse(vars.yesWeth).balanceOf(alice);
        vars.aliceYesUsdcBefore = Verse(vars.yesUsdc).balanceOf(alice);
        
        // Execute swap
        _executeSwap(vars.yesPair, vars.yesWeth, vars.yesUsdc);
        
        // Verify swap results
        assertEq(Verse(vars.yesWeth).balanceOf(alice), vars.aliceYesWethBefore - 1 ether);
        assertGt(Verse(vars.yesUsdc).balanceOf(alice), vars.aliceYesUsdcBefore);
    }
    
    function _executeSwap(address yesPair, address yesWeth, address yesUsdc) internal {
        (uint112 reserve0, uint112 reserve1,) = IUniswapV2Pair(yesPair).getReserves();
        address token0 = IUniswapV2Pair(yesPair).token0();
        
        (uint256 reserveWeth, uint256 reserveUsdc) = token0 == yesWeth 
            ? (uint256(reserve0), uint256(reserve1)) 
            : (uint256(reserve1), uint256(reserve0));
        
        uint256 amountIn = 1 ether;
        uint256 amountOut = getAmountOut(amountIn, reserveWeth, reserveUsdc);
        
        vm.startPrank(alice);
        Verse(yesWeth).transfer(yesPair, amountIn);
        
        if (token0 == yesWeth) {
            IUniswapV2Pair(yesPair).swap(0, amountOut, alice, "");
        } else {
            IUniswapV2Pair(yesPair).swap(amountOut, 0, alice, "");
        }
        vm.stopPrank();
    }
    
    function _resolveAndRedeem(bytes32 marketHash, address yesUsdc) internal {
        oracle.setResolution(marketHash, true);
        multiVerse.resolve(marketHash);
        
        uint256 aliceUsdcBefore = usdc.balanceOf(alice);
        uint256 yesUsdcBalance = Verse(yesUsdc).balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(yesUsdc).approve(address(multiVerse), yesUsdcBalance);
        multiVerse.redeem(yesUsdc, yesUsdcBalance);
        vm.stopPrank();
        
        assertEq(usdc.balanceOf(alice), aliceUsdcBefore + yesUsdcBalance);
    }
    
    function testMultipleVersePairsOnUniswap() public {
        // Setup market
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Create verses
        (address yesWeth, address noWeth) = multiVerse.create(address(weth), marketHash);
        (address yesUsdc, address noUsdc) = multiVerse.create(address(usdc), marketHash);
        
        // Create multiple pairs: YES/NO for WETH, YES/NO for USDC, and cross pairs
        address yesNoWethPair = UNISWAP_V2_FACTORY.createPair(yesWeth, noWeth);
        address yesNoUsdcPair = UNISWAP_V2_FACTORY.createPair(yesUsdc, noUsdc);
        address yesWethYesUsdcPair = UNISWAP_V2_FACTORY.createPair(yesWeth, yesUsdc);
        
        // Verify all pairs were created
        assertTrue(yesNoWethPair != address(0), "YES/NO WETH pair should exist");
        assertTrue(yesNoUsdcPair != address(0), "YES/NO USDC pair should exist");
        assertTrue(yesWethYesUsdcPair != address(0), "YES_WETH/YES_USDC pair should exist");
        
        // This demonstrates that verse tokens can create a rich ecosystem of trading pairs
        // allowing for various trading strategies and market making opportunities
    }
    
    // UniswapV2 library function for calculating output amounts
    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) 
        internal 
        pure 
        returns (uint256 amountOut) 
    {
        require(amountIn > 0, "INSUFFICIENT_INPUT_AMOUNT");
        require(reserveIn > 0 && reserveOut > 0, "INSUFFICIENT_LIQUIDITY");
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn * 1000 + amountInWithFee;
        amountOut = numerator / denominator;
    }
}