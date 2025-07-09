import React, { createContext, useContext, useEffect, useState, useRef, useCallback, useMemo } from 'react';
import { 
  Activity, 
  Clock, 
  Database, 
  Memory, 
  Zap,
  Settings,
  RefreshCw,
  Trash2,
  BarChart3,
  TrendingUp,
  TrendingDown
} from 'lucide-react';

// Performance monitoring types
interface PerformanceMetric {
  id: string;
  name: string;
  value: number;
  unit: string;
  timestamp: number;
  category: 'memory' | 'cpu' | 'network' | 'rendering' | 'cache';
  trend: 'up' | 'down' | 'stable';
}

interface CacheEntry<T> {
  data: T;
  timestamp: number;
  ttl: number;
  accessCount: number;
  lastAccessed: number;
}

interface PerformanceState {
  // Cache management
  cache: Map<string, CacheEntry<any>>;
  cacheStats: {
    hits: number;
    misses: number;
    size: number;
    maxSize: number;
  };
  
  // Performance metrics
  metrics: PerformanceMetric[];
  isMonitoring: boolean;
  
  // Optimization settings
  settings: {
    enableCaching: boolean;
    enableMemoization: boolean;
    enableLazyLoading: boolean;
    enableVirtualScrolling: boolean;
    cacheTTL: number;
    maxCacheSize: number;
    performanceMonitoring: boolean;
  };
  
  // Actions
  setCache: <T>(key: string, data: T, ttl?: number) => void;
  getCache: <T>(key: string) => T | null;
  clearCache: (pattern?: string) => void;
  addMetric: (metric: Omit<PerformanceMetric, 'id' | 'timestamp'>) => void;
  toggleMonitoring: () => void;
  updateSettings: (settings: Partial<PerformanceState['settings']>) => void;
  getCacheStats: () => PerformanceState['cacheStats'];
  getPerformanceReport: () => {
    averageResponseTime: number;
    cacheHitRate: number;
    memoryUsage: number;
    renderCount: number;
  };
}

const PerformanceContext = createContext<PerformanceState | undefined>(undefined);

export const usePerformance = () => {
  const context = useContext(PerformanceContext);
  if (!context) {
    throw new Error('usePerformance must be used within a PerformanceProvider');
  }
  return context;
};

interface PerformanceProviderProps {
  children: React.ReactNode;
  maxCacheSize?: number;
  defaultTTL?: number;
}

