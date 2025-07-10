import React, { useState, useEffect, useCallback } from 'react';
import { systemAPI, SystemInfo, SystemStats } from '../../services/api';
import { showNotification } from '../../components/Common/NotificationSystem';
import LoadingSpinner from '../../components/Common/LoadingSpinner';

interface NodeConfig {
  port: number;
  max_block_size: number;
  block_interval: number;
  round_interval: number;
  peers: string[];
  validator_address: string;
  validator_public_key: string;
}

interface AppSettings {
  theme: 'light' | 'dark' | 'auto';
  language: string;
  notifications: boolean;
  auto_refresh: boolean;
  refresh_interval: number;
  debug_mode: boolean;
}

const Settings: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'general' | 'node' | 'security' | 'advanced'>('general');
  const [nodeConfig, setNodeConfig] = useState<NodeConfig | null>(null);
  const [appSettings, setAppSettings] = useState<AppSettings>({
    theme: 'auto',
    language: 'en',
    notifications: true,
    auto_refresh: true,
    refresh_interval: 30,
    debug_mode: false,
  });
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [systemStats, setSystemStats] = useState<SystemStats | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchSystemData = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [config, info, stats] = await Promise.all([
        systemAPI.getSystemConfig(),
        systemAPI.getSystemInfo(),
        systemAPI.getSystemStats(),
      ]);
      setNodeConfig(config);
      setSystemInfo(info);
      setSystemStats(stats);
    } catch (err) {
      setError('Failed to fetch system data');
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchSystemData();
  }, [fetchSystemData]);

  const handleSaveNodeConfig = async () => {
    if (!nodeConfig) return;
    
    setIsSaving(true);
    try {
      await systemAPI.updateSystemConfig(nodeConfig);
      showNotification({
        type: 'success',
        title: 'Settings Saved',
        message: 'Node configuration has been updated',
      });
      fetchSystemData(); // Refresh data
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Save Failed',
        message: 'Failed to save node configuration',
      });
    } finally {
      setIsSaving(false);
    }
  };

  const handleSaveAppSettings = async () => {
    setIsSaving(true);
    try {
      await systemAPI.updateAppSettings(appSettings);
      showNotification({
        type: 'success',
        title: 'Settings Saved',
        message: 'Application settings have been updated',
      });
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Save Failed',
        message: 'Failed to save application settings',
      });
    } finally {
      setIsSaving(false);
    }
  };

  const handleRestartNode = async () => {
    if (!window.confirm('Are you sure you want to restart the node? This will temporarily disconnect from the network.')) {
      return;
    }

    setIsSaving(true);
    try {
      await systemAPI.restartNode();
      showNotification({
        type: 'success',
        title: 'Node Restarted',
        message: 'The node has been restarted successfully',
      });
      // Wait a moment then refresh data
      setTimeout(() => {
        fetchSystemData();
      }, 3000);
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Restart Failed',
        message: 'Failed to restart the node',
      });
    } finally {
      setIsSaving(false);
    }
  };

  const handleExportConfig = async () => {
    try {
      const configData = {
        node: nodeConfig,
        app: appSettings,
        system: systemInfo,
        timestamp: new Date().toISOString(),
      };
      
      const blob = new Blob([JSON.stringify(configData, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'findag-config.json';
      a.click();
      URL.revokeObjectURL(url);
      
      showNotification({
        type: 'success',
        title: 'Config Exported',
        message: 'Configuration has been exported',
      });
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Export Failed',
        message: 'Failed to export configuration',
      });
    }
  };

  const handleImportConfig = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (e) => {
      try {
        const configData = JSON.parse(e.target?.result as string);
        if (configData.node) {
          await systemAPI.updateSystemConfig(configData.node);
          setNodeConfig(configData.node);
        }
        if (configData.app) {
          await systemAPI.updateAppSettings(configData.app);
          setAppSettings(configData.app);
        }
        showNotification({
          type: 'success',
          title: 'Config Imported',
          message: 'Configuration has been imported',
        });
      } catch (error) {
        showNotification({
          type: 'error',
          title: 'Import Failed',
          message: 'Invalid configuration file',
        });
      }
    };
    reader.readAsText(file);
  };

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const formatUptime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  if (isLoading && !nodeConfig) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64 text-red-600">{error}</div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Settings</h1>
          <p className="text-gray-600 dark:text-gray-400">Configure your FinDAG application</p>
        </div>
        
        <div className="flex space-x-2">
          <button
            onClick={handleRestartNode}
            disabled={isSaving}
            className="px-4 py-2 bg-orange-600 text-white rounded hover:bg-orange-700 transition disabled:opacity-50"
          >
            {isSaving ? <LoadingSpinner size="sm" /> : '🔄'}
            <span className="ml-2">Restart Node</span>
          </button>
        </div>
      </div>

      {/* Settings Container */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
        <div className="border-b border-gray-200 dark:border-gray-700">
          <nav className="flex space-x-8 px-6">
            {(['general', 'node', 'security', 'advanced'] as const).map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={`py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === tab
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                }`}
              >
                {tab.charAt(0).toUpperCase() + tab.slice(1)}
              </button>
            ))}
          </nav>
        </div>

        <div className="p-6">
          {activeTab === 'general' && (
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">General Settings</h3>
              
              {/* Theme Settings */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Theme
                </label>
                <select
                  value={appSettings.theme}
                  onChange={(e) => setAppSettings(prev => ({ ...prev, theme: e.target.value as 'light' | 'dark' | 'auto' }))}
                  className="w-full max-w-xs p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                  <option value="auto">Auto (System)</option>
                </select>
              </div>

              {/* Language Settings */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Language
                </label>
                <select
                  value={appSettings.language}
                  onChange={(e) => setAppSettings(prev => ({ ...prev, language: e.target.value }))}
                  className="w-full max-w-xs p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="en">English</option>
                  <option value="es">Español</option>
                  <option value="fr">Français</option>
                  <option value="de">Deutsch</option>
                  <option value="zh">中文</option>
                </select>
              </div>

              {/* Notifications */}
              <div>
                <label className="flex items-center space-x-2">
                  <input
                    type="checkbox"
                    checked={appSettings.notifications}
                    onChange={(e) => setAppSettings(prev => ({ ...prev, notifications: e.target.checked }))}
                    className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                  />
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    Enable Notifications
                  </span>
                </label>
              </div>

              {/* Auto Refresh */}
              <div>
                <label className="flex items-center space-x-2">
                  <input
                    type="checkbox"
                    checked={appSettings.auto_refresh}
                    onChange={(e) => setAppSettings(prev => ({ ...prev, auto_refresh: e.target.checked }))}
                    className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                  />
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    Auto Refresh Data
                  </span>
                </label>
              </div>

              {/* Debug Mode */}
              <div>
                <label className="flex items-center space-x-2">
                  <input
                    type="checkbox"
                    checked={appSettings.debug_mode}
                    onChange={(e) => setAppSettings(prev => ({ ...prev, debug_mode: e.target.checked }))}
                    className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                  />
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    Debug Mode
                  </span>
                </label>
              </div>

              <button
                onClick={handleSaveAppSettings}
                disabled={isSaving}
                className="btn btn-primary"
              >
                {isSaving ? <LoadingSpinner size="sm" /> : 'Save Settings'}
              </button>
            </div>
          )}

          {activeTab === 'node' && nodeConfig && (
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Node Configuration</h3>
              
              {/* Port */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Node Port
                </label>
                <input
                  type="number"
                  value={nodeConfig.port}
                  onChange={(e) => setNodeConfig(prev => prev ? { ...prev, port: parseInt(e.target.value) || 8080 } : null)}
                  className="w-full max-w-xs p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  min="1024"
                  max="65535"
                />
              </div>

              {/* Max Block Size */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Max Block Size (bytes)
                </label>
                <input
                  type="number"
                  value={nodeConfig.max_block_size}
                  onChange={(e) => setNodeConfig(prev => prev ? { ...prev, max_block_size: parseInt(e.target.value) || 32768 } : null)}
                  className="w-full max-w-xs p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  min="1024"
                  max="131072"
                />
              </div>

              {/* Block Interval */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Block Interval (ms)
                </label>
                <input
                  type="number"
                  value={nodeConfig.block_interval}
                  onChange={(e) => setNodeConfig(prev => prev ? { ...prev, block_interval: parseInt(e.target.value) || 50 } : null)}
                  className="w-full max-w-xs p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  min="10"
                  max="1000"
                />
              </div>

              {/* Round Interval */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Round Interval (ms)
                </label>
                <input
                  type="number"
                  value={nodeConfig.round_interval}
                  onChange={(e) => setNodeConfig(prev => prev ? { ...prev, round_interval: parseInt(e.target.value) || 200 } : null)}
                  className="w-full max-w-xs p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  min="100"
                  max="5000"
                />
              </div>

              <button
                onClick={handleSaveNodeConfig}
                disabled={isSaving}
                className="btn btn-primary"
              >
                {isSaving ? <LoadingSpinner size="sm" /> : 'Save Configuration'}
              </button>
            </div>
          )}

          {activeTab === 'security' && (
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Security Settings</h3>
              
              {/* Validator Address */}
              {nodeConfig && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Validator Address
                  </label>
                  <input
                    type="text"
                    value={nodeConfig.validator_address}
                    onChange={(e) => setNodeConfig(prev => prev ? { ...prev, validator_address: e.target.value } : null)}
                    className="w-full max-w-md p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
                    placeholder="Enter validator address"
                  />
                </div>
              )}

              {/* Validator Public Key */}
              {nodeConfig && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Validator Public Key
                  </label>
                  <textarea
                    value={nodeConfig.validator_public_key}
                    onChange={(e) => setNodeConfig(prev => prev ? { ...prev, validator_public_key: e.target.value } : null)}
                    className="w-full max-w-md p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
                    rows={3}
                    placeholder="Enter validator public key"
                  />
                </div>
              )}

              <div className="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
                <div className="flex">
                  <div className="flex-shrink-0">
                    <span className="text-yellow-400">⚠️</span>
                  </div>
                  <div className="ml-3">
                    <h3 className="text-sm font-medium text-yellow-800 dark:text-yellow-200">
                      Security Notice
                    </h3>
                    <div className="mt-2 text-sm text-yellow-700 dark:text-yellow-300">
                      <p>Changes to security settings may require a node restart to take effect.</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'advanced' && systemInfo && systemStats && (
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">System Information</h3>
              
              {/* System Stats */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                  <h4 className="font-medium text-gray-900 dark:text-white mb-3">System Resources</h4>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">CPU Usage:</span>
                      <span className="font-mono">{systemStats.cpu_usage.toFixed(1)}%</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Memory Usage:</span>
                      <span className="font-mono">{formatBytes(systemStats.memory_usage)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Disk Usage:</span>
                      <span className="font-mono">{formatBytes(systemStats.disk_usage)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Uptime:</span>
                      <span className="font-mono">{formatUptime(systemStats.uptime)}</span>
                    </div>
                  </div>
                </div>

                <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                  <h4 className="font-medium text-gray-900 dark:text-white mb-3">Node Information</h4>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Version:</span>
                      <span className="font-mono">{systemInfo.version}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Platform:</span>
                      <span className="font-mono">{systemInfo.platform}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Architecture:</span>
                      <span className="font-mono">{systemInfo.architecture}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Node ID:</span>
                      <span className="font-mono">{systemInfo.node_id}</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Import/Export */}
              <div className="space-y-4">
                <h4 className="font-medium text-gray-900 dark:text-white">Configuration Management</h4>
                
                <div className="flex space-x-4">
                  <button
                    onClick={handleExportConfig}
                    className="btn btn-secondary"
                  >
                    Export Configuration
                  </button>
                  
                  <label className="btn btn-secondary cursor-pointer">
                    Import Configuration
                    <input
                      type="file"
                      accept=".json"
                      onChange={handleImportConfig}
                      className="hidden"
                    />
                  </label>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default Settings; 