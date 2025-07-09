import React, { useState, useMemo, useCallback } from 'react';
import {
  ComposedChart,
  LineChart,
  Line,
  Area,
  AreaChart,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
  Brush,
  Legend,
} from 'recharts';
import { format } from 'date-fns';
import { 
  TrendingUp, 
  TrendingDown, 
  Minus, 
  Settings, 
  Download,
  Maximize2,
  Minimize2,
  Layers
} from 'lucide-react';
import { PricePoint, TradingPair } from '../../types';
import { formatNumber, formatPrice } from '../../utils/formatters';

interface AdvancedChartProps {
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
}

interface TechnicalIndicators {
  sma20: number[];
  sma50: number[];
  bbUpper: number[];
  bbLower: number[];
  bbMiddle: number[];
  rsi: number[];
  volume: number[];
}

const timeFrameOptions = [
  { value: '1m', label: '1m' },
  { value: '5m', label: '5m' },
  { value: '15m', label: '15m' },
  { value: '1h', label: '1h' },
  { value: '4h', label: '4h' },
  { value: '1d', label: '1d' },
  { value: '1w', label: '1w' },
];

const chartTypeOptions = [
  { value: 'line', label: 'Line' },
  { value: 'candlestick', label: 'Candlestick' },
  { value: 'area', label: 'Area' },
  { value: 'volume', label: 'Volume' },
  { value: 'technical', label: 'Technical' },
];

