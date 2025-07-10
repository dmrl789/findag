import React, { useState, useEffect, useMemo, useCallback, useRef } from 'react';
import {
  LineChart,
  Line,
  AreaChart,
  Area,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  ComposedChart,
  Scatter,
  ScatterChart,
  ZAxis,
  ReferenceLine,
  ReferenceArea,
  Label,
  Sector,
  Cell as PieCell,
} from 'recharts';
import { format } from 'date-fns';
import { 
  BarChart3, 
  TrendingUp, 
  TrendingDown, 
  Activity,
  Layers,
  Minus,
  Settings,
  Download,
  Maximize2,
  Minimize2,
  PenTool,
  Ruler,
  Square,
  Circle,
  Type,
  Eraser,
  Undo2,
  Redo2,
  Save,
  Loader2
} from 'lucide-react';
import { PricePoint, TradingPair } from '../../types';
import { formatNumber, formatPrice } from '../../utils/formatters';

interface DrawingTool {
  id: string;
  type: 'trendline' | 'horizontal' | 'vertical' | 'fibonacci' | 'rectangle' | 'ellipse' | 'text' | 'arrow';
  points: { x: number; y: number }[];
  properties: {
    color: string;
    width: number;
    style: 'solid' | 'dashed' | 'dotted';
    label?: string;
  };
}

interface AdvancedChartProps {
  pair: string;
  data: PricePoint[];
  timeFrame: '1m' | '1h' | '3h' | '1D' | '1W' | '1M' | '1Y' | '5Y';
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



const chartTypeOptions = [
  { value: 'line', label: 'Line' },
  { value: 'candlestick', label: 'Candlestick' },
  { value: 'area', label: 'Area' },
  { value: 'volume', label: 'Volume' },
  { value: 'technical', label: 'Technical' },
];

const drawingTools = [
  { id: 'trendline', icon: TrendingUp, label: 'Trend Line', color: '#3B82F6' },
  { id: 'horizontal', icon: Minus, label: 'Horizontal Line', color: '#10B981' },
  { id: 'vertical', icon: Minus, label: 'Vertical Line', color: '#F59E0B' },
  { id: 'fibonacci', icon: Ruler, label: 'Fibonacci', color: '#8B5CF6' },
  { id: 'rectangle', icon: Square, label: 'Rectangle', color: '#EF4444' },
  { id: 'ellipse', icon: Circle, label: 'Ellipse', color: '#EC4899' },
  { id: 'text', icon: Type, label: 'Text', color: '#6B7280' },
  { id: 'arrow', icon: TrendingUp, label: 'Arrow', color: '#059669' },
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
  const [activeDrawingTool, setActiveDrawingTool] = useState<string | null>(null);
  const [drawings, setDrawings] = useState<DrawingTool[]>([]);
  const [drawingHistory, setDrawingHistory] = useState<DrawingTool[][]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);
  const [isDrawing, setIsDrawing] = useState(false);
  const [currentDrawing, setCurrentDrawing] = useState<DrawingTool | null>(null);
  const chartRef = useRef<HTMLDivElement>(null);

