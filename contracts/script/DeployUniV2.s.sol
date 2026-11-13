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

    /// Pair init code hash hardcoded in v2-periphery's UniswapV2Library.pairFor
    bytes32 constant CANONICAL_PAIR_INIT_CODE_HASH =
        0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f;

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
        // UniswapV2Library hardcodes the init code hash of mainnet's pair bytecode. The pair built
        // here hashes differently, so point the router at the pair this factory actually deploys.
        _replaceWord(
            routerBytecode,
            CANONICAL_PAIR_INIT_CODE_HASH,
            keccak256(vm.getCode("out/UniswapV2Pair.sol/UniswapV2Pair.json"))
        );
        bytes memory routerInitCode =
            abi.encodePacked(routerBytecode, abi.encode(factory, PATH_USD));

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

    /// Replaces every 32-byte occurrence of from in code with to, in place; requires one.
    function _replaceWord(bytes memory code, bytes32 from, bytes32 to) internal pure {
        uint256 found;
        for (uint256 i = 0; i + 32 <= code.length; i++) {
            bytes32 word;
            assembly {
                word := mload(add(add(code, 0x20), i))
            }
            if (word == from) {
                assembly {
                    mstore(add(add(code, 0x20), i), to)
                }
                found++;
                i += 31;
            }
        }
        require(found > 0, "pair init code hash not found in router bytecode");
    }
}
