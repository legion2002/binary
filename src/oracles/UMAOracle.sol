// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IOracle} from "../IOracle.sol";
import {IOptimisticOracleV3} from "../interfaces/IOptimisticOracleV3.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {Ownable} from "solady/auth/Ownable.sol";

/**
 * @title UMAOracle
 * @author Binary Protocol
 * @notice Oracle implementation using UMA's OptimisticOracleV3 for decentralized market resolution
 * @dev This contract bridges Binary's IOracle interface with UMA's assertion-based oracle system.
 *      Market resolutions are asserted with a bond and subject to a dispute period.
 *      If not disputed, assertions become truth. If disputed, UMA's DVM resolves.
 */
contract UMAOracle is IOracle, Ownable {
    // ============================================
    // ERRORS
    // ============================================
    
    error MarketNotAsserted();
    error AssertionNotSettled();
    error InvalidAssertion();
    error AssertionAlreadyMade();
    error InsufficientBond();
    
    // ============================================
    // CONSTANTS
    // ============================================
    
    /// @notice UMA's default identifier for YES/NO questions
    bytes32 public constant YES_NO_IDENTIFIER = bytes32("YES_OR_NO_QUERY");
    
    /// @notice Default liveness period (2 hours)
    uint64 public constant DEFAULT_LIVENESS = 7200;
    
    // ============================================
    // STATE VARIABLES
    // ============================================
    
    /// @notice The OptimisticOracleV3 contract
    IOptimisticOracleV3 public immutable optimisticOracle;
    
    /// @notice Currency used for bonds (e.g., USDC)
    IERC20 public immutable bondCurrency;
    
    /// @notice Required bond amount for assertions
    uint256 public bondAmount;
    
    /// @notice Liveness period for assertions
    uint64 public liveness;
    
    /// @notice Maps market hash to assertion ID
    mapping(bytes32 marketHash => bytes32 assertionId) public marketAssertions;
    
    /// @notice Maps assertion ID to the asserted outcome (true = YES, false = NO)
    mapping(bytes32 assertionId => bool outcome) public assertedOutcomes;
    
    // ============================================
    // EVENTS
    // ============================================
    
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
    
    // ============================================
    // CONSTRUCTOR
    // ============================================
    
    /**
     * @notice Initializes the UMA oracle
     * @param _optimisticOracle Address of UMA's OptimisticOracleV3
     * @param _bondCurrency ERC20 token used for bonds
     * @param _bondAmount Required bond amount
     */
    constructor(
        address _optimisticOracle,
        address _bondCurrency,
        uint256 _bondAmount
    ) {
        optimisticOracle = IOptimisticOracleV3(_optimisticOracle);
        bondCurrency = IERC20(_bondCurrency);
        bondAmount = _bondAmount;
        liveness = DEFAULT_LIVENESS;
        
        _initializeOwner(msg.sender);
    }
    
    // ============================================
    // ORACLE FUNCTIONS
    // ============================================
    
    /**
     * @notice Asserts the outcome of a market
     * @dev Anyone can assert an outcome by posting a bond
     * @param marketHash The market to assert outcome for
     * @param outcome True for YES, false for NO
     */
    function assertMarketOutcome(bytes32 marketHash, bool outcome) external {
        // Check if assertion already exists
        if (marketAssertions[marketHash] != bytes32(0)) {
            revert AssertionAlreadyMade();
        }
        
        // Transfer bond from asserter to this contract
        bondCurrency.transferFrom(msg.sender, address(this), bondAmount);
        
        // Approve OptimisticOracle to spend the bond
        bondCurrency.approve(address(optimisticOracle), bondAmount);
        
        // Create claim for the assertion
        bytes memory claim = abi.encode(
            "Binary market resolution: ",
            marketHash,
            " resolves to ",
            outcome ? "YES" : "NO"
        );
        
        // Make assertion to UMA
        bytes32 assertionId = optimisticOracle.assertTruth(
            claim,
            msg.sender,           // asserter
            address(this),        // callback recipient
            address(0),           // no custom escalation manager
            liveness,             // liveness period
            bondCurrency,         // bond currency
            bondAmount,           // bond amount
            YES_NO_IDENTIFIER,    // identifier
            marketHash            // use market hash as domain ID
        );
        
        // Store assertion data
        marketAssertions[marketHash] = assertionId;
        assertedOutcomes[assertionId] = outcome;
        
        emit MarketAsserted(marketHash, assertionId, outcome, msg.sender);
    }
    
    /**
     * @notice Gets the resolution for a market
     * @dev Implements IOracle interface
     * @param marketHash The market to get resolution for
     * @return isYes True if market resolved to YES, false for NO
     */
    function getResolution(bytes32 marketHash) external override returns (bool isYes) {
        bytes32 assertionId = marketAssertions[marketHash];
        
        // Check if assertion exists
        if (assertionId == bytes32(0)) {
            revert MarketNotAsserted();
        }
        
        // Get assertion data from UMA
        IOptimisticOracleV3.Assertion memory assertion = optimisticOracle.getAssertion(assertionId);
        
        // Try to settle if not already settled
        if (!assertion.settled && block.timestamp >= assertion.expirationTime) {
            optimisticOracle.settleAssertion(assertionId);
            assertion = optimisticOracle.getAssertion(assertionId);
        }
        
        // Check if settled
        if (!assertion.settled) {
            revert AssertionNotSettled();
        }
        
        // If assertion was found to be false (disputed and lost), revert
        if (!assertion.settlementResolution) {
            revert InvalidAssertion();
        }
        
        // Return the asserted outcome
        isYes = assertedOutcomes[assertionId];
        
        emit MarketResolved(marketHash, isYes);
    }
    
    // ============================================
    // ADMIN FUNCTIONS
    // ============================================
    
    /**
     * @notice Updates the bond amount
     * @param _bondAmount New bond amount
     */
    function setBondAmount(uint256 _bondAmount) external onlyOwner {
        bondAmount = _bondAmount;
    }
    
    /**
     * @notice Updates the liveness period
     * @param _liveness New liveness period in seconds
     */
    function setLiveness(uint64 _liveness) external onlyOwner {
        liveness = _liveness;
    }
    
    // ============================================
    // VIEW FUNCTIONS
    // ============================================
    
    /**
     * @notice Checks if a market has an assertion
     * @param marketHash The market to check
     * @return True if market has an assertion
     */
    function hasAssertion(bytes32 marketHash) external view returns (bool) {
        return marketAssertions[marketHash] != bytes32(0);
    }
    
    /**
     * @notice Gets assertion details for a market
     * @param marketHash The market to query
     * @return assertionId The assertion ID
     * @return assertion The assertion data from UMA
     * @return outcome The asserted outcome (YES/NO)
     */
    function getMarketAssertion(bytes32 marketHash) external view returns (
        bytes32 assertionId,
        IOptimisticOracleV3.Assertion memory assertion,
        bool outcome
    ) {
        assertionId = marketAssertions[marketHash];
        if (assertionId != bytes32(0)) {
            assertion = optimisticOracle.getAssertion(assertionId);
            outcome = assertedOutcomes[assertionId];
        }
    }
}