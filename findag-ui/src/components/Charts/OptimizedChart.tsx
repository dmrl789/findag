import React, { useState, useRef, useEffect, useCallback, useMemo } from 'react';
import { 
  LineChart, 
  Line, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer,
  BarChart,
  Bar,
  ComposedChart,
  Area,
  AreaChart
} from 'recharts';
import { 
  Settings, 
  Zap, 
  Activity,
  TrendingUp,
  TrendingDown,
  Clock,
  Database
} from 'lucide-react';
import { AdvancedChart } from './AdvancedChart';
import { usePerformance, useMemoizedValue, OptimizedComponent } from '../Common/PerformanceOptimizer';
import { PricePoint } from '../../types';

export interface OptimizedChartProps {
  pair: string;
  data: PricePoint[];
  timeFrame: '1m' | '5m' | '15m' | '1h' | '4h' | '1d' | '1w';
  chartType: 'line' | 'candlestick' | 'area' | 'volume' | 'technical';
  onTimeFrameChange: (timeFrame: string) => void;
  onChartTypeChange: (type: string) => void;
  loading?: boolean;
  showVolume?: boolean;
  showMA?: boolean;
  showBB?: boolean;
  showRSI?: boolean;
  className?: string;
  enableOptimizations?: boolean;
  dataUpdateInterval?: number;
  maxDataPoints?: number;
}

