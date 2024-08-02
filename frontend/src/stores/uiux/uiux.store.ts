import { type Ref, ref } from 'vue';
import { defineStore } from 'pinia';

import { defaultHSLColor } from '@app/common/types/Animation';


export const useUiUxStore = defineStore('uiux', () => {
  const xPos: Ref<number> = ref(defaultHSLColor);
  const setXPos = (val: number) => xPos.value = val;
  return { xPos, setXPos };
});

export const onMouseMove = (e: any) => {
  const uiuxStore = useUiUxStore();
  uiuxStore.setXPos(e.clientX);
};