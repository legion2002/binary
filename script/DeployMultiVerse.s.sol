// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {MultiVerse} from "../src/MultliVerse.sol";

/**
 * @title DeployMultiVerse
 * @notice Deployment script for MultiVerse contract on Base mainnet
 * @dev Deploy with: forge script script/DeployMultiVerse.s.sol:DeployMultiVerse --rpc-url base --broadcast --verify
 */
contract DeployMultiVerse is Script {
    // Base mainnet configuration
    uint256 constant BASE_CHAIN_ID = 8453;
    
    // Uniswap V2 addresses on Base mainnet
    address constant UNISWAP_V2_FACTORY = 0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6;
    address constant UNISWAP_V2_ROUTER = 0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24;
    
    // Token addresses on Base mainnet
    address constant WETH = 0x4200000000000000000000000000000000000006;
    address constant USDC = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;

    function run() external returns (MultiVerse) {
        // Verify we're on Base mainnet
        require(block.chainid == BASE_CHAIN_ID, "Not on Base mainnet");

        // Get deployer private key
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);

        console2.log("Deploying MultiVerse on Base mainnet");
        console2.log("Deployer:", deployer);
        console2.log("Chain ID:", block.chainid);

        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        // Deploy MultiVerse contract
        MultiVerse multiVerse = new MultiVerse();
        
        console2.log("MultiVerse deployed at:", address(multiVerse));

        vm.stopBroadcast();

        // Log deployment summary
        logDeploymentSummary(address(multiVerse));

        return multiVerse;
    }

    function logDeploymentSummary(address multiVerseAddress) internal view {
        console2.log("\n=== Deployment Summary ===");
        console2.log("Network: Base Mainnet");
        console2.log("Chain ID:", block.chainid);
        console2.log("\nDeployed Contract:");
        console2.log("MultiVerse:", multiVerseAddress);
        console2.log("\nExisting Infrastructure:");
        console2.log("Uniswap V2 Factory:", UNISWAP_V2_FACTORY);
        console2.log("Uniswap V2 Router:", UNISWAP_V2_ROUTER);
        console2.log("\nBase Tokens:");
        console2.log("WETH:", WETH);
        console2.log("USDC:", USDC);
        console2.log("\n=== Frontend Configuration ===");
        console2.log("Update MULTIVERSE address in frontend/src/config/contracts.ts to:");
        console2.log(multiVerseAddress);
        console2.log("========================\n");
    }
}