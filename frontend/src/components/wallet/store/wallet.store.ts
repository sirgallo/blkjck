import { type Ref, ref, shallowRef, watch } from 'vue';
import { defineStore } from 'pinia';
import { PublicKey, LAMPORTS_PER_SOL, SystemProgram, TransactionMessage, VersionedTransaction } from '@solana/web3.js';
import type { SignerWalletAdapter } from '@solana/wallet-adapter-base';

import { useConnection } from '@common/composables/useConnection';


export const useWalletStore = defineStore('wallet', () => {
  const { cluster, connection } = useConnection();
  
  const wallet: Ref<SignerWalletAdapter | null> = shallowRef(null);
  const connected: Ref<boolean> = ref(false);
  const pubKey: Ref<PublicKey | null> = ref(null);
  const balance: Ref<number> = ref(0);

  const connect = async (selectedWallet: SignerWalletAdapter) => {
    if (connected.value) return;

    try {
      wallet.value = selectedWallet;
      await wallet.value.connect();
      
      pubKey.value = wallet.value.publicKey;
      await getBalance();
    } catch (err) {
      throw err; 
    } finally { connected.value = true; }
  };

  const sendTransaction = async (tSize: number, programId: string) => {
    if (! pubKey.value || ! connection.value || ! wallet.value) throw new Error('wallet not initialized, unable to send tx');

    try {
      const { blockhash, lastValidBlockHeight }= await connection.value.getLatestBlockhash();

      const instructions = [
        SystemProgram.transfer({
          fromPubkey: pubKey.value,
          toPubkey: new PublicKey(programId),
          lamports: tSize * LAMPORTS_PER_SOL
        })
      ];
  
      const message = new TransactionMessage({
        payerKey: pubKey.value,
        recentBlockhash: blockhash,
        instructions
      }).compileToV0Message();
  
      const vTransaction = new VersionedTransaction(message);

      const signature = await wallet.value.sendTransaction(vTransaction, connection.value);
      await connection.value.confirmTransaction({ signature, blockhash, lastValidBlockHeight });
      
      await getBalance();
    } catch (err) { throw err; }
  }

  const getBalance = async () => {
    if (! pubKey.value) throw new Error('public key not available');
    
    try {
      const lamports = await connection.value?.getBalance(pubKey.value, 'confirmed') ?? 0;
      balance.value = lamports > 0 ? lamports / LAMPORTS_PER_SOL : 0;
    } catch(err) { throw err; }
  };
    
  const disconnect = async () => {
    if (! wallet.value || ! connected.value) return;

    try {
      await wallet.value.disconnect();
    } catch (err) {
      throw err; 
    } finally { connected.value = false; }
  };

  watch(cluster, async (incoming, old) => {
    if (incoming != old) { await getBalance(); }
  });

  return {
    balance, cluster, connection, connected, pubKey, wallet,
    connect, sendTransaction, getBalance, disconnect
  };
});