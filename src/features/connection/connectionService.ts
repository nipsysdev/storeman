import { invoke } from "@tauri-apps/api/core";
import { $connectionError, $connectionStatus } from "./connectionStore";
import { ConnectionStatus } from "./connectionTypes";

export async function connectToStorage() {
	try {
		$connectionStatus.set(ConnectionStatus.Connecting);
		$connectionError.set(null);
		await invoke("start_node");
	} catch (error) {
		console.error("Failed to connect to Storage:", error);
		$connectionError.set(error as string);
		$connectionStatus.set(ConnectionStatus.Error);
	}
};

export async function disconnectFromStorage() {
        try {
            await invoke("stop_node");
            $connectionStatus.set(ConnectionStatus.Disconnected);
            $connectionError.set(null);
        } catch (error) {
            console.error("Failed to disconnect from Storage:", error);
            $connectionError.set(error as string);
        }
    };
