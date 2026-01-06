#!/usr/bin/env bun
import { getTempoClient } from "./lib/tempo.ts";
import { setUserFeeToken } from "./lib/fee-amm.ts";

const rpcUrl = process.env.RPC_URL || "http://localhost:9545";
const client = getTempoClient({ rpcUrl });
console.log("Setting fee token...");
await setUserFeeToken(client);
console.log("Done!");
