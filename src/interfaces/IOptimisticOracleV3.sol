// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IERC20} from "forge-std/interfaces/IERC20.sol";

/**
 * @title IOptimisticOracleV3
 * @notice Minimal interface for UMA's OptimisticOracleV3 contract
 * @dev This interface includes only the functions needed for Binary protocol integration
 *      Full interface available at: https://github.com/UMAprotocol/protocol
 */
interface IOptimisticOracleV3 {
    // ============================================
    // STRUCTS
    // ============================================
    
    /**
     * @notice Assertion data structure
     * @param asserter Address that made the assertion
     * @param assertionTime Timestamp when assertion was made
     * @param expirationTime Timestamp when assertion can be resolved if not disputed
     * @param settled Whether the assertion has been settled
     * @param settlementResolution Whether the assertion was resolved as true
     * @param disputeCaller Address that disputed the assertion (if any)
     */
    struct Assertion {
        address asserter;
        uint64 assertionTime;
        uint64 expirationTime;
        bool settled;
        bool settlementResolution;
        address disputeCaller;
    }

    // ============================================
    // EVENTS
    // ============================================
    
    event AssertionMade(
        bytes32 indexed assertionId,
        address indexed asserter,
        bytes claim,
        uint256 bond,
        uint64 expirationTime,
        address callbackRecipient,
        address escalationManager
    );

    event AssertionDisputed(
        bytes32 indexed assertionId,
        address indexed caller,
        address indexed disputer
    );

    event AssertionSettled(
        bytes32 indexed assertionId,
        address indexed asserter,
        bool settlementResolution
    );

    // ============================================
    // FUNCTIONS
    // ============================================
    
    /**
     * @notice Makes an assertion about truth of a claim
     * @param claim The claim being asserted (encoded data)
     * @param asserter Address making the assertion
     * @param callbackRecipient Address to receive callback on resolution
     * @param escalationManager Address of custom escalation manager (or address(0))
     * @param liveness Time in seconds before assertion can be resolved
     * @param currency ERC20 token used for bond
     * @param bond Amount of currency to bond
     * @param identifier Price identifier to use with DVM
     * @param domainId Domain identifier for cross-chain assertions
     * @return assertionId Unique identifier for this assertion
     */
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
    ) external returns (bytes32 assertionId);

    /**
     * @notice Gets assertion data by ID
     * @param assertionId Unique identifier of the assertion
     * @return assertion The assertion data structure
     */
    function getAssertion(bytes32 assertionId) external view returns (Assertion memory assertion);

    /**
     * @notice Settles an assertion after liveness period
     * @param assertionId Unique identifier of the assertion
     */
    function settleAssertion(bytes32 assertionId) external;

    /**
     * @notice Disputes an assertion before expiration
     * @param assertionId Unique identifier of the assertion
     * @param disputer Address disputing the assertion
     */
    function disputeAssertion(bytes32 assertionId, address disputer) external;

    /**
     * @notice Gets the minimum bond required for assertions
     * @param currency The currency to check minimum bond for
     * @return minimumBond The minimum bond amount
     */
    function getMinimumBond(address currency) external view returns (uint256 minimumBond);
}