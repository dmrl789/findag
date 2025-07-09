import React, { useState, useRef, useEffect, useCallback } from 'react';
import { Network } from 'vis-network';
import { DataSet } from 'vis-data';
import { 
  Search, 
  Filter, 
  Download, 
  RefreshCw, 
  Settings,
  Eye,
  EyeOff,
  ZoomIn,
  ZoomOut,
  RotateCcw,
  Play,
  Pause,
  Info,
  Layers,
  Share2
} from 'lucide-react';
import { useAccessibility } from '../Common/AccessibilityProvider';

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
  const containerRef = useRef<HTMLDivElement>(null);
  const networkRef = useRef<Network | null>(null);
  const { addAnnouncement } = useAccessibility();

  // State management
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedFilters, setSelectedFilters] = useState<Set<string>>(new Set());
  const [showFilters, setShowFilters] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [selectedNode, setSelectedNode] = useState<DAGNode | null>(null);
  const [isAnimating, setIsAnimating] = useState(false);
  const [animationSpeed, setAnimationSpeed] = useState(1000);
  const [viewMode, setViewMode] = useState<'default' | 'hierarchical' | 'circular'>('default');
  const [showLabels, setShowLabels] = useState(true);
  const [showTimestamps, setShowTimestamps] = useState(false);
  const [zoomLevel, setZoomLevel] = useState(1);

  // Filter options
  const [filterOptions, setFilterOptions] = useState({
    validators: new Set<string>(),
    levels: new Set<number>(),
    statuses: new Set<string>(),
    transactionRanges: new Set<string>(),
  });

  // Process and filter data
  const processedData = useCallback(() => {
    let filteredNodes = data.nodes;
    let filteredEdges = data.edges;

    // Apply search filter
    if (searchTerm) {
      filteredNodes = filteredNodes.filter(node =>
        node.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
        node.validator.toLowerCase().includes(searchTerm.toLowerCase()) ||
        node.hash?.toLowerCase().includes(searchTerm.toLowerCase())
      );
      
      // Filter edges to only include connections between filtered nodes
      const nodeIds = new Set(filteredNodes.map(n => n.id));
      filteredEdges = filteredEdges.filter(edge =>
        nodeIds.has(edge.from) && nodeIds.has(edge.to)
      );
    }

    // Apply selected filters
    if (selectedFilters.size > 0) {
      filteredNodes = filteredNodes.filter(node => {
        return Array.from(selectedFilters).some(filter => {
          const [type, value] = filter.split(':');
          switch (type) {
            case 'validator':
              return node.validator === value;
            case 'level':
              return node.level === parseInt(value);
            case 'status':
              return node.status === value;
            case 'txRange':
              const [min, max] = value.split('-').map(Number);
              return node.transactionCount >= min && node.transactionCount <= max;
            default:
              return false;
          }
        });
      });

      // Update edges for filtered nodes
      const nodeIds = new Set(filteredNodes.map(n => n.id));
      filteredEdges = filteredEdges.filter(edge =>
        nodeIds.has(edge.from) && nodeIds.has(edge.to)
      );
    }

    return { nodes: filteredNodes, edges: filteredEdges };
  }, [data, searchTerm, selectedFilters]);

  // Initialize network
  useEffect(() => {
    if (!containerRef.current) return;

    const { nodes, edges } = processedData();

    // Create datasets
    const nodesDataset = new DataSet(nodes.map(node => ({
      ...node,
      title: `${node.label}\nValidator: ${node.validator}\nTransactions: ${node.transactionCount}\nLevel: ${node.level}`,
      font: { size: 12 },
      borderWidth: node.status === 'confirmed' ? 3 : 1,
      borderColor: node.status === 'confirmed' ? '#10B981' : '#6B7280',
    })));

    const edgesDataset = new DataSet(edges.map(edge => ({
      ...edge,
      smooth: { type: 'curvedCW', roundness: 0.2 },
      width: 2,
      color: { color: '#6B7280', opacity: 0.6 },
    })));

    // Network options
    const options = {
      nodes: {
        shape: 'dot',
        size: 16,
        font: {
          size: 12,
          color: '#374151',
        },
        borderWidth: 2,
        shadow: true,
      },
      edges: {
        width: 2,
        color: { color: '#6B7280', opacity: 0.6 },
        smooth: { type: 'curvedCW', roundness: 0.2 },
        shadow: true,
      },
      physics: {
        enabled: true,
        hierarchicalRepulsion: {
          nodeDistance: 150,
        },
        springLength: 100,
        springConstant: 0.05,
        damping: 0.09,
      },
      layout: {
        hierarchical: {
          enabled: viewMode === 'hierarchical',
          direction: 'UD',
          sortMethod: 'directed',
          levelSeparation: 150,
          nodeSpacing: 100,
        },
      },
      interaction: {
        hover: true,
        tooltipDelay: 200,
        zoomView: true,
        dragView: true,
      },
      manipulation: {
        enabled: false,
      },
    };

    // Create network
    const network = new Network(containerRef.current, {
      nodes: nodesDataset,
      edges: edgesDataset,
    }, options);

    // Event handlers
    network.on('click', (params) => {
      if (params.nodes.length > 0) {
        const nodeId = params.nodes[0];
        const node = nodes.find(n => n.id === nodeId);
        if (node) {
          setSelectedNode(node);
          onNodeClick?.(node);
          addAnnouncement(`Selected node: ${node.label}`);
        }
      } else if (params.edges.length > 0) {
        const edgeId = params.edges[0];
        const edge = edges.find(e => `${e.from}-${e.to}` === edgeId);
        if (edge) {
          onEdgeClick?.(edge);
        }
      }
    });

    network.on('doubleClick', (params) => {
      if (params.nodes.length > 0) {
        const nodeId = params.nodes[0];
        const node = nodes.find(n => n.id === nodeId);
        if (node) {
          onNodeDoubleClick?.(node);
        }
      }
    });

    network.on('zoom', (params) => {
      setZoomLevel(params.scale);
    });

    networkRef.current = network;

    return () => {
      network.destroy();
    };
  }, [processedData, viewMode, onNodeClick, onNodeDoubleClick, onEdgeClick, addAnnouncement]);

  // Update filter options when data changes
  useEffect(() => {
    const validators = new Set(data.nodes.map(n => n.validator));
    const levels = new Set(data.nodes.map(n => n.level));
    const statuses = new Set(data.nodes.map(n => n.status).filter(Boolean));
    
    // Create transaction ranges
    const txCounts = data.nodes.map(n => n.transactionCount);
    const minTx = Math.min(...txCounts);
    const maxTx = Math.max(...txCounts);
    const ranges = [
      `${minTx}-${Math.floor((minTx + maxTx) / 3)}`,
      `${Math.floor((minTx + maxTx) / 3) + 1}-${Math.floor(2 * (minTx + maxTx) / 3)}`,
      `${Math.floor(2 * (minTx + maxTx) / 3) + 1}-${maxTx}`,
    ];

    setFilterOptions({
      validators,
      levels,
      statuses,
      transactionRanges: new Set(ranges),
    });
  }, [data]);

  // Animation functions
  const startAnimation = useCallback(() => {
    if (!networkRef.current || !enableAnimations) return;

    setIsAnimating(true);
    const network = networkRef.current;

    // Animate new nodes
    const { nodes } = processedData();
    nodes.forEach((node, index) => {
      setTimeout(() => {
        network.selectNodes([node.id]);
        setTimeout(() => {
          network.unselectAll();
        }, 200);
      }, index * animationSpeed);
    });

    setTimeout(() => setIsAnimating(false), nodes.length * animationSpeed);
  }, [processedData, animationSpeed, enableAnimations]);

  const stopAnimation = useCallback(() => {
    setIsAnimating(false);
    if (networkRef.current) {
      networkRef.current.unselectAll();
    }
  }, []);

  // Export functions
  const exportDAG = useCallback((format: 'json' | 'csv' | 'png') => {
    const { nodes, edges } = processedData();
    
    switch (format) {
      case 'json':
        const jsonData = JSON.stringify({ nodes, edges }, null, 2);
        const jsonBlob = new Blob([jsonData], { type: 'application/json' });
        const jsonUrl = URL.createObjectURL(jsonBlob);
        const jsonLink = document.createElement('a');
        jsonLink.href = jsonUrl;
        jsonLink.download = `dag-export-${Date.now()}.json`;
        jsonLink.click();
        URL.revokeObjectURL(jsonUrl);
        break;

      case 'csv':
        const csvContent = [
          'Node ID,Label,Level,Validator,Transaction Count,Status,Timestamp',
          ...nodes.map(node => 
            `${node.id},${node.label},${node.level},${node.validator},${node.transactionCount},${node.status || ''},${new Date(node.timestamp).toISOString()}`
          ).join('\n')
        ].join('\n');
        
        const csvBlob = new Blob([csvContent], { type: 'text/csv' });
        const csvUrl = URL.createObjectURL(csvBlob);
        const csvLink = document.createElement('a');
        csvLink.href = csvUrl;
        csvLink.download = `dag-nodes-${Date.now()}.csv`;
        csvLink.click();
        URL.revokeObjectURL(csvUrl);
        break;

      case 'png':
        if (networkRef.current) {
          const canvas = networkRef.current.canvas.frame.canvas;
          const link = document.createElement('a');
          link.download = `dag-visualization-${Date.now()}.png`;
          link.href = canvas.toDataURL();
          link.click();
        }
        break;
    }

    addAnnouncement(`DAG exported as ${format.toUpperCase()}`);
  }, [processedData, addAnnouncement]);

  // Zoom functions
  const zoomIn = useCallback(() => {
    if (networkRef.current) {
      networkRef.current.moveTo({ scale: zoomLevel * 1.2 });
    }
  }, [zoomLevel]);

  const zoomOut = useCallback(() => {
    if (networkRef.current) {
      networkRef.current.moveTo({ scale: zoomLevel / 1.2 });
    }
  }, [zoomLevel]);

  const fitToScreen = useCallback(() => {
    if (networkRef.current) {
      networkRef.current.fit();
    }
  }, []);

  // Filter functions
  const toggleFilter = useCallback((filter: string) => {
    setSelectedFilters(prev => {
      const newFilters = new Set(prev);
      if (newFilters.has(filter)) {
        newFilters.delete(filter);
      } else {
        newFilters.add(filter);
      }
      return newFilters;
    });
  }, []);

  const clearFilters = useCallback(() => {
    setSelectedFilters(new Set());
    setSearchTerm('');
  }, []);

  return (
    <div className={`bg-white rounded-lg shadow-sm border border-gray-200 ${className}`}>
      {/* Header */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <h3 className="text-lg font-medium text-gray-900">DAG Visualization</h3>
            
            {enableSearch && (
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                <input
                  type="text"
                  placeholder="Search nodes..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                />
              </div>
            )}

            {enableFilters && (
              <button
                onClick={() => setShowFilters(!showFilters)}
                className={`p-2 rounded-lg transition-colors ${
                  showFilters ? 'bg-primary-100 text-primary-600' : 'text-gray-600 hover:bg-gray-100'
                }`}
                title="Filters"
              >
                <Filter className="w-4 h-4" />
              </button>
            )}
          </div>

          <div className="flex items-center space-x-2">
            {/* View Mode */}
            <select
              value={viewMode}
              onChange={(e) => setViewMode(e.target.value as any)}
              className="border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="default">Default</option>
              <option value="hierarchical">Hierarchical</option>
              <option value="circular">Circular</option>
            </select>

            {/* Animation Controls */}
            {enableAnimations && (
              <div className="flex items-center space-x-1">
                <button
                  onClick={isAnimating ? stopAnimation : startAnimation}
                  className={`p-2 rounded-lg transition-colors ${
                    isAnimating ? 'bg-red-100 text-red-600' : 'text-gray-600 hover:bg-gray-100'
                  }`}
                  title={isAnimating ? 'Stop Animation' : 'Start Animation'}
                >
                  {isAnimating ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
                </button>
              </div>
            )}

            {/* Zoom Controls */}
            <div className="flex items-center space-x-1">
              <button
                onClick={zoomOut}
                className="p-2 text-gray-600 hover:bg-gray-100 rounded-lg"
                title="Zoom Out"
              >
                <ZoomOut className="w-4 h-4" />
              </button>
              <button
                onClick={fitToScreen}
                className="p-2 text-gray-600 hover:bg-gray-100 rounded-lg"
                title="Fit to Screen"
              >
                <RotateCcw className="w-4 h-4" />
              </button>
              <button
                onClick={zoomIn}
                className="p-2 text-gray-600 hover:bg-gray-100 rounded-lg"
                title="Zoom In"
              >
                <ZoomIn className="w-4 h-4" />
              </button>
            </div>

            {/* Export */}
            {enableExport && (
              <div className="relative">
                <button
                  onClick={() => setShowSettings(!showSettings)}
                  className="p-2 text-gray-600 hover:bg-gray-100 rounded-lg"
                  title="Export Options"
                >
                  <Download className="w-4 h-4" />
                </button>
                
                {showSettings && (
                  <div className="absolute right-0 top-full mt-2 w-48 bg-white rounded-lg shadow-lg border border-gray-200 z-10">
                    <div className="p-2">
                      <button
                        onClick={() => exportDAG('json')}
                        className="w-full text-left px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded"
                      >
                        Export as JSON
                      </button>
                      <button
                        onClick={() => exportDAG('csv')}
                        className="w-full text-left px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded"
                      >
                        Export as CSV
                      </button>
                      <button
                        onClick={() => exportDAG('png')}
                        className="w-full text-left px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded"
                      >
                        Export as PNG
                      </button>
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Filters Panel */}
        {showFilters && enableFilters && (
          <div className="mt-4 p-4 bg-gray-50 rounded-lg">
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              {/* Validators */}
              <div>
                <h4 className="text-sm font-medium text-gray-900 mb-2">Validators</h4>
                <div className="space-y-1 max-h-32 overflow-y-auto">
                  {Array.from(filterOptions.validators).map(validator => (
                    <label key={validator} className="flex items-center">
                      <input
                        type="checkbox"
                        checked={selectedFilters.has(`validator:${validator}`)}
                        onChange={() => toggleFilter(`validator:${validator}`)}
                        className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                      />
                      <span className="ml-2 text-sm text-gray-700">{validator}</span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Levels */}
              <div>
                <h4 className="text-sm font-medium text-gray-900 mb-2">Levels</h4>
                <div className="space-y-1">
                  {Array.from(filterOptions.levels).sort((a, b) => a - b).map(level => (
                    <label key={level} className="flex items-center">
                      <input
                        type="checkbox"
                        checked={selectedFilters.has(`level:${level}`)}
                        onChange={() => toggleFilter(`level:${level}`)}
                        className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                      />
                      <span className="ml-2 text-sm text-gray-700">Level {level}</span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Statuses */}
              <div>
                <h4 className="text-sm font-medium text-gray-900 mb-2">Status</h4>
                <div className="space-y-1">
                  {Array.from(filterOptions.statuses).map(status => (
                    <label key={status} className="flex items-center">
                      <input
                        type="checkbox"
                        checked={selectedFilters.has(`status:${status}`)}
                        onChange={() => toggleFilter(`status:${status}`)}
                        className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                      />
                      <span className="ml-2 text-sm text-gray-700 capitalize">{status}</span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Transaction Ranges */}
              <div>
                <h4 className="text-sm font-medium text-gray-900 mb-2">Transactions</h4>
                <div className="space-y-1">
                  {Array.from(filterOptions.transactionRanges).map(range => (
                    <label key={range} className="flex items-center">
                      <input
                        type="checkbox"
                        checked={selectedFilters.has(`txRange:${range}`)}
                        onChange={() => toggleFilter(`txRange:${range}`)}
                        className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                      />
                      <span className="ml-2 text-sm text-gray-700">{range}</span>
                    </label>
                  ))}
                </div>
              </div>
            </div>

            <div className="mt-4 flex justify-between items-center">
              <div className="text-sm text-gray-600">
                {selectedFilters.size} filter(s) active
              </div>
              <button
                onClick={clearFilters}
                className="text-sm text-primary-600 hover:text-primary-800"
              >
                Clear All
              </button>
            </div>
          </div>
        )}
      </div>

      {/* DAG Container */}
      <div className="relative" style={{ height }}>
        {loading && (
          <div className="absolute inset-0 bg-white bg-opacity-75 flex items-center justify-center z-10">
            <div className="text-center">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600 mx-auto"></div>
              <p className="mt-2 text-sm text-gray-600">Loading DAG...</p>
            </div>
          </div>
        )}

        <div ref={containerRef} className="w-full h-full" />

        {/* Zoom Level Indicator */}
        <div className="absolute bottom-4 right-4 bg-white bg-opacity-90 rounded-lg px-3 py-1 text-sm text-gray-600">
          {Math.round(zoomLevel * 100)}%
        </div>

        {/* Node Count */}
        <div className="absolute bottom-4 left-4 bg-white bg-opacity-90 rounded-lg px-3 py-1 text-sm text-gray-600">
          {processedData().nodes.length} nodes, {processedData().edges.length} edges
        </div>
      </div>

      {/* Selected Node Details */}
      {selectedNode && (
        <div className="px-6 py-4 border-t border-gray-200 bg-gray-50">
          <div className="flex items-center justify-between">
            <div>
              <h4 className="text-sm font-medium text-gray-900">{selectedNode.label}</h4>
              <div className="text-sm text-gray-600 mt-1">
                Validator: {selectedNode.validator} | Level: {selectedNode.level} | 
                Transactions: {selectedNode.transactionCount}
              </div>
              {selectedNode.hash && (
                <div className="text-xs text-gray-500 mt-1 font-mono">
                  Hash: {selectedNode.hash.substring(0, 16)}...
                </div>
              )}
            </div>
            <button
              onClick={() => setSelectedNode(null)}
              className="text-gray-400 hover:text-gray-600"
            >
              ✕
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

// Hook for DAG state management
export const useEnhancedDAG = () => {
  const [data, setData] = useState<DAGData>({ nodes: [], edges: [] });
  const [loading, setLoading] = useState(false);
  const [selectedNode, setSelectedNode] = useState<DAGNode | null>(null);
  const [filters, setFilters] = useState<Set<string>>(new Set());
  const [searchTerm, setSearchTerm] = useState('');

  const updateData = (newData: DAGData) => {
    setData(newData);
  };

  const addNode = (node: DAGNode) => {
    setData(prev => ({
      nodes: [...prev.nodes, node],
      edges: prev.edges,
    }));
  };

  const addEdge = (edge: DAGEdge) => {
    setData(prev => ({
      nodes: prev.nodes,
      edges: [...prev.edges, edge],
    }));
  };

  const removeNode = (nodeId: string) => {
    setData(prev => ({
      nodes: prev.nodes.filter(n => n.id !== nodeId),
      edges: prev.edges.filter(e => e.from !== nodeId && e.to !== nodeId),
    }));
  };

  const clearData = () => {
    setData({ nodes: [], edges: [] });
  };

  return {
    data,
    loading,
    selectedNode,
    filters,
    searchTerm,
    setData: updateData,
    setLoading,
    setSelectedNode,
    setFilters,
    setSearchTerm,
    addNode,
    addEdge,
    removeNode,
    clearData,
  };
}; 