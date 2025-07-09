import React, { createContext, useContext, useState, useCallback } from 'react';
import { LoadingSpinner } from './LoadingSpinner';

interface LoadingState {
  [key: string]: boolean;
}

interface LoadingContextType {
  loadingStates: LoadingState;
  setLoading: (key: string, loading: boolean) => void;
  isLoading: (key: string) => boolean;
  isAnyLoading: () => boolean;
  clearLoading: (key?: string) => void;
  withLoading: <T>(key: string, fn: () => Promise<T>) => Promise<T>;
}

const LoadingContext = createContext<LoadingContextType | undefined>(undefined);

export const useLoading = () => {
  const context = useContext(LoadingContext);
  if (!context) {
    throw new Error('useLoading must be used within a LoadingProvider');
  }
  return context;
};

interface LoadingProviderProps {
  children: React.ReactNode;
}

export const LoadingProvider: React.FC<LoadingProviderProps> = ({ children }) => {
  const [loadingStates, setLoadingStates] = useState<LoadingState>({});

  const setLoading = useCallback((key: string, loading: boolean) => {
    setLoadingStates(prev => ({
      ...prev,
      [key]: loading,
    }));
  }, []);

  const isLoading = useCallback((key: string) => {
    return loadingStates[key] || false;
  }, [loadingStates]);

  const isAnyLoading = useCallback(() => {
    return Object.values(loadingStates).some(Boolean);
  }, [loadingStates]);

  const clearLoading = useCallback((key?: string) => {
    if (key) {
      setLoadingStates(prev => {
        const newState = { ...prev };
        delete newState[key];
        return newState;
      });
    } else {
      setLoadingStates({});
    }
  }, []);

  const withLoading = useCallback(async <T,>(key: string, fn: () => Promise<T>): Promise<T> => {
    setLoading(key, true);
    try {
      const result = await fn();
      return result;
    } finally {
      setLoading(key, false);
    }
  }, [setLoading]);

  const value: LoadingContextType = {
    loadingStates,
    setLoading,
    isLoading,
    isAnyLoading,
    clearLoading,
    withLoading,
  };

  return (
    <LoadingContext.Provider value={value}>
      {children}
    </LoadingContext.Provider>
  );
};

// Loading overlay component
interface LoadingOverlayProps {
  isLoading: boolean;
  text?: string;
  children: React.ReactNode;
  className?: string;
}

export const LoadingOverlay: React.FC<LoadingOverlayProps> = ({
  isLoading,
  text = 'Loading...',
  children,
  className = '',
}) => {
  if (!isLoading) {
    return <>{children}</>;
  }

  return (
    <div className={`relative ${className}`}>
      {children}
      <div className="absolute inset-0 bg-white bg-opacity-75 flex items-center justify-center z-10">
        <LoadingSpinner size="lg" text={text} />
      </div>
    </div>
  );
};

// Skeleton loading components
interface SkeletonProps {
  className?: string;
  lines?: number;
  height?: string;
}

export const Skeleton: React.FC<SkeletonProps> = ({ 
  className = '', 
  lines = 1, 
  height = 'h-4' 
}) => {
  return (
    <div className={`animate-pulse ${className}`}>
      {Array.from({ length: lines }).map((_, index) => (
        <div
          key={index}
          className={`bg-gray-200 rounded ${height} ${index < lines - 1 ? 'mb-2' : ''}`}
        />
      ))}
    </div>
  );
};

export const SkeletonCard: React.FC<{ className?: string }> = ({ className = '' }) => {
  return (
    <div className={`bg-white rounded-lg shadow-sm border border-gray-200 p-6 ${className}`}>
      <div className="animate-pulse space-y-4">
        <div className="flex items-center space-x-3">
          <div className="h-8 w-8 bg-gray-200 rounded"></div>
          <div className="flex-1">
            <div className="h-4 bg-gray-200 rounded w-3/4"></div>
            <div className="h-3 bg-gray-200 rounded w-1/2 mt-2"></div>
          </div>
        </div>
        <div className="space-y-2">
          <div className="h-4 bg-gray-200 rounded"></div>
          <div className="h-4 bg-gray-200 rounded w-5/6"></div>
          <div className="h-4 bg-gray-200 rounded w-4/6"></div>
        </div>
      </div>
    </div>
  );
};

