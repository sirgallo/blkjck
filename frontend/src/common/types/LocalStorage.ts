export const keys = <const>[ 'cluster' ];
export type KeyType = typeof keys[number];

export const LOCAL_STORAGE_KEY_PREFIX = 'blkjck';
export type LocalStorageKey <T extends KeyType> = `${typeof LOCAL_STORAGE_KEY_PREFIX}-${T}`;

