import { type Ref, ref, watchEffect } from 'vue';
import { storeToRefs } from 'pinia';

import { LAMPORTS_PER_SOL } from '@solana/web3.js';

import { useWalletStore } from '@wallet/store/wallet.store';


export const useRequestAirdrop = () => {
  const walletStore = useWalletStore();
  const { connection, pubKey } = storeToRefs(walletStore);

  const loadingRef: Ref<boolean> = ref(false);
  const errorRef = ref<Error>();

  const reqAirdrop = async () => {
    if (! pubKey.value || ! connection.value) return null;
    try {
      const { blockhash, lastValidBlockHeight } = await connection.value.getLatestBlockhash();

      const signature = await connection.value.requestAirdrop(pubKey.value, LAMPORTS_PER_SOL);
      await connection.value.confirmTransaction({ signature, blockhash, lastValidBlockHeight });
      await walletStore.getBalance();
    } catch (err) { 
      loadingRef.value = false;
      errorRef.value = err as Error;
    } finally { loadingRef.value = false; }
  };

  watchEffect(async () => { if (loadingRef.value) await reqAirdrop(); });
  loadingRef.value = true;
  
  return { loadingRef, errorRef };
};