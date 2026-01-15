/**
 * Utility to parse blockchain/viem errors and return user-friendly messages
 */

interface ParsedError {
  title: string;
  message: string;
  suggestion?: string;
}

/**
 * Common error patterns and their user-friendly messages
 */
const ERROR_PATTERNS: Array<{
  pattern: RegExp | string;
  result: ParsedError;
}> = [
  // Liquidity errors
  {
    pattern: /INSUFFICIENT_LIQUIDITY/i,
    result: {
      title: "Insufficient Liquidity",
      message: "There isn't enough liquidity in the pool to complete this trade.",
      suggestion: "Try a smaller amount or wait for more liquidity to be added.",
    },
  },
  {
    pattern: /INSUFFICIENT_OUTPUT_AMOUNT/i,
    result: {
      title: "Insufficient Output",
      message: "The trade would result in less tokens than the minimum acceptable.",
      suggestion: "Try a smaller amount or increase slippage tolerance.",
    },
  },
  {
    pattern: /INSUFFICIENT_INPUT_AMOUNT/i,
    result: {
      title: "Insufficient Input",
      message: "The input amount is too small for this trade.",
      suggestion: "Try a larger amount.",
    },
  },
  // Balance errors
  {
    pattern: /insufficient funds for gas/i,
    result: {
      title: "Insufficient Gas",
      message: "You don't have enough tokens to pay for transaction fees.",
      suggestion: "Get some tokens from the faucet first.",
    },
  },
  {
    pattern: /insufficient balance/i,
    result: {
      title: "Insufficient Balance",
      message: "You don't have enough tokens for this transaction.",
      suggestion: "Check your balance and try a smaller amount.",
    },
  },
  {
    pattern: /transfer amount exceeds balance/i,
    result: {
      title: "Insufficient Balance",
      message: "You're trying to transfer more tokens than you have.",
      suggestion: "Check your balance and try a smaller amount.",
    },
  },
  // Approval errors
  {
    pattern: /insufficient allowance/i,
    result: {
      title: "Approval Required",
      message: "Token approval is required for this transaction.",
      suggestion: "The app should handle this automatically. Try again.",
    },
  },
  // Slippage/deadline errors
  {
    pattern: /EXPIRED/i,
    result: {
      title: "Transaction Expired",
      message: "The transaction took too long and expired.",
      suggestion: "Try the transaction again.",
    },
  },
  {
    pattern: /price.*changed|slippage/i,
    result: {
      title: "Price Changed",
      message: "The price changed while your transaction was pending.",
      suggestion: "Try again. The new quote will reflect current prices.",
    },
  },
  // User rejection
  {
    pattern: /user rejected|user denied|cancelled/i,
    result: {
      title: "Transaction Cancelled",
      message: "You cancelled the transaction.",
      suggestion: undefined,
    },
  },
  // Network errors
  {
    pattern: /network|connection|timeout/i,
    result: {
      title: "Network Error",
      message: "There was a problem connecting to the network.",
      suggestion: "Check your connection and try again.",
    },
  },
  // Contract errors
  {
    pattern: /execution reverted/i,
    result: {
      title: "Transaction Failed",
      message: "The transaction was rejected by the contract.",
      suggestion: "This may be due to price changes. Try again with a fresh quote.",
    },
  },
  // K invariant (AMM math error - usually means bad reserves)
  {
    pattern: /UniswapV2: K/i,
    result: {
      title: "AMM Error",
      message: "The liquidity pool is in an invalid state.",
      suggestion: "Try a smaller amount or contact support.",
    },
  },
  // Pair doesn't exist
  {
    pattern: /ZERO_ADDRESS|pair.*not.*exist/i,
    result: {
      title: "No Liquidity Pool",
      message: "No liquidity pool exists for this trading pair.",
      suggestion: "This market may not have been set up correctly.",
    },
  },
];

/**
 * Parse an error and return a user-friendly message
 */
export function parseError(error: unknown): ParsedError {
  if (!error) {
    return {
      title: "Unknown Error",
      message: "An unexpected error occurred.",
      suggestion: "Please try again.",
    };
  }

  // Get the error message string
  let errorMessage: string;
  if (error instanceof Error) {
    // Check for nested error messages (common in viem)
    errorMessage = error.message;
    // Also check cause if present
    if ("cause" in error && error.cause instanceof Error) {
      errorMessage += " " + error.cause.message;
    }
    // Check for shortMessage (viem specific)
    if ("shortMessage" in error && typeof error.shortMessage === "string") {
      errorMessage = error.shortMessage + " " + errorMessage;
    }
  } else if (typeof error === "string") {
    errorMessage = error;
  } else {
    errorMessage = JSON.stringify(error);
  }

  // Try to match against known patterns
  for (const { pattern, result } of ERROR_PATTERNS) {
    if (typeof pattern === "string") {
      if (errorMessage.includes(pattern)) {
        return result;
      }
    } else if (pattern.test(errorMessage)) {
      return result;
    }
  }

  // Default: return a truncated version of the original error
  const truncated =
    errorMessage.length > 200
      ? errorMessage.substring(0, 200) + "..."
      : errorMessage;

  return {
    title: "Transaction Failed",
    message: truncated,
    suggestion: "Please try again or contact support if the issue persists.",
  };
}

/**
 * Get a simple one-line error message for inline display
 */
export function getErrorMessage(error: unknown): string {
  const parsed = parseError(error);
  return parsed.message;
}

/**
 * Format error for display - returns title and message
 */
export function formatError(error: unknown): { title: string; message: string } {
  const parsed = parseError(error);
  let message = parsed.message;
  if (parsed.suggestion) {
    message += " " + parsed.suggestion;
  }
  return {
    title: parsed.title,
    message,
  };
}
