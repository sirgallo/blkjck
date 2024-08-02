<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';

import { networkClusters } from '@app/common/types/Cluster';
import { useNetworkStore } from '@stores/network/network.store';

import network from '@components/network/network.vue';


const networkStore = useNetworkStore();
const { cluster } = storeToRefs(networkStore);

const showNotification = ref(false);
const notificationMessage = computed(() => `current cluster: ${cluster.value}`);

const selected = ref(cluster.value)
const options = ref(networkClusters.map(c =>  ({ label: c, action: () => c })));

watch(selected, selected => {
  if (cluster.value != selected) {
    cluster.value = selected;
    showNotification.value = true;
  }
})
</script>

<template>
  
 <!--<v-notification 
    :showNotification="showNotification"
    :timeToLive="3000"
    :detail="notificationMessage" 
    summary="update cluster"
    severity="info">
  </v-notification>-->

  <network></network>

  <v-dropdown
    v-model:selection="selected"
    :button="{ icon: 'fa-solid fa-gear', overrideBtnClass: 'network-button' }"
    :options="options">
  </v-dropdown>

</template>

<style lang="scss">
@import '@components/settings/settings.scss';
</style>