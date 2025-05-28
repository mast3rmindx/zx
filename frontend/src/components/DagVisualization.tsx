import React, { useEffect, useState } from 'react';
import ForceGraph2D from 'react-force-graph-2d';
import { dagApi } from '../api/dagApi';
import { Block, GraphData, GraphNode, GraphLink } from '../types/dag';

const DagVisualization: React.FC = () => {
    const [graphData, setGraphData] = useState<GraphData>({ nodes: [], links: [] });
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchData = async () => {
            try {
                setLoading(true);
                const blocks = await dagApi.getBlocks();
                setGraphData(dagApi.convertToGraphData(blocks));
                setError(null);
            } catch (err) {
                setError('Failed to fetch DAG data');
                console.error(err);
            } finally {
                setLoading(false);
            }
        };

        fetchData();
        const interval = setInterval(fetchData, 5000); // Update every 5 seconds

        return () => clearInterval(interval);
    }, []);

    const getNodeColor = (node: GraphNode) => {
        if (node.in_k_cluster) return '#00ff00'; // Green for k-cluster
        if (node.confidence >= 80) return '#0088ff'; // Blue for confirmed
        return '#ff8800'; // Orange for unconfirmed
    };

    const getNodeSize = (node: GraphNode) => {
        return 10 + (node.confidence / 10); // Size based on confidence
    };

    if (loading) return <div>Loading DAG visualization...</div>;
    if (error) return <div>Error: {error}</div>;

    return (
        <div style={{ width: '100%', height: '100vh' }}>
            <ForceGraph2D
                graphData={graphData}
                nodeColor={getNodeColor}
                nodeVal={getNodeSize}
                linkDirectional={true}
                linkDirectionalParticles={2}
                linkDirectionalParticleSpeed={0.005}
                nodeLabel={(node: any) => `
                    ID: ${node.id}
                    Height: ${node.height}
                    Confidence: ${node.confidence}%
                    Network Delay: ${node.network_delay.toFixed(2)}ms
                `}
                cooldownTicks={100}
                onEngineStop={() => {
                    // Adjust graph after initial layout
                }}
            />
        </div>
    );
};

export default DagVisualization; 