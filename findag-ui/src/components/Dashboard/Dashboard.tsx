import React, { useEffect } from 'react';
import { 
  Activity, 
  Users, 
  Zap, 
  Clock, 
  TrendingUp, 
  Network,
  Database,
  Shield
} from 'lucide-react';
import { MetricsCard } from './MetricsCard';
import { useAppStore } from '../../store';
import { formatNumber, formatLatency } from '../../utils/formatters';

export const Dashboard: React.FC = () => {
  const {
    networkMetrics,
    nodeMetrics,
    currentRound,
    isLoading,
    errors,
    connectWebSocket,
    disconnectWebSocket,
  } = useAppStore();

  useEffect(() => {
    connectWebSocket();
    return () => {
      disconnectWebSocket();
    };
  }, [connectWebSocket, disconnectWebSocket]);

  const getActiveNodes = () => {
    return nodeMetrics.filter(node => node.uptime > 0).length;
  };

  const getAverageLatency = () => {
    if (nodeMetrics.length === 0) return 0;
    const totalLatency = nodeMetrics.reduce((sum, node) => sum + node.latency, 0);
    return totalLatency / nodeMetrics.length;
  };

  const getTotalTPS = () => {
    return nodeMetrics.reduce((sum, node) => sum + node.tps, 0);
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
          <p className="text-gray-600">Real-time network overview and performance metrics</p>
        </div>
        <div className="flex items-center space-x-4">
          <div className="text-sm text-gray-500">
            Last updated: {new Date().toLocaleTimeString()}
          </div>
        </div>
      </div>

      {/* Error Display */}
      {Object.keys(errors).length > 0 && (
        <div className="bg-danger-50 border border-danger-200 rounded-lg p-4">
          <h3 className="text-sm font-medium text-danger-800">Connection Issues</h3>
          <div className="mt-2 text-sm text-danger-700">
            {Object.entries(errors).map(([key, error]) => (
              <div key={key}>{error}</div>
            ))}
          </div>
        </div>
      )}

      {/* Key Metrics Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <MetricsCard
          title="Total TPS"
          value={formatNumber(getTotalTPS())}
          subtitle="Transactions per second"
          icon={Zap}
          color="primary"
          loading={isLoading.networkMetrics}
        />
        
        <MetricsCard
          title="Active Nodes"
          value={getActiveNodes()}
          subtitle={`of ${nodeMetrics.length} total`}
          icon={Users}
          color="success"
          loading={isLoading.nodeMetrics}
        />
        
        <MetricsCard
          title="Average Latency"
          value={formatLatency(getAverageLatency())}
          subtitle="Network response time"
          icon={Clock}
          color="warning"
          loading={isLoading.nodeMetrics}
        />
        
        <MetricsCard
          title="Current Round"
          value={currentRound?.number || 'N/A'}
          subtitle={currentRound?.status || 'No active round'}
          icon={Activity}
          color="gray"
          loading={isLoading.networkMetrics}
        />
      </div>

      {/* Network Overview */}
      {networkMetrics && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="card">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Network Performance</h3>
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-600">Total Transactions</span>
                <span className="font-medium">{formatNumber(networkMetrics.totalTransactions)}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-600">Finalized Blocks</span>
                <span className="font-medium">{formatNumber(networkMetrics.finalizedBlocks)}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-600">Network TPS</span>
                <span className="font-medium">{formatNumber(networkMetrics.totalTPS)}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-600">Average Latency</span>
                <span className="font-medium">{formatLatency(networkMetrics.averageLatency)}</span>
              </div>
            </div>
          </div>

          <div className="card">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Node Distribution</h3>
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-600">Total Nodes</span>
                <span className="font-medium">{networkMetrics.totalNodes}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-600">Active Nodes</span>
                <span className="font-medium">{networkMetrics.activeNodes}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-600">Uptime Rate</span>
                <span className="font-medium">
                  {networkMetrics.totalNodes > 0 
                    ? `${((networkMetrics.activeNodes / networkMetrics.totalNodes) * 100).toFixed(1)}%`
                    : 'N/A'
                  }
                </span>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Recent Activity */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Recent Blocks</h3>
          <div className="space-y-3">
            {/* Placeholder for recent blocks list */}
            <div className="text-sm text-gray-500">Loading recent blocks...</div>
          </div>
        </div>

        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Recent Transactions</h3>
          <div className="space-y-3">
            {/* Placeholder for recent transactions list */}
            <div className="text-sm text-gray-500">Loading recent transactions...</div>
          </div>
        </div>
      </div>

      {/* System Status */}
      <div className="card">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">System Status</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 bg-success-500 rounded-full"></div>
            <span className="text-sm text-gray-600">Network</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 bg-success-500 rounded-full"></div>
            <span className="text-sm text-gray-600">Consensus</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 bg-success-500 rounded-full"></div>
            <span className="text-sm text-gray-600">Storage</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 bg-success-500 rounded-full"></div>
            <span className="text-sm text-gray-600">Security</span>
          </div>
        </div>
      </div>
    </div>
  );
}; 