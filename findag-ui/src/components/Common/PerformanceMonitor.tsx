import React, { useState, useEffect } from 'react';
import { Activity, AlertTriangle, TrendingUp, TrendingDown, Database, Zap, Clock } from 'lucide-react';
import { performanceMonitor } from '../../utils/cache';
import { cache } from '../../utils/cache';

interface PerformanceMetrics {
  [endpoint: string]: {
    avg: number;
    min: number;
    max: number;
    count: number;
  };
}

interface PerformanceAlert {
  type: string;
  message: string;
  timestamp: number;
}

export const PerformanceMonitor: React.FC = () => {
  const [metrics, setMetrics] = useState<PerformanceMetrics>({});
  const [alerts, setAlerts] = useState<PerformanceAlert[]>([]);
  const [cacheStats, setCacheStats] = useState<any>({});
  const [isVisible, setIsVisible] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(true);

  useEffect(() => {
    const updateMetrics = () => {
      setMetrics(performanceMonitor.getMetrics());
      setAlerts(performanceMonitor.getAlerts());
      setCacheStats(cache.getStats());
    };

    updateMetrics();

    if (autoRefresh) {
      const interval = setInterval(updateMetrics, 5000); // Update every 5 seconds
      return () => clearInterval(interval);
    }
  }, [autoRefresh]);

  const formatDuration = (ms: number): string => {
    if (ms < 1000) return `${ms.toFixed(0)}ms`;
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
    return `${(ms / 60000).toFixed(1)}m`;
  };

  const getPerformanceColor = (avg: number): string => {
    if (avg < 100) return 'text-green-600';
    if (avg < 500) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getCacheHitRateColor = (hitRate: number): string => {
    if (hitRate > 0.8) return 'text-green-600';
    if (hitRate > 0.5) return 'text-yellow-600';
    return 'text-red-600';
  };

  if (!isVisible) {
    return (
      <div className="fixed bottom-4 right-4 z-50">
        <button
          onClick={() => setIsVisible(true)}
          className="bg-blue-600 text-white p-3 rounded-full shadow-lg hover:bg-blue-700 transition-colors"
          title="Performance Monitor"
        >
          <Activity className="w-5 h-5" />
        </button>
      </div>
    );
  }

  return (
    <div className="fixed bottom-4 right-4 z-50 w-96 max-h-96 bg-white rounded-lg shadow-xl border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="bg-gray-50 px-4 py-3 border-b border-gray-200 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <Activity className="w-5 h-5 text-blue-600" />
          <h3 className="text-sm font-semibold text-gray-900">Performance Monitor</h3>
        </div>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => setAutoRefresh(!autoRefresh)}
            className={`text-xs px-2 py-1 rounded ${
              autoRefresh ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'
            }`}
          >
            {autoRefresh ? 'Auto' : 'Manual'}
          </button>
          <button
            onClick={() => setIsVisible(false)}
            className="text-gray-400 hover:text-gray-600"
          >
            ×
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="max-h-80 overflow-y-auto">
        {/* Cache Statistics */}
        <div className="p-4 border-b border-gray-200">
          <h4 className="text-sm font-medium text-gray-900 mb-3 flex items-center">
            <Database className="w-4 h-4 mr-2" />
            Cache Statistics
          </h4>
          <div className="grid grid-cols-2 gap-3 text-xs">
            <div>
              <span className="text-gray-500">Size:</span>
              <span className="ml-1 font-medium">{cacheStats.size}/{cacheStats.maxSize}</span>
            </div>
            <div>
              <span className="text-gray-500">Hit Rate:</span>
              <span className={`ml-1 font-medium ${getCacheHitRateColor(cacheStats.hitRate)}`}>
                {(cacheStats.hitRate * 100).toFixed(1)}%
              </span>
            </div>
            <div>
              <span className="text-gray-500">Total Hits:</span>
              <span className="ml-1 font-medium">{cacheStats.totalHits}</span>
            </div>
            <div>
              <span className="text-gray-500">Total Misses:</span>
              <span className="ml-1 font-medium">{cacheStats.totalMisses}</span>
            </div>
          </div>
        </div>

        {/* API Performance */}
        <div className="p-4 border-b border-gray-200">
          <h4 className="text-sm font-medium text-gray-900 mb-3 flex items-center">
            <Zap className="w-4 h-4 mr-2" />
            API Performance
          </h4>
          <div className="space-y-2">
            {Object.entries(metrics).map(([endpoint, data]) => (
              <div key={endpoint} className="text-xs">
                <div className="flex items-center justify-between">
                  <span className="text-gray-600 truncate">{endpoint}</span>
                  <span className={`font-medium ${getPerformanceColor(data.avg)}`}>
                    {formatDuration(data.avg)}
                  </span>
                </div>
                <div className="flex items-center space-x-4 text-gray-500">
                  <span>Min: {formatDuration(data.min)}</span>
                  <span>Max: {formatDuration(data.max)}</span>
                  <span>Calls: {data.count}</span>
                </div>
              </div>
            ))}
            {Object.keys(metrics).length === 0 && (
              <p className="text-xs text-gray-500">No API calls recorded yet</p>
            )}
          </div>
        </div>

        {/* Performance Alerts */}
        {alerts.length > 0 && (
          <div className="p-4">
            <h4 className="text-sm font-medium text-gray-900 mb-3 flex items-center">
              <AlertTriangle className="w-4 h-4 mr-2 text-orange-600" />
              Performance Alerts
            </h4>
            <div className="space-y-2">
              {alerts.slice(-5).map((alert, index) => (
                <div key={index} className="text-xs p-2 bg-orange-50 border border-orange-200 rounded">
                  <div className="text-orange-800">{alert.message}</div>
                  <div className="text-orange-600">
                    {new Date(alert.timestamp).toLocaleTimeString()}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="bg-gray-50 px-4 py-2 border-t border-gray-200">
        <div className="flex items-center justify-between text-xs text-gray-600">
          <span>Last updated: {new Date().toLocaleTimeString()}</span>
          <button
            onClick={() => {
              performanceMonitor.clearOldAlerts();
              setAlerts(performanceMonitor.getAlerts());
            }}
            className="text-blue-600 hover:text-blue-800"
          >
            Clear Alerts
          </button>
        </div>
      </div>
    </div>
  );
};

// Performance monitoring hook
export const usePerformanceMonitoring = () => {
  const trackApiCall = (endpoint: string, startTime: number) => {
    const duration = Date.now() - startTime;
    performanceMonitor.trackApiCall(endpoint, duration);
  };

  const getMetrics = () => performanceMonitor.getMetrics();
  const getAlerts = () => performanceMonitor.getAlerts();
  const clearAlerts = () => performanceMonitor.clearOldAlerts();

  return {
    trackApiCall,
    getMetrics,
    getAlerts,
    clearAlerts
  };
};

// Performance wrapper for API calls
export const withPerformanceTracking = <T extends any[], R>(
  fn: (...args: T) => Promise<R>,
  endpoint: string
) => {
  return async (...args: T): Promise<R> => {
    const startTime = Date.now();
    try {
      const result = await fn(...args);
      performanceMonitor.trackApiCall(endpoint, Date.now() - startTime);
      return result;
    } catch (error) {
      performanceMonitor.trackApiCall(endpoint, Date.now() - startTime);
      throw error;
    }
  };
}; 