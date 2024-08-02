type ErrorMessage = 'WalletNotSelectedError' | 'WalletNotReadyError';

export const ERROR_MESSAGES: Record<ErrorMessage, string> = {
  WalletNotSelectedError: 'wallet not currently selected',
  WalletNotReadyError:'wallet not ready'
}