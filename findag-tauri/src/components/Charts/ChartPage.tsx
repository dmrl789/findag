import React, { useState, useEffect, useCallback } from 'react';
import { systemAPI, networkAPI, NodeStatus, SystemInfo, NetworkStats } from '../../services/api';
import LoadingSpinner from '../Common/LoadingSpinner';
import { showNotification } from '../Common/NotificationSystem';
import AdvancedChart from './AdvancedChart';
import ChartAnnotations from './ChartAnnotations';

interface ChartData {
  timestamp: number;
  value: number;
  label?: string;
}

interface ChartConfig {
  type: 'line' | 'bar' | 'area' | 'candlestick';
  title: string;
  data: ChartData[];
  color: string;
  yAxisLabel: string;
  xAxisLabel: string;
}

interface Annotation {
  id: string;
  x: number;
  y: number;
  text: string;
  color: string;
  type: 'point' | 'line' | 'text';
}

const ChartPage: React.FC = () => {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [networkStats, setNetworkStats] = useState<NetworkStats | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedTimeframe, setSelectedTimeframe] = useState<'1h' | '24h' | '7d' | '30d'>('24h');
  const [selectedChart, setSelectedChart] = useState<'performance' | 'network' | 'trading' | 'system'>('performance');
  const [chartData, setChartData] = useState<Record<string, ChartData[]>>({});
  const [annotations, setAnnotations] = useState<Annotation[]>([]);
  const [error, setError] = useState<string | null>(null);

  const fetchChartData = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [sys, net] = await Promise.all([
        systemAPI.getSystemInfo(),
        networkAPI.getStatus(),
      ]);
      setSystemInfo(sys);
      setNetworkStats(net);

      // Generate mock chart data based on timeframe
      const now = Date.now();
      const dataPoints = selectedTimeframe === '1h' ? 60 : 
                        selectedTimeframe === '24h' ? 24 : 
                        selectedTimeframe === '7d' ? 168 : 720;
      
      const interval = selectedTimeframe === '1h' ? 60000 : 
                      selectedTimeframe === '24h' ? 3600000 : 
                      selectedTimeframe === '7d' ? 3600000 : 86400000;

      const performanceData: ChartData[] = Array.from({ length: dataPoints }, (_, i) => ({
        timestamp: now - (dataPoints - i) * interval,
        value: Math.random() * 100,
        label: new Date(now - (dataPoints - i) * interval).toLocaleTimeString(),
      }));

      const networkData: ChartData[] = Array.from({ length: dataPoints }, (_, i) => ({
        timestamp: now - (dataPoints - i) * interval,
        value: Math.random() * 1000,
        label: new Date(now - (dataPoints - i) * interval).toLocaleTimeString(),
      }));

      const tradingData: ChartData[] = Array.from({ length: dataPoints }, (_, i) => ({
        timestamp: now - (dataPoints - i) * interval,
        value: Math.random() * 10000,
        label: new Date(now - (dataPoints - i) * interval).toLocaleTimeString(),
      }));

      const systemData: ChartData[] = Array.from({ length: dataPoints }, (_, i) => ({
        timestamp: now - (dataPoints - i) * interval,
        value: Math.random() * 100,
        label: new Date(now - (dataPoints - i) * interval).toLocaleTimeString(),
      }));

      setChartData({
        performance: performanceData,
        network: networkData,
        trading: tradingData,
        system: systemData,
      });
    } catch (err) {
      setError('Failed to fetch chart data');
      showNotification({ type: 'error', title: 'Error', message: 'Failed to fetch chart data' });
    } finally {
      setIsLoading(false);
    }
  }, [selectedTimeframe]);

  useEffect(() => {
    fetchChartData();
    const interval = setInterval(() => {
      fetchChartData();
    }, 30000);
    return () => clearInterval(interval);
  }, [fetchChartData]);

  const handleTimeframeChange = (timeframe: '1h' | '24h' | '7d' | '30d') => {
    setSelectedTimeframe(timeframe);
  };

  const handleChartTypeChange = (chartType: 'performance' | 'network' | 'trading' | 'system') => {
    setSelectedChart(chartType);
  };

  const chartConfigs: Record<string, ChartConfig> = {
    performance: {
      type: 'line',
      title: 'Performance Metrics',
      data: chartData.performance || [],
      color: '#3B82F6',
      yAxisLabel: 'Performance (%)',
      xAxisLabel: 'Time',
    },
    network: {
      type: 'area',
      title: 'Network Activity',
      data: chartData.network || [],
      color: '#10B981',
      yAxisLabel: 'Bandwidth (MB/s)',
      xAxisLabel: 'Time',
    },
    trading: {
      type: 'candlestick',
      title: 'Trading Volume',
      data: chartData.trading || [],
      color: '#F59E0B',
      yAxisLabel: 'Volume',
      xAxisLabel: 'Time',
    },
    system: {
      type: 'bar',
      title: 'System Resources',
      data: chartData.system || [],
      color: '#EF4444',
      yAxisLabel: 'Usage (%)',
      xAxisLabel: 'Time',
    },
  };

  const currentConfig = chartConfigs[selectedChart];

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
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Advanced Charts</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Real-time performance and analytics visualization
          </p>
        </div>
        
        {/* Controls */}
        <div className="flex items-center space-x-4">
          {/* Chart Type Selector */}
          <div className="flex bg-gray-100 dark:bg-gray-700 rounded-lg p-1">
            {(['performance', 'network', 'trading', 'system'] as const).map((chartType) => (
              <button
                key={chartType}
                onClick={() => handleChartTypeChange(chartType)}
                className={`px-3 py-1 text-sm font-medium rounded-md transition-colors duration-200 ${
                  selectedChart === chartType
                    ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
                    : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white'
                }`}
              >
                {chartType.charAt(0).toUpperCase() + chartType.slice(1)}
              </button>
            ))}
          </div>
          
          {/* Timeframe Selector */}
          <div className="flex bg-gray-100 dark:bg-gray-700 rounded-lg p-1">
            {(['1h', '24h', '7d', '30d'] as const).map((timeframe) => (
              <button
                key={timeframe}
                onClick={() => handleTimeframeChange(timeframe)}
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
        </div>
      </div>

      {/* Chart Container */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div className="mb-4">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
            {currentConfig.title}
          </h2>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {selectedTimeframe} timeframe • {currentConfig.data.length} data points
          </p>
        </div>
        
        <div className="h-96">
          <AdvancedChart
            config={currentConfig}
            annotations={annotations}
            onAnnotationAdd={(annotation: Annotation) => setAnnotations([...annotations, annotation])}
            onAnnotationRemove={(id: string) => setAnnotations(annotations.filter(a => a.id !== id))}
          />
        </div>
      </div>

      {/* Annotations Panel */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Chart Annotations
        </h3>
        <ChartAnnotations
          annotations={annotations}
          onAdd={(annotation: Annotation) => setAnnotations([...annotations, annotation])}
          onRemove={(id: string) => setAnnotations(annotations.filter(a => a.id !== id))}
          onUpdate={(id: string, updatedAnnotation: Annotation) => 
            setAnnotations(annotations.map(a => a.id === id ? updatedAnnotation : a))
          }
        />
      </div>

      {/* Metrics Summary */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">CPU Usage</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {systemInfo ? `${systemInfo.cpu_usage.toFixed(1)}%` : '-'}
              </p>
            </div>
            <div className="text-3xl">🖥️</div>
          </div>
        </div>
        
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Memory Usage</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {systemInfo ? `${((systemInfo.memory_total - systemInfo.memory_available) / (1024 ** 3)).toFixed(1)} GB` : '-'}
              </p>
            </div>
            <div className="text-3xl">💾</div>
          </div>
        </div>
        
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Network Peers</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {networkStats ? networkStats.connected_peers : '-'}
              </p>
            </div>
            <div className="text-3xl">🌐</div>
          </div>
        </div>
        
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Bandwidth</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {networkStats ? `${(networkStats.total_bandwidth / (1024 ** 2)).toFixed(1)} MB` : '-'}
              </p>
            </div>
            <div className="text-3xl">📡</div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ChartPage; 