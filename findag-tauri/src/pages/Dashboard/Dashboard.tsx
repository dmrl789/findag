import React, { useState, useEffect, useCallback } from 'react';
import { nodeAPI, systemAPI, networkAPI, NodeStatus, SystemInfo, NetworkStats } from '../../services/api';
import LoadingSpinner from '../../components/Common/LoadingSpinner';
import { showNotification } from '../../components/Common/NotificationSystem';

interface MetricCard {
  title: string;
  value: string | number;
  change?: number;
  icon: string;
  color: string;
}

const Dashboard: React.FC = () => {
  const [nodeStatus, setNodeStatus] = useState<NodeStatus | null>(null);
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [networkStats, setNetworkStats] = useState<NetworkStats | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [selectedTimeframe, setSelectedTimeframe] = useState<'1h' | '24h' | '7d'>('24h');
  const [error, setError] = useState<string | null>(null);

  const fetchAll = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [status, sys, net] = await Promise.all([
        nodeAPI.getStatus(),
        systemAPI.getSystemInfo(),
        networkAPI.getStatus(),
      ]);
      setNodeStatus(status);
      setSystemInfo(sys);
      setNetworkStats(net);
    } catch (err: any) {
      setError('Failed to fetch dashboard data');
      showNotification({ type: 'error', title: 'Error', message: 'Failed to fetch dashboard data' });
    } finally {
      setIsLoading(false);
      setIsRefreshing(false);
    }
  }, []);

  useEffect(() => {
    fetchAll();
    const interval = setInterval(() => {
      fetchAll();
    }, 30000);
    return () => clearInterval(interval);
  }, [fetchAll]);

  const handleRefresh = () => {
    setIsRefreshing(true);
    fetchAll();
  };

  const handleStartNode = async () => {
    try {
      await nodeAPI.startNode();
      showNotification({ type: 'success', title: 'Node Started', message: 'FinDAG node is now running' });
      fetchAll();
    } catch (error) {
      showNotification({ type: 'error', title: 'Start Failed', message: 'Failed to start FinDAG node' });
    }
  };

  const handleStopNode = async () => {
    try {
      await nodeAPI.stopNode();
      showNotification({ type: 'success', title: 'Node Stopped', message: 'FinDAG node has been stopped' });
      fetchAll();
    } catch (error) {
      showNotification({ type: 'error', title: 'Stop Failed', message: 'Failed to stop FinDAG node' });
    }
  };

  // Example metrics (expand as needed)
  const metricCards: MetricCard[] = [
    {
      title: 'Node Status',
      value: nodeStatus?.is_running ? 'Online' : 'Offline',
      icon: nodeStatus?.is_running ? '🟢' : '🔴',
      color: nodeStatus?.is_running ? 'text-green-600' : 'text-red-600',
    },
    {
      title: 'Transactions/sec',
      value: nodeStatus?.tps ?? '-',
      icon: '📈',
      color: 'text-blue-600',
    },
    {
      title: 'Network Peers',
      value: nodeStatus?.peers ?? '-',
      icon: '🌐',
      color: 'text-purple-600',
    },
    {
      title: 'CPU Usage',
      value: systemInfo ? `${systemInfo.cpu_usage.toFixed(1)}%` : '-',
      icon: '🖥️',
      color: 'text-orange-600',
    },
    {
      title: 'Memory Used',
      value: systemInfo ? `${((systemInfo.memory_total - systemInfo.memory_available) / (1024 ** 3)).toFixed(1)} GB` : '-',
      icon: '💾',
      color: 'text-pink-600',
    },
    {
      title: 'Bandwidth',
      value: networkStats ? `${(networkStats.total_bandwidth / (1024 ** 2)).toFixed(1)} MB` : '-',
      icon: '📡',
      color: 'text-cyan-600',
    },
  ];

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64 text-red-600">
        {error}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Dashboard</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Welcome! Here is your FinDAG node overview.
          </p>
        </div>
        <div className="flex items-center space-x-4">
          {/* Timeframe Selector */}
          <div className="flex bg-gray-100 dark:bg-gray-700 rounded-lg p-1">
            {(['1h', '24h', '7d'] as const).map((timeframe) => (
              <button
                key={timeframe}
                onClick={() => setSelectedTimeframe(timeframe)}
                className={`px-3 py-1 text-sm font-medium rounded-md transition-colors duration-200 ${
                  selectedTimeframe === timeframe
                    ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
                    : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white'
                }`}
              >
                {timeframe}
              </button>
            ))}
          </div>
          {/* Refresh Button */}
          <button
            onClick={handleRefresh}
            disabled={isRefreshing}
            className="p-2 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors duration-200"
          >
            <span className="text-lg">🔄</span>
          </button>
        </div>
      </div>
      {/* Metrics Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {metricCards.map((card, index) => (
          <div key={index} className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600 dark:text-gray-400">{card.title}</p>
                <p className={`text-2xl font-bold ${card.color}`}>{card.value}</p>
                {card.change !== undefined && (
                  <p className={`text-sm ${card.change >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                    {card.change >= 0 ? '+' : ''}{card.change}%
                  </p>
                )}
              </div>
              <div className="text-3xl">{card.icon}</div>
            </div>
          </div>
        ))}
      </div>
      {/* Node Controls */}
      <div className="flex space-x-4">
        <button
          onClick={handleStartNode}
          className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition"
          disabled={nodeStatus?.is_running}
        >
          Start Node
        </button>
        <button
          onClick={handleStopNode}
          className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition"
          disabled={!nodeStatus?.is_running}
        >
          Stop Node
        </button>
      </div>
      {/* System Info */}
      {systemInfo && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-lg font-semibold mb-2">System Info</h2>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
            <div><span className="font-medium">Platform:</span> {systemInfo.platform}</div>
            <div><span className="font-medium">CPU Cores:</span> {systemInfo.cpu_cores}</div>
            <div><span className="font-medium">CPU Usage:</span> {systemInfo.cpu_usage.toFixed(1)}%</div>
            <div><span className="font-medium">Memory Total:</span> {(systemInfo.memory_total / (1024 ** 3)).toFixed(1)} GB</div>
            <div><span className="font-medium">Memory Available:</span> {(systemInfo.memory_available / (1024 ** 3)).toFixed(1)} GB</div>
            <div><span className="font-medium">Disk Total:</span> {(systemInfo.disk_total / (1024 ** 3)).toFixed(1)} GB</div>
            <div><span className="font-medium">Disk Available:</span> {(systemInfo.disk_available / (1024 ** 3)).toFixed(1)} GB</div>
          </div>
        </div>
      )}
      {/* Network Stats */}
      {networkStats && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-lg font-semibold mb-2">Network Stats</h2>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
            <div><span className="font-medium">Total Peers:</span> {networkStats.total_peers}</div>
            <div><span className="font-medium">Connected Peers:</span> {networkStats.connected_peers}</div>
            <div><span className="font-medium">Bandwidth:</span> {(networkStats.total_bandwidth / (1024 ** 2)).toFixed(1)} MB</div>
            <div><span className="font-medium">Avg Latency:</span> {networkStats.average_latency} ms</div>
            <div><span className="font-medium">Blocks Received:</span> {networkStats.blocks_received}</div>
            <div><span className="font-medium">Blocks Sent:</span> {networkStats.blocks_sent}</div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard; 