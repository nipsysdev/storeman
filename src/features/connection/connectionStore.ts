import { atom, computed, onMount } from 'nanostores';
import { updateNodeInfo, updateNodeStatus } from '../node/nodeService';
import { connectToStorage } from './connectionService';
import { ConnectionStatus } from './connectionTypes';

export const $isConnectionDialogOpened = atom(false);
export const $connectionStatus = atom<ConnectionStatus>(ConnectionStatus.Disconnected);
export const $connectionError = atom<string | null>(null);

export const $isConnected = computed($connectionStatus, connectionStatus => connectionStatus === ConnectionStatus.Connected)

onMount($connectionStatus, () => {
  updateNodeStatus().then((status) => {
        if (
          status === ConnectionStatus.Disconnected ||
          status === ConnectionStatus.Initialized
        ) {
          connectToStorage();
          $isConnectionDialogOpened.set(true);
        }
      });
  
      const nodePolling = async () => {
        try {
          await updateNodeStatus();
          await updateNodeInfo();
        } catch (error) {
          console.error("Failed to update Storage status:", error);
        }
      };
  
      nodePolling();
      const interval = setInterval(nodePolling, 500);
  return () => {
    clearInterval(interval)
  }
})