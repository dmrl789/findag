import React, { useState, useEffect, useCallback } from 'react';
import { systemAPI, LogEntry } from '../../services/api';
import { showNotification } from '../../components/Common/NotificationSystem';
import LoadingSpinner from '../../components/Common/LoadingSpinner';

const Logs: React.FC = () => {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [filteredLogs, setFilteredLogs] = useState<LogEntry[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isAutoRefresh, setIsAutoRefresh] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedLevel, setSelectedLevel] = useState<'all' | 'debug' | 'info' | 'warn' | 'error'>('all');
  const [selectedComponent, setSelectedComponent] = useState('all');
  const [isPaused, setIsPaused] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [logLimit, setLogLimit] = useState(100);

  const fetchLogs = useCallback(async () => {
    if (isPaused) return;
    
    setIsLoading(true);
    setError(null);
    try {
      const level = selectedLevel === 'all' ? undefined : selectedLevel;
      const component = selectedComponent === 'all' ? undefined : selectedComponent;
      
      const logEntries = await systemAPI.getLogs(level, component, logLimit);
      setLogs(logEntries);
    } catch (err) {
      setError('Failed to fetch logs');
      showNotification({
        type: 'error',
        title: 'Fetch Failed',
        message: 'Failed to fetch log entries',
      });
    } finally {
      setIsLoading(false);
    }
  }, [selectedLevel, selectedComponent, logLimit, isPaused]);

  // Initial load
  useEffect(() => {
    fetchLogs();
  }, [fetchLogs]);

  // Auto-refresh logs
  useEffect(() => {
    if (!isAutoRefresh || isPaused) return;

    const interval = setInterval(() => {
      fetchLogs();
    }, 5000); // Refresh every 5 seconds

    return () => clearInterval(interval);
  }, [isAutoRefresh, isPaused, fetchLogs]);

  // Filter logs based on search term
  useEffect(() => {
    let filtered = logs;

    // Filter by search term
    if (searchTerm) {
      filtered = filtered.filter(log =>
        log.message.toLowerCase().includes(searchTerm.toLowerCase()) ||
        log.component.toLowerCase().includes(searchTerm.toLowerCase()) ||
        (log.details && JSON.stringify(log.details).toLowerCase().includes(searchTerm.toLowerCase()))
      );
    }

    setFilteredLogs(filtered);
  }, [logs, searchTerm]);

  const handleClearLogs = () => {
    setLogs([]);
    setFilteredLogs([]);
    showNotification({
      type: 'success',
      title: 'Logs Cleared',
      message: 'All logs have been cleared from view',
    });
  };

  const handleExportLogs = async () => {
    try {
      const logText = filteredLogs.map(log => {
        const timestamp = new Date(log.timestamp * 1000).toISOString();
        const details = log.details ? ` | ${JSON.stringify(log.details)}` : '';
        return `[${timestamp}] ${log.level.toUpperCase()} [${log.component}] ${log.message}${details}`;
      }).join('\n');

      const blob = new Blob([logText], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `findag-logs-${new Date().toISOString().split('T')[0]}.txt`;
      a.click();
      URL.revokeObjectURL(url);

      showNotification({
        type: 'success',
        title: 'Logs Exported',
        message: 'Logs have been exported successfully',
      });
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Export Failed',
        message: 'Failed to export logs',
      });
    }
  };

  const handleRefresh = () => {
    fetchLogs();
  };

  const getLevelColor = (level: LogEntry['level']) => {
    switch (level) {
      case 'error':
        return 'text-red-600 bg-red-100 dark:bg-red-900 dark:text-red-200';
      case 'warn':
        return 'text-yellow-600 bg-yellow-100 dark:bg-yellow-900 dark:text-yellow-200';
      case 'info':
        return 'text-blue-600 bg-blue-100 dark:bg-blue-900 dark:text-blue-200';
      case 'debug':
        return 'text-gray-600 bg-gray-100 dark:bg-gray-700 dark:text-gray-300';
      default:
        return 'text-gray-600 bg-gray-100 dark:bg-gray-700 dark:text-gray-300';
    }
  };

  const getComponentColor = (component: string) => {
    const colors = {
      node: 'text-purple-600',
      network: 'text-blue-600',
      consensus: 'text-green-600',
      trading: 'text-orange-600',
      wallet: 'text-indigo-600',
      api: 'text-pink-600',
      storage: 'text-teal-600',
      security: 'text-red-600',
      validator: 'text-yellow-600',
    };
    return colors[component as keyof typeof colors] || 'text-gray-600';
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const components = Array.from(new Set(logs.map(log => log.component))).sort();
  const levels = ['all', 'debug', 'info', 'warn', 'error'] as const;

  if (isLoading && logs.length === 0) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  if (error && logs.length === 0) {
    return (
      <div className="flex items-center justify-center h-64 text-red-600">{error}</div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">System Logs</h1>
          <p className="text-gray-600 dark:text-gray-400">Monitor and analyze system logs</p>
        </div>
        
        <div className="flex space-x-2">
          <button
            onClick={() => setIsPaused(!isPaused)}
            className={`px-4 py-2 rounded transition ${
              isPaused 
                ? 'bg-green-600 text-white hover:bg-green-700' 
                : 'bg-yellow-600 text-white hover:bg-yellow-700'
            }`}
          >
            {isPaused ? '▶️ Resume' : '⏸️ Pause'}
          </button>
          <button
            onClick={handleRefresh}
            disabled={isLoading}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition disabled:opacity-50"
          >
            {isLoading ? <LoadingSpinner size="sm" /> : '🔄'}
            <span className="ml-2">Refresh</span>
          </button>
          <button
            onClick={handleExportLogs}
            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition"
          >
            📥 Export
          </button>
          <button
            onClick={handleClearLogs}
            className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition"
          >
            🗑️ Clear
          </button>
        </div>
      </div>

      {/* Filters */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Search */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Search
            </label>
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder="Search logs..."
              className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
            />
          </div>

          {/* Level Filter */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Level
            </label>
            <select
              value={selectedLevel}
              onChange={(e) => setSelectedLevel(e.target.value as typeof selectedLevel)}
              className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
            >
              {levels.map(level => (
                <option key={level} value={level}>
                  {level.charAt(0).toUpperCase() + level.slice(1)}
                </option>
              ))}
            </select>
          </div>

          {/* Component Filter */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Component
            </label>
            <select
              value={selectedComponent}
              onChange={(e) => setSelectedComponent(e.target.value)}
              className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
            >
              <option value="all">All Components</option>
              {components.map(component => (
                <option key={component} value={component}>
                  {component.charAt(0).toUpperCase() + component.slice(1)}
                </option>
              ))}
            </select>
          </div>

          {/* Limit */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Limit
            </label>
            <select
              value={logLimit}
              onChange={(e) => setLogLimit(parseInt(e.target.value))}
              className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
            >
              <option value={50}>50 entries</option>
              <option value={100}>100 entries</option>
              <option value={200}>200 entries</option>
              <option value={500}>500 entries</option>
            </select>
          </div>
        </div>

        {/* Auto-refresh toggle */}
        <div className="mt-4">
          <label className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={isAutoRefresh}
              onChange={(e) => setIsAutoRefresh(e.target.checked)}
              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
              Auto-refresh logs (every 5 seconds)
            </span>
          </label>
        </div>
      </div>

      {/* Log Statistics */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500 dark:text-gray-400">Total Logs</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{logs.length}</p>
            </div>
            <div className="text-3xl">📊</div>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500 dark:text-gray-400">Filtered</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{filteredLogs.length}</p>
            </div>
            <div className="text-3xl">🔍</div>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500 dark:text-gray-400">Errors</p>
              <p className="text-2xl font-bold text-red-600">{logs.filter(log => log.level === 'error').length}</p>
            </div>
            <div className="text-3xl">❌</div>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500 dark:text-gray-400">Warnings</p>
              <p className="text-2xl font-bold text-yellow-600">{logs.filter(log => log.level === 'warn').length}</p>
            </div>
            <div className="text-3xl">⚠️</div>
          </div>
        </div>
      </div>

      {/* Log Entries */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
        <div className="p-4 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
            Log Entries ({filteredLogs.length})
          </h3>
        </div>
        
        <div className="max-h-96 overflow-y-auto">
          {filteredLogs.length > 0 ? (
            <div className="divide-y divide-gray-200 dark:divide-gray-700">
              {filteredLogs.map((log) => (
                <div key={log.id} className="p-4 hover:bg-gray-50 dark:hover:bg-gray-700">
                  <div className="flex items-start space-x-3">
                    <div className="flex-shrink-0">
                      <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${getLevelColor(log.level)}`}>
                        {log.level.toUpperCase()}
                      </span>
                    </div>
                    
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center space-x-2 mb-1">
                        <span className={`text-sm font-medium ${getComponentColor(log.component)}`}>
                          {log.component}
                        </span>
                        <span className="text-sm text-gray-500 dark:text-gray-400">
                          {formatTimestamp(log.timestamp)}
                        </span>
                      </div>
                      
                      <p className="text-sm text-gray-900 dark:text-white mb-2">
                        {log.message}
                      </p>
                      
                      {log.details && (
                        <details className="text-xs text-gray-600 dark:text-gray-400">
                          <summary className="cursor-pointer hover:text-gray-800 dark:hover:text-gray-200">
                            Show Details
                          </summary>
                          <pre className="mt-2 p-2 bg-gray-100 dark:bg-gray-600 rounded overflow-x-auto">
                            {JSON.stringify(log.details, null, 2)}
                          </pre>
                        </details>
                      )}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="p-8 text-center text-gray-500 dark:text-gray-400">
              {isLoading ? (
                <div className="flex items-center justify-center space-x-2">
                  <LoadingSpinner size="sm" />
                  <span>Loading logs...</span>
                </div>
              ) : (
                <div>
                  <div className="text-4xl mb-4">📝</div>
                  <p>No logs found</p>
                  <p className="text-sm">Try adjusting your filters or check if the system is generating logs</p>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default Logs; 