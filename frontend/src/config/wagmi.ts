import { unstable_porto } from "porto/wagmi";
import { http, createConfig, createStorage } from "wagmi";
import { base } from "wagmi/chains";

export const wagmiConfig = createConfig({
  chains: [base],
  connectors: [unstable_porto()],
  storage: createStorage({ storage: localStorage }),
  transports: {
    [base.id]: http(),
  },
});
