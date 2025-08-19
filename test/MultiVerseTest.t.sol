// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {MultiVerse} from "../src/MultliVerse.sol";
import {Verse} from "../src/Verse.sol";
import {TrustedOracle} from "../src/TrustedOracle.sol";
import {MockERC20} from "lib/solady/test/utils/mocks/MockERC20.sol";

contract MultiVerseTest is Test {
    MultiVerse public multiVerse;
    TrustedOracle public oracle;
    MockERC20 public asset;

    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");
    address public charlie = makeAddr("charlie");

    bytes32 public constant QUESTION_HASH = keccak256("Will ETH reach $10k in 2024?");
    uint32 public constant RESOLUTION_DEADLINE = 1735689600; // Jan 1, 2025

    event Transfer(address indexed from, address indexed to, uint256 value);

    function setUp() public {
        multiVerse = new MultiVerse();
        oracle = new TrustedOracle();
        asset = new MockERC20("Test Token", "TEST", 18);

        // Fund test accounts
        asset.mint(alice, 1000e18);
        asset.mint(bob, 1000e18);
        asset.mint(charlie, 1000e18);

        // Approve multiVerse to spend tokens
        vm.prank(alice);
        asset.approve(address(multiVerse), type(uint256).max);
        vm.prank(bob);
        asset.approve(address(multiVerse), type(uint256).max);
        vm.prank(charlie);
        asset.approve(address(multiVerse), type(uint256).max);
    }

    function testOpenMarket() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        (uint32 deadline, address oracleAddr, bytes32 qHash) = multiVerse.markets(marketHash);
        assertEq(deadline, RESOLUTION_DEADLINE);
        assertEq(oracleAddr, address(oracle));
        assertEq(qHash, QUESTION_HASH);
        assertEq(uint8(multiVerse.resolutions(marketHash)), uint8(MultiVerse.Resolution.UNRESOLVED));
    }

    function testCannotOpenMarketTwice() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        
        vm.expectRevert(MultiVerse.MarketAlreadyOpened.selector);
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
    }

    function testCreateVerses() public {
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        assertTrue(yesVerse != address(0));
        assertTrue(noVerse != address(0));
        assertTrue(yesVerse != noVerse);
        
        // Check verse properties
        assertEq(Verse(yesVerse).name(), "Yes");
        assertEq(Verse(yesVerse).symbol(), "YES");
        assertEq(Verse(yesVerse).ASSET(), address(asset));
        assertEq(Verse(yesVerse).MARKET_HASH(), marketHash);
        assertEq(Verse(yesVerse).owner(), address(multiVerse));
        
        assertEq(Verse(noVerse).name(), "No");
        assertEq(Verse(noVerse).symbol(), "NO");
        assertEq(Verse(noVerse).ASSET(), address(asset));
        assertEq(Verse(noVerse).MARKET_HASH(), marketHash);
        assertEq(Verse(noVerse).owner(), address(multiVerse));
    }

    function testGetVerseAddress() public {
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        (address expectedYes, address expectedNo) = multiVerse.getVerseAddress(address(asset), marketHash);
        (address actualYes, address actualNo) = multiVerse.create(address(asset), marketHash);
        
        assertEq(expectedYes, actualYes);
        assertEq(expectedNo, actualNo);
    }

    function testYesAndNoVersesHaveDifferentAddresses() public {
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        assertTrue(yesVerse != address(0), "YES verse should not be zero address");
        assertTrue(noVerse != address(0), "NO verse should not be zero address");
        assertTrue(yesVerse != noVerse, "YES and NO verses must have different addresses");
        
        // Additional verification using getVerseAddress
        (address expectedYes, address expectedNo) = multiVerse.getVerseAddress(address(asset), marketHash);
        assertTrue(expectedYes != expectedNo, "Expected YES and NO verses should have different addresses");
        
        // Verify the addresses are deterministic and match
        assertEq(yesVerse, expectedYes, "YES verse address should match expected");
        assertEq(noVerse, expectedNo, "NO verse address should match expected");
    }

    function testCannotCreateVersesWhenTheyAlreadyExist() public {
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // First creation should succeed
        (address yesVerse1, address noVerse1) = multiVerse.create(address(asset), marketHash);
        assertTrue(yesVerse1 != address(0), "First YES verse creation should succeed");
        assertTrue(noVerse1 != address(0), "First NO verse creation should succeed");
        
        // Verify verses were deployed with correct code
        assertTrue(yesVerse1.code.length > 0, "YES verse should have code");
        assertTrue(noVerse1.code.length > 0, "NO verse should have code");
        
        // Second creation attempt should revert because verses already exist at those addresses
        // CREATE2 will fail with CreateCollision when trying to deploy to an address with existing code
        // The EVM reverts without specific error data when CREATE2 encounters an existing contract
        vm.expectRevert();
        multiVerse.create(address(asset), marketHash);
    }

    function testSplit() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Create verses first
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        uint256 aliceBalanceBefore = asset.balanceOf(alice);
        uint256 splitAmount = 100e18;
        
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        // Check balances
        assertEq(asset.balanceOf(alice), aliceBalanceBefore - splitAmount);
        assertEq(Verse(yesVerse).balanceOf(alice), splitAmount);
        assertEq(Verse(noVerse).balanceOf(alice), splitAmount);
    }

    function testCannotSplitAfterResolution() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        multiVerse.create(address(asset), marketHash);
        
        // Resolve the market
        oracle.setResolution(marketHash, true);
        multiVerse.resolve(marketHash);
        
        // Try to split after resolution
        vm.prank(alice);
        vm.expectRevert(MultiVerse.InvalidMarketState.selector);
        multiVerse.split(address(asset), 100e18, marketHash);
    }

    function testCannotSplitUnopennedMarket() public {
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Try to split on unopened market
        vm.prank(alice);
        vm.expectRevert(MultiVerse.InvalidMarketState.selector);
        multiVerse.split(address(asset), 100e18, marketHash);
    }

    function testCombine() public {
        // Setup market and split tokens
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        uint256 splitAmount = 100e18;
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        // Check initial balances
        assertEq(Verse(yesVerse).balanceOf(alice), splitAmount);
        assertEq(Verse(noVerse).balanceOf(alice), splitAmount);
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        
        // Approve multiverse to spend verse tokens
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), splitAmount);
        Verse(noVerse).approve(address(multiVerse), splitAmount);
        
        // Combine tokens back to asset
        multiVerse.combine(address(asset), splitAmount, marketHash);
        vm.stopPrank();
        
        // Check final balances
        assertEq(Verse(yesVerse).balanceOf(alice), 0);
        assertEq(Verse(noVerse).balanceOf(alice), 0);
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + splitAmount);
    }

    function testCombinePartialAmount() public {
        // Setup market and split tokens
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        uint256 splitAmount = 100e18;
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        // Combine only half
        uint256 combineAmount = 50e18;
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), combineAmount);
        Verse(noVerse).approve(address(multiVerse), combineAmount);
        
        multiVerse.combine(address(asset), combineAmount, marketHash);
        vm.stopPrank();
        
        // Check balances
        assertEq(Verse(yesVerse).balanceOf(alice), splitAmount - combineAmount);
        assertEq(Verse(noVerse).balanceOf(alice), splitAmount - combineAmount);
    }


    function testCannotCombineAfterResolution() public {
        // Setup and resolve market
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        vm.prank(alice);
        multiVerse.split(address(asset), 100e18, marketHash);
        
        // Resolve market
        oracle.setResolution(marketHash, true);
        multiVerse.resolve(marketHash);
        
        // Try to combine after resolution
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), 100e18);
        Verse(noVerse).approve(address(multiVerse), 100e18);
        
        vm.expectRevert(MultiVerse.InvalidMarketState.selector);
        multiVerse.combine(address(asset), 100e18, marketHash);
        vm.stopPrank();
    }

    function testCannotCombineUnopennedMarket() public {
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        vm.prank(alice);
        vm.expectRevert(MultiVerse.InvalidMarketState.selector);
        multiVerse.combine(address(asset), 100e18, marketHash);
    }

    function testFuzzCombine(uint256 amount) public {
        amount = bound(amount, 1, 1000e18);
        
        // Setup market and split tokens
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        vm.prank(alice);
        multiVerse.split(address(asset), amount, marketHash);
        
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        
        // Combine tokens
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), amount);
        Verse(noVerse).approve(address(multiVerse), amount);
        
        multiVerse.combine(address(asset), amount, marketHash);
        vm.stopPrank();
        
        // Check balances
        assertEq(Verse(yesVerse).balanceOf(alice), 0);
        assertEq(Verse(noVerse).balanceOf(alice), 0);
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + amount);
    }

    function testResolveMarket() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Set oracle resolution
        oracle.setResolution(marketHash, true); // true = YES
        
        multiVerse.resolve(marketHash);
        
        assertEq(uint8(multiVerse.resolutions(marketHash)), uint8(MultiVerse.Resolution.YES));
    }

    function testResolveAfterDeadline() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Set oracle resolution to YES
        oracle.setResolution(marketHash, true); // true = YES
        
        // Warp past deadline
        vm.warp(RESOLUTION_DEADLINE + 1);
        
        // Should resolve to EVEN despite oracle saying YES
        multiVerse.resolve(marketHash);
        
        assertEq(uint8(multiVerse.resolutions(marketHash)), uint8(MultiVerse.Resolution.EVEN));
    }

    function testCannotResolveTwice() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        oracle.setResolution(marketHash, true); // true = YES
        multiVerse.resolve(marketHash);
        
        vm.expectRevert(MultiVerse.InvalidMarketState.selector);
        multiVerse.resolve(marketHash);
    }

    function testRedeemYesWins() public {
        // Setup
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        // Alice splits tokens
        uint256 splitAmount = 100e18;
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        // Resolve market to YES
        oracle.setResolution(marketHash, true); // true = YES
        multiVerse.resolve(marketHash);
        
        // Alice redeems YES tokens (winning token)
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), splitAmount);
        uint256 redeemed = multiVerse.redeem(yesVerse, splitAmount);
        vm.stopPrank();
        
        assertEq(redeemed, splitAmount, "YES token should redeem for full amount");
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + splitAmount, "Should receive full asset amount");
        assertEq(Verse(yesVerse).balanceOf(alice), 0, "All YES tokens should be burned");
        
        // Try to redeem NO tokens (losing token) - should revert
        vm.startPrank(alice);
        Verse(noVerse).approve(address(multiVerse), splitAmount);
        vm.expectRevert(MultiVerse.InvalidResolution.selector);
        multiVerse.redeem(noVerse, splitAmount);
        vm.stopPrank();
        
        assertEq(Verse(noVerse).balanceOf(alice), splitAmount, "NO tokens should remain unchanged");
    }

    function testRedeemNoWins() public {
        // Setup
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        // Alice splits tokens
        uint256 splitAmount = 100e18;
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        // Resolve market to NO
        oracle.setResolution(marketHash, false); // false = NO
        multiVerse.resolve(marketHash);
        
        // Alice redeems NO tokens (winning token)
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(noVerse).approve(address(multiVerse), splitAmount);
        uint256 redeemed = multiVerse.redeem(noVerse, splitAmount);
        vm.stopPrank();
        
        assertEq(redeemed, splitAmount, "NO token should redeem for full amount");
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + splitAmount, "Should receive full asset amount");
        assertEq(Verse(noVerse).balanceOf(alice), 0, "All NO tokens should be burned");
        
        // Try to redeem YES tokens (losing token) - should revert
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), splitAmount);
        vm.expectRevert(MultiVerse.InvalidResolution.selector);
        multiVerse.redeem(yesVerse, splitAmount);
        vm.stopPrank();
        
        assertEq(Verse(yesVerse).balanceOf(alice), splitAmount, "YES tokens should remain unchanged");
    }

    function testRedeemEven() public {
        // Setup
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        // Alice splits tokens
        uint256 splitAmount = 100e18;
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        // Resolve market to EVEN by waiting past deadline
        vm.warp(RESOLUTION_DEADLINE + 1);
        multiVerse.resolve(marketHash);
        
        // Alice redeems YES tokens (gets half)
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        uint256 yesBalanceBefore = Verse(yesVerse).balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), splitAmount);
        uint256 redeemedYes = multiVerse.redeem(yesVerse, splitAmount);
        vm.stopPrank();
        
        assertEq(redeemedYes, splitAmount / 2, "YES token should redeem for half amount in EVEN resolution");
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + splitAmount / 2, "Should receive half asset amount");
        assertEq(Verse(yesVerse).balanceOf(alice), yesBalanceBefore - splitAmount, "All YES tokens should be burned");
        
        // Alice redeems NO tokens (gets half)
        aliceAssetBalanceBefore = asset.balanceOf(alice);
        uint256 noBalanceBefore = Verse(noVerse).balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(noVerse).approve(address(multiVerse), splitAmount);
        uint256 redeemedNo = multiVerse.redeem(noVerse, splitAmount);
        vm.stopPrank();
        
        assertEq(redeemedNo, splitAmount / 2, "NO token should redeem for half amount in EVEN resolution");
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + splitAmount / 2, "Should receive half asset amount");
        assertEq(Verse(noVerse).balanceOf(alice), noBalanceBefore - splitAmount, "All NO tokens should be burned");
        
        // Total redeemed should equal original split amount
        assertEq(redeemedYes + redeemedNo, splitAmount, "Total redeemed should equal original amount");
    }

    function testRedeemInvalidVerse() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // Create a fake verse contract
        Verse fakeVerse = new Verse("Fake", "FAKE", address(asset), marketHash);
        
        oracle.setResolution(marketHash, true); // true = YES
        multiVerse.resolve(marketHash);
        
        vm.expectRevert(MultiVerse.InvalidVerse.selector);
        multiVerse.redeem(address(fakeVerse), 100e18);
    }

    function testRedeemWithUnresolvedMarket() public {
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, ) = multiVerse.create(address(asset), marketHash);
        
        // Split tokens
        vm.prank(alice);
        multiVerse.split(address(asset), 100e18, marketHash);
        
        // Try to redeem without resolving
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), 100e18);
        vm.expectRevert(MultiVerse.InvalidResolution.selector);
        multiVerse.redeem(yesVerse, 100e18);
        vm.stopPrank();
    }

    function testRedeemLosingToken() public {
        // Setup
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        // Alice splits tokens
        uint256 splitAmount = 100e18;
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        // Resolve market to NO (YES loses)
        oracle.setResolution(marketHash, false); // false = NO
        multiVerse.resolve(marketHash);
        
        // Try to redeem YES tokens (losing token)
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), splitAmount);
        vm.expectRevert(MultiVerse.InvalidResolution.selector);
        multiVerse.redeem(yesVerse, splitAmount);
        vm.stopPrank();
        
        // Verify NO tokens (winning token) can be redeemed for full amount
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        vm.startPrank(alice);
        Verse(noVerse).approve(address(multiVerse), splitAmount);
        uint256 redeemed = multiVerse.redeem(noVerse, splitAmount);
        vm.stopPrank();
        
        assertEq(redeemed, splitAmount, "Should redeem full amount for winning token");
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + splitAmount, "Should receive full asset amount");
    }

    function testFullFlow() public {
        // 1. Open market
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        
        // 2. Create verses
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        // 3. Multiple users split tokens
        vm.prank(alice);
        multiVerse.split(address(asset), 200e18, marketHash);
        
        vm.prank(bob);
        multiVerse.split(address(asset), 300e18, marketHash);
        
        // 4. Users trade (transfer tokens between each other)
        vm.prank(alice);
        Verse(yesVerse).transfer(charlie, 50e18); // Alice sells some YES to Charlie
        
        vm.prank(bob);
        Verse(noVerse).transfer(charlie, 100e18); // Bob sells some NO to Charlie
        
        // Check balances
        assertEq(Verse(yesVerse).balanceOf(alice), 150e18);
        assertEq(Verse(noVerse).balanceOf(alice), 200e18);
        assertEq(Verse(yesVerse).balanceOf(bob), 300e18);
        assertEq(Verse(noVerse).balanceOf(bob), 200e18);
        assertEq(Verse(yesVerse).balanceOf(charlie), 50e18);
        assertEq(Verse(noVerse).balanceOf(charlie), 100e18);
        
        // 5. Resolve market to YES
        oracle.setResolution(marketHash, true); // true = YES
        multiVerse.resolve(marketHash);
        
        // 6. Winners redeem
        // Alice redeems her YES tokens
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), 150e18);
        multiVerse.redeem(yesVerse, 150e18);
        vm.stopPrank();
        
        // Bob redeems his YES tokens
        vm.startPrank(bob);
        Verse(yesVerse).approve(address(multiVerse), 300e18);
        multiVerse.redeem(yesVerse, 300e18);
        vm.stopPrank();
        
        // Charlie redeems his YES tokens
        vm.startPrank(charlie);
        Verse(yesVerse).approve(address(multiVerse), 50e18);
        multiVerse.redeem(yesVerse, 50e18);
        vm.stopPrank();
        
        // Final balances check
        assertEq(asset.balanceOf(alice), 950e18); // Started with 1000, spent 200, got back 150
        assertEq(asset.balanceOf(bob), 1000e18); // Started with 1000, spent 300, got back 300
        assertEq(asset.balanceOf(charlie), 1050e18); // Started with 1000, got 50 from redeeming
    }

    // Fuzz tests
    function testFuzzSplit(uint256 amount) public {
        amount = bound(amount, 1, 1000e18);
        
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        uint256 aliceBalanceBefore = asset.balanceOf(alice);
        
        vm.prank(alice);
        multiVerse.split(address(asset), amount, marketHash);
        
        assertEq(asset.balanceOf(alice), aliceBalanceBefore - amount);
        assertEq(Verse(yesVerse).balanceOf(alice), amount);
        assertEq(Verse(noVerse).balanceOf(alice), amount);
    }

    function testFuzzRedeemYes(uint256 splitAmount, uint256 redeemAmount) public {
        splitAmount = bound(splitAmount, 1, 1000e18);
        redeemAmount = bound(redeemAmount, 1, splitAmount);
        
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, address noVerse) = multiVerse.create(address(asset), marketHash);
        
        vm.prank(alice);
        multiVerse.split(address(asset), splitAmount, marketHash);
        
        oracle.setResolution(marketHash, true); // true = YES
        multiVerse.resolve(marketHash);
        
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), redeemAmount);
        uint256 redeemed = multiVerse.redeem(yesVerse, redeemAmount);
        vm.stopPrank();
        
        assertEq(redeemed, redeemAmount, "Should redeem exact amount requested for winning token");
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + redeemAmount, "Should receive exact asset amount");
        
        // Verify NO tokens cannot be redeemed
        vm.startPrank(alice);
        Verse(noVerse).approve(address(multiVerse), splitAmount);
        vm.expectRevert(MultiVerse.InvalidResolution.selector);
        multiVerse.redeem(noVerse, splitAmount);
        vm.stopPrank();
    }

    function testFuzzRedeemEven(uint256 amount) public {
        amount = bound(amount, 2, 1000e18); // At least 2 to avoid rounding to 0
        
        multiVerse.open(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(QUESTION_HASH, RESOLUTION_DEADLINE, address(oracle)));
        (address yesVerse, ) = multiVerse.create(address(asset), marketHash);
        
        vm.prank(alice);
        multiVerse.split(address(asset), amount, marketHash);
        
        // Test EVEN resolution by warping past deadline
        vm.warp(RESOLUTION_DEADLINE + 1);
        multiVerse.resolve(marketHash);
        
        uint256 aliceAssetBalanceBefore = asset.balanceOf(alice);
        
        vm.startPrank(alice);
        Verse(yesVerse).approve(address(multiVerse), amount);
        uint256 redeemed = multiVerse.redeem(yesVerse, amount);
        vm.stopPrank();
        
        assertEq(redeemed, amount / 2);
        assertEq(asset.balanceOf(alice), aliceAssetBalanceBefore + amount / 2);
    }
}