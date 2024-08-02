import { type Ref, ref, watchEffect } from 'vue';

import { useWalletStore } from '@app/components/wallet/store/wallet.store';


export const useWalletSendTransaction = (txSize: number, programId: string) => {
  const walletStore = useWalletStore();

  const loading: Ref<boolean> = ref(false);
  const error = ref<Error>();
  const txId = ref<string>();

  const sendTx = async () => {
    try {
      await walletStore.sendTransaction(txSize, programId);
    } catch(err: any) {
      loading.value = false;
      error.value = err
    } finally { loading.value = false; }
  };

  watchEffect(async () => { if (loading.value) await sendTx(); });
  loading.value = true;
  
  return { txId, loading, error };
}