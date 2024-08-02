import type { Ref } from 'vue';
import type { Cluster } from '@solana/web3.js';

import type { PerformanceMetrics } from '@app/common/types/Statistics';
import { toMs } from '@app/common/utils/Utils';


export interface NetworkState {
  cluster: Ref<Cluster>;
  metrics: Ref<PerformanceMetrics>;
  stable: Ref<boolean>;
}

export const METRICS_FETCH_INTERVAL = toMs.sec(2);
export const SOL_API_ENDPOINTS: { [cluster in Cluster]: string } = {
  'devnet': 'https://api.devnet.solana.com/',
  'mainnet-beta': 'https://api.mainnet-beta.solana.com/',
  'testnet': 'https://api.testnet.solana.com/'
};