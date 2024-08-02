<script setup lang="ts">
import type { Ref } from 'vue';
import { ref, watch } from 'vue';
import { storeToRefs } from 'pinia';

import { useNetworkStore } from '@stores/network/network.store';


const networkStore = useNetworkStore();
const { cluster, tps, stable } = storeToRefs(networkStore);

const networkContainerStyle= ref({ 'background-color': 'transparent', border: 'none', height: 'fit-content' });
const blinkerStyle: Ref<string> = ref('var(--v-soft-blue-highlight)');

watch(stable, newStable => {
  blinkerStyle.value = newStable ? 'var(--v-green)' : 'var(--v-soft-blue-highlight)';
});
</script>

<template>

  <v-container orientation="horizontal" :style="networkContainerStyle">
    <v-tag
      :content="{ text: cluster.toUpperCase() }"
      color="var(--v-text-primary)"
      bgColor="var(--v-green)">
    </v-tag>

    <h4>
      TPS: {{ tps }}
    </h4>

    <font-awesome-icon 
      icon="fa-solid fa-circle" 
      class="blinker-icon"
      :style="{ color: blinkerStyle }">
    </font-awesome-icon>
  </v-container>

</template>

<style lang="scss">
@import '@components/network/network.scss';
</style>