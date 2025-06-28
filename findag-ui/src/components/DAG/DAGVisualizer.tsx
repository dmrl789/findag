import React, { useEffect, useRef } from 'react';
import { Network } from 'vis-network';
import { DataSet } from 'vis-data';
import { DAGData, DAGNode, DAGEdge } from '../../types';

interface DAGVisualizerProps {
  data: DAGData;
  height?: string;
  onNodeClick?: (nodeId: string) => void;
}

export const DAGVisualizer: React.FC<DAGVisualizerProps> = ({
  data,
  height = '600px',
  onNodeClick,
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const networkRef = useRef<Network | null>(null);

  useEffect(() => {
    if (!containerRef.current) return;

    // Create nodes dataset
    const nodes = new DataSet(
      data.nodes.map((node) => ({
        id: node.id,
        label: node.label,
        level: node.level,
        title: `${node.label}\nValidator: ${node.validator}\nTransactions: ${node.transactionCount}\nTime: ${new Date(node.timestamp).toLocaleTimeString()}`,
        color: {
          background: '#3B82F6',
          border: '#1D4ED8',
          highlight: {
            background: '#60A5FA',
            border: '#3B82F6',
          },
        },
        font: {
          color: '#FFFFFF',
          size: 12,
        },
        shape: 'circle',
        size: Math.min(20 + node.transactionCount * 2, 40),
      }))
    );

    // Create edges dataset
    const edges = new DataSet(
      data.edges.map((edge) => ({
        from: edge.from,
        to: edge.to,
        arrows: edge.arrows,
        color: {
          color: '#6B7280',
          highlight: '#3B82F6',
        },
        width: 2,
        smooth: {
          type: 'curvedCW',
          roundness: 0.2,
        },
      }))
    );

    // Network options
    const options = {
      layout: {
        hierarchical: {
          direction: 'UD',
          sortMethod: 'directed',
          nodeSpacing: 150,
          levelSeparation: 200,
        },
      },
      physics: {
        enabled: false,
      },
      interaction: {
        hover: true,
        tooltipDelay: 200,
      },
      edges: {
        smooth: {
          type: 'curvedCW',
          roundness: 0.2,
        },
      },
      nodes: {
        shadow: true,
        borderWidth: 2,
      },
    };

    // Create network
    const network = new Network(containerRef.current, { nodes, edges }, options);
    networkRef.current = network;

    // Add event listeners
    if (onNodeClick) {
      network.on('click', (params) => {
        if (params.nodes.length > 0) {
          onNodeClick(params.nodes[0]);
        }
      });
    }

    // Cleanup
    return () => {
      if (networkRef.current) {
        networkRef.current.destroy();
        networkRef.current = null;
      }
    };
  }, [data, onNodeClick]);

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">DAG Structure</h3>
        <div className="text-sm text-gray-500">
          {data.nodes.length} nodes, {data.edges.length} connections
        </div>
      </div>
      <div
        ref={containerRef}
        style={{ height, width: '100%' }}
        className="border border-gray-200 rounded-lg"
      />
    </div>
  );
}; 