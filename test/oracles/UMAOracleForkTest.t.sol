// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {UMAOracle} from "../../src/oracles/UMAOracle.sol";
import {IOptimisticOracleV3} from "../../src/interfaces/IOptimisticOracleV3.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

/**
 * @title UMAOracleForkTest
 * @notice Fork tests for UMAOracle using real UMA contracts on mainnet
 * @dev Run with: forge test --match-contract UMAOracleForkTest --fork-url $ETH_RPC_URL
 */
contract UMAOracleForkTest is Test {
    // ============================================
    // CONSTANTS
    // ============================================
    
    // UMA mainnet addresses
    address constant OPTIMISTIC_ORACLE_V3 = 0xfb55F43fB9F48F63f9269DB7Dde3BbBe1ebDC0dE;
    address constant USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
    address constant FINDER = 0x40f941E48A552bF496B154Af6bf55725f18D77c3;
    
    // Test addresses
    address constant USDC_WHALE = 0x40ec5B33f54e0E8A33A975908C5BA1c14e5BbbDf; // Polygon bridge
    
    UMAOracle public umaOracle;
    
    address public owner = makeAddr("owner");
    address public asserter = makeAddr("asserter");
    
    uint256 public constant BOND_AMOUNT = 1000e6; // 1000 USDC
    bytes32 public constant MARKET_HASH = keccak256("Will FOCIL be included in Glamsterdam?");
    
    function setUp() public {
        // Fork mainnet at recent block
        vm.createSelectFork(vm.envString("ETH_RPC_URL"));
        
        // Deploy UMAOracle
        vm.prank(owner);
        umaOracle = new UMAOracle(
            OPTIMISTIC_ORACLE_V3,
            USDC,
            BOND_AMOUNT
        );
        
        // Use deal to fund asserter with USDC instead of transfer
        deal(USDC, asserter, 10_000e6); // 10k USDC
        
        // Approve UMAOracle to spend asserter's USDC
        vm.prank(asserter);
        IERC20(USDC).approve(address(umaOracle), type(uint256).max);
    }
    
    function testForkUMAOracleDeployment() public view {
        assertEq(address(umaOracle.optimisticOracle()), OPTIMISTIC_ORACLE_V3);
        assertEq(address(umaOracle.bondCurrency()), USDC);
        assertEq(umaOracle.bondAmount(), BOND_AMOUNT);
        assertEq(umaOracle.liveness(), 7200); // 2 hours
    }
    
    function testForkAssertMarketOutcome() public {
        // Record balances before
        uint256 asserterBalanceBefore = IERC20(USDC).balanceOf(asserter);
        uint256 oracleBalanceBefore = IERC20(USDC).balanceOf(OPTIMISTIC_ORACLE_V3);
        
        // Assert market outcome
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
        
        // Verify assertion was made
        assertTrue(umaOracle.hasAssertion(MARKET_HASH));
        
        // Verify bond was transferred
        assertEq(
            IERC20(USDC).balanceOf(asserter),
            asserterBalanceBefore - BOND_AMOUNT
        );
        assertGe(
            IERC20(USDC).balanceOf(OPTIMISTIC_ORACLE_V3),
            oracleBalanceBefore + BOND_AMOUNT
        );
        
        // Get assertion details
        (bytes32 assertionId, IOptimisticOracleV3.Assertion memory assertion, bool outcome) = 
            umaOracle.getMarketAssertion(MARKET_HASH);
        
        // Verify assertion data
        assertTrue(assertionId != bytes32(0));
        assertEq(assertion.asserter, asserter);
        assertEq(assertion.expirationTime, block.timestamp + 7200);
        assertFalse(assertion.settled);
        assertTrue(outcome);
    }
    
    function testForkSettlementFlow() public {
        // Make assertion
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, false);
        
        // Get assertion ID
        (bytes32 assertionId, , ) = umaOracle.getMarketAssertion(MARKET_HASH);
        
        // Fast forward past liveness period
        vm.warp(block.timestamp + 7201);
        
        // Get resolution (should trigger settlement)
        bool isYes = umaOracle.getResolution(MARKET_HASH);
        assertFalse(isYes);
        
        // Verify assertion is now settled
        IOptimisticOracleV3.Assertion memory assertion = 
            IOptimisticOracleV3(OPTIMISTIC_ORACLE_V3).getAssertion(assertionId);
        assertTrue(assertion.settled);
        assertTrue(assertion.settlementResolution);
    }
    
    function testForkMinimumBond() public view {
        // Check minimum bond for USDC
        uint256 minimumBond = IOptimisticOracleV3(OPTIMISTIC_ORACLE_V3).getMinimumBond(USDC);
        
        // UMA typically requires at least some minimum bond
        assertGt(minimumBond, 0);
        
        // Our configured bond should be at least the minimum
        assertGe(BOND_AMOUNT, minimumBond);
    }
    
    function testForkMultipleMarketsWithDifferentOutcomes() public {
        bytes32 market1 = keccak256("Market 1");
        bytes32 market2 = keccak256("Market 2");
        bytes32 market3 = keccak256("Market 3");
        
        // Fund asserter with more USDC
        deal(USDC, asserter, 10_000e6);
        
        // Assert different outcomes for each market
        vm.startPrank(asserter);
        umaOracle.assertMarketOutcome(market1, true);
        umaOracle.assertMarketOutcome(market2, false);
        umaOracle.assertMarketOutcome(market3, true);
        vm.stopPrank();
        
        // Verify all assertions
        assertTrue(umaOracle.hasAssertion(market1));
        assertTrue(umaOracle.hasAssertion(market2));
        assertTrue(umaOracle.hasAssertion(market3));
        
        // Check outcomes
        (, , bool outcome1) = umaOracle.getMarketAssertion(market1);
        (, , bool outcome2) = umaOracle.getMarketAssertion(market2);
        (, , bool outcome3) = umaOracle.getMarketAssertion(market3);
        
        assertTrue(outcome1);
        assertFalse(outcome2);
        assertTrue(outcome3);
    }
    
    function testForkRealWorldScenario() public {
        // Simulate a real prediction market scenario
        bytes32 realMarket = keccak256(
            "Will Ethereum successfully implement EIP-4844 by March 2024?"
        );
        
        // Make assertion (YES - it was implemented)
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(realMarket, true);
        
        // In real scenario, wait for liveness period
        // For testing, we fast forward
        vm.warp(block.timestamp + 7201);
        
        // Anyone can call getResolution to settle
        bool result = umaOracle.getResolution(realMarket);
        assertTrue(result);
        
        // This resolution can now be used by Binary's MultiVerse
        // to determine payouts for YES/NO tokens
    }
}