export const PerformanceProvider: React.FC<PerformanceProviderProps> = ({
  children,
  maxCacheSize = 100,
  defaultTTL = 5 * 60 * 1000, // 5 minutes
}) => {
  const [cache, setCacheState] = useState<Map<string, CacheEntry<any>>>(new Map());
  const [metrics, setMetrics] = useState<PerformanceMetric[]>([]);
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [cacheStats, setCacheStats] = useState({
    hits: 0,
    misses: 0,
    size: 0,
    maxSize: maxCacheSize,
  });

  const [settings, setSettings] = useState<PerformanceState['settings']>({
    enableCaching: true,
    enableMemoization: true,
    enableLazyLoading: true,
    enableVirtualScrolling: true,
    cacheTTL: defaultTTL,
    maxCacheSize,
    performanceMonitoring: true,
  });

  const monitoringInterval = useRef<NodeJS.Timeout | null>(null);
  const renderCount = useRef(0);

  // Cache management
  const setCache = useCallback(<T,>(key: string, data: T, ttl: number = settings.cacheTTL) => {
    if (!settings.enableCaching) return;

    const now = Date.now();
    const entry: CacheEntry<T> = {
      data,
      timestamp: now,
      ttl,
      accessCount: 0,
      lastAccessed: now,
    };

    setCacheState(prev => {
      const newCache = new Map(prev);
      
      // Check if we need to evict entries
      if (newCache.size >= settings.maxCacheSize) {
        const entries = Array.from(newCache.entries());
        entries.sort((a, b) => a[1].lastAccessed - b[1].lastAccessed);
        newCache.delete(entries[0][0]);
      }
      
      newCache.set(key, entry);
      return newCache;
    });

    setCacheStats(prev => ({ ...prev, size: cache.size + 1 }));
  }, [settings.enableCaching, settings.cacheTTL, settings.maxCacheSize, cache.size]);

  const getCache = useCallback(<T,>(key: string): T | null => {
    if (!settings.enableCaching) return null;

    const entry = cache.get(key);
    if (!entry) {
      setCacheStats(prev => ({ ...prev, misses: prev.misses + 1 }));
      return null;
    }

    const now = Date.now();
    if (now - entry.timestamp > entry.ttl) {
      setCacheState(prev => {
        const newCache = new Map(prev);
        newCache.delete(key);
        return newCache;
      });
      setCacheStats(prev => ({ ...prev, misses: prev.misses + 1, size: prev.size - 1 }));
      return null;
    }

    // Update access stats
    entry.accessCount++;
    entry.lastAccessed = now;
    setCacheStats(prev => ({ ...prev, hits: prev.hits + 1 }));
    
    return entry.data;
  }, [settings.enableCaching, cache]);

  const clearCache = useCallback((pattern?: string) => {
    setCacheState(prev => {
      if (!pattern) {
        setCacheStats(prev => ({ ...prev, size: 0 }));
        return new Map();
      }
      
      const newCache = new Map(prev);
      const regex = new RegExp(pattern);
      let deletedCount = 0;
      
      for (const [key] of newCache) {
        if (regex.test(key)) {
          newCache.delete(key);
          deletedCount++;
        }
      }
      
      setCacheStats(prev => ({ ...prev, size: prev.size - deletedCount }));
      return newCache;
    });
  }, []);

  // Performance monitoring
  const addMetric = useCallback((metric: Omit<PerformanceMetric, 'id' | 'timestamp'>) => {
    if (!settings.performanceMonitoring) return;

    const newMetric: PerformanceMetric = {
      ...metric,
      id: `${metric.name}-${Date.now()}`,
      timestamp: Date.now(),
    };

    setMetrics(prev => {
      const updated = [...prev, newMetric];
      // Keep only last 100 metrics
      return updated.slice(-100);
    });
  }, [settings.performanceMonitoring]);

  const toggleMonitoring = useCallback(() => {
    setIsMonitoring(prev => !prev);
  }, []);

  const updateSettings = useCallback((newSettings: Partial<PerformanceState['settings']>) => {
    setSettings(prev => ({ ...prev, ...newSettings }));
  }, []);

  const getCacheStats = useCallback(() => {
    return cacheStats;
  }, [cacheStats]);

  const getPerformanceReport = useCallback(() => {
    const recentMetrics = metrics.slice(-20);
    const responseTimes = recentMetrics
      .filter(m => m.category === 'network')
      .map(m => m.value);
    
    const averageResponseTime = responseTimes.length > 0 
      ? responseTimes.reduce((a, b) => a + b, 0) / responseTimes.length 
      : 0;

    const hitRate = cacheStats.hits + cacheStats.misses > 0
      ? (cacheStats.hits / (cacheStats.hits + cacheStats.misses)) * 100
      : 0;

    const memoryMetrics = metrics
      .filter(m => m.category === 'memory')
      .slice(-5);
    
    const memoryUsage = memoryMetrics.length > 0
      ? memoryMetrics[memoryMetrics.length - 1].value
      : 0;

    return {
      averageResponseTime,
      cacheHitRate: hitRate,
      memoryUsage,
      renderCount: renderCount.current,
    };
  }, [metrics, cacheStats]);

  // Performance monitoring effect
  useEffect(() => {
    if (!isMonitoring || !settings.performanceMonitoring) {
      if (monitoringInterval.current) {
        clearInterval(monitoringInterval.current);
        monitoringInterval.current = null;
      }
      return;
    }

    monitoringInterval.current = setInterval(() => {
      // Memory usage
      if ('memory' in performance) {
        const memory = (performance as any).memory;
        addMetric({
          name: 'Memory Usage',
          value: memory.usedJSHeapSize / 1024 / 1024, // MB
          unit: 'MB',
          category: 'memory',
          trend: memory.usedJSHeapSize > (memory.usedJSHeapSize || 0) ? 'up' : 'down',
        });
      }

      // Cache stats
      addMetric({
        name: 'Cache Hit Rate',
        value: getCacheStats().hits / (getCacheStats().hits + getCacheStats().misses) * 100,
        unit: '%',
        category: 'cache',
        trend: 'stable',
      });
    }, 5000);

    return () => {
      if (monitoringInterval.current) {
        clearInterval(monitoringInterval.current);
      }
    };
  }, [isMonitoring, settings.performanceMonitoring, addMetric, getCacheStats]);

  // Cleanup expired cache entries
  useEffect(() => {
    const cleanupInterval = setInterval(() => {
      const now = Date.now();
      let deletedCount = 0;

      setCacheState(prev => {
        const newCache = new Map(prev);
        for (const [key, entry] of newCache) {
          if (now - entry.timestamp > entry.ttl) {
            newCache.delete(key);
            deletedCount++;
          }
        }
        return newCache;
      });

      if (deletedCount > 0) {
        setCacheStats(prev => ({ ...prev, size: prev.size - deletedCount }));
      }
    }, 60000); // Cleanup every minute

    return () => clearInterval(cleanupInterval);
  }, []);

  const value: PerformanceState = {
    cache,
    cacheStats,
    metrics,
    isMonitoring,
    settings,
    setCache,
    getCache,
    clearCache,
    addMetric,
    toggleMonitoring,
    updateSettings,
    getCacheStats,
    getPerformanceReport,
  };

  return (
    <PerformanceContext.Provider value={value}>
      {children}
    </PerformanceContext.Provider>
  );
};

