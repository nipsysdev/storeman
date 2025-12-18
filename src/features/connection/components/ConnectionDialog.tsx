import { useStore } from '@nanostores/react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  Progress,
  Typography,
} from '@nipsysdev/lsd-react';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import {
  $connectionError,
  $connectionStatus,
  $isConnectionDialogOpened,
  $nodeAddresses,
  $nodePeerId,
  $nodeVersion,
  ConnectionStatus,
} from '../connectionStore';

const connectToStorage = async () => {
  try {
    $connectionStatus.set(ConnectionStatus.Connecting);
    $connectionError.set(null);
    await invoke('connect_to_storage');
    // Status will be updated by the polling effect
  } catch (error) {
    console.error('Failed to connect to Storage:', error);
    $connectionError.set(error as string);
    $connectionStatus.set(ConnectionStatus.Error);
  }
};

export default function StorageConnectionDialog() {
  const isDialogOpened = useStore($isConnectionDialogOpened);
  const connectionStatus = useStore($connectionStatus);
  const connectionError = useStore($connectionError);
  const nodePeerId = useStore($nodePeerId);
  const nodeVersion = useStore($nodeVersion);
  const nodeAddresses = useStore($nodeAddresses);

  const [peerId, setPeerId] = useState("");
  const [addresses, setAddresses] = useState([""]);
  const [showPeerConnect, setShowPeerConnect] = useState(false);
  const [connectionRequested, setConnectionRequested] =useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleConnectToPeer = async () => {
    try {
      await invoke("connect_to_peer", {
        peerId,
        addresses: addresses.filter((a) => a.trim()),
      });
      setShowPeerConnect(false);
      setPeerId("");
      setAddresses([""]);
      setError(null);
    } catch (error) {
      setError(`Failed to connect: ${error}`);
    }
  };

  const handleAddAddress = () => {
    setAddresses([...addresses, ""]);
  };

  const handleUpdateAddress = (index: number, value: string) => {
    const newAddresses = [...addresses];
    newAddresses[index] = value;
    setAddresses(newAddresses);
  };

  const handleRemoveAddress = (index: number) => {
    setAddresses(addresses.filter((_, i) => i !== index));
  };


  useEffect(() => {
    if (isDialogOpened) {
      // Only attempt to connect if currently disconnected
      // This prevents trying to create a new node when one already exists
      if (connectionStatus === ConnectionStatus.Disconnected && !connectionRequested) {
        setConnectionRequested(true)
        connectToStorage();
      }
    }
  }, [isDialogOpened, connectionRequested, connectionStatus]);

  useEffect(() => {
    // Update status immediately when dialog opens to ensure fresh data
    // The main App component handles the regular polling
    const updateStatusOnce = async () => {
      try {
        const status = await invoke<ConnectionStatus>('get_storage_status');
        $connectionStatus.set(status);
        
        const error = await invoke<string | null>('get_storage_error');
        $connectionError.set(error);
        
        const peerId = await invoke<string | null>('get_storage_peer_id');
        $nodePeerId.set(peerId);
        
        const version = await invoke<string | null>('get_storage_version');
        $nodeVersion.set(version);

        // Also update node addresses
        try {
          const addresses = await invoke<string[]>("get_node_addresses");
          $nodeAddresses.set(addresses);
        } catch (addrError) {
          // Don't fail the whole status update if addresses fail
          console.warn("Failed to get node addresses:", addrError);
        }
      } catch (error) {
        console.error('Failed to update Storage status:', error);
      }
    };

    // Update status immediately when dialog opens
    if (isDialogOpened) {
      updateStatusOnce();
    }
  }, [isDialogOpened]);

  const disconnectFromStorage = async () => {
    try {
      await invoke('disconnect_from_storage');
      $connectionStatus.set(ConnectionStatus.Disconnected);
      $connectionError.set(null);
      $nodePeerId.set(null);
      $nodeVersion.set(null);
    } catch (error) {
      console.error('Failed to disconnect from Storage:', error);
      $connectionError.set(error as string);
    }
  };

  const getStatusDescription = (status: ConnectionStatus) => {
    switch (status) {
      case ConnectionStatus.Connecting:
        return 'Connecting to Storage network...';
      case ConnectionStatus.Connected:
        return 'Connected to Storage successfully';
      case ConnectionStatus.Error:
        return connectionError || 'An error occurred while connecting to Storage';
      case ConnectionStatus.Disconnected:
        return 'Disconnected from Storage network';
      default:
        return '';
    }
  };


  return (
    <Dialog
      open={isDialogOpened}
      onOpenChange={(open) => {
        $isConnectionDialogOpened.set(open);
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            Connection status: <span className="capitalize">{connectionStatus}</span>
          </DialogTitle>
          <DialogDescription>
            {getStatusDescription(connectionStatus)}
          </DialogDescription>
        </DialogHeader>

        <Progress
          indeterminate={connectionStatus === ConnectionStatus.Connecting}
          value={connectionStatus === ConnectionStatus.Connected ? 100 : undefined}
        />

        {connectionStatus === ConnectionStatus.Connected && (
          <div className="mt-4 space-y-2">
            {nodeVersion && (
              <Typography variant="body2">
                Storage Version: {nodeVersion}
              </Typography>
            )}
            {nodePeerId && (
              <div className="flex flex-col">
                <Typography variant="body2">
                Peer ID:
              </Typography>
                <Typography variant="body2" style={{"overflowWrap": "anywhere"}}>
                {nodePeerId}
              </Typography>

                </div>
            )}

            {/* Show user's node addresses */}
            {nodeAddresses.length > 0 && (
              <div className="flex flex-col mt-2">
                <Typography variant="body2" color="secondary">
                  Your Addresses (share these with others):
                </Typography>
                {nodeAddresses.map((addr, index) => (
                  <Typography
                    key={`addr-${index}-${addr.slice(0, 10)}`}
                    variant="body2"
                    className="font-mono text-xs break-all"
                  >
                    {addr}
                  </Typography>
                ))}
              </div>
            )}

            <button
              type="button"
              onClick={() => setShowPeerConnect(!showPeerConnect)}
              className="px-3 py-1 bg-lsd-surface-secondary hover:bg-lsd-surface-tertiary rounded-md text-sm transition-colors"
            >
              {showPeerConnect ? "Hide" : "Show"} Peer Connection
            </button>

            {showPeerConnect && (
              <div className="mt-4 space-y-4 p-4 bg-lsd-surface-secondary rounded-md">
                <Typography variant="h6">Connect to Peer</Typography>

                <div>
                  <div className="block text-sm font-medium mb-1">Peer ID:</div>
                  <input
                    type="text"
                    value={peerId}
                    onChange={(e) => setPeerId(e.target.value)}
                    placeholder="12D3KooW..."
                    className="w-full px-3 py-2 border border-lsd-border rounded-md bg-lsd-surface-primary"
                  />
                </div>

                <div>
                  <div className="block text-sm font-medium mb-1">Addresses:</div>
                  {addresses.map((addr, index) => (
                    <div key={`addr-${index}-${addr.slice(0, 8)}`} className="flex space-x-2 mb-2 items-center">
                      <input
                        type="text"
                        value={addr}
                        onChange={(e) => handleUpdateAddress(index, e.target.value)}
                        placeholder="/ip4/192.168.1.100/tcp/8080"
                        className="flex-1 px-3 py-2 border border-lsd-border rounded-md bg-lsd-surface-primary"
                      />
                      {addresses.length > 1 && (
                        <Button
                          size="sm"
                          variant="outlined"
                          onClick={() => handleRemoveAddress(index)}
                        >
                          Remove
                        </Button>
                      )}
                    </div>
                  ))}

                  <Button
                    onClick={handleAddAddress}
                    size="sm"
                    variant="outlined"
                  >
                    Add Address
                  </Button>
                </div>

                {error && (
                  <Typography variant="body2" color="secondary">
                    {error}
                  </Typography>
                )}

                <div className="flex space-x-2">
                  <Button
                    variant="outlined"
                    onClick={handleConnectToPeer}
                    disabled={!peerId.trim() || !addresses.some((a) => a.trim())}
                  >
                    Connect to Peer
                  </Button>
                </div>
              </div>
            )}
          </div>
        )}

        {connectionStatus === ConnectionStatus.Error && (
          <div className="mt-4">
            <Typography variant="body2" color="secondary">
              {connectionError}
            </Typography>
          </div>
        )}

        <div className="mt-6 flex justify-end space-x-2">
          <button
              type="button"
              onClick={disconnectFromStorage}
              className="px-4 py-2 bg-lsd-surface-secondary hover:bg-lsd-surface-tertiary rounded-md transition-colors"
            >
              Disconnect
            </button>
          
          {(connectionStatus === ConnectionStatus.Disconnected ||
            connectionStatus === ConnectionStatus.Error) && (
            <button
              type="button"
              onClick={connectToStorage}
              className="px-4 py-2 bg-lsd-primary hover:bg-lsd-primary-hover text-white rounded-md transition-colors"
            >
              Connect
            </button>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}