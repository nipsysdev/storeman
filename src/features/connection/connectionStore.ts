import { atom } from 'nanostores';

export enum ConnectionStatus {
  Disconnected = 'Disconnected',
  Connecting = 'Connecting',
  Connected = 'Connected',
  Error = 'Error',
}

export const $isConnectionDialogOpened = atom(false);
export const $connectionStatus = atom<ConnectionStatus>(ConnectionStatus.Disconnected);
export const $connectionError = atom<string | null>(null);