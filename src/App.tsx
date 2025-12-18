import { useStore } from "@nanostores/react";
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
	Typography,
} from "@nipsysdev/lsd-react";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
import StorageConnectionDialog from "./features/connection/components/ConnectionDialog";
import {
	$connectionError,
	$connectionStatus,
	$isConnectionDialogOpened,
	ConnectionStatus,
} from "./features/connection/connectionStore";
import DownloadTab from "./features/download/components/DownloadTab";
import UploadTab from "./features/upload/components/UploadTab";
import "./App.css";
import NodeTab from "./features/node-info/components/NodeTab";
import {
	$nodeAddresses,
	$nodePeerId,
	$nodeVersion,
} from "./features/node-info/nodeStore";

function App() {
	const connectionStatus = useStore($connectionStatus);
	const nodePeerId = useStore($nodePeerId);
	const nodeVersion = useStore($nodeVersion);
	const connectionError = useStore($connectionError);

	useEffect(() => {
		// Show connection dialog immediately on app load
		$isConnectionDialogOpened.set(true);

		// Update status from backend on mount
		const updateStatus = async () => {
			try {
				const status = await invoke<ConnectionStatus>("get_storage_status");
				$connectionStatus.set(status);

				const error = await invoke<string | null>("get_storage_error");
				$connectionError.set(error);

				const peerId = await invoke<string | null>("get_storage_peer_id");
				$nodePeerId.set(peerId);

				const version = await invoke<string | null>("get_storage_version");
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
				console.error("Failed to update Storage status:", error);
			}
		};

		updateStatus();
		const interval = setInterval(updateStatus, 2000);
		return () => clearInterval(interval);
	}, []);

	const getStatusText = () => {
		switch (connectionStatus) {
			case ConnectionStatus.Connected:
				return "Connected";
			case ConnectionStatus.Connecting:
				return "Connecting...";
			case ConnectionStatus.Error:
				return "Error";
			case ConnectionStatus.Disconnected:
			default:
				return "Disconnected";
		}
	};

	const getStatusColor = () => {
		switch (connectionStatus) {
			case ConnectionStatus.Connected:
				return "primary";
			case ConnectionStatus.Connecting:
				return "secondary";
			case ConnectionStatus.Error:
				return "secondary";
			case ConnectionStatus.Disconnected:
			default:
				return "secondary";
		}
	};

	const openConnectionDialog = () => {
		$isConnectionDialogOpened.set(true);
	};

	return (
		<div className="size-full flex flex-col bg-lsd-surface-primary pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]">
			<header className="flex p-6 justify-between items-center">
				<Typography variant="h3">λ | StoreMan</Typography>
				<div className="flex items-center space-x-4">
					<Typography
						variant="subtitle1"
						color={getStatusColor() as any}
						className="cursor-pointer font-bold hover:opacity-80"
						onClick={openConnectionDialog}
						title={
							connectionError ||
							`Peer ID: ${nodePeerId || "N/A"}\nVersion: ${nodeVersion || "N/A"}`
						}
					>
						{getStatusText()}
					</Typography>
				</div>
			</header>

			<Tabs defaultValue="upload" className="flex-auto flex flex-col px-0.5">
				<TabsList fullWidth>
					<TabsTrigger value="upload">Upload</TabsTrigger>
					<TabsTrigger value="download">Download</TabsTrigger>
					<TabsTrigger value="node">Node</TabsTrigger>
					<TabsTrigger value="peers">Peers</TabsTrigger>
				</TabsList>

				<TabsContent value="upload" className="flex-auto mt-0 mb-0">
					<UploadTab />
				</TabsContent>

				<TabsContent value="download" className="flex-auto mt-0 mb-0">
					<DownloadTab />
				</TabsContent>

				<TabsContent value="node" className="flex-auto mt-0 mb-0">
					<NodeTab />
				</TabsContent>
			</Tabs>

			<StorageConnectionDialog />
		</div>
	);
}

export default App;