export const AdvancedChart: React.FC<AdvancedChartProps> = ({
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
}) => {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [selectedRange, setSelectedRange] = useState<[number, number] | null>(null);

  // Calculate technical indicators
  const technicalData = useMemo(() => {
    if (data.length === 0) return { chartData: [], indicators: null };

    const chartData = data.map((point) => ({
      ...point,
      time: format(new Date(point.timestamp), 'HH:mm'),
      date: format(new Date(point.timestamp), 'MMM dd'),
      formattedPrice: formatPrice(point.close),
      formattedVolume: formatNumber(point.volume),
    }));

    // Calculate Simple Moving Averages
    const sma20 = calculateSMA(data.map(d => d.close), 20);
    const sma50 = calculateSMA(data.map(d => d.close), 50);

    // Calculate Bollinger Bands
    const bb = calculateBollingerBands(data.map(d => d.close), 20, 2);

    // Calculate RSI
    const rsi = calculateRSI(data.map(d => d.close), 14);

    // Add indicators to chart data
    const enhancedData = chartData.map((point, index) => ({
      ...point,
      sma20: sma20[index] || null,
      sma50: sma50[index] || null,
      bbUpper: bb.upper[index] || null,
      bbLower: bb.lower[index] || null,
      bbMiddle: bb.middle[index] || null,
      rsi: rsi[index] || null,
    }));

    return {
      chartData: enhancedData,
      indicators: {
        sma20,
        sma50,
        bbUpper: bb.upper,
        bbLower: bb.lower,
        bbMiddle: bb.middle,
        rsi,
        volume: data.map(d => d.volume),
      }
    };
  }, [data]);

  // Calculate price statistics
  const stats = useMemo(() => {
    if (data.length === 0) return null;

    const latest = data[data.length - 1];
    const previous = data[0];
    
    const change = latest.close - previous.close;
    const changePercent = previous.close > 0 ? (change / previous.close) * 100 : 0;
    
    const high = Math.max(...data.map(d => d.high));
    const low = Math.min(...data.map(d => d.low));
    const volume = data.reduce((sum, d) => sum + d.volume, 0);

    return {
      current: latest.close,
      change,
      changePercent,
      high,
      low,
      volume,
      open: previous.open,
    };
  }, [data]);

  const handleRangeChange = useCallback((range: [number, number]) => {
    setSelectedRange(range);
  }, []);

  const handleExport = () => {
    const csvContent = generateCSV(data);
    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${pair}-${timeFrame}-${format(new Date(), 'yyyy-MM-dd')}.csv`;
    a.click();
    window.URL.revokeObjectURL(url);
  };

  const getPriceChangeColor = () => {
    if (!stats) return 'text-gray-600';
    if (stats.change > 0) return 'text-success-600';
    if (stats.change < 0) return 'text-danger-600';
    return 'text-gray-600';
  };

  const getPriceChangeIcon = () => {
    if (!stats) return <Minus className="w-4 h-4" />;
    if (stats.change > 0) return <TrendingUp className="w-4 h-4" />;
    if (stats.change < 0) return <TrendingDown className="w-4 h-4" />;
    return <Minus className="w-4 h-4" />;
  };

  if (loading) {
    return (
      <div className="card">
        <div className="animate-pulse">
          <div className="h-8 bg-gray-200 rounded w-32 mb-4"></div>
          <div className="h-96 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }

  return (
    <div className={`card ${isFullscreen ? 'fixed inset-0 z-50 m-4' : ''}`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">{pair}</h3>
          <div className="flex items-center space-x-4 mt-1">
            <span className="text-2xl font-bold text-gray-900">
              {stats ? formatPrice(stats.current) : 'N/A'}
            </span>
            <div className={`flex items-center space-x-1 ${getPriceChangeColor()}`}>
              {getPriceChangeIcon()}
              <span className="font-medium">
                {stats && stats.change >= 0 ? '+' : ''}{stats ? formatPrice(stats.change) : 'N/A'}
              </span>
              <span className="text-sm">
                ({stats && stats.changePercent >= 0 ? '+' : ''}{stats ? stats.changePercent.toFixed(2) : 'N/A'}%)
              </span>
            </div>
          </div>
        </div>

        {/* Controls */}
        <div className="flex items-center space-x-4">
          {/* Time frame selector */}
          <div className="flex bg-gray-100 rounded-lg p-1">
            {timeFrameOptions.map((option) => (
              <button
                key={option.value}
                onClick={() => onTimeFrameChange(option.value)}
                className={`px-3 py-1 text-sm font-medium rounded-md transition-colors ${
                  timeFrame === option.value
                    ? 'bg-white text-primary-600 shadow-sm'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                {option.label}
              </button>
            ))}
          </div>

          {/* Chart type selector */}
          <div className="flex bg-gray-100 rounded-lg p-1">
            {chartTypeOptions.map((option) => (
              <button
                key={option.value}
                onClick={() => onChartTypeChange(option.value)}
                className={`px-3 py-1 text-sm font-medium rounded-md transition-colors ${
                  chartType === option.value
                    ? 'bg-white text-primary-600 shadow-sm'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                {option.label}
              </button>
            ))}
          </div>

          {/* Action buttons */}
          <div className="flex items-center space-x-2">
            <button
              onClick={() => setShowSettings(!showSettings)}
              className="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg"
              title="Chart Settings"
            >
              <Settings className="w-4 h-4" />
            </button>
            <button
              onClick={handleExport}
              className="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg"
              title="Export Data"
            >
              <Download className="w-4 h-4" />
            </button>
            <button
              onClick={() => setIsFullscreen(!isFullscreen)}
              className="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg"
              title={isFullscreen ? 'Exit Fullscreen' : 'Fullscreen'}
            >
              {isFullscreen ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
            </button>
          </div>
        </div>
      </div>

      {/* Settings Panel */}
      {showSettings && (
        <div className="mb-6 p-4 bg-gray-50 rounded-lg">
          <h4 className="text-sm font-medium text-gray-900 mb-3">Chart Settings</h4>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <label className="flex items-center">
              <input
                type="checkbox"
                checked={showVolume}
                onChange={(e) => {/* Handle volume toggle */}}
                className="mr-2"
              />
              <span className="text-sm text-gray-700">Volume</span>
            </label>
            <label className="flex items-center">
              <input
                type="checkbox"
                checked={showMA}
                onChange={(e) => {/* Handle MA toggle */}}
                className="mr-2"
              />
              <span className="text-sm text-gray-700">Moving Averages</span>
            </label>
            <label className="flex items-center">
              <input
                type="checkbox"
                checked={showBB}
                onChange={(e) => {/* Handle BB toggle */}}
                className="mr-2"
              />
              <span className="text-sm text-gray-700">Bollinger Bands</span>
            </label>
            <label className="flex items-center">
              <input
                type="checkbox"
                checked={showRSI}
                onChange={(e) => {/* Handle RSI toggle */}}
                className="mr-2"
              />
              <span className="text-sm text-gray-700">RSI</span>
            </label>
          </div>
        </div>
      )}

      {/* Statistics */}
      {stats && (
        <div className="grid grid-cols-4 gap-4 mb-6">
          <div className="text-center">
            <div className="text-sm text-gray-500">Open</div>
            <div className="font-medium">{formatPrice(stats.open)}</div>
          </div>
          <div className="text-center">
            <div className="text-sm text-gray-500">High</div>
            <div className="font-medium">{formatPrice(stats.high)}</div>
          </div>
          <div className="text-center">
            <div className="text-sm text-gray-500">Low</div>
            <div className="font-medium">{formatPrice(stats.low)}</div>
          </div>
          <div className="text-center">
            <div className="text-sm text-gray-500">Volume</div>
            <div className="font-medium">{formatNumber(stats.volume)}</div>
          </div>
        </div>
      )}

      {/* Chart */}
      <div className="h-96">
        <ResponsiveContainer width="100%" height="100%">
          {renderChart()}
        </ResponsiveContainer>
      </div>

      {/* Technical Analysis Panel */}
      {chartType === 'technical' && technicalData.indicators && (
        <div className="mt-6 grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* RSI Chart */}
          <div className="h-48">
            <h4 className="text-sm font-medium text-gray-900 mb-2">RSI (14)</h4>
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={technicalData.chartData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
                <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
                <YAxis domain={[0, 100]} stroke="#6b7280" fontSize={12} />
                <Tooltip />
                <ReferenceLine y={70} stroke="#ef4444" strokeDasharray="3 3" />
                <ReferenceLine y={30} stroke="#10b981" strokeDasharray="3 3" />
                <Line type="monotone" dataKey="rsi" stroke="#8b5cf6" strokeWidth={2} />
              </LineChart>
            </ResponsiveContainer>
          </div>

          {/* Volume Chart */}
          <div className="h-48">
            <h4 className="text-sm font-medium text-gray-900 mb-2">Volume</h4>
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={technicalData.chartData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
                <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
                <YAxis stroke="#6b7280" fontSize={12} />
                <Tooltip />
                <Bar dataKey="volume" fill="#10b981" opacity={0.8} />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      )}
    </div>
  );

  function renderChart() {
    const { chartData } = technicalData;

    switch (chartType) {
      case 'line':
        return (
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} tickFormatter={formatPrice} />
            <Tooltip content={renderTooltip} />
            <Line type="monotone" dataKey="close" stroke="#3b82f6" strokeWidth={2} />
            {showMA && (
              <>
                <Line type="monotone" dataKey="sma20" stroke="#f59e0b" strokeWidth={1} strokeDasharray="3 3" />
                <Line type="monotone" dataKey="sma50" stroke="#8b5cf6" strokeWidth={1} strokeDasharray="3 3" />
              </>
            )}
            {showBB && (
              <>
                <Line type="monotone" dataKey="bbUpper" stroke="#ef4444" strokeWidth={1} strokeDasharray="2 2" />
                <Line type="monotone" dataKey="bbLower" stroke="#ef4444" strokeWidth={1} strokeDasharray="2 2" />
                <Line type="monotone" dataKey="bbMiddle" stroke="#6b7280" strokeWidth={1} />
              </>
            )}
            {showVolume && (
              <Bar dataKey="volume" fill="#10b981" opacity={0.3} yAxisId={1} />
            )}
          </LineChart>
        );

      case 'candlestick':
        return (
          <ComposedChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} tickFormatter={formatPrice} />
            <Tooltip content={renderTooltip} />
            <Area dataKey="high" stroke="#10b981" fill="#10b981" fillOpacity={0.1} strokeWidth={1} />
            <Area dataKey="low" stroke="#ef4444" fill="#ef4444" fillOpacity={0.1} strokeWidth={1} />
            <Line type="monotone" dataKey="close" stroke="#3b82f6" strokeWidth={2} />
            {showMA && (
              <>
                <Line type="monotone" dataKey="sma20" stroke="#f59e0b" strokeWidth={1} strokeDasharray="3 3" />
                <Line type="monotone" dataKey="sma50" stroke="#8b5cf6" strokeWidth={1} strokeDasharray="3 3" />
              </>
            )}
            {showVolume && (
              <Bar dataKey="volume" fill="#10b981" opacity={0.3} yAxisId={1} />
            )}
          </ComposedChart>
        );

      case 'area':
        return (
          <AreaChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} tickFormatter={formatPrice} />
            <Tooltip content={renderTooltip} />
            <Area type="monotone" dataKey="close" stroke="#3b82f6" fill="#3b82f6" fillOpacity={0.3} />
          </AreaChart>
        );

      case 'volume':
        return (
          <BarChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} tickFormatter={formatNumber} />
            <Tooltip content={renderTooltip} />
            <Bar dataKey="volume" fill="#10b981" opacity={0.8} />
          </BarChart>
        );

      default:
        return (
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} tickFormatter={formatPrice} />
            <Tooltip content={renderTooltip} />
            <Line type="monotone" dataKey="close" stroke="#3b82f6" strokeWidth={2} />
          </LineChart>
        );
    }
  }

  function renderTooltip({ active, payload, label }: any) {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div className="bg-white p-3 border border-gray-200 rounded-lg shadow-lg">
          <p className="font-medium">{label}</p>
          <p className="text-sm text-gray-600">
            Open: {formatPrice(data.open)}
          </p>
          <p className="text-sm text-gray-600">
            High: {formatPrice(data.high)}
          </p>
          <p className="text-sm text-gray-600">
            Low: {formatPrice(data.low)}
          </p>
          <p className="text-sm text-gray-600">
            Close: {formatPrice(data.close)}
          </p>
          <p className="text-sm text-gray-600">
            Volume: {formatNumber(data.volume)}
          </p>
          {data.sma20 && (
            <p className="text-sm text-gray-600">
              SMA20: {formatPrice(data.sma20)}
            </p>
          )}
          {data.rsi && (
            <p className="text-sm text-gray-600">
              RSI: {data.rsi.toFixed(2)}
            </p>
          )}
        </div>
      );
    }
    return null;
  }
}

// Technical analysis helper functions
function calculateSMA(data: number[], period: number): number[] {
  const sma: number[] = [];
  for (let i = 0; i < data.length; i++) {
    if (i < period - 1) {
      sma.push(NaN);
    } else {
      const sum = data.slice(i - period + 1, i + 1).reduce((a, b) => a + b, 0);
      sma.push(sum / period);
    }
  }
  return sma;
}

function calculateBollingerBands(data: number[], period: number, stdDev: number) {
  const sma = calculateSMA(data, period);
  const upper: number[] = [];
  const lower: number[] = [];
  const middle: number[] = [];

  for (let i = 0; i < data.length; i++) {
    if (i < period - 1) {
      upper.push(NaN);
      lower.push(NaN);
      middle.push(NaN);
    } else {
      const slice = data.slice(i - period + 1, i + 1);
      const mean = sma[i];
      const variance = slice.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / period;
      const standardDeviation = Math.sqrt(variance);
      
      middle.push(mean);
      upper.push(mean + (standardDeviation * stdDev));
      lower.push(mean - (standardDeviation * stdDev));
    }
  }

  return { upper, lower, middle };
}

function calculateRSI(data: number[], period: number): number[] {
  const rsi: number[] = [];
  
  for (let i = 0; i < data.length; i++) {
    if (i < period) {
      rsi.push(NaN);
    } else {
      let gains = 0;
      let losses = 0;
      
      for (let j = i - period + 1; j <= i; j++) {
        const change = data[j] - data[j - 1];
        if (change > 0) {
          gains += change;
        } else {
          losses -= change;
        }
      }
      
      const avgGain = gains / period;
      const avgLoss = losses / period;
      const rs = avgGain / avgLoss;
      const rsiValue = 100 - (100 / (1 + rs));
      
      rsi.push(rsiValue);
    }
  }
  
  return rsi;
}

function generateCSV(data: PricePoint[]): string {
  const headers = ['Timestamp', 'Open', 'High', 'Low', 'Close', 'Volume'];
  const rows = data.map(point => [
    new Date(point.timestamp).toISOString(),
    point.open,
    point.high,
    point.low,
    point.close,
    point.volume
  ]);
  
  return [headers, ...rows].map(row => row.join(',')).join('\n');
} 