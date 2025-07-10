import React, { useState, useEffect } from 'react';
import { Network, Wifi, Activity, Globe, Server, Users } from 'lucide-react';
import { useAppStore } from '../../store';
import { NodeMetrics } from '../../types';
import { formatNumber, formatLatency, formatUptime, formatTimestamp } from '../../utils/formatters';

export const NetworkPage: React.FC = () => {
  const { nodeMetrics, isLoading, fetchNodeMetrics } = useAppStore();
  const [selectedNode, setSelectedNode] = useState<NodeMetrics | null>(null);

  useEffect(() => {
    fetchNodeMetrics();
  }, [fetchNodeMetrics]);

  const activeNodes = nodeMetrics.filter(node => node.uptime > 0);
  const totalConnections = nodeMetrics.reduce((sum, node) => sum + node.connectedPeers, 0);
  const averageLatency = nodeMetrics.length > 0 
    ? nodeMetrics.reduce((sum, node) => sum + node.latency, 0) / nodeMetrics.length 
    : 0;

  const getNodeStatusColor = (uptime: number) => {
    if (uptime > 0) return 'bg-green-100 text-green-800';
    return 'bg-red-100 text-red-800';
  };

  const getNodeStatusIcon = (uptime: number) => {
    if (uptime > 0) return <Activity className="w-4 h-4" />;
    return <Server className="w-4 h-4" />;
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Network</h1>
          <p className="text-gray-600">P2P network topology and health</p>
        </div>
        <div className="flex items-center space-x-3">
          <button className="btn-secondary flex items-center space-x-2">
            <Globe className="w-4 h-4" />
            <span>Network Map</span>
          </button>
        </div>
      </div>

      {/* Network Overview Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center">
                <Server className="w-5 h-5 text-blue-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Total Nodes</p>
              <p className="text-2xl font-semibold text-gray-900">{nodeMetrics.length}</p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-green-100 rounded-lg flex items-center justify-center">
                <Activity className="w-5 h-5 text-green-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Active Nodes</p>
              <p className="text-2xl font-semibold text-gray-900">{activeNodes.length}</p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-purple-100 rounded-lg flex items-center justify-center">
                <Wifi className="w-5 h-5 text-purple-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Connections</p>
              <p className="text-2xl font-semibold text-gray-900">{totalConnections}</p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-yellow-100 rounded-lg flex items-center justify-center">
                <Network className="w-5 h-5 text-yellow-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Avg Latency</p>
              <p className="text-2xl font-semibold text-gray-900">{formatLatency(averageLatency)}</p>
            </div>
          </div>
        </div>
      </div>

      {/* Network Topology Visualization */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Network Topology</h3>
        <div className="bg-gray-100 rounded-lg h-64 flex items-center justify-center">
          <div className="text-center">
            <Network className="w-12 h-12 text-gray-400 mx-auto mb-2" />
            <p className="text-gray-600">Network topology visualization</p>
            <p className="text-sm text-gray-500">Interactive node connection map</p>
          </div>
        </div>
      </div>

      {/* Nodes Table */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900">Network Nodes</h3>
        </div>
        
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Node ID
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Uptime
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  TPS
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Latency
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Connections
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Last Block
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {isLoading.nodeMetrics ? (
                <tr>
                  <td colSpan={8} className="px-6 py-4 text-center text-gray-500">
                    Loading nodes...
                  </td>
                </tr>
              ) : nodeMetrics.length === 0 ? (
                <tr>
                  <td colSpan={8} className="px-6 py-4 text-center text-gray-500">
                    No nodes found
                  </td>
                </tr>
              ) : (
                nodeMetrics.map((node) => (
                  <tr key={node.nodeId} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div className="flex-shrink-0 h-8 w-8">
                          <div className="h-8 w-8 rounded-full bg-gray-300 flex items-center justify-center">
                            <span className="text-xs font-medium text-gray-700">
                              {node.nodeId.slice(0, 2).toUpperCase()}
                            </span>
                          </div>
                        </div>
                        <div className="ml-3">
                          <div className="text-sm font-medium text-gray-900">
                            {node.nodeId}
                          </div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getNodeStatusColor(node.uptime)}`}>
                        {getNodeStatusIcon(node.uptime)}
                        <span className="ml-1">{node.uptime > 0 ? 'Online' : 'Offline'}</span>
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {formatUptime(node.uptime)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {formatNumber(node.tps)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {formatLatency(node.latency)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {node.connectedPeers}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {formatTimestamp(node.lastBlockTime)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                      <button
                        onClick={() => setSelectedNode(node)}
                        className="text-blue-600 hover:text-blue-900"
                      >
                        View Details
                      </button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Node Details Modal */}
      {selectedNode && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">Node Details</h2>
              <button
                onClick={() => setSelectedNode(null)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-500">Node ID</label>
                  <p className="text-sm text-gray-900">{selectedNode.nodeId}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Status</label>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getNodeStatusColor(selectedNode.uptime)}`}>
                    {getNodeStatusIcon(selectedNode.uptime)}
                    <span className="ml-1">{selectedNode.uptime > 0 ? 'Online' : 'Offline'}</span>
                  </span>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Uptime</label>
                  <p className="text-sm text-gray-900">{formatUptime(selectedNode.uptime)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">TPS</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedNode.tps)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Latency</label>
                  <p className="text-sm text-gray-900">{formatLatency(selectedNode.latency)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Connected Peers</label>
                  <p className="text-sm text-gray-900">{selectedNode.connectedPeers}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Memory Usage</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedNode.memoryUsage)} bytes</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">CPU Usage</label>
                  <p className="text-sm text-gray-900">{(selectedNode.cpuUsage * 100).toFixed(1)}%</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Last Block Time</label>
                  <p className="text-sm text-gray-900">{formatTimestamp(selectedNode.lastBlockTime)}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 