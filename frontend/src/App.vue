<script setup lang="ts">
import settings from '@components/settings/settings.vue';
import wallet from '@components/wallet/wallet.vue';

import { ref } from 'vue';
import { RouterView } from 'vue-router'


const routerLinks = ref([
  { path: '/', label: 'overview' }
]);
</script>

<template>

  <div class="app-container">

    <v-nav
      :title="{ title: 'blkjck', subTitle: 'blackjack...but crypto' }"
      :routerLinks="routerLinks">
      <template #actions>
        <settings></settings>
        <wallet></wallet>
      </template>
    </v-nav>

    <div class="app-content">
      <RouterView></RouterView>
    </div>
    
  </div>

</template>

<style lang="scss">
@import '@app/App.scss';
</style>