  // Calculate technical indicators
  const technicalData = useMemo(() => {
    if (data.length === 0) return { chartData: [], indicators: null };

    try {
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
    } catch (error) {
      console.error('Error processing chart data:', error);
      return { chartData: [], indicators: null };
    }
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

  // Drawing tools handlers
  const handleDrawingToolSelect = (toolId: string) => {
    setActiveDrawingTool(activeDrawingTool === toolId ? null : toolId);
    setIsDrawing(false);
    setCurrentDrawing(null);
  };

  const handleChartClick = (event: any) => {
    if (!activeDrawingTool || !chartRef.current) return;

    const rect = chartRef.current.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    if (!isDrawing) {
      // Start drawing
      const newDrawing: DrawingTool = {
        id: Date.now().toString(),
        type: activeDrawingTool as any,
        points: [{ x, y }],
        properties: {
          color: drawingTools.find(t => t.id === activeDrawingTool)?.color || '#3B82F6',
          width: 2,
          style: 'solid',
        }
      };
      setCurrentDrawing(newDrawing);
      setIsDrawing(true);
    } else {
      // Complete drawing
      if (currentDrawing) {
        const completedDrawing = {
          ...currentDrawing,
          points: [...currentDrawing.points, { x, y }]
        };
        
        addToHistory();
        setDrawings(prev => [...prev, completedDrawing]);
        setCurrentDrawing(null);
        setIsDrawing(false);
        setActiveDrawingTool(null);
      }
    }
  };

  const addToHistory = () => {
    setDrawingHistory(prev => {
      const newHistory = prev.slice(0, historyIndex + 1);
      return [...newHistory, [...drawings]];
    });
    setHistoryIndex(prev => prev + 1);
  };

  const undo = () => {
    if (historyIndex > 0) {
      setHistoryIndex(prev => prev - 1);
      setDrawings([...drawingHistory[historyIndex - 1]]);
    }
  };

  const redo = () => {
    if (historyIndex < drawingHistory.length - 1) {
      setHistoryIndex(prev => prev + 1);
      setDrawings([...drawingHistory[historyIndex + 1]]);
    }
  };

  const clearDrawings = () => {
    addToHistory();
    setDrawings([]);
  };

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

      {/* Drawing Tools Toolbar */}
      <div className="flex items-center justify-between p-3 bg-gray-50 border-b">
        <div className="flex items-center space-x-2">
          <span className="text-sm font-medium text-gray-700">Drawing Tools:</span>
          <div className="flex items-center space-x-1">
            {drawingTools.map((tool) => {
              const Icon = tool.icon;
              return (
                <button
                  key={tool.id}
                  onClick={() => handleDrawingToolSelect(tool.id)}
                  className={`p-2 rounded-md transition-colors ${
                    activeDrawingTool === tool.id
                      ? 'bg-blue-100 text-blue-700 border border-blue-300'
                      : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
                  }`}
                  title={tool.label}
                >
                  <Icon className="w-4 h-4" />
                </button>
              );
            })}
          </div>
        </div>
        
        <div className="flex items-center space-x-2">
          <button
            onClick={undo}
            disabled={historyIndex <= 0}
            className={`p-2 rounded-md transition-colors ${
              historyIndex <= 0
                ? 'text-gray-400 cursor-not-allowed'
                : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
            }`}
            title="Undo"
          >
            <Undo2 className="w-4 h-4" />
          </button>
          <button
            onClick={redo}
            disabled={historyIndex >= drawingHistory.length - 1}
            className={`p-2 rounded-md transition-colors ${
              historyIndex >= drawingHistory.length - 1
                ? 'text-gray-400 cursor-not-allowed'
                : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
            }`}
            title="Redo"
          >
            <Redo2 className="w-4 h-4" />
          </button>
          <button
            onClick={clearDrawings}
            className="p-2 text-red-600 hover:text-red-700 hover:bg-red-50 rounded-md"
            title="Clear All Drawings"
          >
            <Eraser className="w-4 h-4" />
          </button>
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
        <div className="h-96 relative" ref={chartRef} onClick={handleChartClick}>
          <ResponsiveContainer width="100%" height="100%">
            {renderChart()}
          </ResponsiveContainer>
          
          {/* Drawing overlay */}
          <svg className="absolute inset-0 w-full h-full pointer-events-none">
            {drawings.map((drawing) => (
              <g key={drawing.id}>
                {drawing.type === 'trendline' && drawing.points.length === 2 && (
                  <line
                    x1={drawing.points[0].x}
                    y1={drawing.points[0].y}
                    x2={drawing.points[1].x}
                    y2={drawing.points[1].y}
                    stroke={drawing.properties.color}
                    strokeWidth={drawing.properties.width}
                    strokeDasharray={drawing.properties.style === 'dashed' ? '5,5' : drawing.properties.style === 'dotted' ? '2,2' : 'none'}
                  />
                )}
                {drawing.type === 'horizontal' && drawing.points.length === 1 && (
                  <line
                    x1={0}
                    y1={drawing.points[0].y}
                    x2="100%"
                    y2={drawing.points[0].y}
                    stroke={drawing.properties.color}
                    strokeWidth={drawing.properties.width}
                    strokeDasharray={drawing.properties.style === 'dashed' ? '5,5' : drawing.properties.style === 'dotted' ? '2,2' : 'none'}
                  />
                )}
                {drawing.type === 'vertical' && drawing.points.length === 1 && (
                  <line
                    x1={drawing.points[0].x}
                    y1={0}
                    x2={drawing.points[0].x}
                    y2="100%"
                    stroke={drawing.properties.color}
                    strokeWidth={drawing.properties.width}
                    strokeDasharray={drawing.properties.style === 'dashed' ? '5,5' : drawing.properties.style === 'dotted' ? '2,2' : 'none'}
                  />
                )}
                {drawing.type === 'rectangle' && drawing.points.length === 2 && (
                  <rect
                    x={Math.min(drawing.points[0].x, drawing.points[1].x)}
                    y={Math.min(drawing.points[0].y, drawing.points[1].y)}
                    width={Math.abs(drawing.points[1].x - drawing.points[0].x)}
                    height={Math.abs(drawing.points[1].y - drawing.points[0].y)}
                    fill="none"
                    stroke={drawing.properties.color}
                    strokeWidth={drawing.properties.width}
                    strokeDasharray={drawing.properties.style === 'dashed' ? '5,5' : drawing.properties.style === 'dotted' ? '2,2' : 'none'}
                  />
                )}
                {drawing.type === 'ellipse' && drawing.points.length === 2 && (
                  <ellipse
                    cx={(drawing.points[0].x + drawing.points[1].x) / 2}
                    cy={(drawing.points[0].y + drawing.points[1].y) / 2}
                    rx={Math.abs(drawing.points[1].x - drawing.points[0].x) / 2}
                    ry={Math.abs(drawing.points[1].y - drawing.points[0].y) / 2}
                    fill="none"
                    stroke={drawing.properties.color}
                    strokeWidth={drawing.properties.width}
                    strokeDasharray={drawing.properties.style === 'dashed' ? '5,5' : drawing.properties.style === 'dotted' ? '2,2' : 'none'}
                  />
                )}
                {drawing.type === 'arrow' && drawing.points.length === 2 && (
                  <g>
                    <line
                      x1={drawing.points[0].x}
                      y1={drawing.points[0].y}
                      x2={drawing.points[1].x}
                      y2={drawing.points[1].y}
                      stroke={drawing.properties.color}
                      strokeWidth={drawing.properties.width}
                      markerEnd="url(#arrowhead)"
                    />
                    <defs>
                      <marker
                        id="arrowhead"
                        markerWidth="10"
                        markerHeight="7"
                        refX="9"
                        refY="3.5"
                        orient="auto"
                      >
                        <polygon
                          points="0 0, 10 3.5, 0 7"
                          fill={drawing.properties.color}
                        />
                      </marker>
                    </defs>
                  </g>
                )}
              </g>
            ))}
            
            {/* Current drawing preview */}
            {currentDrawing && currentDrawing.points.length === 1 && (
              <g>
                {currentDrawing.type === 'trendline' && (
                  <line
                    x1={currentDrawing.points[0].x}
                    y1={currentDrawing.points[0].y}
                    x2={currentDrawing.points[0].x}
                    y2={currentDrawing.points[0].y}
                    stroke={currentDrawing.properties.color}
                    strokeWidth={currentDrawing.properties.width}
                    strokeDasharray="5,5"
                  />
                )}
                {currentDrawing.type === 'horizontal' && (
                  <line
                    x1={0}
                    y1={currentDrawing.points[0].y}
                    x2="100%"
                    y2={currentDrawing.points[0].y}
                    stroke={currentDrawing.properties.color}
                    strokeWidth={currentDrawing.properties.width}
                    strokeDasharray="5,5"
                  />
                )}
                {currentDrawing.type === 'vertical' && (
                  <line
                    x1={currentDrawing.points[0].x}
                    y1={0}
                    x2={currentDrawing.points[0].x}
                    y2="100%"
                    stroke={currentDrawing.properties.color}
                    strokeWidth={currentDrawing.properties.width}
                    strokeDasharray="5,5"
                  />
                )}
              </g>
            )}
          </svg>
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
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="time" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} tickFormatter={formatPrice} />
            <Tooltip content={renderTooltip} />
            
            {/* OHLC representation as lines */}
            <Line type="monotone" dataKey="open" stroke="#6b7280" strokeWidth={1} strokeDasharray="2 2" />
            <Line type="monotone" dataKey="high" stroke="#10b981" strokeWidth={1} />
            <Line type="monotone" dataKey="low" stroke="#ef4444" strokeWidth={1} />
            <Line type="monotone" dataKey="close" stroke="#3b82f6" strokeWidth={2} />
            
            {/* Moving averages */}
            {showMA && (
              <>
                <Line type="monotone" dataKey="sma20" stroke="#f59e0b" strokeWidth={1} strokeDasharray="3 3" />
                <Line type="monotone" dataKey="sma50" stroke="#8b5cf6" strokeWidth={1} strokeDasharray="3 3" />
              </>
            )}
            
            {/* Volume */}
            {showVolume && (
              <Bar dataKey="volume" fill="#10b981" opacity={0.3} yAxisId={1} />
            )}
          </LineChart>
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