import type { Ref } from 'vue';
import { ref } from 'vue';

import type { LocalStorageKey, KeyType } from '@common/types/LocalStorage';
import { LOCAL_STORAGE_KEY_PREFIX } from '@common/types/LocalStorage';


const genPrefixedKey = <T extends KeyType>(key: T): LocalStorageKey<T> => `${LOCAL_STORAGE_KEY_PREFIX}-${key}`;

export const useLocalStorage = () => {
  const localStorageRef = ref(localStorage);
  const updateValueForKeyMap: Ref<Partial<{ [key in KeyType]: boolean; }>> = ref({});

  const setItem = (key: KeyType, value: string) => {
    const prefixedKey = genPrefixedKey(key);

    localStorageRef.value.setItem(prefixedKey, value);
    updateValueForKeyMap.value[key] = true;
  };

  const getItem = (key: KeyType): string | undefined => {
    const prefixedKey = genPrefixedKey(key); 

    const value = localStorageRef.value.getItem(prefixedKey);
    if (value) return value;
  };

  const deleteItem = (key: KeyType) => {
    const prefixedKey = genPrefixedKey(key);

    localStorageRef.value.removeItem(prefixedKey);
    updateValueForKeyMap.value[key] = true;
  }

  const clear = () => localStorageRef.value.clear();

  return { 
    localStorageRef, updateValueForKeyMap,
    setItem, getItem, deleteItem, clear 
  };
};