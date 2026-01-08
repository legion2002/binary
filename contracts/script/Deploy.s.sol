// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/MultiVerse.sol";
import "../src/TrustedOracle.sol";

/**
 * @title Deploy
 * @notice Deploys Binary core contracts (MultiVerse + TrustedOracle)
 * @dev Markets are created via the admin API, not in this script
 */
contract Deploy is Script {
    function run() external {
        vm.startBroadcast();

        TrustedOracle oracle = new TrustedOracle();
        console.log("TrustedOracle deployed at:", address(oracle));

        MultiVerse multiverse = new MultiVerse();
        console.log("MultiVerse deployed at:", address(multiverse));

        vm.stopBroadcast();

        console.log("\n========================================");
        console.log("DEPLOYMENT SUMMARY");
        console.log("========================================");
        console.log("TrustedOracle:", address(oracle));
        console.log("MultiVerse:", address(multiverse));
    }
}
