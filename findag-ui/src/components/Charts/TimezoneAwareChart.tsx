import React, { useState, useRef, useEffect } from 'react';
import { 
  Clock, 
  Globe, 
  Settings,
  Calendar,
  Sun,
  Moon
} from 'lucide-react';
import { AdvancedChart } from './AdvancedChart';
import { EnhancedChart } from './EnhancedChart';
import { useTimezone, TimezoneSelector, TimezoneDisplay, CurrentTimeDisplay } from '../Common/TimezoneProvider';
import { PricePoint } from '../../types';

export interface TimezoneAwareChartProps {
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
  enableTimezoneControl?: boolean;
}

export const TimezoneAwareChart: React.FC<TimezoneAwareChartProps> = ({
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
  enableTimezoneControl = true,
}) => {
  const {
    timezone,
    setTimezone,
    formatChartTime,
    formatChartTooltip,
    convertDataToTimezone,
    convertDataFromTimezone,
    getTimeRange,
    availableTimezones,
  } = useTimezone();

  const [showTimezoneSettings, setShowTimezoneSettings] = useState(false);
  const [showTimeInfo, setShowTimeInfo] = useState(false);
  const [convertedData, setConvertedData] = useState<PricePoint[]>(data);

  // Convert data to current timezone when data or timezone changes
  useEffect(() => {
    if (data.length > 0) {
      const converted = convertDataToTimezone(data, timezone);
      setConvertedData(converted);
    }
  }, [data, timezone, convertDataToTimezone]);

  // Custom tooltip formatter for timezone-aware display
  const customTooltipFormatter = (value: any, name: string, props: any) => {
    if (props.payload && props.payload.timestamp) {
      return formatChartTooltip(props.payload.timestamp, value);
    }
    return [value, name];
  };

  // Custom tick formatter for timezone-aware axis labels
  const customTickFormatter = (value: any) => {
    if (typeof value === 'number') {
      return formatChartTime(value, 'HH:mm');
    }
    return value;
  };

  // Get time range for current timezone
  const timeRange = getTimeRange(timeFrame, timezone);

  const handleTimezoneChange = (newTimezone: string) => {
    setTimezone(newTimezone);
  };

  const handleExportData = () => {
    // Export data in current timezone
    const csvContent = [
      'Timestamp,Open,High,Low,Close,Volume',
      ...convertedData.map(point => 
        `${formatChartTime(point.timestamp, 'yyyy-MM-dd HH:mm:ss')},${point.open},${point.high},${point.low},${point.close},${point.volume}`
      ).join('\n')
    ].join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `${pair}-data-${timezone}-${Date.now()}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className={`bg-white rounded-lg shadow-sm border border-gray-200 ${className}`}>
      {/* Header with timezone controls */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <h3 className="text-lg font-medium text-gray-900">{pair} Chart</h3>
            
            {enableTimezoneControl && (
              <div className="flex items-center space-x-2">
                <Globe className="w-4 h-4 text-gray-500" />
                <TimezoneDisplay className="text-sm text-gray-600" />
                <button
                  onClick={() => setShowTimezoneSettings(!showTimezoneSettings)}
                  className="p-1 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded"
                  title="Timezone Settings"
                >
                  <Settings className="w-4 h-4" />
                </button>
              </div>
            )}
          </div>

          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2 text-sm text-gray-600">
              <Clock className="w-4 h-4" />
              <CurrentTimeDisplay timezone={timezone} format="HH:mm:ss" />
            </div>
            
            <button
              onClick={() => setShowTimeInfo(!showTimeInfo)}
              className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
              title="Time Information"
            >
              <Calendar className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Timezone Settings Panel */}
        {showTimezoneSettings && enableTimezoneControl && (
          <div className="mt-4 p-4 bg-gray-50 rounded-lg">
            <div className="flex items-center justify-between mb-3">
              <h4 className="text-sm font-medium text-gray-900">Timezone Settings</h4>
              <button
                onClick={() => setShowTimezoneSettings(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Chart Timezone
                </label>
                <TimezoneSelector
                  value={timezone}
                  onChange={handleTimezoneChange}
                  showDescription={true}
                />
              </div>
              
              <div className="space-y-2">
                <div className="text-sm">
                  <span className="font-medium">Current Time:</span>{' '}
                  <CurrentTimeDisplay timezone={timezone} format="MMM dd, yyyy HH:mm:ss" />
                </div>
                <div className="text-sm">
                  <span className="font-medium">Time Range:</span>{' '}
                  {formatChartTime(timeRange.start.getTime(), 'MMM dd, HH:mm')} - {formatChartTime(timeRange.end.getTime(), 'MMM dd, HH:mm')}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Time Information Panel */}
        {showTimeInfo && (
          <div className="mt-4 p-4 bg-blue-50 rounded-lg">
            <div className="flex items-center justify-between mb-3">
              <h4 className="text-sm font-medium text-blue-900">Time Information</h4>
              <button
                onClick={() => setShowTimeInfo(false)}
                className="text-blue-400 hover:text-blue-600"
              >
                ✕
              </button>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
              <div>
                <div className="font-medium text-blue-900 mb-1">Data Points</div>
                <div className="text-blue-700">{convertedData.length} points</div>
              </div>
              
              <div>
                <div className="font-medium text-blue-900 mb-1">Time Range</div>
                <div className="text-blue-700">
                  {convertedData.length > 0 && (
                    <>
                      {formatChartTime(convertedData[0].timestamp, 'MMM dd, HH:mm')} - {formatChartTime(convertedData[convertedData.length - 1].timestamp, 'MMM dd, HH:mm')}
                    </>
                  )}
                </div>
              </div>
              
              <div>
                <div className="font-medium text-blue-900 mb-1">Timezone Offset</div>
                <div className="text-blue-700">
                  {availableTimezones.find(tz => tz.value === timezone)?.offset}
                </div>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Chart Container */}
      <div className="relative">
        <EnhancedChart
          pair={pair}
          data={convertedData}
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
      </div>

      {/* Footer with timezone info */}
      <div className="px-6 py-3 border-t border-gray-200 bg-gray-50">
        <div className="flex items-center justify-between text-sm text-gray-600">
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-1">
              <Globe className="w-4 h-4" />
              <span>Timezone: <TimezoneDisplay /></span>
            </div>
            
            <div className="flex items-center space-x-1">
              <Clock className="w-4 h-4" />
              <span>Last Update: <CurrentTimeDisplay format="HH:mm:ss" /></span>
            </div>
          </div>
          
          <div className="flex items-center space-x-2">
            <button
              onClick={handleExportData}
              className="text-primary-600 hover:text-primary-800 text-sm font-medium"
            >
              Export Data ({timezone})
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

// Hook for timezone-aware chart state
export const useTimezoneAwareChart = () => {
  const {
    timezone,
    setTimezone,
    formatChartTime,
    formatChartTooltip,
    convertDataToTimezone,
    convertDataFromTimezone,
    getTimeRange,
  } = useTimezone();

  const [showTimezoneSettings, setShowTimezoneSettings] = useState(false);
  const [showTimeInfo, setShowTimeInfo] = useState(false);

  const convertChartData = <T extends { timestamp: number }>(data: T[]) => {
    return convertDataToTimezone(data, timezone);
  };

  const formatTimeForChart = (timestamp: number, formatString?: string) => {
    return formatChartTime(timestamp, formatString);
  };

  const formatTooltipForChart = (timestamp: number, value: any) => {
    return formatChartTooltip(timestamp, value);
  };

  const getChartTimeRange = (timeFrame: string) => {
    return getTimeRange(timeFrame, timezone);
  };

  return {
    // State
    timezone,
    showTimezoneSettings,
    showTimeInfo,
    
    // Actions
    setTimezone,
    setShowTimezoneSettings,
    setShowTimeInfo,
    
    // Utilities
    convertChartData,
    formatTimeForChart,
    formatTooltipForChart,
    getChartTimeRange,
    
    // Convenience methods
    toggleTimezoneSettings: () => setShowTimezoneSettings(!showTimezoneSettings),
    toggleTimeInfo: () => setShowTimeInfo(!showTimeInfo),
  };
}; 