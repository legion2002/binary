// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {UMAOracle} from "../../src/oracles/UMAOracle.sol";
import {IOptimisticOracleV3} from "../../src/interfaces/IOptimisticOracleV3.sol";
import {MockERC20} from "lib/solady/test/utils/mocks/MockERC20.sol";
import {ERC20} from "solady/tokens/ERC20.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

/**
 * @title MockOptimisticOracleV3
 * @notice Mock implementation of UMA's OptimisticOracleV3 for testing
 */
contract MockOptimisticOracleV3 is IOptimisticOracleV3 {
    uint256 private assertionCounter;
    mapping(bytes32 => Assertion) private assertions;
    mapping(address => uint256) public minimumBonds;
    
    function assertTruth(
        bytes calldata claim,
        address asserter,
        address callbackRecipient,
        address escalationManager,
        uint64 liveness,
        IERC20 currency,
        uint256 bond,
        bytes32 identifier,
        bytes32 domainId
    ) external returns (bytes32 assertionId) {
        // Generate assertion ID
        assertionId = keccak256(abi.encode(assertionCounter++, claim, asserter));
        
        // Store assertion
        assertions[assertionId] = Assertion({
            asserter: asserter,
            assertionTime: uint64(block.timestamp),
            expirationTime: uint64(block.timestamp + liveness),
            settled: false,
            settlementResolution: false,
            disputeCaller: address(0)
        });
        
        // Transfer bond from the calling contract (UMAOracle)
        currency.transferFrom(msg.sender, address(this), bond);
        
        emit AssertionMade(
            assertionId,
            asserter,
            claim,
            bond,
            uint64(block.timestamp + liveness),
            callbackRecipient,
            escalationManager
        );
    }
    
    function getAssertion(bytes32 assertionId) external view returns (Assertion memory) {
        return assertions[assertionId];
    }
    
    function settleAssertion(bytes32 assertionId) external {
        Assertion storage assertion = assertions[assertionId];
        require(!assertion.settled, "Already settled");
        require(block.timestamp >= assertion.expirationTime, "Not expired");
        
        assertion.settled = true;
        assertion.settlementResolution = true; // Default to true if not disputed
        
        emit AssertionSettled(assertionId, assertion.asserter, true);
    }
    
    function disputeAssertion(bytes32 assertionId, address disputer) external {
        Assertion storage assertion = assertions[assertionId];
        require(!assertion.settled, "Already settled");
        require(block.timestamp < assertion.expirationTime, "Expired");
        
        assertion.disputeCaller = disputer;
        emit AssertionDisputed(assertionId, msg.sender, disputer);
    }
    
    function getMinimumBond(address currency) external view returns (uint256) {
        return minimumBonds[currency];
    }
    
    // Test helper functions
    function setAssertionOutcome(bytes32 assertionId, bool outcome) external {
        assertions[assertionId].settlementResolution = outcome;
    }
    
    function setAssertionSettled(bytes32 assertionId, bool settled) external {
        assertions[assertionId].settled = settled;
    }
}

/**
 * @title UMAOracleTest
 * @notice Comprehensive test suite for UMAOracle
 */
