import { defineStore } from 'pinia';

import { useLocalStorage } from '@app/common/composables/useLocalStorage';


export const useLocalStorageStore = defineStore('local-storage', () => {
  const { updateValueForKeyMap, setItem, getItem, deleteItem, clear } = useLocalStorage();
  return { updateValueForKeyMap, setItem, getItem, deleteItem, clear };
});