export const SkeletonTable: React.FC<{ rows?: number; columns?: number }> = ({ 
  rows = 5, 
  columns = 4 
}) => {
  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="h-6 bg-gray-200 rounded w-32"></div>
      </div>
      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              {Array.from({ length: columns }).map((_, index) => (
                <th key={index} className="px-6 py-3">
                  <div className="h-4 bg-gray-200 rounded w-20"></div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {Array.from({ length: rows }).map((_, rowIndex) => (
              <tr key={rowIndex}>
                {Array.from({ length: columns }).map((_, colIndex) => (
                  <td key={colIndex} className="px-6 py-4">
                    <div className="h-4 bg-gray-200 rounded w-16"></div>
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

// Progress indicator component
interface ProgressIndicatorProps {
  progress: number; // 0-100
  text?: string;
  className?: string;
  showPercentage?: boolean;
}

export const ProgressIndicator: React.FC<ProgressIndicatorProps> = ({
  progress,
  text,
  className = '',
  showPercentage = true,
}) => {
  const clampedProgress = Math.max(0, Math.min(100, progress));

  return (
    <div className={`space-y-2 ${className}`}>
      {text && (
        <div className="flex justify-between text-sm text-gray-600">
          <span>{text}</span>
          {showPercentage && <span>{Math.round(clampedProgress)}%</span>}
        </div>
      )}
      <div className="w-full bg-gray-200 rounded-full h-2">
        <div
          className="bg-primary-600 h-2 rounded-full transition-all duration-300 ease-out"
          style={{ width: `${clampedProgress}%` }}
        />
      </div>
    </div>
  );
};

// Loading button component
interface LoadingButtonProps {
  loading: boolean;
  onClick: () => void | Promise<void>;
  children: React.ReactNode;
  disabled?: boolean;
  className?: string;
  loadingText?: string;
}

export const LoadingButton: React.FC<LoadingButtonProps> = ({
  loading,
  onClick,
  children,
  disabled = false,
  className = '',
  loadingText = 'Loading...',
}) => {
  const handleClick = async () => {
    if (loading || disabled) return;
    await onClick();
  };

  return (
    <button
      onClick={handleClick}
      disabled={loading || disabled}
      className={`inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed ${className}`}
    >
      {loading ? (
        <>
          <LoadingSpinner size="sm" className="mr-2" />
          {loadingText}
        </>
      ) : (
        children
      )}
    </button>
  );
};

// Global loading indicator
export const GlobalLoadingIndicator: React.FC = () => {
  const { isAnyLoading } = useLoading();

  if (!isAnyLoading()) {
    return null;
  }

  return (
    <div className="fixed top-4 left-1/2 transform -translate-x-1/2 z-50">
      <div className="bg-white rounded-lg shadow-lg border border-gray-200 px-4 py-2">
        <div className="flex items-center space-x-2">
          <LoadingSpinner size="sm" />
          <span className="text-sm text-gray-600">Loading...</span>
        </div>
      </div>
    </div>
  );
};

// Loading states for common operations
export const LOADING_KEYS = {
  // Authentication
  LOGIN: 'auth:login',
  LOGOUT: 'auth:logout',
  
  // Data fetching
  FETCH_NETWORK_METRICS: 'data:network_metrics',
  FETCH_NODE_METRICS: 'data:node_metrics',
  FETCH_BLOCKS: 'data:blocks',
  FETCH_TRANSACTIONS: 'data:transactions',
  FETCH_VALIDATORS: 'data:validators',
  FETCH_ROUNDS: 'data:rounds',
  
  // Trading
  FETCH_TRADING_PAIRS: 'trading:pairs',
  FETCH_PRICE_HISTORY: 'trading:price_history',
  FETCH_ORDER_BOOK: 'trading:order_book',
  FETCH_RECENT_TRADES: 'trading:recent_trades',
  PLACE_ORDER: 'trading:place_order',
  CANCEL_ORDER: 'trading:cancel_order',
  
  // User data
  FETCH_USER_ORDERS: 'user:orders',
  FETCH_USER_BALANCE: 'user:balance',
  FETCH_AVAILABLE_ASSETS: 'user:assets',
  
  // WebSocket
  WEBSOCKET_CONNECT: 'websocket:connect',
  WEBSOCKET_RECONNECT: 'websocket:reconnect',
  
  // File operations
  UPLOAD_FILE: 'file:upload',
  DOWNLOAD_FILE: 'file:download',
  
  // System operations
  SYSTEM_UPDATE: 'system:update',
  SYSTEM_RESTART: 'system:restart',
} as const; 