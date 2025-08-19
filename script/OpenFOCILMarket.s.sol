// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {MultiVerse} from "../src/MultliVerse.sol";

/**
 * @title OpenFOCILMarket
 * @notice Script to open a market about FOCIL inclusion in Glamsterdam hardfork
 * @dev Deploy with: forge script script/OpenFOCILMarket.s.sol:OpenFOCILMarket --rpc-url $RPC_84532 --broadcast
 */
contract OpenFOCILMarket is Script {
    // Base Sepolia deployed contracts
    address constant MULTIVERSE = 0x4420E60140Ec9CC4A09711E46aa5904e89001Ca3;
    address constant TRUSTED_ORACLE = 0xB96B6Ac5fa219b8541e37D7Ec189189445cB25b7;
    
    // Market parameters
    string constant QUESTION = "Will FOCIL be included in the Glamsterdam hardfork?";
    uint32 constant RESOLUTION_DEADLINE = 1767225600; // January 1, 2026 00:00:00 UTC

    function run() external {
        // Get deployer private key
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);

        console2.log("Opening market on Base Sepolia");
        console2.log("Deployer:", deployer);
        console2.log("\nMarket Details:");
        console2.log("Question:", QUESTION);
        console2.log("Resolution Deadline:", RESOLUTION_DEADLINE);
        console2.log("Oracle:", TRUSTED_ORACLE);

        // Calculate the question hash
        bytes32 questionHash = keccak256(bytes(QUESTION));
        console2.log("\nQuestion Hash:");
        console2.logBytes32(questionHash);

        // Calculate the market hash
        bytes32 marketHash = keccak256(abi.encode(questionHash, RESOLUTION_DEADLINE, TRUSTED_ORACLE));
        console2.log("\nExpected Market Hash:");
        console2.logBytes32(marketHash);

        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        // Get MultiVerse contract
        MultiVerse multiVerse = MultiVerse(MULTIVERSE);

        // Open the market
        multiVerse.open(questionHash, RESOLUTION_DEADLINE, TRUSTED_ORACLE);

        console2.log("\n✅ Market opened successfully!");

        // Verify the market was created
        (uint32 deadline, address oracle, bytes32 qHash) = multiVerse.markets(marketHash);
        
        console2.log("\nMarket Verification:");
        console2.log("Stored Deadline:", deadline);
        console2.log("Stored Oracle:", oracle);
        console2.log("Stored Question Hash:");
        console2.logBytes32(qHash);

        vm.stopBroadcast();

        // Log summary
        logMarketSummary(marketHash, questionHash);
    }

    function logMarketSummary(bytes32 marketHash, bytes32 questionHash) internal view {
        console2.log("\n=== Market Summary ===");
        console2.log("Network: Base Sepolia");
        console2.log("\nMarket Information:");
        console2.log("Question: Will FOCIL be included in the Glamsterdam hardfork?");
        console2.log("Market Hash:");
        console2.logBytes32(marketHash);
        console2.log("Question Hash:");
        console2.logBytes32(questionHash);
        console2.log("\nContract Addresses:");
        console2.log("MultiVerse:", MULTIVERSE);
        console2.log("TrustedOracle:", TRUSTED_ORACLE);
        console2.log("\nResolution:");
        console2.log("Deadline: January 1, 2026 00:00:00 UTC");
        console2.log("Timestamp:", RESOLUTION_DEADLINE);
        console2.log("\n=== Next Steps ===");
        console2.log("1. Users can now split assets into YES/NO tokens");
        console2.log("2. Oracle owner can set resolution after the event");
        console2.log("3. Users can redeem winning tokens after resolution");
        console2.log("========================\n");
    }
}