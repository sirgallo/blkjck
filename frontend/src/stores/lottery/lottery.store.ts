import { type Ref, ref, computed } from 'vue';
import { defineStore } from 'pinia';

import { LOCAL_PROGRAM_ID } from '@app/common/types/Program';
import { cleanValue, withinRange } from '@app/common/utils/Utils';
import { useBalance } from '@app/components/wallet/composables/useBalance';
import { useWalletSendTransaction } from '@app/components/wallet/composables/useWalletSendTransaction';
import { useRequestAirdrop } from '@app/components/wallet/composables/useRequestAirdrop';


export const useLotteryStore = defineStore('lottery', () => {
  const { balance } = useBalance();

  const lotteryToggleDeposit: Ref<boolean> = ref(true);
  const enqueuedDeposit: Ref<number> = ref(0);
  const enqueuedWithdrawl: Ref<number> = ref(0);
  const stakeInPool: Ref<number> = ref(0);

  const loading: Ref<boolean> = ref(false);
  const error = ref<Error>();
  const txId = ref<string>();

  const displayCleanDeposit = computed(() => cleanValue(enqueuedDeposit.value));
  const displayCleanWithdrawl = computed(() => cleanValue(enqueuedWithdrawl.value));

  const setLotteryToggleDeposit = (truthy: boolean) => lotteryToggleDeposit.value = truthy;
  
  const setEnqueuedDeposit = (deposit: number) => {
    enqueuedDeposit.value = withinRange(deposit, balance.value)
  };
  
  const depositIntoPool = () => {
    if (enqueuedDeposit.value === 0) return;

    const tx = useWalletSendTransaction(enqueuedDeposit.value, LOCAL_PROGRAM_ID);

    loading.value = tx.loading.value;
    error.value = tx.error.value;
    txId.value = tx.txId.value
  };

  const withdrawFunds = () => {
    console.log('testing...');
  };

  const devRequestAirdrop = () => {
    const airdrop = useRequestAirdrop();

    loading.value = airdrop.loadingRef.value;
    error.value = airdrop.errorRef.value;
  };

  return {
    balance, lotteryToggleDeposit, stakeInPool, loading, error, txId,
    enqueuedDeposit, enqueuedWithdrawl, displayCleanDeposit, displayCleanWithdrawl,
    setLotteryToggleDeposit, setEnqueuedDeposit, depositIntoPool, withdrawFunds, devRequestAirdrop
  };
});