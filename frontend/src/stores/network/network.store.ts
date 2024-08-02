import { type Ref, ref, watch, computed } from 'vue';
import { defineStore } from 'pinia';

import { getPerformanceMetrics } from '@common/utils/Statistics';
import { usePeriodicDataFetch } from '@common/composables/usePeriodicDataFetch';
import type { PerformanceMetrics } from '@common/types/Statistics';
import { useConnection } from '@common/composables/useConnection';
import { METRICS_FETCH_INTERVAL, SOL_API_ENDPOINTS } from '@stores/network/network.store.types';


export const useNetworkStore = defineStore('network', () => {
  const { cluster } = useConnection();
  
  const metrics: Ref<PerformanceMetrics> = ref({ tps: 0 });
  const stable: Ref<boolean> = ref(false);
  const tps = computed(() => metrics.value.tps); 

  const periodicFunction = computed(() => {
    const apiEndpoint = SOL_API_ENDPOINTS[cluster.value];
    return (async () => getPerformanceMetrics(apiEndpoint));
  });

  const { data } = usePeriodicDataFetch(METRICS_FETCH_INTERVAL, periodicFunction);
  watch(data, incomingMetrics => {
    if (incomingMetrics) {
      metrics.value = incomingMetrics;
      stable.value = tps.value < 1300 ? false : true;
    }
  });

  return { cluster, tps, stable };
});