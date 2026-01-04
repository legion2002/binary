// Multiverse prediction market contract ABI
export const MULTIVERSE_ABI = [
  {
    type: "function",
    name: "combine",
    inputs: [
      { name: "asset", type: "address", internalType: "address" },
      { name: "amount", type: "uint256", internalType: "uint256" },
      { name: "marketHash", type: "bytes32", internalType: "bytes32" },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "create",
    inputs: [
      { name: "asset", type: "address", internalType: "address" },
      { name: "marketHash", type: "bytes32", internalType: "bytes32" },
    ],
    outputs: [
      { name: "yesVerse", type: "address", internalType: "address" },
      { name: "noVerse", type: "address", internalType: "address" },
    ],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "getVerseAddress",
    inputs: [
      { name: "asset", type: "address", internalType: "address" },
      { name: "marketHash", type: "bytes32", internalType: "bytes32" },
    ],
    outputs: [
      { name: "yesVerse", type: "address", internalType: "address" },
      { name: "noVerse", type: "address", internalType: "address" },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "isVerse",
    inputs: [{ name: "verse", type: "address", internalType: "address" }],
    outputs: [{ name: "", type: "bool", internalType: "bool" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "markets",
    inputs: [{ name: "marketHash", type: "bytes32", internalType: "bytes32" }],
    outputs: [
      { name: "resolutionDeadline", type: "uint32", internalType: "uint32" },
      { name: "oracle", type: "address", internalType: "address" },
      { name: "questionHash", type: "bytes32", internalType: "bytes32" },
      {
        name: "resolution",
        type: "uint8",
        internalType: "enum MultiVerse.Resolution",
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "open",
    inputs: [
      { name: "questionHash", type: "bytes32", internalType: "bytes32" },
      { name: "resolutionDeadline", type: "uint32", internalType: "uint32" },
      { name: "oracle", type: "address", internalType: "address" },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "redeem",
    inputs: [
      { name: "verse", type: "address", internalType: "address" },
      { name: "amount", type: "uint256", internalType: "uint256" },
    ],
    outputs: [
      { name: "redeemedAmount", type: "uint256", internalType: "uint256" },
    ],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "resolve",
    inputs: [{ name: "marketHash", type: "bytes32", internalType: "bytes32" }],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "split",
    inputs: [
      { name: "asset", type: "address", internalType: "address" },
      { name: "amount", type: "uint256", internalType: "uint256" },
      { name: "marketHash", type: "bytes32", internalType: "bytes32" },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  { type: "error", name: "InvalidMarketState", inputs: [] },
  { type: "error", name: "InvalidResolution", inputs: [] },
  { type: "error", name: "InvalidVerse", inputs: [] },
  { type: "error", name: "MarketAlreadyOpened", inputs: [] },
  { type: "error", name: "VersesAlreadyCreated", inputs: [] },
] as const;

// Standard ERC20 ABI - kept for compatibility with custom tokens
// Note: On Tempo, prefer using viem/tempo Actions.token for TIP20 tokens
export const ERC20_ABI = [
  {
    type: "function",
    name: "allowance",
    inputs: [
      { name: "owner", type: "address", internalType: "address" },
      { name: "spender", type: "address", internalType: "address" },
    ],
    outputs: [{ name: "", type: "uint256", internalType: "uint256" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "approve",
    inputs: [
      { name: "spender", type: "address", internalType: "address" },
      { name: "amount", type: "uint256", internalType: "uint256" },
    ],
    outputs: [{ name: "", type: "bool", internalType: "bool" }],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "balanceOf",
    inputs: [{ name: "account", type: "address", internalType: "address" }],
    outputs: [{ name: "", type: "uint256", internalType: "uint256" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "decimals",
    inputs: [],
    outputs: [{ name: "", type: "uint8", internalType: "uint8" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "name",
    inputs: [],
    outputs: [{ name: "", type: "string", internalType: "string" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "symbol",
    inputs: [],
    outputs: [{ name: "", type: "string", internalType: "string" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "totalSupply",
    inputs: [],
    outputs: [{ name: "", type: "uint256", internalType: "uint256" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "transfer",
    inputs: [
      { name: "to", type: "address", internalType: "address" },
      { name: "amount", type: "uint256", internalType: "uint256" },
    ],
    outputs: [{ name: "", type: "bool", internalType: "bool" }],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "transferFrom",
    inputs: [
      { name: "from", type: "address", internalType: "address" },
      { name: "to", type: "address", internalType: "address" },
      { name: "amount", type: "uint256", internalType: "uint256" },
    ],
    outputs: [{ name: "", type: "bool", internalType: "bool" }],
    stateMutability: "nonpayable",
  },
] as const;
