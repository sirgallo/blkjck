import type { SignerWalletAdapter } from '@solana/wallet-adapter-base';
import { PhantomWalletAdapter, LedgerWalletAdapter } from '@solana/wallet-adapter-wallets';


export const getWallets = (): SignerWalletAdapter[] => {
  return [
    new LedgerWalletAdapter(),
    new PhantomWalletAdapter()
  ];
};