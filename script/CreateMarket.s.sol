// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {MultiVerse} from "../src/MultliVerse.sol";

interface IUniswapV2Factory {
    function createPair(address tokenA, address tokenB) external returns (address pair);
    function getPair(address tokenA, address tokenB) external view returns (address pair);
}

/**
 * @title CreateMarket
 * @notice Script to create prediction markets, verse tokens, and UniswapV2 pairs on Base mainnet
 * @dev Run with: forge script script/CreateMarket.s.sol:CreateMarket --rpc-url base --broadcast
 */
contract CreateMarket is Script {
    // Base mainnet configuration
    uint256 constant BASE_CHAIN_ID = 8453;
    
    // Token addresses on Base mainnet
    address constant WETH = 0x4200000000000000000000000000000000000006;
    address constant USDC = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;
    
    // Uniswap V2 Factory on Base mainnet
    address constant UNISWAP_V2_FACTORY = 0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6;

    function run() external {
        // Verify we're on Base mainnet
        require(block.chainid == BASE_CHAIN_ID, "Not on Base mainnet");

        // Get configuration from environment
        address multiVerseAddress = vm.envAddress("MULTIVERSE_ADDRESS");
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);

        console2.log("Creating market on Base mainnet");
        console2.log("MultiVerse:", multiVerseAddress);
        console2.log("Deployer:", deployer);

        MultiVerse multiVerse = MultiVerse(multiVerseAddress);

        // Example market parameters
        bytes32 questionHash = keccak256(abi.encodePacked("Will FOCIL be included in the Ethereum Pectra hardfork?"));
        uint32 resolutionDeadline = uint32(block.timestamp + 90 days); // 90 days from now
        address oracle = deployer; // Using deployer as oracle for example

        vm.startBroadcast(deployerPrivateKey);

        // Open the market
        console2.log("\nOpening market...");
        multiVerse.open(questionHash, resolutionDeadline, oracle);
        
        // Calculate market hash
        bytes32 marketHash = keccak256(abi.encode(questionHash, resolutionDeadline, oracle));
        console2.log("Market hash:", vm.toString(marketHash));

        // Create verse tokens for WETH
        console2.log("\nCreating WETH verse tokens...");
        (address wethYesVerse, address wethNoVerse) = multiVerse.create(WETH, marketHash);
        console2.log("WETH YES Verse:", wethYesVerse);
        console2.log("WETH NO Verse:", wethNoVerse);

        // Create verse tokens for USDC
        console2.log("\nCreating USDC verse tokens...");
        (address usdcYesVerse, address usdcNoVerse) = multiVerse.create(USDC, marketHash);
        console2.log("USDC YES Verse:", usdcYesVerse);
        console2.log("USDC NO Verse:", usdcNoVerse);

        // Create UniswapV2 pairs
        console2.log("\nCreating UniswapV2 pairs...");
        IUniswapV2Factory factory = IUniswapV2Factory(UNISWAP_V2_FACTORY);
        
        // Create 4 pairs for verse token trading
        address pair1 = createOrGetPair(factory, wethYesVerse, usdcYesVerse, "YES_WETH/YES_USDC");
        address pair2 = createOrGetPair(factory, wethNoVerse, usdcNoVerse, "NO_WETH/NO_USDC");
        address pair3 = createOrGetPair(factory, wethYesVerse, usdcNoVerse, "YES_WETH/NO_USDC");
        address pair4 = createOrGetPair(factory, wethNoVerse, usdcYesVerse, "NO_WETH/YES_USDC");

        vm.stopBroadcast();

        // Log summary
        logMarketSummary(
            questionHash, 
            marketHash, 
            resolutionDeadline, 
            oracle,
            [wethYesVerse, wethNoVerse, usdcYesVerse, usdcNoVerse],
            [pair1, pair2, pair3, pair4]
        );
    }
    
    function createOrGetPair(
        IUniswapV2Factory factory,
        address tokenA,
        address tokenB,
        string memory pairName
    ) internal returns (address pair) {
        // Check if pair already exists
        pair = factory.getPair(tokenA, tokenB);
        
        if (pair == address(0)) {
            // Create new pair
            pair = factory.createPair(tokenA, tokenB);
            console2.log("Created pair", pairName, "at:", pair);
        } else {
            console2.log("Pair", pairName, "already exists at:", pair);
        }
        
        return pair;
    }

    function logMarketSummary(
        bytes32 questionHash,
        bytes32 marketHash,
        uint32 resolutionDeadline,
        address oracle,
        address[4] memory verseTokens,
        address[4] memory pairs
    ) internal pure {
        console2.log("\n=== Market Creation Summary ===");
        console2.log("Question Hash:", vm.toString(questionHash));
        console2.log("Market Hash:", vm.toString(marketHash));
        console2.log("Resolution Deadline:", resolutionDeadline);
        console2.log("Oracle:", oracle);
        console2.log("\nVerse tokens created:");
        console2.log("- WETH YES:", verseTokens[0]);
        console2.log("- WETH NO:", verseTokens[1]);
        console2.log("- USDC YES:", verseTokens[2]);
        console2.log("- USDC NO:", verseTokens[3]);
        console2.log("\nUniswapV2 pairs created:");
        console2.log("- YES_WETH/YES_USDC:", pairs[0]);
        console2.log("- NO_WETH/NO_USDC:", pairs[1]);
        console2.log("- YES_WETH/NO_USDC:", pairs[2]);
        console2.log("- NO_WETH/YES_USDC:", pairs[3]);
        console2.log("==============================\n");
    }
}