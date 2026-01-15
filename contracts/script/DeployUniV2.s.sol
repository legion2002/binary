// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0 <0.9.0;

import "forge-std/Script.sol";

/**
 * @title DeployUniV2
 * @notice Deploys UniswapV2 Factory and Router02 contracts
 * @dev Uses PATH_USD as the "WETH" equivalent since Tempo is stablecoin-native
 * 
 * Due to Solidity version incompatibilities between Factory (0.5.16) and Router (0.6.6),
 * we deploy them using bytecode from the built artifacts.
 */
contract DeployUniV2 is Script {
    // PATH_USD address - used as WETH equivalent on Tempo
    address constant PATH_USD = 0x20C0000000000000000000000000000000000000;

    function run() external {
        vm.startBroadcast();

        // Deploy Factory with deployer as feeToSetter
        // Get bytecode from artifact path relative to project root
        bytes memory factoryBytecode = vm.getCode("out/UniswapV2Factory.sol/UniswapV2Factory.json");
        bytes memory factoryInitCode = abi.encodePacked(factoryBytecode, abi.encode(msg.sender));
        
        address factory;
        assembly {
            factory := create(0, add(factoryInitCode, 0x20), mload(factoryInitCode))
        }
        require(factory != address(0), "Factory deployment failed");
        console.log("UniswapV2Factory deployed at:", factory);

        // Deploy Router02 with factory and PATH_USD as WETH
        bytes memory routerBytecode = vm.getCode("out/UniswapV2Router02.sol/UniswapV2Router02.json");
        bytes memory routerInitCode = abi.encodePacked(routerBytecode, abi.encode(factory, PATH_USD));
        
        address router;
        assembly {
            router := create(0, add(routerInitCode, 0x20), mload(routerInitCode))
        }
        require(router != address(0), "Router deployment failed");
        console.log("UniswapV2Router02 deployed at:", router);

        vm.stopBroadcast();

        console.log("\n========================================");
        console.log("UNIV2 DEPLOYMENT SUMMARY");
        console.log("========================================");
        console.log("Factory:", factory);
        console.log("Router:", router);
        console.log("WETH (PATH_USD):", PATH_USD);
    }
}
