import type { Address } from "viem";
import deploymentsJson from "../deployments.json";

export interface ChainDeployment {
  name: string;
  multiverse: Address;
  oracle: Address;
  uniV2Factory: Address;
  uniV2Router: Address;
}

type DeploymentsMap = Record<string, ChainDeployment>;

// Type-safe deployments with Address casting
const deployments: DeploymentsMap = Object.fromEntries(
  Object.entries(deploymentsJson).map(([chainId, deployment]) => [
    chainId,
    {
      name: deployment.name,
      multiverse: deployment.multiverse as Address,
      oracle: deployment.oracle as Address,
      uniV2Factory: deployment.uniV2Factory as Address,
      uniV2Router: deployment.uniV2Router as Address,
    },
  ])
);

export function getDeployment(chainId: number): ChainDeployment {
  const deployment = deployments[String(chainId)];
  if (!deployment) {
    throw new Error(`No deployment found for chain ${chainId}`);
  }
  return deployment;
}

export default deployments;
