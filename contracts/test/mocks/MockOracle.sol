// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IOracle} from "../../src/IOracle.sol";
import {Ownable} from "solady/auth/Ownable.sol";

/**
 * @title MockOracle
 * @author Binary Protocol
 * @notice Mock implementation of IOracle for testing Binary prediction markets
 * @dev This contract simulates oracle functionality for test environments:
 *      - Allows manual setting of market resolutions
 *      - Tracks which markets have been set
 *      - Reverts on queries for unset markets
 *
 *      In production, real oracles would integrate with:
 *      - Chainlink for price/data feeds
 *      - UMA for optimistic oracle resolution
 *      - Custom governance mechanisms
 *      - External data providers
 */
contract MockOracle is IOracle, Ownable {
    // ============================================
    // ERRORS
    // ============================================

    /// @notice Thrown when querying resolution for a market that hasn't been set
    error MarketNotSet();

    // ============================================
    // STATE VARIABLES
    // ============================================

    /// @notice Stores the YES/NO resolution for each market
    mapping(bytes32 => bool) public resolutions;

    /// @notice Tracks whether a resolution has been set for each market
    mapping(bytes32 => bool) public marketSet;

    // ============================================
    // CONSTRUCTOR
    // ============================================

    /**
     * @notice Initializes the mock oracle with the deployer as owner
     * @dev Owner can set resolutions for testing different market outcomes
     */
    constructor() {
        _initializeOwner(msg.sender);
    }

    // ============================================
    // OWNER FUNCTIONS
    // ============================================

    /**
     * @notice Sets the resolution for a specific market
     * @dev Only callable by owner, typically used in test setups
     * @param marketHash Unique identifier of the market
     * @param isYes True for YES resolution, false for NO resolution
     */
    function setResolution(bytes32 marketHash, bool isYes) external onlyOwner {
        resolutions[marketHash] = isYes;
        marketSet[marketHash] = true;
    }

    // ============================================
    // ORACLE INTERFACE
    // ============================================

    /**
     * @notice Returns the resolution for a specific market
     * @dev Implements IOracle interface. Reverts if resolution not set.
     * @param marketHash Unique identifier of the market
     * @return True for YES resolution, false for NO resolution
     */
    function getResolution(bytes32 marketHash) external view override returns (bool) {
        if (!marketSet[marketHash]) revert MarketNotSet();
        return resolutions[marketHash];
    }
}
