// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IOracle {
    enum Resolution {
        NULL,
        UNRESOLVED,
        YES,
        NO,
        EVEN
    }

    function getResolution(bytes32 marketHash) external returns (Resolution resolution);
}
