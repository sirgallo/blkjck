<script setup lang="ts">
import { ref, watch } from 'vue';

import { useWalletStore } from '@wallet/store/wallet.store';
import { useBalance } from '@wallet/composables/useBalance'


const walletStore = useWalletStore();
const { balance } = useBalance();

const selected = ref<string>();
const options = ref([
  { label: 'disconnect', action: async () => 'disconnect' }
]);

watch(selected, async selected => { if (selected) await walletStore.disconnect(); });
</script>

<template>

  <v-dropdown
    v-model:selection="selected"
    :options="options"
    :button="{ message: `SOL: ${balance}`, overrideBtnClass: 'wallet-button' }">
  </v-dropdown>

</template>

<style lang="scss">
@import '@wallet/wallet.scss';
</style>