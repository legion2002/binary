// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {MultiVerse} from "../../src/MultliVerse.sol";
import {Verse} from "../../src/Verse.sol";
import {UMAOracle} from "../../src/oracles/UMAOracle.sol";
import {MockOptimisticOracleV3} from "../oracles/UMAOracleTest.t.sol";
import {MockERC20} from "lib/solady/test/utils/mocks/MockERC20.sol";

/**
 * @title BinaryUMAIntegrationTest
 * @notice Integration test showing full flow of Binary protocol with UMA oracle
 * @dev Demonstrates how Binary markets resolve using UMA's optimistic oracle
 */
contract BinaryUMAIntegrationTest is Test {
    // ============================================
    // CONTRACTS
    // ============================================
    
    MultiVerse public multiVerse;
    UMAOracle public umaOracle;
    MockOptimisticOracleV3 public mockOO;
    
    // Tokens
    MockERC20 public weth;
    MockERC20 public usdc;
    MockERC20 public bondToken;
    
    // Test addresses
    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");
    address public asserter = makeAddr("asserter");
    
    // Market parameters
    bytes32 public constant QUESTION_HASH = keccak256("Will FOCIL be included in Glamsterdam hardfork?");
    uint32 public constant RESOLUTION_DEADLINE = 1798761600; // Jan 1, 2027
    uint256 public constant BOND_AMOUNT = 1000e6; // 1000 USDC
    
    // ============================================
    // SETUP
    // ============================================
    
    function setUp() public {
        // Deploy mock tokens
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        usdc = new MockERC20("USD Coin", "USDC", 6);
        bondToken = usdc; // Use USDC for bonds
        
        // Deploy mock OptimisticOracleV3
        mockOO = new MockOptimisticOracleV3();
        
        // Deploy UMAOracle
        umaOracle = new UMAOracle(
            address(mockOO),
            address(bondToken),
            BOND_AMOUNT
        );
        
        // Deploy MultiVerse
        multiVerse = new MultiVerse();
        
        // Fund users
        weth.mint(alice, 100 ether);
        weth.mint(bob, 100 ether);
        usdc.mint(alice, 100_000e6);
        usdc.mint(bob, 100_000e6);
        bondToken.mint(asserter, 10_000e6);
        
        // Approve multiverse
        vm.prank(alice);
        weth.approve(address(multiVerse), type(uint256).max);
        vm.prank(alice);
        usdc.approve(address(multiVerse), type(uint256).max);
        
        vm.prank(bob);
        weth.approve(address(multiVerse), type(uint256).max);
        vm.prank(bob);
        usdc.approve(address(multiVerse), type(uint256).max);
        
        // Approve bond for asserter
        vm.prank(asserter);
        bondToken.approve(address(umaOracle), type(uint256).max);
    }
    
    // ============================================
    // INTEGRATION TEST
    // ============================================
    
    /**
     * @notice Tests complete Binary + UMA flow
     * @dev Steps:
     *      1. Create Binary market with UMA oracle
     *      2. Users split tokens and trade
     *      3. Market resolution via UMA assertion
     *      4. Settlement and redemption
     */
    function testFullBinaryUMAFlow() public {
        // Step 1: Open market with UMA oracle
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(umaOracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(umaOracle)));
        
        // Step 2: Create verse tokens for WETH
        (address yesWeth, address noWeth) = multiVerse.create(address(weth), marketHash);
        
        // Step 3: Users split tokens
        vm.prank(alice);
        multiVerse.split(address(weth), 10 ether, marketHash);
        
        vm.prank(bob);
        multiVerse.split(address(weth), 5 ether, marketHash);
        
        // Verify split worked
        assertEq(Verse(yesWeth).balanceOf(alice), 10 ether);
        assertEq(Verse(noWeth).balanceOf(alice), 10 ether);
        assertEq(Verse(yesWeth).balanceOf(bob), 5 ether);
        assertEq(Verse(noWeth).balanceOf(bob), 5 ether);
        
        // Step 4: Simulate trading (Alice thinks YES, Bob thinks NO)
        vm.prank(alice);
        Verse(noWeth).transfer(bob, 10 ether); // Alice sells her NO tokens
        
        vm.prank(bob);
        Verse(yesWeth).transfer(alice, 5 ether); // Bob sells his YES tokens
        
        // Final positions:
        // Alice: 15 YES_WETH, 0 NO_WETH (bullish on FOCIL)
        // Bob: 0 YES_WETH, 15 NO_WETH (bearish on FOCIL)
        
        // Step 5: Assert market outcome via UMA
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(marketHash, true); // Assert FOCIL will be included
        
        // Step 6: Wait for UMA liveness period
        vm.warp(block.timestamp + umaOracle.liveness() + 1);
        
        // Step 7: Resolve market in Binary
        multiVerse.resolve(marketHash);
        
        // Step 8: Users redeem winning tokens
        uint256 aliceWethBefore = weth.balanceOf(alice);
        uint256 bobWethBefore = weth.balanceOf(bob);
        
        // Alice redeems her winning YES tokens
        vm.startPrank(alice);
        Verse(yesWeth).approve(address(multiVerse), 15 ether);
        multiVerse.redeem(yesWeth, 15 ether);
        vm.stopPrank();
        
        // Bob tries to redeem losing NO tokens (should revert)
        vm.startPrank(bob);
        Verse(noWeth).approve(address(multiVerse), 15 ether);
        vm.expectRevert(MultiVerse.InvalidResolution.selector);
        multiVerse.redeem(noWeth, 15 ether);
        vm.stopPrank();
        
        // Verify redemptions
        assertEq(weth.balanceOf(alice), aliceWethBefore + 15 ether); // Alice wins
        assertEq(weth.balanceOf(bob), bobWethBefore); // Bob gets nothing
        assertEq(Verse(noWeth).balanceOf(bob), 15 ether); // Bob still holds worthless NO tokens
    }
    
    /**
     * @notice Tests market resolution with NO outcome
     */
    function testBinaryUMAFlowNoOutcome() public {
        // Setup market
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(umaOracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(umaOracle)));
        
        // Create verses and split
        (address yesUsdc, address noUsdc) = multiVerse.create(address(usdc), marketHash);
        
        vm.prank(alice);
        multiVerse.split(address(usdc), 1000e6, marketHash);
        
        // Assert NO outcome
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(marketHash, false); // FOCIL NOT included
        
        // Fast forward and resolve
        vm.warp(block.timestamp + umaOracle.liveness() + 1);
        multiVerse.resolve(marketHash);
        
        // Alice redeems NO tokens (winning)
        uint256 aliceUsdcBefore = usdc.balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(noUsdc).approve(address(multiVerse), 1000e6);
        multiVerse.redeem(noUsdc, 1000e6);
        vm.stopPrank();
        
        assertEq(usdc.balanceOf(alice), aliceUsdcBefore + 1000e6);
    }
    
    /**
     * @notice Tests disputed assertion flow
     */
    function testDisputedAssertionFlow() public {
        // Setup market
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(umaOracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(umaOracle)));
        
        // Make assertion
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(marketHash, true);
        
        // Get assertion ID
        (bytes32 assertionId, , ) = umaOracle.getMarketAssertion(marketHash);
        
        // Dispute the assertion
        address disputer = makeAddr("disputer");
        vm.prank(disputer);
        mockOO.disputeAssertion(assertionId, disputer);
        
        // In real UMA, this would go to DVM for voting
        // For testing, we'll simulate the assertion being found false
        mockOO.setAssertionOutcome(assertionId, false);
        mockOO.setAssertionSettled(assertionId, true);
        
        // Resolution should now fail
        vm.expectRevert(UMAOracle.InvalidAssertion.selector);
        multiVerse.resolve(marketHash);
    }
    
    /**
     * @notice Tests multiple markets resolving with UMA
     */
    function testMultipleMarketsWithUMA() public {
        // Create 3 different markets
        bytes32 q1 = keccak256("Market 1");
        bytes32 q2 = keccak256("Market 2");
        bytes32 q3 = keccak256("Market 3");
        
        multiVerse.open(q1, RESOLUTION_DEADLINE, address(umaOracle));
        multiVerse.open(q2, RESOLUTION_DEADLINE, address(umaOracle));
        multiVerse.open(q3, RESOLUTION_DEADLINE, address(umaOracle));
        
        bytes32 m1 = keccak256(abi.encode(q1, RESOLUTION_DEADLINE, address(umaOracle)));
        bytes32 m2 = keccak256(abi.encode(q2, RESOLUTION_DEADLINE, address(umaOracle)));
        bytes32 m3 = keccak256(abi.encode(q3, RESOLUTION_DEADLINE, address(umaOracle)));
        
        // Fund asserter more
        bondToken.mint(asserter, 10_000e6);
        
        // Assert different outcomes
        vm.startPrank(asserter);
        umaOracle.assertMarketOutcome(m1, true);
        umaOracle.assertMarketOutcome(m2, false);
        umaOracle.assertMarketOutcome(m3, true);
        vm.stopPrank();
        
        // Fast forward and resolve all
        vm.warp(block.timestamp + umaOracle.liveness() + 1);
        
        multiVerse.resolve(m1);
        multiVerse.resolve(m2);
        multiVerse.resolve(m3);
        
        // Verify resolutions
        assertEq(uint(multiVerse.resolutions(m1)), uint(MultiVerse.Resolution.YES));
        assertEq(uint(multiVerse.resolutions(m2)), uint(MultiVerse.Resolution.NO));
        assertEq(uint(multiVerse.resolutions(m3)), uint(MultiVerse.Resolution.YES));
    }
}