// Performance monitoring component
interface PerformanceMonitorProps {
  className?: string;
}

export const PerformanceMonitor: React.FC<PerformanceMonitorProps> = ({ className = '' }) => {
  const {
    metrics,
    isMonitoring,
    toggleMonitoring,
    getPerformanceReport,
    getCacheStats,
    clearCache,
  } = usePerformance();

  const [showDetails, setShowDetails] = useState(false);
  const report = getPerformanceReport();
  const cacheStats = getCacheStats();

  const recentMetrics = metrics.slice(-10);

  return (
    <div className={`bg-white rounded-lg shadow-sm border border-gray-200 p-4 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center space-x-2">
          <Activity className="w-5 h-5 text-gray-600" />
          <h3 className="text-lg font-medium text-gray-900">Performance Monitor</h3>
        </div>
        
        <div className="flex items-center space-x-2">
          <button
            onClick={toggleMonitoring}
            className={`px-3 py-1 rounded-lg text-sm font-medium ${
              isMonitoring
                ? 'bg-green-100 text-green-800'
                : 'bg-gray-100 text-gray-600'
            }`}
          >
            {isMonitoring ? 'Monitoring' : 'Start Monitor'}
          </button>
          
          <button
            onClick={() => setShowDetails(!showDetails)}
            className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
          >
            <Settings className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Quick Stats */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
        <div className="text-center">
          <div className="text-2xl font-bold text-gray-900">
            {report.averageResponseTime.toFixed(1)}
          </div>
          <div className="text-sm text-gray-500">Avg Response (ms)</div>
        </div>
        
        <div className="text-center">
          <div className="text-2xl font-bold text-gray-900">
            {report.cacheHitRate.toFixed(1)}%
          </div>
          <div className="text-sm text-gray-500">Cache Hit Rate</div>
        </div>
        
        <div className="text-center">
          <div className="text-2xl font-bold text-gray-900">
            {report.memoryUsage.toFixed(1)}
          </div>
          <div className="text-sm text-gray-500">Memory (MB)</div>
        </div>
        
        <div className="text-center">
          <div className="text-2xl font-bold text-gray-900">
            {cacheStats.size}
          </div>
          <div className="text-sm text-gray-500">Cache Items</div>
        </div>
      </div>

      {/* Detailed Metrics */}
      {showDetails && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h4 className="text-sm font-medium text-gray-900">Recent Metrics</h4>
            <button
              onClick={() => clearCache()}
              className="text-sm text-red-600 hover:text-red-800"
            >
              Clear Cache
            </button>
          </div>
          
          <div className="space-y-2 max-h-48 overflow-y-auto">
            {recentMetrics.map((metric) => (
              <div
                key={metric.id}
                className="flex items-center justify-between p-2 bg-gray-50 rounded-lg"
              >
                <div className="flex items-center space-x-2">
                  <div className={`w-2 h-2 rounded-full ${
                    metric.trend === 'up' ? 'bg-green-500' :
                    metric.trend === 'down' ? 'bg-red-500' : 'bg-gray-500'
                  }`} />
                  <span className="text-sm text-gray-900">{metric.name}</span>
                </div>
                <div className="text-sm text-gray-600">
                  {metric.value.toFixed(2)} {metric.unit}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

// Memoization hook
export const useMemoizedValue = <T,>(
  factory: () => T,
  dependencies: React.DependencyList,
  key?: string
): T => {
  const { settings } = usePerformance();
  
  if (!settings.enableMemoization) {
    return factory();
  }

  return useMemo(factory, dependencies);
};

// Lazy loading hook
export const useLazyLoad = <T,>(
  loader: () => Promise<T>,
  dependencies: React.DependencyList = []
) => {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const { settings } = usePerformance();

  const load = useCallback(async () => {
    if (!settings.enableLazyLoading) return;
    
    setLoading(true);
    setError(null);
    
    try {
      const result = await loader();
      setData(result);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [loader, settings.enableLazyLoading]);

  useEffect(() => {
    load();
  }, dependencies);

  return { data, loading, error, reload: load };
};

// Virtual scrolling hook
export const useVirtualScroll = <T,>(
  items: T[],
  itemHeight: number,
  containerHeight: number,
  overscan: number = 5
) => {
  const { settings } = usePerformance();
  
  if (!settings.enableVirtualScrolling) {
    return {
      virtualItems: items.map((item, index) => ({ item, index, offsetTop: index * itemHeight })),
      totalHeight: items.length * itemHeight,
    };
  }

  const [scrollTop, setScrollTop] = useState(0);
  
  const startIndex = Math.max(0, Math.floor(scrollTop / itemHeight) - overscan);
  const endIndex = Math.min(
    items.length - 1,
    Math.floor((scrollTop + containerHeight) / itemHeight) + overscan
  );
  
  const virtualItems = items
    .slice(startIndex, endIndex + 1)
    .map((item, index) => ({
      item,
      index: startIndex + index,
      offsetTop: (startIndex + index) * itemHeight,
    }));

  const totalHeight = items.length * itemHeight;

  return {
    virtualItems,
    totalHeight,
    scrollTop,
    setScrollTop,
  };
};

// Performance optimization wrapper
interface OptimizedComponentProps {
  children: React.ReactNode;
  cacheKey?: string;
  dependencies?: React.DependencyList;
  className?: string;
}

export const OptimizedComponent: React.FC<OptimizedComponentProps> = ({
  children,
  cacheKey,
  dependencies = [],
  className = '',
}) => {
  const { settings } = usePerformance();
  
  const memoizedChildren = useMemoizedValue(
    () => children,
    dependencies,
    cacheKey
  );

  if (!settings.enableMemoization) {
    return <div className={className}>{children}</div>;
  }

  return <div className={className}>{memoizedChildren}</div>;
}; 