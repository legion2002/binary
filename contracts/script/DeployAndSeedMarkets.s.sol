// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/MultiVerse.sol";
import "../src/TrustedOracle.sol";
import {ITIP20} from "../src/interfaces/ITIP20.sol";

/**
 * @title DeployAndSeedMarkets
 * @notice Deploys Binary contracts and creates mock markets on Tempo testnet
 * @dev Run with: forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
 *      --rpc-url tempo_testnet --broadcast --private-key $PRIVATE_KEY -vvvv
 */
contract DeployAndSeedMarkets is Script {
    // Tempo precompile addresses
    ITIP20 constant PATH_USD = ITIP20(0x20C0000000000000000000000000000000000000);

    // Deployed contract addresses (set after deployment)
    MultiVerse public multiverse;
    TrustedOracle public oracle;

    function run() external {
        vm.startBroadcast();

        // ============================================
        // 1. DEPLOY CORE CONTRACTS
        // ============================================

        oracle = new TrustedOracle();
        console.log("TrustedOracle deployed at:", address(oracle));

        multiverse = new MultiVerse();
        console.log("MultiVerse deployed at:", address(multiverse));

        // ============================================
        // 2. CREATE MOCK MARKETS
        // ============================================

        // Market 1: ETH Price Prediction
        bytes32 questionHash1 = keccak256("Will ETH be above $5000 by end of Q1 2026?");
        uint32 deadline1 = uint32(block.timestamp + 90 days);
        multiverse.open(questionHash1, deadline1, address(oracle));
        bytes32 marketHash1 = keccak256(abi.encode(questionHash1, deadline1, address(oracle)));
        console.log("Market 1 created - ETH $5000 Q1 2026");
        console.log("  Market Hash:", vm.toString(marketHash1));

        // Market 2: Bitcoin Halving Impact
        bytes32 questionHash2 = keccak256("Will BTC reach new ATH within 6 months of 2024 halving?");
        uint32 deadline2 = uint32(block.timestamp + 180 days);
        multiverse.open(questionHash2, deadline2, address(oracle));
        bytes32 marketHash2 = keccak256(abi.encode(questionHash2, deadline2, address(oracle)));
        console.log("Market 2 created - BTC ATH post-halving");
        console.log("  Market Hash:", vm.toString(marketHash2));

        // Market 3: Ethereum Upgrade
        bytes32 questionHash3 = keccak256("Will Pectra upgrade deploy on mainnet by June 2025?");
        uint32 deadline3 = uint32(block.timestamp + 60 days);
        multiverse.open(questionHash3, deadline3, address(oracle));
        bytes32 marketHash3 = keccak256(abi.encode(questionHash3, deadline3, address(oracle)));
        console.log("Market 3 created - Pectra upgrade");
        console.log("  Market Hash:", vm.toString(marketHash3));

        // Market 4: Stablecoin Adoption
        bytes32 questionHash4 = keccak256("Will stablecoin market cap exceed $200B by end of 2025?");
        uint32 deadline4 = uint32(block.timestamp + 365 days);
        multiverse.open(questionHash4, deadline4, address(oracle));
        bytes32 marketHash4 = keccak256(abi.encode(questionHash4, deadline4, address(oracle)));
        console.log("Market 4 created - Stablecoin $200B");
        console.log("  Market Hash:", vm.toString(marketHash4));

        // ============================================
        // 3. CREATE VERSE TOKENS FOR MARKETS
        // ============================================

        // Create verse tokens for Market 1 with PATH_USD as the underlying asset
        (address yes1, address no1) = multiverse.create(address(PATH_USD), marketHash1);
        console.log("Market 1 Verse Tokens:");
        console.log("  YES:", yes1);
        console.log("  NO:", no1);

        // Create verse tokens for Market 2
        (address yes2, address no2) = multiverse.create(address(PATH_USD), marketHash2);
        console.log("Market 2 Verse Tokens:");
        console.log("  YES:", yes2);
        console.log("  NO:", no2);

        // Create verse tokens for Market 3
        (address yes3, address no3) = multiverse.create(address(PATH_USD), marketHash3);
        console.log("Market 3 Verse Tokens:");
        console.log("  YES:", yes3);
        console.log("  NO:", no3);

        // Create verse tokens for Market 4
        (address yes4, address no4) = multiverse.create(address(PATH_USD), marketHash4);
        console.log("Market 4 Verse Tokens:");
        console.log("  YES:", yes4);
        console.log("  NO:", no4);

        vm.stopBroadcast();

        // ============================================
        // 4. OUTPUT DEPLOYMENT SUMMARY
        // ============================================

        console.log("\n========================================");
        console.log("DEPLOYMENT SUMMARY");
        console.log("========================================");
        console.log("Network: Tempo Testnet (Chain ID: 42429)");
        console.log("TrustedOracle:", address(oracle));
        console.log("MultiVerse:", address(multiverse));
        console.log("\nMarkets Created: 4");
        console.log("Verse Token Pairs Created: 4");
        console.log("\nUpdate frontend/src/config/contracts.ts with:");
        console.log("  MULTIVERSE:", address(multiverse));
        console.log("  ORACLE:", address(oracle));
    }
}
