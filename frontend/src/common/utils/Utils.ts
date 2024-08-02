import { MAX_SIG_FIGS } from '@app/common/types/SigFigs';


export const cleanValue = (val: number): number => val ? parseFloat(val.toFixed(MAX_SIG_FIGS)) : 0;

export const toMs = {
  sec: (sec: number): number => sec * 1000,
  min: (min: number): number => min * 60000
}

export const sleep = async (timeout: number) => new Promise(res => setTimeout(res, timeout));

export const withinRange = (val: number, compareVal: number) => {
  if (val < 0) return 0;
  if (val > compareVal) return compareVal;
  
  return val;
};

export const extractErrorMessage = (err: Error) => err.message;