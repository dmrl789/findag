import React, { useState, useEffect, useMemo } from 'react';
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
} from 'recharts';
import { format } from 'date-fns';
import { TrendingUp, TrendingDown, Minus } from 'lucide-react';
import { PricePoint, TradingPair } from '../../types';
import { formatNumber, formatPrice } from '../../utils/formatters';

interface PriceChartProps {
  pair: string;
  data: PricePoint[];
  timeFrame: '1m' | '5m' | '15m' | '1h' | '4h' | '1d' | '1w';
  chartType: 'line' | 'candlestick' | 'volume';
  onTimeFrameChange: (timeFrame: string) => void;
  onChartTypeChange: (type: string) => void;
  loading?: boolean;
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
  { value: 'volume', label: 'Volume' },
];

export const PriceChart: React.FC<PriceChartProps> = ({
  pair,
  data,
  timeFrame,
  chartType,
  onTimeFrameChange,
  onChartTypeChange,
  loading = false,
}) => {
  const [currentPrice, setCurrentPrice] = useState<number | null>(null);
  const [priceChange, setPriceChange] = useState<number>(0);
  const [priceChangePercent, setPriceChangePercent] = useState<number>(0);

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

  // Update current price when data changes
  useEffect(() => {
    if (data.length > 0) {
      const latest = data[data.length - 1];
      setCurrentPrice(latest.close);
      setPriceChange(latest.close - data[0].close);
      setPriceChangePercent(
        data[0].close > 0 ? ((latest.close - data[0].close) / data[0].close) * 100 : 0
      );
    }
  }, [data]);

  // Format data for charts
  const chartData = useMemo(() => {
    return data.map((point) => ({
      ...point,
      time: format(new Date(point.timestamp), 'HH:mm'),
      date: format(new Date(point.timestamp), 'MMM dd'),
      formattedPrice: formatPrice(point.close),
      formattedVolume: formatNumber(point.volume),
    }));
  }, [data]);

  const getPriceChangeColor = () => {
    if (priceChange > 0) return 'text-success-600';
    if (priceChange < 0) return 'text-danger-600';
    return 'text-gray-600';
  };

  const getPriceChangeIcon = () => {
    if (priceChange > 0) return <TrendingUp className="w-4 h-4" />;
    if (priceChange < 0) return <TrendingDown className="w-4 h-4" />;
    return <Minus className="w-4 h-4" />;
  };

  if (loading) {
    return (
      <div className="card">
        <div className="animate-pulse">
          <div className="h-8 bg-gray-200 rounded w-32 mb-4"></div>
          <div className="h-64 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="card">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">{pair}</h3>
          <div className="flex items-center space-x-4 mt-1">
            <span className="text-2xl font-bold text-gray-900">
              {currentPrice ? formatPrice(currentPrice) : 'N/A'}
            </span>
            <div className={`flex items-center space-x-1 ${getPriceChangeColor()}`}>
              {getPriceChangeIcon()}
              <span className="font-medium">
                {priceChange >= 0 ? '+' : ''}{formatPrice(priceChange)}
              </span>
              <span className="text-sm">
                ({priceChangePercent >= 0 ? '+' : ''}{priceChangePercent.toFixed(2)}%)
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
        </div>
      </div>

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
          {chartType === 'line' ? (
            <LineChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis
                dataKey="time"
                stroke="#6b7280"
                fontSize={12}
                tickLine={false}
                axisLine={false}
              />
              <YAxis
                stroke="#6b7280"
                fontSize={12}
                tickLine={false}
                axisLine={false}
                tickFormatter={(value) => formatPrice(value)}
              />
              <Tooltip
                content={({ active, payload, label }) => {
                  if (active && payload && payload.length) {
                    const data = payload[0].payload;
                    return (
                      <div className="bg-white p-3 border border-gray-200 rounded-lg shadow-lg">
                        <p className="font-medium">{label}</p>
                        <p className="text-sm text-gray-600">
                          Price: {formatPrice(data.close)}
                        </p>
                        <p className="text-sm text-gray-600">
                          Volume: {formatNumber(data.volume)}
                        </p>
                      </div>
                    );
                  }
                  return null;
                }}
              />
              <Line
                type="monotone"
                dataKey="close"
                stroke="#3b82f6"
                strokeWidth={2}
                dot={false}
                activeDot={{ r: 4, fill: '#3b82f6' }}
              />
            </LineChart>
          ) : chartType === 'volume' ? (
            <BarChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis
                dataKey="time"
                stroke="#6b7280"
                fontSize={12}
                tickLine={false}
                axisLine={false}
              />
              <YAxis
                stroke="#6b7280"
                fontSize={12}
                tickLine={false}
                axisLine={false}
                tickFormatter={(value) => formatNumber(value)}
              />
              <Tooltip
                content={({ active, payload, label }) => {
                  if (active && payload && payload.length) {
                    const data = payload[0].payload;
                    return (
                      <div className="bg-white p-3 border border-gray-200 rounded-lg shadow-lg">
                        <p className="font-medium">{label}</p>
                        <p className="text-sm text-gray-600">
                          Volume: {formatNumber(data.volume)}
                        </p>
                        <p className="text-sm text-gray-600">
                          Price: {formatPrice(data.close)}
                        </p>
                      </div>
                    );
                  }
                  return null;
                }}
              />
              <Bar dataKey="volume" fill="#10b981" opacity={0.8} />
            </BarChart>
          ) : (
            <ComposedChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis
                dataKey="time"
                stroke="#6b7280"
                fontSize={12}
                tickLine={false}
                axisLine={false}
              />
              <YAxis
                stroke="#6b7280"
                fontSize={12}
                tickLine={false}
                axisLine={false}
                tickFormatter={(value) => formatPrice(value)}
              />
              <Tooltip
                content={({ active, payload, label }) => {
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
                      </div>
                    );
                  }
                  return null;
                }}
              />
              <Area
                dataKey="high"
                stroke="#10b981"
                fill="#10b981"
                fillOpacity={0.1}
                strokeWidth={1}
              />
              <Area
                dataKey="low"
                stroke="#ef4444"
                fill="#ef4444"
                fillOpacity={0.1}
                strokeWidth={1}
              />
              <Line
                type="monotone"
                dataKey="close"
                stroke="#3b82f6"
                strokeWidth={2}
                dot={false}
                activeDot={{ r: 4, fill: '#3b82f6' }}
              />
            </ComposedChart>
          )}
        </ResponsiveContainer>
      </div>
    </div>
  );
}; 