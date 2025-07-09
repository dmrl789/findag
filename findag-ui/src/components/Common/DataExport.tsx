import React, { useState, useCallback, useMemo } from 'react';
import { Download, Upload, FileText, Calendar, Settings, Trash2, Plus, Check } from 'lucide-react';

export interface ExportConfig {
  id: string;
  name: string;
  format: 'csv' | 'excel' | 'pdf' | 'json';
  dataType: string;
  columns: string[];
  filters: Record<string, any>;
  schedule?: {
    enabled: boolean;
    frequency: 'daily' | 'weekly' | 'monthly';
    time: string;
    dayOfWeek?: number;
    dayOfMonth?: number;
  };
  lastExport?: Date;
  nextExport?: Date;
}

export interface ImportConfig {
  id: string;
  name: string;
  format: 'csv' | 'excel' | 'json';
  dataType: string;
  mapping: Record<string, string>;
  validation: Record<string, any>;
}

interface DataExportProps {
  onExport: (config: ExportConfig) => Promise<void>;
  onImport: (config: ImportConfig, file: File) => Promise<void>;
  dataTypes: Array<{id: string, name: string, description: string}>;
  availableColumns: Record<string, Array<{id: string, name: string, type: string}>>;
  className?: string;
}

export const DataExport: React.FC<DataExportProps> = ({
  onExport,
  onImport,
  dataTypes,
  availableColumns,
  className = ""
}) => {
  const [activeTab, setActiveTab] = useState<'export' | 'import' | 'templates' | 'scheduled'>('export');
  const [exportConfigs, setExportConfigs] = useState<ExportConfig[]>([]);
  const [importConfigs, setImportConfigs] = useState<ImportConfig[]>([]);
  const [selectedDataType, setSelectedDataType] = useState('');
  const [selectedFormat, setSelectedFormat] = useState<'csv' | 'excel' | 'pdf' | 'json'>('csv');
  const [selectedColumns, setSelectedColumns] = useState<string[]>([]);
  const [exportName, setExportName] = useState('');
  const [isExporting, setIsExporting] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const [importFile, setImportFile] = useState<File | null>(null);
  const [showSchedule, setShowSchedule] = useState(false);
  const [scheduleConfig, setScheduleConfig] = useState({
    enabled: false,
    frequency: 'daily' as const,
    time: '09:00',
    dayOfWeek: 1,
    dayOfMonth: 1
  });

  // Load saved configurations from localStorage
  React.useEffect(() => {
    const savedExports = localStorage.getItem('findag-export-configs');
    if (savedExports) {
      setExportConfigs(JSON.parse(savedExports));
    }

    const savedImports = localStorage.getItem('findag-import-configs');
    if (savedImports) {
      setImportConfigs(JSON.parse(savedImports));
    }
  }, []);

  // Save configurations to localStorage
  const saveConfigs = useCallback(() => {
    localStorage.setItem('findag-export-configs', JSON.stringify(exportConfigs));
    localStorage.setItem('findag-import-configs', JSON.stringify(importConfigs));
  }, [exportConfigs, importConfigs]);

  React.useEffect(() => {
    saveConfigs();
  }, [exportConfigs, importConfigs, saveConfigs]);

  const handleExport = useCallback(async () => {
    if (!selectedDataType || selectedColumns.length === 0 || !exportName.trim()) {
      return;
    }

    setIsExporting(true);
    try {
      const config: ExportConfig = {
        id: Date.now().toString(),
        name: exportName,
        format: selectedFormat,
        dataType: selectedDataType,
        columns: selectedColumns,
        filters: {},
        schedule: showSchedule ? scheduleConfig : undefined
      };

      await onExport(config);
      
      // Save configuration
      setExportConfigs(prev => [...prev, config]);
      
      // Reset form
      setExportName('');
      setSelectedColumns([]);
      setShowSchedule(false);
      
    } catch (error) {
      console.error('Export error:', error);
    } finally {
      setIsExporting(false);
    }
  }, [selectedDataType, selectedFormat, selectedColumns, exportName, showSchedule, scheduleConfig, onExport]);

  const handleImport = useCallback(async () => {
    if (!importFile) return;

    setIsImporting(true);
    try {
      const config: ImportConfig = {
        id: Date.now().toString(),
        name: `Import ${importFile.name}`,
        format: importFile.name.endsWith('.csv') ? 'csv' : 
                importFile.name.endsWith('.xlsx') ? 'excel' : 'json',
        dataType: selectedDataType,
        mapping: {},
        validation: {}
      };

      await onImport(config, importFile);
      
      // Save configuration
      setImportConfigs(prev => [...prev, config]);
      
      // Reset form
      setImportFile(null);
      
    } catch (error) {
      console.error('Import error:', error);
    } finally {
      setIsImporting(false);
    }
  }, [importFile, selectedDataType, onImport]);

  const handleFileSelect = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      setImportFile(file);
    }
  }, []);

  const deleteExportConfig = useCallback((id: string) => {
    setExportConfigs(prev => prev.filter(config => config.id !== id));
  }, []);

  const deleteImportConfig = useCallback((id: string) => {
    setImportConfigs(prev => prev.filter(config => config.id !== id));
  }, []);

  const availableColumnsForType = useMemo(() => {
    return selectedDataType ? availableColumns[selectedDataType] || [] : [];
  }, [selectedDataType, availableColumns]);

  const scheduledExports = useMemo(() => {
    return exportConfigs.filter(config => config.schedule?.enabled);
  }, [exportConfigs]);

  const formatOptions = [
    { id: 'csv', name: 'CSV', icon: '📄' },
    { id: 'excel', name: 'Excel', icon: '📊' },
    { id: 'pdf', name: 'PDF', icon: '📋' },
    { id: 'json', name: 'JSON', icon: '🔧' }
  ];

  const frequencyOptions = [
    { id: 'daily', name: 'Daily' },
    { id: 'weekly', name: 'Weekly' },
    { id: 'monthly', name: 'Monthly' }
  ];

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg ${className}`}>
      {/* Tabs */}
      <div className="border-b border-gray-200 dark:border-gray-700">
        <nav className="flex space-x-8 px-6">
          {[
            { id: 'export', name: 'Export', icon: Download },
            { id: 'import', name: 'Import', icon: Upload },
            { id: 'templates', name: 'Templates', icon: FileText },
            { id: 'scheduled', name: 'Scheduled', icon: Calendar }
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2 ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              <tab.icon className="h-4 w-4" />
              <span>{tab.name}</span>
            </button>
          ))}
        </nav>
      </div>

      <div className="p-6">
        {/* Export Tab */}
        {activeTab === 'export' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Export Data</h3>
              
              {/* Data Type Selection */}
              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Data Type
                </label>
                <select
                  value={selectedDataType}
                  onChange={(e) => {
                    setSelectedDataType(e.target.value);
                    setSelectedColumns([]);
                  }}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="">Select data type...</option>
                  {dataTypes.map((type) => (
                    <option key={type.id} value={type.id}>
                      {type.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* Format Selection */}
              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Export Format
                </label>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                  {formatOptions.map((format) => (
                    <button
                      key={format.id}
                      onClick={() => setSelectedFormat(format.id as any)}
                      className={`p-3 border rounded-lg text-center ${
                        selectedFormat === format.id
                          ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                          : 'border-gray-300 dark:border-gray-600 hover:border-gray-400'
                      }`}
                    >
                      <div className="text-2xl mb-1">{format.icon}</div>
                      <div className="text-sm font-medium">{format.name}</div>
                    </button>
                  ))}
                </div>
              </div>

              {/* Column Selection */}
              {selectedDataType && (
                <div className="mb-4">
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Columns to Export
                  </label>
                  <div className="grid grid-cols-2 md:grid-cols-3 gap-2 max-h-40 overflow-y-auto">
                    {availableColumnsForType.map((column) => (
                      <label key={column.id} className="flex items-center space-x-2">
                        <input
                          type="checkbox"
                          checked={selectedColumns.includes(column.id)}
                          onChange={(e) => {
                            if (e.target.checked) {
                              setSelectedColumns(prev => [...prev, column.id]);
                            } else {
                              setSelectedColumns(prev => prev.filter(c => c !== column.id));
                            }
                          }}
                          className="rounded"
                        />
                        <span className="text-sm">{column.name}</span>
                      </label>
                    ))}
                  </div>
                </div>
              )}

              {/* Export Name */}
              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Export Name
                </label>
                <input
                  type="text"
                  value={exportName}
                  onChange={(e) => setExportName(e.target.value)}
                  placeholder="Enter export name..."
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                />
              </div>

              {/* Schedule Export */}
              <div className="mb-4">
                <label className="flex items-center space-x-2">
                  <input
                    type="checkbox"
                    checked={showSchedule}
                    onChange={(e) => setShowSchedule(e.target.checked)}
                    className="rounded"
                  />
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    Schedule recurring export
                  </span>
                </label>

                {showSchedule && (
                  <div className="mt-3 p-3 bg-gray-50 dark:bg-gray-700 rounded-lg space-y-3">
                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                          Frequency
                        </label>
                        <select
                          value={scheduleConfig.frequency}
                          onChange={(e) => setScheduleConfig(prev => ({
                            ...prev,
                            frequency: e.target.value as any
                          }))}
                          className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                        >
                          {frequencyOptions.map((freq) => (
                            <option key={freq.id} value={freq.id}>
                              {freq.name}
                            </option>
                          ))}
                        </select>
                      </div>
                      <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                          Time
                        </label>
                        <input
                          type="time"
                          value={scheduleConfig.time}
                          onChange={(e) => setScheduleConfig(prev => ({
                            ...prev,
                            time: e.target.value
                          }))}
                          className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                        />
                      </div>
                    </div>
                  </div>
                )}
              </div>

              {/* Export Button */}
              <button
                onClick={handleExport}
                disabled={!selectedDataType || selectedColumns.length === 0 || !exportName.trim() || isExporting}
                className="w-full bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
              >
                {isExporting ? (
                  <>
                    <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                    <span>Exporting...</span>
                  </>
                ) : (
                  <>
                    <Download className="h-4 w-4" />
                    <span>Export Data</span>
                  </>
                )}
              </button>
            </div>
          </div>
        )}

        {/* Import Tab */}
        {activeTab === 'import' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Import Data</h3>
              
              {/* File Upload */}
              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Select File
                </label>
                <div className="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-6 text-center">
                  <input
                    type="file"
                    accept=".csv,.xlsx,.json"
                    onChange={handleFileSelect}
                    className="hidden"
                    id="file-upload"
                  />
                  <label htmlFor="file-upload" className="cursor-pointer">
                    <Upload className="mx-auto h-12 w-12 text-gray-400" />
                    <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
                      Click to upload or drag and drop
                    </p>
                    <p className="text-xs text-gray-500 dark:text-gray-500">
                      CSV, Excel, or JSON files
                    </p>
                  </label>
                </div>
                {importFile && (
                  <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
                    Selected: {importFile.name}
                  </p>
                )}
              </div>

              {/* Data Type Selection */}
              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Import Type
                </label>
                <select
                  value={selectedDataType}
                  onChange={(e) => setSelectedDataType(e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="">Select import type...</option>
                  {dataTypes.map((type) => (
                    <option key={type.id} value={type.id}>
                      {type.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* Import Button */}
              <button
                onClick={handleImport}
                disabled={!importFile || !selectedDataType || isImporting}
                className="w-full bg-green-500 text-white py-2 px-4 rounded-md hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
              >
                {isImporting ? (
                  <>
                    <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                    <span>Importing...</span>
                  </>
                ) : (
                  <>
                    <Upload className="h-4 w-4" />
                    <span>Import Data</span>
                  </>
                )}
              </button>
            </div>
          </div>
        )}

        {/* Templates Tab */}
        {activeTab === 'templates' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Export Templates</h3>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Export Templates */}
                <div>
                  <h4 className="text-md font-medium text-gray-700 dark:text-gray-300 mb-3">Saved Exports</h4>
                  <div className="space-y-2">
                    {exportConfigs.map((config) => (
                      <div key={config.id} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                        <div>
                          <p className="font-medium text-sm">{config.name}</p>
                          <p className="text-xs text-gray-500 dark:text-gray-400">
                            {config.dataType} • {config.format.toUpperCase()}
                          </p>
                        </div>
                        <button
                          onClick={() => deleteExportConfig(config.id)}
                          className="text-red-500 hover:text-red-700"
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </div>
                    ))}
                    {exportConfigs.length === 0 && (
                      <p className="text-sm text-gray-500 dark:text-gray-400">No saved exports</p>
                    )}
                  </div>
                </div>

                {/* Import Templates */}
                <div>
                  <h4 className="text-md font-medium text-gray-700 dark:text-gray-300 mb-3">Saved Imports</h4>
                  <div className="space-y-2">
                    {importConfigs.map((config) => (
                      <div key={config.id} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                        <div>
                          <p className="font-medium text-sm">{config.name}</p>
                          <p className="text-xs text-gray-500 dark:text-gray-400">
                            {config.dataType} • {config.format.toUpperCase()}
                          </p>
                        </div>
                        <button
                          onClick={() => deleteImportConfig(config.id)}
                          className="text-red-500 hover:text-red-700"
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </div>
                    ))}
                    {importConfigs.length === 0 && (
                      <p className="text-sm text-gray-500 dark:text-gray-400">No saved imports</p>
                    )}
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Scheduled Tab */}
        {activeTab === 'scheduled' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Scheduled Exports</h3>
              
              <div className="space-y-3">
                {scheduledExports.map((config) => (
                  <div key={config.id} className="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="font-medium text-sm">{config.name}</p>
                        <p className="text-xs text-gray-500 dark:text-gray-400">
                          {config.dataType} • {config.format.toUpperCase()}
                        </p>
                        {config.schedule && (
                          <p className="text-xs text-gray-500 dark:text-gray-400">
                            {config.schedule.frequency} at {config.schedule.time}
                          </p>
                        )}
                      </div>
                      <div className="flex items-center space-x-2">
                        <span className="text-xs text-green-600 dark:text-green-400 flex items-center">
                          <Check className="h-3 w-3 mr-1" />
                          Active
                        </span>
                        <button
                          onClick={() => deleteExportConfig(config.id)}
                          className="text-red-500 hover:text-red-700"
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </div>
                    </div>
                  </div>
                ))}
                {scheduledExports.length === 0 && (
                  <p className="text-sm text-gray-500 dark:text-gray-400">No scheduled exports</p>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}; 