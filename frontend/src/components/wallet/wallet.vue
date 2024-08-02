<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useWalletStore } from '@wallet/store/wallet.store';

import walletconnect from '@wallet/connect/wallet-connect.vue';
import walletconnected from '@wallet/connected/wallet-connected.vue';

const walletStore = useWalletStore();
const { connected } = storeToRefs(walletStore);
</script>

<template>

  <!--<v-notification 
    :showNotification="connected"
    :timeToLive="3000"
    :summary="connected ? 'wallet connected' : 'wallet disconnected'"
    :detail="wallet?.name"
    severity="info">
  </v-notification>-->

  <walletconnected v-if="connected"></walletconnected>
  <walletconnect v-else></walletconnect>

</template>

<style lang="scss">
@import '@components/wallet/wallet.scss'
</style>