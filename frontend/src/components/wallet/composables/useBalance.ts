import { type Ref, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';

import { useWalletStore } from '@wallet/store/wallet.store';


export const useBalance = () => {
  const walletStore = useWalletStore();
  const { cluster, connected, balance } = storeToRefs(walletStore);

  const error = ref<Error>();
  const shouldUpdateBalance: Ref<boolean> = ref(false);

  watch(cluster, async () => {
    try {
      if (! connected.value) return 0;
      await walletStore.getBalance();
    } catch (err) { throw (err as Error).message; }
  });
  
  return { balance, error, shouldUpdateBalance };
}