<script setup lang="ts">
import { ref, watch } from 'vue';
import { map, transform } from 'lodash';
import type { SignerWalletAdapter } from '@solana/wallet-adapter-base';

import { useWalletStore } from '@wallet/store/wallet.store'
import { getWallets } from '@wallet/store/utils/Wallet';
import { storeToRefs } from 'pinia';

const walletStore = useWalletStore();
const { wallet }  = storeToRefs(walletStore);

const walletNameToWalletMap = transform(
  getWallets(),
  (acc, value) => acc[value.name] = value,
  {} as { [walletName: string]: SignerWalletAdapter }
);

const selected = ref<string>();
const options = map(getWallets(), wallet => ({ label: wallet.name, action: () => wallet.name }));

watch(selected, async selected => {
  if (selected && selected !== wallet.value?.name) {
    await walletStore.connect(walletNameToWalletMap[selected]);
  }
});
</script>

<template> 

  <!--<div class="wallet-button-container">
    <SplitButton :model="options" class="v-wallet-button">
      <span class="dialog">Connect</span>
    </SplitButton>
  </div>-->
  <v-dropdown
    v-model:selection="selected"
    :options="options"
    :button="{ message: 'connect', overrideBtnClass: 'wallet-button' }">
  </v-dropdown>
</template>

<style lang="scss">
@import '@wallet/wallet.scss';
</style>