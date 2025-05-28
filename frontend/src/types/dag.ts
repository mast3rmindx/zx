// DAG Types for visualization

export interface Block {
    id: string;
    hash: string;
    parents: string[];
    timestamp: string;
    height: number;
    confidence: number;
    in_k_cluster: boolean;
    network_delay: number;
}

export interface Node {
    id: string;
    address: string;
    last_seen: string;
    status: NodeStatus;
    avg_delay: number;
    blocks_validated: number;
}

export enum NodeStatus {
    Active = 'Active',
    Inactive = 'Inactive',
    Validating = 'Validating',
    Syncing = 'Syncing'
}

export interface GraphData {
    nodes: GraphNode[];
    links: GraphLink[];
}

export interface GraphNode {
    id: string;
    height: number;
    confidence: number;
    in_k_cluster: boolean;
    network_delay: number;
}

export interface GraphLink {
    source: string;
    target: string;
} 