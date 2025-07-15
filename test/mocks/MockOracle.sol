// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IOracle} from "../../src/IOracle.sol";
import {Ownable} from "solady/auth/Ownable.sol";

contract MockOracle is IOracle, Ownable {
    error MarketNotSet();

    mapping(bytes32 => bool) public resolutions;
    mapping(bytes32 => bool) public marketSet;

    constructor() {
        _initializeOwner(msg.sender);
    }

    function setResolution(bytes32 marketHash, bool isYes) external onlyOwner {
        resolutions[marketHash] = isYes;
        marketSet[marketHash] = true;
    }

    function getResolution(bytes32 marketHash) external view override returns (bool) {
        if (!marketSet[marketHash]) revert MarketNotSet();
        return resolutions[marketHash];
    }
}