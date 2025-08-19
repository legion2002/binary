// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {MultiVerse} from "../src/MultliVerse.sol";
import {TrustedOracle} from "../src/TrustedOracle.sol";

/**
 * @title DeployBaseSepolia
 * @notice Deployment script for MultiVerse and TrustedOracle contracts on Base Sepolia testnet
 * @dev Deploy with: forge script script/DeployBaseSepolia.s.sol:DeployBaseSepolia --rpc-url $RPC_84532 --broadcast --verify
 */
contract DeployBaseSepolia is Script {
    // Base Sepolia configuration
    uint256 constant BASE_SEPOLIA_CHAIN_ID = 84532;
    
    // Uniswap V2 addresses on Base Sepolia
    // Note: These addresses might differ on testnet, using placeholder values
    address constant UNISWAP_V2_FACTORY = 0x7Ae58f10f7849cA6F5fB71b7f45CB416c9204b1e;
    address constant UNISWAP_V2_ROUTER = 0x1689E7B1F10000AE47eBfE339a4f69dECd19F602;
    
    // Token addresses on Base Sepolia testnet
    address constant WETH = 0x4200000000000000000000000000000000000006; // Wrapped ETH on Base
    address constant USDC = 0x036CbD53842c5426634e7929541eC2318f3dCF7e; // USDC on Base Sepolia

    function run() external returns (MultiVerse, TrustedOracle) {
        // Verify we're on Base Sepolia
        require(block.chainid == BASE_SEPOLIA_CHAIN_ID, "Not on Base Sepolia");

        // Get deployer private key
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);

        console2.log("Deploying contracts on Base Sepolia");
        console2.log("Deployer:", deployer);
        console2.log("Chain ID:", block.chainid);
        console2.log("Deployer balance:", deployer.balance);

        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        // Deploy MultiVerse contract
        MultiVerse multiVerse = new MultiVerse();
        
        console2.log("MultiVerse deployed at:", address(multiVerse));
        
        // Deploy TrustedOracle contract
        TrustedOracle trustedOracle = new TrustedOracle();
        
        console2.log("TrustedOracle deployed at:", address(trustedOracle));

        vm.stopBroadcast();

        // Log deployment summary
        logDeploymentSummary(address(multiVerse), address(trustedOracle));

        return (multiVerse, trustedOracle);
    }

    function logDeploymentSummary(address multiVerseAddress, address trustedOracleAddress) internal view {
        console2.log("\n=== Deployment Summary ===");
        console2.log("Network: Base Sepolia Testnet");
        console2.log("Chain ID:", block.chainid);
        console2.log("\nDeployed Contracts:");
        console2.log("MultiVerse:", multiVerseAddress);
        console2.log("TrustedOracle:", trustedOracleAddress);
        console2.log("\nExisting Infrastructure:");
        console2.log("Uniswap V2 Factory:", UNISWAP_V2_FACTORY);
        console2.log("Uniswap V2 Router:", UNISWAP_V2_ROUTER);
        console2.log("\nBase Sepolia Tokens:");
        console2.log("WETH:", WETH);
        console2.log("USDC:", USDC);
        console2.log("\n=== Frontend Configuration ===");
        console2.log("Update addresses in frontend/src/config/contracts.ts:");
        console2.log("MULTIVERSE:", multiVerseAddress);
        console2.log("TRUSTED_ORACLE:", trustedOracleAddress);
        console2.log("========================\n");
    }
}