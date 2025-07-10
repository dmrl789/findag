import React, { useState } from 'react';
import { 
  Search, 
  Network,
  Activity,
  Clock,
  Hash,
  Eye,
  Filter
} from 'lucide-react';

export interface DAGNode {
  id: string;
  label: string;
  level: number;
  timestamp: number;
  validator: string;
  transactionCount: number;
  hash?: string;
  parentHashes?: string[];
  status?: 'confirmed' | 'pending' | 'orphaned';
  color?: string;
  size?: number;
  metadata?: Record<string, any>;
}

export interface DAGEdge {
  from: string;
  to: string;
  arrows?: string;
  color?: string;
  width?: number;
  smooth?: boolean;
}

export interface DAGData {
  nodes: DAGNode[];
  edges: DAGEdge[];
}

export interface EnhancedDAGVisualizerProps {
  data: DAGData;
  loading?: boolean;
  onNodeClick?: (node: DAGNode) => void;
  onNodeDoubleClick?: (node: DAGNode) => void;
  onEdgeClick?: (edge: DAGEdge) => void;
  className?: string;
  height?: string;
  enableAnimations?: boolean;
  enableSearch?: boolean;
  enableFilters?: boolean;
  enableExport?: boolean;
  showControls?: boolean;
}

export const EnhancedDAGVisualizer: React.FC<EnhancedDAGVisualizerProps> = ({
  data,
  loading = false,
  onNodeClick,
  onNodeDoubleClick,
  onEdgeClick,
  className = '',
  height = '600px',
  enableAnimations = true,
  enableSearch = true,
  enableFilters = true,
  enableExport = true,
  showControls = true,
}) => {
  const [selectedNode, setSelectedNode] = useState<DAGNode | null>(null);
  const [viewMode, setViewMode] = useState<'graph' | 'list' | 'timeline'>('graph');
  const [searchTerm, setSearchTerm] = useState('');
  const [showDetails, setShowDetails] = useState(false);

  // Generate mock DAG data if none provided
  const mockData: DAGData = {
    nodes: [
      { id: '1', label: 'Block #45678', level: 0, timestamp: Date.now() - 60000, validator: 'validator-01', transactionCount: 1250, status: 'confirmed', hash: '0x1234...' },
      { id: '2', label: 'Block #45679', level: 1, timestamp: Date.now() - 45000, validator: 'validator-02', transactionCount: 1180, status: 'confirmed', hash: '0x5678...' },
      { id: '3', label: 'Block #45680', level: 1, timestamp: Date.now() - 30000, validator: 'validator-03', transactionCount: 1320, status: 'confirmed', hash: '0x9abc...' },
      { id: '4', label: 'Block #45681', level: 2, timestamp: Date.now() - 15000, validator: 'validator-01', transactionCount: 980, status: 'pending', hash: '0xdef0...' },
      { id: '5', label: 'Block #45682', level: 2, timestamp: Date.now() - 10000, validator: 'validator-02', transactionCount: 1100, status: 'pending', hash: '0x1111...' },
      { id: '6', label: 'Block #45683', level: 3, timestamp: Date.now() - 5000, validator: 'validator-03', transactionCount: 890, status: 'pending', hash: '0x2222...' },
    ],
    edges: [
      { from: '1', to: '2' },
      { from: '1', to: '3' },
      { from: '2', to: '4' },
      { from: '2', to: '5' },
      { from: '3', to: '5' },
      { from: '4', to: '6' },
      { from: '5', to: '6' },
    ]
  };

  const displayData = data.nodes.length > 0 ? data : mockData;

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'confirmed': return 'bg-green-500';
      case 'pending': return 'bg-yellow-500';
      case 'orphaned': return 'bg-red-500';
      default: return 'bg-gray-500';
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case 'confirmed': return 'Confirmed';
      case 'pending': return 'Pending';
      case 'orphaned': return 'Orphaned';
      default: return 'Unknown';
    }
  };

  const filteredNodes = displayData.nodes.filter(node =>
    node.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
    node.validator.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleNodeClick = (node: DAGNode) => {
    setSelectedNode(node);
    setShowDetails(true);
    onNodeClick?.(node);
  };

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold text-gray-900">DAG Explorer</h2>
          <p className="text-gray-600">Visual blockchain structure with {displayData.nodes.length} blocks</p>
        </div>
        <div className="flex items-center space-x-2">
          {enableSearch && (
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="text"
                placeholder="Search blocks..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-10 pr-4 py-2 border border-gray-300 rounded-md text-sm"
              />
            </div>
          )}
          <button
            onClick={() => setViewMode('graph')}
            className={`px-3 py-2 text-sm rounded-md ${
              viewMode === 'graph' ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-700'
            }`}
          >
            Graph
          </button>
          <button
            onClick={() => setViewMode('list')}
            className={`px-3 py-2 text-sm rounded-md ${
              viewMode === 'list' ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-700'
            }`}
          >
            List
          </button>
          <button
            onClick={() => setViewMode('timeline')}
            className={`px-3 py-2 text-sm rounded-md ${
              viewMode === 'timeline' ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-700'
            }`}
          >
            Timeline
          </button>
        </div>
      </div>

      {/* DAG Visualization */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        {viewMode === 'graph' && (
          <div className="space-y-6">
            {/* Graph View */}
            <div className="bg-gray-50 rounded-lg p-8 h-96 flex items-center justify-center">
              <div className="text-center">
                <Network className="w-16 h-16 text-gray-400 mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">DAG Graph View</h3>
                <p className="text-gray-600 mb-4">Interactive network visualization of blockchain structure</p>
                <div className="grid grid-cols-3 gap-4 max-w-md mx-auto">
                  {displayData.nodes.slice(0, 6).map((node) => (
                    <div
                      key={node.id}
                      onClick={() => handleNodeClick(node)}
                      className="bg-white p-3 rounded-lg border border-gray-200 cursor-pointer hover:border-primary-300 hover:shadow-sm transition-all"
                    >
                      <div className="flex items-center justify-between mb-2">
                        <span className="text-sm font-medium text-gray-900">{node.label}</span>
                        <div className={`w-2 h-2 rounded-full ${getStatusColor(node.status || 'pending')}`} />
                      </div>
                      <div className="text-xs text-gray-500">
                        <div>Validator: {node.validator}</div>
                        <div>TX: {node.transactionCount}</div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        )}

        {viewMode === 'list' && (
          <div className="space-y-4">
            <div className="grid gap-4">
              {filteredNodes.map((node) => (
                <div
                  key={node.id}
                  onClick={() => handleNodeClick(node)}
                  className="flex items-center justify-between p-4 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors"
                >
                  <div className="flex items-center space-x-4">
                    <div className={`w-3 h-3 rounded-full ${getStatusColor(node.status || 'pending')}`} />
                    <div>
                      <h4 className="font-medium text-gray-900">{node.label}</h4>
                      <p className="text-sm text-gray-500">Validator: {node.validator}</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-medium text-gray-900">{node.transactionCount} transactions</div>
                    <div className="text-xs text-gray-500">
                      {new Date(node.timestamp).toLocaleTimeString()}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {viewMode === 'timeline' && (
          <div className="space-y-4">
            <div className="relative">
              <div className="absolute left-4 top-0 bottom-0 w-0.5 bg-gray-200"></div>
              {filteredNodes.map((node, index) => (
                <div key={node.id} className="relative flex items-center space-x-4 mb-6">
                  <div className={`w-3 h-3 rounded-full ${getStatusColor(node.status || 'pending')} z-10`}></div>
                  <div
                    onClick={() => handleNodeClick(node)}
                    className="flex-1 bg-white p-4 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors"
                  >
                    <div className="flex items-center justify-between">
                      <div>
                        <h4 className="font-medium text-gray-900">{node.label}</h4>
                        <p className="text-sm text-gray-500">Validator: {node.validator}</p>
                      </div>
                      <div className="text-right">
                        <div className="text-sm font-medium text-gray-900">{node.transactionCount} transactions</div>
                        <div className="text-xs text-gray-500">
                          {new Date(node.timestamp).toLocaleTimeString()}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Node Details Modal */}
      {showDetails && selectedNode && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-900">Block Details</h3>
              <button
                onClick={() => setShowDetails(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                ×
              </button>
            </div>
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium text-gray-700">Block ID</label>
                <p className="text-sm text-gray-900">{selectedNode.label}</p>
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700">Validator</label>
                <p className="text-sm text-gray-900">{selectedNode.validator}</p>
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700">Status</label>
                <div className="flex items-center space-x-2">
                  <div className={`w-2 h-2 rounded-full ${getStatusColor(selectedNode.status || 'pending')}`} />
                  <span className="text-sm text-gray-900">{getStatusText(selectedNode.status || 'pending')}</span>
                </div>
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700">Transactions</label>
                <p className="text-sm text-gray-900">{selectedNode.transactionCount}</p>
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700">Timestamp</label>
                <p className="text-sm text-gray-900">{new Date(selectedNode.timestamp).toLocaleString()}</p>
              </div>
              {selectedNode.hash && (
                <div>
                  <label className="text-sm font-medium text-gray-700">Hash</label>
                  <p className="text-sm text-gray-900 font-mono">{selectedNode.hash}</p>
                </div>
              )}
            </div>
            <div className="mt-6 flex justify-end space-x-3">
              <button
                onClick={() => setShowDetails(false)}
                className="px-4 py-2 text-sm border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 