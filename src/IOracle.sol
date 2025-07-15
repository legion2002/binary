// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IOracle {
    function getResolution(bytes32 marketHash) external returns (bool isYes);
}
