// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title IOracle
 * @author Binary Protocol
 * @notice Interface for oracles that resolve binary prediction markets
 * @dev Oracles are responsible for providing YES/NO resolutions to markets.
 *      They are the source of truth for determining which outcome occurred.
 *
 *      Implementation requirements:
 *      - Must return a binary resolution (YES = true, NO = false)
 *      - Should revert if market is unknown or unresolved
 *      - Resolution should be immutable once set
 *
 *      Example implementations:
 *      - Chainlink: Use price feeds to resolve price-based markets
 *      - UMA: Use optimistic oracle for general purpose resolution
 *      - Governance: Use token voting for subjective questions
 *      - Multisig: Use trusted parties for specific events
 */
interface IOracle {
    /**
     * @notice Gets the resolution for a specific market
     * @param marketHash Unique identifier of the market to resolve
     * @return isYes True if the market resolved to YES, false for NO
     */
    function getResolution(bytes32 marketHash) external returns (bool isYes);
}
