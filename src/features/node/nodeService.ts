import { invoke } from "@tauri-apps/api/core";
import { $connectionStatus, type ConnectionStatus } from "../connection/connectionStore";
import { $nodeInfo } from "./nodeStore";
import type { NodeInfo } from "./nodeTypes";

export async function updateNodeStatus(): Promise<ConnectionStatus> {
    const status = await invoke<ConnectionStatus>("get_node_status");
	$connectionStatus.set(status);
    return status
}

export async function updateNodeInfo() {
    const nodeInfo = await invoke<NodeInfo | null>("get_node_info");
    console.log(nodeInfo)
    $nodeInfo.set(nodeInfo);
}