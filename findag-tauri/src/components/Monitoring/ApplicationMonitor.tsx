import React, { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface PerformanceMetrics {
  cpu_usage: number;
  memory_usage: number;
  disk_usage: number;
  network_connections: number;
  active_processes: number;
  uptime: number;
}

interface ErrorReport {
  id: string;
  timestamp: number;
  level: 'error' | 'warning' | 'info';
  message: string;
  stack_trace?: string;
  user_agent: string;
  version: string;
  platform: string;
}

interface UsageAnalytics {
  session_duration: number;
  features_used: string[];
  transactions_count: number;
  trading_volume: number;
  network_peers: number;
  wallet_operations: number;
}

interface ApplicationMonitorProps {
  enabled?: boolean;
  onError?: (error: ErrorReport) => void;
  onPerformanceAlert?: (metrics: PerformanceMetrics) => void;
}

const ApplicationMonitor: React.FC<ApplicationMonitorProps> = ({
  enabled = true,
  onError,
  onPerformanceAlert,
}) => {
  const [metrics, setMetrics] = useState<PerformanceMetrics | null>(null);
  const [analytics, setAnalytics] = useState<UsageAnalytics | null>(null);
  const [errors, setErrors] = useState<ErrorReport[]>([]);
  const [isMonitoring, setIsMonitoring] = useState(false);

  // Performance monitoring interval
  const PERFORMANCE_INTERVAL = 5000; // 5 seconds
  const ANALYTICS_INTERVAL = 60000; // 1 minute

  // Error boundary for catching React errors
  const captureError = useCallback((error: Error, errorInfo?: any) => {
    const errorReport: ErrorReport = {
      id: `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      timestamp: Date.now(),
      level: 'error',
      message: error.message,
      stack_trace: error.stack,
      user_agent: navigator.userAgent,
      version: '1.0.0', // This should come from package.json
      platform: navigator.platform,
    };

    setErrors(prev => [...prev, errorReport]);
    onError?.(errorReport);

    // Send error to backend for logging
    invoke('log_error', { error: errorReport }).catch(console.error);
  }, [onError]);

  // Performance monitoring
  const monitorPerformance = useCallback(async () => {
    try {
      const systemStats = await invoke<PerformanceMetrics>('get_system_stats');
      setMetrics(systemStats);

      // Check for performance alerts
      if (systemStats.cpu_usage > 80 || systemStats.memory_usage > 85) {
        onPerformanceAlert?.(systemStats);
      }
    } catch (error) {
      console.error('Failed to get performance metrics:', error);
    }
  }, [onPerformanceAlert]);

  // Usage analytics collection
  const collectAnalytics = useCallback(async () => {
    try {
      const analyticsData = await invoke<UsageAnalytics>('get_usage_analytics');
      setAnalytics(analyticsData);
    } catch (error) {
      console.error('Failed to collect analytics:', error);
    }
  }, []);

  // Start monitoring
  const startMonitoring = useCallback(() => {
    if (!enabled || isMonitoring) return;

    setIsMonitoring(true);

    // Set up performance monitoring
    const performanceInterval = setInterval(monitorPerformance, PERFORMANCE_INTERVAL);

    // Set up analytics collection
    const analyticsInterval = setInterval(collectAnalytics, ANALYTICS_INTERVAL);

    // Initial collection
    monitorPerformance();
    collectAnalytics();

    // Cleanup function
    return () => {
      clearInterval(performanceInterval);
      clearInterval(analyticsInterval);
      setIsMonitoring(false);
    };
  }, [enabled, isMonitoring, monitorPerformance, collectAnalytics]);

  // Stop monitoring
  const stopMonitoring = useCallback(() => {
    setIsMonitoring(false);
  }, []);

  // Global error handler
  useEffect(() => {
    if (!enabled) return;

    const handleGlobalError = (event: ErrorEvent) => {
      const error = new Error(event.message);
      error.stack = event.error?.stack;
      captureError(error);
    };

    const handleUnhandledRejection = (event: PromiseRejectionEvent) => {
      const error = new Error(event.reason?.message || 'Unhandled promise rejection');
      error.stack = event.reason?.stack;
      captureError(error);
    };

    window.addEventListener('error', handleGlobalError);
    window.addEventListener('unhandledrejection', handleUnhandledRejection);

    return () => {
      window.removeEventListener('error', handleGlobalError);
      window.removeEventListener('unhandledrejection', handleUnhandledRejection);
    };
  }, [enabled, captureError]);

  // Start monitoring on mount
  useEffect(() => {
    const cleanup = startMonitoring();
    return cleanup;
  }, [startMonitoring]);

  // Export monitoring data
  const exportMonitoringData = useCallback(async () => {
    try {
      const data = {
        metrics,
        analytics,
        errors,
        timestamp: Date.now(),
      };

      await invoke('export_monitoring_data', { data });
      return data;
    } catch (error) {
      console.error('Failed to export monitoring data:', error);
      throw error;
    }
  }, [metrics, analytics, errors]);

  // Clear error logs
  const clearErrors = useCallback(() => {
    setErrors([]);
  }, []);

  // Get monitoring summary
  const getMonitoringSummary = useCallback(() => {
    return {
      isMonitoring,
      metricsCount: metrics ? 1 : 0,
      analyticsCount: analytics ? 1 : 0,
      errorCount: errors.length,
      lastUpdate: Date.now(),
    };
  }, [isMonitoring, metrics, analytics, errors]);

  // Expose methods to parent components
  const monitorRef = React.useRef<any>(null);
  React.useImperativeHandle(monitorRef, () => ({
    startMonitoring,
    stopMonitoring,
    exportMonitoringData,
    clearErrors,
    getMonitoringSummary,
    captureError,
  }));

  // Render monitoring status (optional UI component)
  if (!enabled) return null;

  return (
    <div className="application-monitor" style={{ display: 'none' }}>
      {/* This component is primarily for monitoring, not UI display */}
      {/* UI elements can be added here if needed for debugging */}
    </div>
  );
};

export default ApplicationMonitor;

// Hook for using application monitor
export const useApplicationMonitor = () => {
  const monitorRef = React.useRef<any>(null);

  const startMonitoring = useCallback(() => {
    monitorRef.current?.startMonitoring();
  }, []);

  const stopMonitoring = useCallback(() => {
    monitorRef.current?.stopMonitoring();
  }, []);

  const exportData = useCallback(async () => {
    return await monitorRef.current?.exportMonitoringData();
  }, []);

  const clearErrors = useCallback(() => {
    monitorRef.current?.clearErrors();
  }, []);

  const getSummary = useCallback(() => {
    return monitorRef.current?.getMonitoringSummary();
  }, []);

  return {
    monitorRef,
    startMonitoring,
    stopMonitoring,
    exportData,
    clearErrors,
    getSummary,
  };
}; 