// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {MultiVerse} from "../src/MultliVerse.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

/**
 * @title OpenFOCILMarketAndSplit
 * @notice Script to open a market about FOCIL and split USDC/WETH into YES/NO tokens
 * @dev Deploy with: forge script script/OpenFOCILMarketAndSplit.s.sol:OpenFOCILMarketAndSplit --rpc-url $RPC_84532 --broadcast
 */
contract OpenFOCILMarketAndSplit is Script {
    // Base Sepolia deployed contracts
    address constant MULTIVERSE = 0x4420E60140Ec9CC4A09711E46aa5904e89001Ca3;
    address constant TRUSTED_ORACLE = 0xB96B6Ac5fa219b8541e37D7Ec189189445cB25b7;
    
    // Token addresses on Base Sepolia
    address constant USDC = 0x08c288C2d48A2FEa04B0457A7a3780De16A8569b;
    address constant WETH = 0x4200000000000000000000000000000000000006;
    
    // Market parameters
    string constant QUESTION = "Will FOCIL be included in the Glamsterdam hardfork?";
    uint32 constant RESOLUTION_DEADLINE = 1767225600; // January 1, 2026 00:00:00 UTC
    
    // Split amounts (adjust as needed)
    uint256 constant USDC_SPLIT_AMOUNT = 100 * 10**6; // 100 USDC (6 decimals)
    uint256 constant WETH_SPLIT_AMOUNT = 0.01 ether; // 0.01 WETH (18 decimals)

    function run() external {
        // Get deployer private key
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);

        console2.log("Opening market and splitting assets on Base Sepolia");
        console2.log("Deployer:", deployer);
        
        // Calculate the question hash
        bytes32 questionHash = keccak256(bytes(QUESTION));
        
        // Calculate the market hash
        bytes32 marketHash = keccak256(abi.encode(questionHash, RESOLUTION_DEADLINE, TRUSTED_ORACLE));
        
        console2.log("\nMarket Details:");
        console2.log("Question:", QUESTION);
        console2.log("Question Hash:");
        console2.logBytes32(questionHash);
        console2.log("Market Hash:");
        console2.logBytes32(marketHash);

        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        // Get contracts
        MultiVerse multiVerse = MultiVerse(MULTIVERSE);

        // Open the market
        console2.log("\n1. Opening market...");
        multiVerse.open(questionHash, RESOLUTION_DEADLINE, TRUSTED_ORACLE);
        console2.log("   Market opened successfully!");

        // Create and split USDC
        _handleUSDC(multiVerse, marketHash, deployer);
        
        // Create and split WETH  
        _handleWETH(multiVerse, marketHash, deployer);

        vm.stopBroadcast();

        console2.log("\n=== Market Summary ===");
        console2.log("Network: Base Sepolia");
        console2.log("Market Hash:");
        console2.logBytes32(marketHash);
        console2.log("\nContract Addresses:");
        console2.log("MultiVerse:", MULTIVERSE);
        console2.log("TrustedOracle:", TRUSTED_ORACLE);
        console2.log("\n=== Next Steps ===");
        console2.log("1. Trade YES/NO tokens on DEXs or P2P");
        console2.log("2. Combine YES+NO tokens back to original assets anytime");
        console2.log("3. Oracle resolves market after FOCIL decision");
        console2.log("4. Redeem winning tokens 1:1 for original assets");
        console2.log("========================\n");
    }
    
    function _handleUSDC(MultiVerse multiVerse, bytes32 marketHash, address deployer) internal {
        IERC20 usdc = IERC20(USDC);
        uint256 usdcBalance = usdc.balanceOf(deployer);
        
        console2.log("\n2. Handling USDC...");
        console2.log("   Balance:", usdcBalance / 10**6, "USDC");
        
        // Create verses for USDC
        try multiVerse.create(USDC, marketHash) returns (address yesUsdc, address noUsdc) {
            console2.log("   USDC verses created!");
            console2.log("   YES_USDC:", yesUsdc);
            console2.log("   NO_USDC:", noUsdc);
        } catch {
            console2.log("   INFO: USDC verses already exist");
            (address yesUsdc, address noUsdc) = multiVerse.getVerseAddress(USDC, marketHash);
            console2.log("   YES_USDC:", yesUsdc);
            console2.log("   NO_USDC:", noUsdc);
        }
        
        // Split USDC if balance is sufficient
        if (usdcBalance >= USDC_SPLIT_AMOUNT) {
            console2.log("   Splitting", USDC_SPLIT_AMOUNT / 10**6, "USDC...");
            
            // Approve and split
            usdc.approve(address(multiVerse), USDC_SPLIT_AMOUNT);
            multiVerse.split(USDC, USDC_SPLIT_AMOUNT, marketHash);
            
            console2.log("   Successfully split USDC into YES/NO tokens!");
        } else {
            console2.log("   Insufficient USDC balance to split");
        }
    }
    
    function _handleWETH(MultiVerse multiVerse, bytes32 marketHash, address deployer) internal {
        IERC20 weth = IERC20(WETH);
        uint256 wethBalance = weth.balanceOf(deployer);
        
        console2.log("\n3. Handling WETH...");
        console2.log("   Balance:", wethBalance / 10**18, "WETH");
        
        // Create verses for WETH
        try multiVerse.create(WETH, marketHash) returns (address yesWeth, address noWeth) {
            console2.log("   WETH verses created!");
            console2.log("   YES_WETH:", yesWeth);
            console2.log("   NO_WETH:", noWeth);
        } catch {
            console2.log("   INFO: WETH verses already exist");
            (address yesWeth, address noWeth) = multiVerse.getVerseAddress(WETH, marketHash);
            console2.log("   YES_WETH:", yesWeth);
            console2.log("   NO_WETH:", noWeth);
        }
        
        // Split WETH if balance is sufficient
        if (wethBalance >= WETH_SPLIT_AMOUNT) {
            console2.log("   Splitting", WETH_SPLIT_AMOUNT / 10**18, "WETH...");
            
            // Approve and split
            weth.approve(address(multiVerse), WETH_SPLIT_AMOUNT);
            multiVerse.split(WETH, WETH_SPLIT_AMOUNT, marketHash);
            
            console2.log("   Successfully split WETH into YES/NO tokens!");
        } else {
            console2.log("   Insufficient WETH balance to split");
        }
    }
}