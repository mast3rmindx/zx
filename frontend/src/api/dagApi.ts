import axios from 'axios';
import { Block, Node, GraphData } from '../types/dag';

const API_BASE_URL = 'http://localhost:8000';

export const dagApi = {
    // Get all blocks in the DAG
    async getBlocks(): Promise<Block[]> {
        const response = await axios.get(`${API_BASE_URL}/blocks`);
        return response.data;
    },

    // Get blocks in the k-cluster
    async getKCluster(): Promise<Block[]> {
        const response = await axios.get(`${API_BASE_URL}/blocks/k-cluster`);
        return response.data;
    },

    // Get confirmed blocks
    async getConfirmedBlocks(): Promise<Block[]> {
        const response = await axios.get(`${API_BASE_URL}/blocks/confirmed`);
        return response.data;
    },

    // Get active nodes
    async getActiveNodes(): Promise<Node[]> {
        const response = await axios.get(`${API_BASE_URL}/nodes/active`);
        return response.data;
    },

    // Convert blocks to graph data
    convertToGraphData(blocks: Block[]): GraphData {
        const nodes = blocks.map(block => ({
            id: block.id,
            height: block.height,
            confidence: block.confidence,
            in_k_cluster: block.in_k_cluster,
            network_delay: block.network_delay
        }));

        const links: { source: string; target: string }[] = [];
        blocks.forEach(block => {
            block.parents.forEach(parentId => {
                links.push({
                    source: parentId,
                    target: block.id
                });
            });
        });

        return { nodes, links };
    }
}; 