export const OptimizedChart: React.FC<OptimizedChartProps> = ({
  pair,
  data,
  timeFrame,
  chartType,
  onTimeFrameChange,
  onChartTypeChange,
  loading = false,
  showVolume = true,
  showMA = true,
  showBB = false,
  showRSI = false,
  className = '',
  enableOptimizations = true,
  dataUpdateInterval = 1000,
  maxDataPoints = 1000,
}) => {
  const {
    setCache,
    getCache,
    addMetric,
    settings,
    getPerformanceReport,
  } = usePerformance();

  const [optimizedData, setOptimizedData] = useState<PricePoint[]>([]);
  const [renderCount, setRenderCount] = useState(0);
  const [performanceMetrics, setPerformanceMetrics] = useState({
    renderTime: 0,
    dataProcessingTime: 0,
    cacheHits: 0,
    cacheMisses: 0,
  });

  const chartRef = useRef<HTMLDivElement>(null);
  const lastRenderTime = useRef<number>(0);
  const dataProcessingStart = useRef<number>(0);

  // Cache key for this chart's data
  const cacheKey = useMemo(() => 
    `chart-${pair}-${timeFrame}-${chartType}-${data.length}`, 
    [pair, timeFrame, chartType, data.length]
  );

  // Optimize data processing
  const processData = useCallback((rawData: PricePoint[]): PricePoint[] => {
    dataProcessingStart.current = performance.now();

    // Check cache first
    if (settings.enableCaching) {
      const cached = getCache<PricePoint[]>(cacheKey);
      if (cached) {
        addMetric({
          name: 'Chart Cache Hit',
          value: 1,
          unit: 'hit',
          category: 'cache',
          trend: 'up',
        });
        setPerformanceMetrics(prev => ({ ...prev, cacheHits: prev.cacheHits + 1 }));
        return cached;
      }
    }

    // Process data
    let processedData = [...rawData];

    // Limit data points for performance
    if (processedData.length > maxDataPoints) {
      const step = Math.ceil(processedData.length / maxDataPoints);
      processedData = processedData.filter((_, index) => index % step === 0);
    }

    // Sort by timestamp
    processedData.sort((a, b) => a.timestamp - b.timestamp);

    // Cache the processed data
    if (settings.enableCaching) {
      setCache(cacheKey, processedData, 5 * 60 * 1000); // 5 minutes TTL
      addMetric({
        name: 'Chart Cache Miss',
        value: 1,
        unit: 'miss',
        category: 'cache',
        trend: 'down',
      });
      setPerformanceMetrics(prev => ({ ...prev, cacheMisses: prev.cacheMisses + 1 }));
    }

    const processingTime = performance.now() - dataProcessingStart.current;
    addMetric({
      name: 'Data Processing Time',
      value: processingTime,
      unit: 'ms',
      category: 'rendering',
      trend: processingTime > 16 ? 'up' : 'down', // 60fps threshold
    });

    setPerformanceMetrics(prev => ({ 
      ...prev, 
      dataProcessingTime: processingTime 
    }));

    return processedData;
  }, [settings.enableCaching, getCache, setCache, cacheKey, maxDataPoints, addMetric]);

  // Memoized data processing
  const processedData = useMemoizedValue(
    () => processData(data),
    [data, processData],
    cacheKey
  );

  // Update optimized data when processed data changes
  useEffect(() => {
    setOptimizedData(processedData);
  }, [processedData]);

  // Performance monitoring
  useEffect(() => {
    if (!enableOptimizations || !settings.performanceMonitoring) return;

    const interval = setInterval(() => {
      const report = getPerformanceReport();
      
      addMetric({
        name: 'Chart Render Count',
        value: renderCount,
        unit: 'renders',
        category: 'rendering',
        trend: renderCount > 100 ? 'up' : 'stable',
      });

      addMetric({
        name: 'Chart Memory Usage',
        value: report.memoryUsage,
        unit: 'MB',
        category: 'memory',
        trend: report.memoryUsage > 100 ? 'up' : 'down',
      });
    }, 10000); // Every 10 seconds

    return () => clearInterval(interval);
  }, [enableOptimizations, settings.performanceMonitoring, renderCount, addMetric, getPerformanceReport]);

  // Optimized render handler
  const handleRender = useCallback(() => {
    const startTime = performance.now();
    setRenderCount(prev => prev + 1);
    
    const renderTime = performance.now() - startTime;
    lastRenderTime.current = renderTime;

    addMetric({
      name: 'Chart Render Time',
      value: renderTime,
      unit: 'ms',
      category: 'rendering',
      trend: renderTime > 16 ? 'up' : 'down',
    });

    setPerformanceMetrics(prev => ({ 
      ...prev, 
      renderTime 
    }));
  }, [addMetric]);

  // Debounced data updates
  const debouncedDataUpdate = useCallback(
    debounce((newData: PricePoint[]) => {
      setOptimizedData(newData);
    }, dataUpdateInterval),
    [dataUpdateInterval]
  );

  // Handle real-time data updates
  useEffect(() => {
    if (enableOptimizations) {
      debouncedDataUpdate(processedData);
    } else {
      setOptimizedData(processedData);
    }
  }, [processedData, enableOptimizations, debouncedDataUpdate]);

  // Performance settings panel
  const [showPerformancePanel, setShowPerformancePanel] = useState(false);

  return (
    <div className={`bg-white rounded-lg shadow-sm border border-gray-200 ${className}`}>
      {/* Header with performance controls */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <h3 className="text-lg font-medium text-gray-900">{pair} Chart</h3>
            
            {enableOptimizations && (
              <div className="flex items-center space-x-2">
                <Zap className="w-4 h-4 text-green-600" />
                <span className="text-sm text-green-600 font-medium">Optimized</span>
                
                <button
                  onClick={() => setShowPerformancePanel(!showPerformancePanel)}
                  className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
                  title="Performance Settings"
                >
                  <Activity className="w-4 h-4" />
                </button>
              </div>
            )}
          </div>

          <div className="flex items-center space-x-2 text-sm text-gray-600">
            <Clock className="w-4 h-4" />
            <span>Render: {renderCount}</span>
            <span>•</span>
            <span>{performanceMetrics.renderTime.toFixed(1)}ms</span>
          </div>
        </div>

        {/* Performance Panel */}
        {showPerformancePanel && enableOptimizations && (
          <div className="mt-4 p-4 bg-blue-50 rounded-lg">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div>
                <div className="text-sm font-medium text-blue-900">Render Time</div>
                <div className="text-lg font-semibold text-blue-700">
                  {performanceMetrics.renderTime.toFixed(1)}ms
                </div>
              </div>
              
              <div>
                <div className="text-sm font-medium text-blue-900">Processing Time</div>
                <div className="text-lg font-semibold text-blue-700">
                  {performanceMetrics.dataProcessingTime.toFixed(1)}ms
                </div>
              </div>
              
              <div>
                <div className="text-sm font-medium text-blue-900">Cache Hits</div>
                <div className="text-lg font-semibold text-blue-700">
                  {performanceMetrics.cacheHits}
                </div>
              </div>
              
              <div>
                <div className="text-sm font-medium text-blue-900">Cache Misses</div>
                <div className="text-lg font-semibold text-blue-700">
                  {performanceMetrics.cacheMisses}
                </div>
              </div>
            </div>
            
            <div className="mt-3 pt-3 border-t border-blue-200">
              <div className="flex items-center justify-between text-sm">
                <span className="text-blue-700">Data Points: {optimizedData.length}</span>
                <span className="text-blue-700">Max: {maxDataPoints}</span>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Chart Container */}
      <div ref={chartRef} className="relative">
        <OptimizedComponent
          cacheKey={`chart-render-${cacheKey}`}
          dependencies={[optimizedData, timeFrame, chartType]}
        >
          <AdvancedChart
            pair={pair}
            data={optimizedData}
            timeFrame={timeFrame}
            chartType={chartType}
            onTimeFrameChange={onTimeFrameChange}
            onChartTypeChange={onChartTypeChange}
            loading={loading}
            showVolume={showVolume}
            showMA={showMA}
            showBB={showBB}
            showRSI={showRSI}
          />
        </OptimizedComponent>
      </div>

      {/* Performance Footer */}
      {enableOptimizations && (
        <div className="px-6 py-3 border-t border-gray-200 bg-gray-50">
          <div className="flex items-center justify-between text-sm text-gray-600">
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-1">
                <Database className="w-4 h-4" />
                <span>Cache: {performanceMetrics.cacheHits} hits, {performanceMetrics.cacheMisses} misses</span>
              </div>
              
              <div className="flex items-center space-x-1">
                <TrendingUp className="w-4 h-4" />
                <span>Render: {performanceMetrics.renderTime.toFixed(1)}ms</span>
              </div>
            </div>
            
            <div className="flex items-center space-x-2">
              {performanceMetrics.renderTime > 16 && (
                <div className="flex items-center space-x-1 text-yellow-600">
                  <TrendingDown className="w-4 h-4" />
                  <span>Slow</span>
                </div>
              )}
              
              {performanceMetrics.renderTime <= 16 && (
                <div className="flex items-center space-x-1 text-green-600">
                  <TrendingUp className="w-4 h-4" />
                  <span>Fast</span>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

// Debounce utility function
function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}

// Hook for optimized chart state
export const useOptimizedChart = () => {
  const [optimizations, setOptimizations] = useState({
    enableCaching: true,
    enableMemoization: true,
    enableLazyLoading: true,
    enableVirtualScrolling: true,
    dataUpdateInterval: 1000,
    maxDataPoints: 1000,
  });

  const [performanceSettings, setPerformanceSettings] = useState({
    showPerformancePanel: false,
    enableRealTimeMonitoring: true,
    enableCacheStats: true,
    enableRenderMetrics: true,
  });

  const updateOptimizations = (newOptimizations: Partial<typeof optimizations>) => {
    setOptimizations(prev => ({ ...prev, ...newOptimizations }));
  };

  const updatePerformanceSettings = (newSettings: Partial<typeof performanceSettings>) => {
    setPerformanceSettings(prev => ({ ...prev, ...newSettings }));
  };

  return {
    optimizations,
    performanceSettings,
    updateOptimizations,
    updatePerformanceSettings,
  };
}; 