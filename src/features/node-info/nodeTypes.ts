export interface NodeInfo {
    peer_id: string
    repo_path: string
    version: string
    debug_info: DebugInfo | null
}

export interface DebugInfo {
    id: string
    spr: string
    addrs: string[]
    announceAddresses: string[]
    table: {
        localNode: Node,
        nodes: Node[]
    }
}

export interface Node {
    address: string
    nodeId: string
    peerId: string
    record: string
    seen: boolean
}