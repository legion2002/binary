import { useState, useCallback, useEffect } from "react";
import { Account, WebAuthnP256 } from "viem/tempo";

const STORAGE_KEY = "binary_passkey_credential";

interface StoredCredential {
  id: string;
  publicKey: `0x${string}`;
}

export function usePasskeyAccount() {
  const [account, setAccount] = useState<ReturnType<
    typeof Account.fromWebAuthnP256
  > | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  // Try to restore session on mount
  useEffect(() => {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      // Don't auto-sign-in, just indicate there's a saved credential
      // User needs to explicitly sign in to trigger passkey prompt
    }
  }, []);

  // Sign up - create new passkey
  const signUp = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const credential = await WebAuthnP256.createCredential({
        name: "Binary Markets",
      });

      // Store credential ID and public key for future sign-ins
      const storedCred: StoredCredential = {
        id: credential.id,
        publicKey: credential.publicKey,
      };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(storedCred));

      const acc = Account.fromWebAuthnP256({ credential });
      setAccount(acc);
      return acc;
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      setError(error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Sign in - load existing passkey
  const signIn = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (!stored) {
        throw new Error("No saved credential found. Please sign up first.");
      }

      const { id, publicKey } = JSON.parse(stored) as StoredCredential;

      const credential = await WebAuthnP256.getCredential({
        credentialId: id,
        getPublicKey: async () => publicKey,
      });

      const acc = Account.fromWebAuthnP256({ credential });
      setAccount(acc);
      return acc;
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      setError(error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Sign out - clear current session (keeps stored credential for re-signin)
  const signOut = useCallback(() => {
    setAccount(null);
    setError(null);
  }, []);

  // Clear credential - fully remove saved passkey
  const clearCredential = useCallback(() => {
    localStorage.removeItem(STORAGE_KEY);
    setAccount(null);
    setError(null);
  }, []);

  // Check if there's a saved credential
  const hasSavedCredential = useCallback(() => {
    return localStorage.getItem(STORAGE_KEY) !== null;
  }, []);

  return {
    account,
    address: account?.address,
    isConnected: !!account,
    signUp,
    signIn,
    signOut,
    clearCredential,
    hasSavedCredential,
    isLoading,
    error,
  };
}
