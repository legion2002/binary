// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "../src/MultiVerse.sol";
import "../src/TrustedOracle.sol";

contract DeployAll is Script {
    function run() external {
        vm.startBroadcast();

        // Deploy TrustedOracle first
        TrustedOracle oracle = new TrustedOracle();
        console.log("TrustedOracle deployed at:", address(oracle));

        // Deploy MultiVerse
        MultiVerse multiverse = new MultiVerse();
        console.log("MultiVerse deployed at:", address(multiverse));

        vm.stopBroadcast();
    }
}