contract UMAOracleTest is Test {
    UMAOracle public umaOracle;
    MockOptimisticOracleV3 public mockOO;
    MockERC20 public bondToken;
    
    address public owner = makeAddr("owner");
    address public asserter = makeAddr("asserter");
    address public disputer = makeAddr("disputer");
    
    uint256 public constant BOND_AMOUNT = 100e6; // 100 USDC
    bytes32 public constant MARKET_HASH = keccak256("test-market");
    
    event MarketAsserted(
        bytes32 indexed marketHash,
        bytes32 indexed assertionId,
        bool outcome,
        address asserter
    );
    
    event MarketResolved(
        bytes32 indexed marketHash,
        bool outcome
    );
    
    function setUp() public {
        // Deploy mocks
        mockOO = new MockOptimisticOracleV3();
        bondToken = new MockERC20("USD Coin", "USDC", 6);
        
        // Deploy UMAOracle
        vm.prank(owner);
        umaOracle = new UMAOracle(
            address(mockOO),
            address(bondToken),
            BOND_AMOUNT
        );
        
        // Fund asserter
        bondToken.mint(asserter, 1000e6);
        vm.prank(asserter);
        bondToken.approve(address(umaOracle), type(uint256).max);
        
        // Pre-approve UMAOracle to spend to mockOO (simulating the approval in assertMarketOutcome)
        vm.prank(address(umaOracle));
        bondToken.approve(address(mockOO), type(uint256).max);
    }
    
    // ============================================
    // ASSERTION TESTS
    // ============================================
    
    function testAssertMarketOutcomeYES() public {
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
        
        // Check assertion was recorded
        assertTrue(umaOracle.hasAssertion(MARKET_HASH));
        
        // Check assertion details
        (bytes32 assertionId, , bool outcome) = umaOracle.getMarketAssertion(MARKET_HASH);
        assertEq(outcome, true);
        assertTrue(assertionId != bytes32(0));
    }
    
    function testAssertMarketOutcomeNO() public {
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, false);
        
        // Check assertion details
        (bytes32 assertionId, , bool outcome) = umaOracle.getMarketAssertion(MARKET_HASH);
        assertEq(outcome, false);
        assertTrue(assertionId != bytes32(0));
    }
    
    function testCannotAssertTwice() public {
        // First assertion
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
        
        // Second assertion should fail
        vm.prank(asserter);
        vm.expectRevert(UMAOracle.AssertionAlreadyMade.selector);
        umaOracle.assertMarketOutcome(MARKET_HASH, false);
    }
    
    function testAssertionRequiresBondApproval() public {
        // Remove approval
        vm.prank(asserter);
        bondToken.approve(address(umaOracle), 0);
        
        // Assertion should fail
        vm.prank(asserter);
        vm.expectRevert(ERC20.InsufficientAllowance.selector);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
    }
    
    function testAssertionTransfersBond() public {
        uint256 balanceBefore = bondToken.balanceOf(asserter);
        
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
        
        uint256 balanceAfter = bondToken.balanceOf(asserter);
        assertEq(balanceBefore - balanceAfter, BOND_AMOUNT);
        
        // Verify bond was transferred to OptimisticOracle
        assertEq(bondToken.balanceOf(address(mockOO)), BOND_AMOUNT);
    }
    
    // ============================================
    // RESOLUTION TESTS
    // ============================================
    
    function testGetResolutionAfterSettlement() public {
        // Make assertion
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
        
        // Get assertion ID
        (bytes32 assertionId, , ) = umaOracle.getMarketAssertion(MARKET_HASH);
        
        // Fast forward past liveness
        vm.warp(block.timestamp + umaOracle.liveness() + 1);
        
        // Settle assertion in mock
        mockOO.settleAssertion(assertionId);
        
        // Get resolution
        bool isYes = umaOracle.getResolution(MARKET_HASH);
        assertTrue(isYes);
    }
    
    function testGetResolutionAutoSettles() public {
        // Make assertion
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, false);
        
        // Fast forward past liveness
        vm.warp(block.timestamp + umaOracle.liveness() + 1);
        
        // Get resolution (should auto-settle)
        bool isYes = umaOracle.getResolution(MARKET_HASH);
        assertFalse(isYes);
    }
    
    function testCannotGetResolutionBeforeAssertion() public {
        vm.expectRevert(UMAOracle.MarketNotAsserted.selector);
        umaOracle.getResolution(MARKET_HASH);
    }
    
    function testCannotGetResolutionBeforeSettlement() public {
        // Make assertion
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
        
        // Try to get resolution before settlement
        vm.expectRevert(UMAOracle.AssertionNotSettled.selector);
        umaOracle.getResolution(MARKET_HASH);
    }
    
    function testGetResolutionRevertsOnInvalidAssertion() public {
        // Make assertion
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(MARKET_HASH, true);
        
        // Get assertion ID
        (bytes32 assertionId, , ) = umaOracle.getMarketAssertion(MARKET_HASH);
        
        // Fast forward and settle as false (disputed and lost)
        vm.warp(block.timestamp + umaOracle.liveness() + 1);
        mockOO.setAssertionOutcome(assertionId, false);
        mockOO.setAssertionSettled(assertionId, true);
        
        // Should revert
        vm.expectRevert(UMAOracle.InvalidAssertion.selector);
        umaOracle.getResolution(MARKET_HASH);
    }
    
    // ============================================
    // ADMIN TESTS
    // ============================================
    
    function testSetBondAmount() public {
        uint256 newBond = 200e6;
        
        vm.prank(owner);
        umaOracle.setBondAmount(newBond);
        
        assertEq(umaOracle.bondAmount(), newBond);
    }
    
    function testSetLiveness() public {
        uint64 newLiveness = 3600; // 1 hour
        
        vm.prank(owner);
        umaOracle.setLiveness(newLiveness);
        
        assertEq(umaOracle.liveness(), newLiveness);
    }
    
    function testOnlyOwnerCanSetBondAmount() public {
        vm.prank(asserter);
        vm.expectRevert();
        umaOracle.setBondAmount(200e6);
    }
    
    function testOnlyOwnerCanSetLiveness() public {
        vm.prank(asserter);
        vm.expectRevert();
        umaOracle.setLiveness(3600);
    }
    
    // ============================================
    // FUZZ TESTS
    // ============================================
    
    function testFuzzAssertMarketOutcome(bool outcome, bytes32 marketHash) public {
        vm.assume(marketHash != bytes32(0));
        
        vm.prank(asserter);
        umaOracle.assertMarketOutcome(marketHash, outcome);
        
        (bytes32 assertionId, , bool storedOutcome) = umaOracle.getMarketAssertion(marketHash);
        assertEq(storedOutcome, outcome);
        assertTrue(assertionId != bytes32(0));
    }
    
    function testFuzzMultipleMarkets(uint8 numMarkets) public {
        vm.assume(numMarkets > 0 && numMarkets <= 10);
        
        for (uint8 i = 0; i < numMarkets; i++) {
            bytes32 marketHash = keccak256(abi.encode("market", i));
            bool outcome = i % 2 == 0;
            
            // Fund asserter for this market
            bondToken.mint(asserter, BOND_AMOUNT);
            
            vm.prank(asserter);
            umaOracle.assertMarketOutcome(marketHash, outcome);
            
            // Verify
            assertTrue(umaOracle.hasAssertion(marketHash));
            (, , bool storedOutcome) = umaOracle.getMarketAssertion(marketHash);
            assertEq(storedOutcome, outcome);
        }
    }
}