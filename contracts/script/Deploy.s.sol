// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/MultiVerse.sol";
import "../src/TrustedOracle.sol";
import "../src/CouncilOracle.sol";

/**
 * @title Deploy
 * @notice Deploys Binary core contracts (MultiVerse + TrustedOracle, and CouncilOracle on request)
 * @dev Markets are created via the admin API, not in this script.
 *      Set COUNCIL_SIGNER (the council's record signer address) to also deploy CouncilOracle,
 *      which resolves markets from signed AI council decision records. It is deployed last so
 *      the other contracts keep their addresses. The deployer becomes its owner and resolver;
 *      COUNCIL_RESOLVER optionally adds another resolver (such as a separate backend key).
 */
contract Deploy is Script {
    function run() external {
        address councilSigner = vm.envOr("COUNCIL_SIGNER", address(0));
        address councilResolver = vm.envOr("COUNCIL_RESOLVER", address(0));

        vm.startBroadcast();

        TrustedOracle oracle = new TrustedOracle();
        console.log("TrustedOracle deployed at:", address(oracle));

        MultiVerse multiverse = new MultiVerse();
        console.log("MultiVerse deployed at:", address(multiverse));

        CouncilOracle councilOracle;
        if (councilSigner != address(0)) {
            councilOracle = new CouncilOracle(councilSigner);
            if (councilResolver != address(0)) councilOracle.setResolver(councilResolver, true);
            console.log("CouncilOracle deployed at:", address(councilOracle));
        }

        vm.stopBroadcast();

        console.log("\n========================================");
        console.log("DEPLOYMENT SUMMARY");
        console.log("========================================");
        console.log("TrustedOracle:", address(oracle));
        console.log("MultiVerse:", address(multiverse));
        if (address(councilOracle) != address(0)) {
            console.log("CouncilOracle:", address(councilOracle));
        }
    }
}
