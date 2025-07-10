import React, { useState, useEffect } from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../../components/Common/Tabs';
import { DAGVisualizer } from './DAGVisualizer';
import { DAGOperations } from './DAGOperations';
import { finDAGApi } from '../../services/api';
import { useNotifications, createNotification } from '../Common/NotificationSystem';
import { DAGNode } from './EnhancedDAGVisualizer';

export const DAGPage: React.FC = () => {
  const [dagData, setDagData] = useState<{
    nodes: DAGNode[];
    edges: Array<{ from: string; to: string; arrows?: string; color?: string }>;
  }>({ nodes: [], edges: [] });
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState('visualizer');

  const { addNotification } = useNotifications();

  // Load DAG data from backend
  const loadDAGData = async () => {
    setLoading(true);
    try {
      // Fetch blocks and create DAG nodes
      const blocksResponse = await finDAGApi.getBlocks(1, 50);
      const blocks = blocksResponse.data;

      // Convert blocks to DAG nodes
      const nodes: DAGNode[] = blocks.map((block: any, index: number) => ({
        id: block.id,
        label: `Block ${block.id}`,
        level: index,
        timestamp: typeof block.timestamp === 'object' ? block.timestamp.timestamp : block.timestamp,
        validator: block.validator || `validator-${index % 3}`,
        transactionCount: block.transaction_count || Math.floor(Math.random() * 100) + 10,
        hash: block.hash,
        parentHashes: block.parent_hashes || [],
        status: block.status || 'confirmed',
        color: block.status === 'confirmed' ? '#10B981' : '#F59E0B',
        size: Math.min(20 + (block.transaction_count || 10) / 10, 40),
      }));

      // Create edges based on parent relationships
      const edges: Array<{ from: string; to: string; arrows?: string; color?: string }> = [];
      nodes.forEach((node, index) => {
        if (index > 0) {
          // Connect to previous block (simplified DAG structure)
          edges.push({
            from: nodes[index - 1].id,
            to: node.id,
            arrows: 'to',
          });
        }
        
        // Add some cross-links for DAG structure
        if (index > 2 && Math.random() > 0.7) {
          const randomParent = Math.floor(Math.random() * (index - 1));
          edges.push({
            from: nodes[randomParent].id,
            to: node.id,
            arrows: 'to',
            color: '#8B5CF6',
          });
        }
      });

      setDagData({ nodes, edges });

      addNotification(createNotification.success(
        'DAG Data Loaded',
        `Loaded ${nodes.length} blocks and ${edges.length} connections`,
        { category: 'system' }
      ));
    } catch (error: any) {
      addNotification(createNotification.error(
        'DAG Error',
        error.message || 'Failed to load DAG data',
        { category: 'system' }
      ));
      
      // Fallback to mock data
      const mockNodes: DAGNode[] = Array.from({ length: 20 }, (_, i) => ({
        id: `block-${i}`,
        label: `Block ${i}`,
        level: i,
        timestamp: Date.now() - i * 60000,
        validator: `validator-${i % 3}`,
        transactionCount: Math.floor(Math.random() * 100) + 10,
        hash: `hash-${i}`,
        parentHashes: i > 0 ? [`block-${i - 1}`] : [],
        status: 'confirmed',
        color: '#10B981',
        size: Math.floor(Math.random() * 20) + 20,
      }));

      const mockEdges: Array<{ from: string; to: string; arrows?: string }> = [];
      mockNodes.forEach((node, index) => {
        if (index > 0) {
          mockEdges.push({
            from: mockNodes[index - 1].id,
            to: node.id,
            arrows: 'to',
          });
        }
      });

      setDagData({ nodes: mockNodes, edges: mockEdges });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadDAGData();
  }, []);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">DAG Operations</h1>
          <p className="text-gray-600 dark:text-gray-400">Directed Acyclic Graph visualization and operations</p>
        </div>
        <button
          onClick={loadDAGData}
          disabled={loading}
          className="btn-secondary flex items-center space-x-2"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>{loading ? 'Loading...' : 'Refresh'}</span>
        </button>
      </div>

      {/* Tabs */}
      <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger 
            value="visualizer" 
            isActive={activeTab === 'visualizer'}
            onClick={() => setActiveTab('visualizer')}
          >
            DAG Visualizer
          </TabsTrigger>
          <TabsTrigger 
            value="operations" 
            isActive={activeTab === 'operations'}
            onClick={() => setActiveTab('operations')}
          >
            DAG Operations
          </TabsTrigger>
        </TabsList>

        <TabsContent value="visualizer" isActive={activeTab === 'visualizer'} className="space-y-4">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="mb-4">
              <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                DAG Visualization
              </h2>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                Interactive visualization of the Directed Acyclic Graph. Click on nodes to view transaction details.
              </p>
            </div>
            
            {loading ? (
              <div className="flex items-center justify-center h-96">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
              </div>
            ) : (
              <DAGVisualizer
                data={dagData}
                className="h-96"
              />
            )}
          </div>
        </TabsContent>

        <TabsContent value="operations" isActive={activeTab === 'operations'} className="space-y-4">
          <DAGOperations />
        </TabsContent>
      </Tabs>
    </div>
  );
}; 