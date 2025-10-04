// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IOracle} from "./IOracle.sol";
import {Ownable} from "solady/auth/Ownable.sol";

/**
 * @title TrustedOracle
 * @author Binary Protocol
 * @notice Trusted oracle implementation for Binary prediction markets
 * @dev This contract provides oracle functionality where a trusted entity (owner)
 *      can set market resolutions. Suitable for:
 *      - Controlled environments where a trusted party resolves markets
 *      - Initial deployments before decentralized oracle integration
 *      - Markets that require manual verification or judgement
 *
 *      Features:
 *      - Owner-controlled resolution setting
 *      - Tracking of set/unset markets
 *      - Revert protection for unset markets
 */
contract TrustedOracle is IOracle, Ownable {
    // ============================================
    // EVENTS
    // ============================================

    event ResolutionSet(bytes32 indexed marketHash, bool isYes);

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
     * @notice Initializes the trusted oracle with the deployer as owner
     * @dev Owner has exclusive rights to set market resolutions
     */
    constructor() {
        _initializeOwner(msg.sender);
    }

    // ============================================
    // OWNER FUNCTIONS
    // ============================================

    /**
     * @notice Sets the resolution for a specific market
     * @dev Only callable by owner
     * @param marketHash Unique identifier of the market
     * @param isYes True for YES resolution, false for NO resolution
     */
    function setResolution(bytes32 marketHash, bool isYes) external onlyOwner {
        resolutions[marketHash] = isYes;
        marketSet[marketHash] = true;

        emit ResolutionSet(marketHash, isYes);
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
