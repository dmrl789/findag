import React, { useState, useEffect } from 'react';
import { BarChart3, TrendingUp, Activity, Zap, Clock, Database, AlertTriangle, CheckCircle } from 'lucide-react';
import { useAppStore } from '../../store';
import { formatNumber, formatLatency, formatUptime } from '../../utils/formatters';
import { finDAGApi } from '../../services/api';

interface TimeSeriesData {
  timestamp: number;
  value: number;
}

interface PerformanceTimeSeries {
  tps: TimeSeriesData[];
  latency: TimeSeriesData[];
  nodes: TimeSeriesData[];
  blocks: TimeSeriesData[];
  message: string;
}

export const MetricsPage: React.FC = () => {
  const { networkMetrics, nodeMetrics, isLoading, fetchNetworkMetrics, fetchNodeMetrics } = useAppStore();
  const [timeRange, setTimeRange] = useState<'1h' | '6h' | '24h' | '7d'>('24h');
  const [selectedMetric, setSelectedMetric] = useState<string>('tps');
  const [timeSeriesData, setTimeSeriesData] = useState<PerformanceTimeSeries | null>(null);
  const [isLoadingTimeSeries, setIsLoadingTimeSeries] = useState(false);
  const [timeSeriesError, setTimeSeriesError] = useState<string | null>(null);

  useEffect(() => {
    fetchNetworkMetrics();
    fetchNodeMetrics();
  }, [fetchNetworkMetrics, fetchNodeMetrics]);

  useEffect(() => {
    fetchTimeSeriesData();
  }, [timeRange]);

  const fetchTimeSeriesData = async () => {
    setIsLoadingTimeSeries(true);
    setTimeSeriesError(null);
    
    try {
      const data = await finDAGApi.getPerformanceMetricsTimeSeries(timeRange);
      setTimeSeriesData(data);
    } catch (error) {
      console.error('Failed to fetch time-series data:', error);
      setTimeSeriesError('Failed to load performance data');
    } finally {
      setIsLoadingTimeSeries(false);
    }
  };

  const getMetricData = (): TimeSeriesData[] => {
    if (!timeSeriesData) return [];
    
    switch (selectedMetric) {
      case 'tps':
        return timeSeriesData.tps;
      case 'latency':
        return timeSeriesData.latency;
      case 'nodes':
        return timeSeriesData.nodes;
      case 'blocks':
        return timeSeriesData.blocks;
      default:
        return timeSeriesData.tps;
    }
  };

  const metricData = getMetricData();

  const getMetricLabel = (metric: string): string => {
    switch (metric) {
      case 'tps':
        return 'Transactions Per Second';
      case 'latency':
        return 'Latency (ms)';
      case 'nodes':
        return 'Active Nodes';
      case 'blocks':
        return 'Blocks Produced';
      default:
        return metric.toUpperCase();
    }
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Metrics</h1>
          <p className="text-gray-600">Performance analytics and system metrics</p>
        </div>
        <div className="flex items-center space-x-3">
          <select
            value={timeRange}
            onChange={(e) => setTimeRange(e.target.value as any)}
            className="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="1h">Last Hour</option>
            <option value="6h">Last 6 Hours</option>
            <option value="24h">Last 24 Hours</option>
            <option value="7d">Last 7 Days</option>
          </select>
          <button className="btn-secondary flex items-center space-x-2">
            <BarChart3 className="w-4 h-4" />
            <span>Export Report</span>
          </button>
        </div>
      </div>

      {/* Key Performance Indicators */}
      {networkMetrics && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center">
                  <Zap className="w-5 h-5 text-blue-600" />
                </div>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Total TPS</p>
                <p className="text-2xl font-semibold text-gray-900">{formatNumber(networkMetrics.totalTPS)}</p>
                <p className="text-xs text-green-600">+12.5% from last period</p>
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
                <p className="text-2xl font-semibold text-gray-900">{networkMetrics.activeNodes}</p>
                <p className="text-xs text-green-600">+2 from last period</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-yellow-100 rounded-lg flex items-center justify-center">
                  <Clock className="w-5 h-5 text-yellow-600" />
                </div>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Avg Latency</p>
                <p className="text-2xl font-semibold text-gray-900">{formatLatency(networkMetrics.averageLatency)}</p>
                <p className="text-xs text-red-600">+5ms from last period</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-purple-100 rounded-lg flex items-center justify-center">
                  <Database className="w-5 h-5 text-purple-600" />
                </div>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Finalized Blocks</p>
                <p className="text-2xl font-semibold text-gray-900">{formatNumber(networkMetrics.finalizedBlocks)}</p>
                <p className="text-xs text-green-600">+1,234 from last period</p>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Metric Selection and Chart */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-6">
          <h3 className="text-lg font-semibold text-gray-900">Performance Trends</h3>
          <div className="flex space-x-2">
            <button
              onClick={() => setSelectedMetric('tps')}
              className={`px-3 py-1 text-sm font-medium rounded-md transition-colors ${
                selectedMetric === 'tps'
                  ? 'bg-blue-100 text-blue-700'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              TPS
            </button>
            <button
              onClick={() => setSelectedMetric('latency')}
              className={`px-3 py-1 text-sm font-medium rounded-md transition-colors ${
                selectedMetric === 'latency'
                  ? 'bg-blue-100 text-blue-700'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              Latency
            </button>
            <button
              onClick={() => setSelectedMetric('nodes')}
              className={`px-3 py-1 text-sm font-medium rounded-md transition-colors ${
                selectedMetric === 'nodes'
                  ? 'bg-blue-100 text-blue-700'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              Nodes
            </button>
            <button
              onClick={() => setSelectedMetric('blocks')}
              className={`px-3 py-1 text-sm font-medium rounded-md transition-colors ${
                selectedMetric === 'blocks'
                  ? 'bg-blue-100 text-blue-700'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              Blocks
            </button>
          </div>
        </div>
        
        {/* Chart Placeholder */}
        <div className="bg-gray-50 rounded-lg h-64 flex items-center justify-center">
          {isLoadingTimeSeries ? (
            <div className="text-center">
              <TrendingUp className="w-12 h-12 text-gray-400 mx-auto mb-2" />
              <p className="text-gray-600">Loading performance data...</p>
            </div>
          ) : timeSeriesError ? (
            <div className="text-center">
              <AlertTriangle className="w-12 h-12 text-red-400 mx-auto mb-2" />
              <p className="text-gray-600">{timeSeriesError}</p>
            </div>
          ) : (
            <div className="text-center">
              <TrendingUp className="w-12 h-12 text-gray-400 mx-auto mb-2" />
              <p className="text-gray-600">Performance chart for {getMetricLabel(selectedMetric).toLowerCase()}</p>
              <p className="text-sm text-gray-500">Time range: {timeRange}</p>
              <div className="mt-4 text-xs text-gray-500">
                {metricData.length} data points available
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Node Performance Table */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900">Node Performance</h3>
        </div>
        
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Node
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  TPS
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Latency
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Uptime
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Memory
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  CPU
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Peers
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {isLoading.nodeMetrics ? (
                <tr>
                  <td colSpan={7} className="px-6 py-4 text-center text-gray-500">
                    Loading node metrics...
                  </td>
                </tr>
              ) : nodeMetrics.length === 0 ? (
                <tr>
                  <td colSpan={7} className="px-6 py-4 text-center text-gray-500">
                    No node metrics available
                  </td>
                </tr>
              ) : (
                nodeMetrics.map((node) => (
                  <tr key={node.nodeId} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm font-medium text-gray-900">
                        {node.nodeId}
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
                        {formatUptime(node.uptime)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {formatNumber(node.memoryUsage)} bytes
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {(node.cpuUsage * 100).toFixed(1)}%
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {node.connectedPeers}
                      </div>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* System Health Overview */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">System Health</h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">Network Connectivity</span>
              <span className="text-sm font-medium text-green-600">Healthy</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">Consensus Status</span>
              <span className="text-sm font-medium text-green-600">Active</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">Storage Health</span>
              <span className="text-sm font-medium text-green-600">Good</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">Memory Usage</span>
              <span className="text-sm font-medium text-yellow-600">75%</span>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Recent Alerts</h3>
          <div className="space-y-3">
            <div className="flex items-center space-x-3 p-3 bg-yellow-50 rounded-lg">
              <AlertTriangle className="w-4 h-4 text-yellow-600" />
              <div>
                <p className="text-sm font-medium text-yellow-800">High Memory Usage</p>
                <p className="text-xs text-yellow-700">Node validator-03 showing 85% memory usage</p>
              </div>
            </div>
            <div className="flex items-center space-x-3 p-3 bg-green-50 rounded-lg">
              <CheckCircle className="w-4 h-4 text-green-600" />
              <div>
                <p className="text-sm font-medium text-green-800">Network Recovery</p>
                <p className="text-xs text-green-700">All nodes back online after maintenance</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}; 