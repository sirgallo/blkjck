import { type Ref, type ShallowRef, ref, shallowRef, watch } from 'vue';
import { type Cluster, Connection, clusterApiUrl } from '@solana/web3.js';

import { useLocalStorageStore } from '@stores/local-storage/local-storage.store';


const DEFAULT_NETWORK: Cluster = 'devnet';
const createConnection = (cluster: Cluster): Connection => new Connection(clusterApiUrl(cluster));

export const useConnection = () => {
  const { getItem, setItem } = useLocalStorageStore();

  const cluster: Ref<Cluster> = ref(getItem('cluster') as Cluster ?? DEFAULT_NETWORK);
  const connection: ShallowRef<Connection> = shallowRef(new Connection(clusterApiUrl(cluster?.value ?? DEFAULT_NETWORK)));
  const setConnection = () => connection.value = createConnection(cluster.value);

  watch(cluster, (incoming, old) => {
    if (incoming !== old) {
      setItem('cluster', cluster.value); 
      setConnection();
    }
  });

  return { cluster, connection };
}