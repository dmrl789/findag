import React, { useState, useRef, useEffect } from 'react';
import { 
  LineChart, 
  Line, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer,
  BarChart,
  Bar,
  ComposedChart,
  Area,
  AreaChart
} from 'recharts';
import { 
  Download, 
  Settings, 
  Eye, 
  EyeOff, 
  Palette,
  Layers,
  Save,
  Upload,
  RotateCcw
} from 'lucide-react';
import { AdvancedChart } from './AdvancedChart';
import { ChartAnnotations, useChartAnnotations, Annotation } from './ChartAnnotations';
import { PricePoint } from '../../types';

export interface EnhancedChartProps {
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
}

export const EnhancedChart: React.FC<EnhancedChartProps> = ({
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
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [showAnnotations, setShowAnnotations] = useState(true);
  const [isDrawingMode, setIsDrawingMode] = useState(false);
  const [showAnnotationPanel, setShowAnnotationPanel] = useState(false);
  const [chartDimensions, setChartDimensions] = useState({ width: 0, height: 0 });

  const {
    annotations,
    selectedAnnotation,
    setSelectedAnnotation,
    addAnnotation,
    updateAnnotation,
    deleteAnnotation,
    clearAllAnnotations,
    saveAnnotations,
    loadAnnotations,
  } = useChartAnnotations();

  // Update chart dimensions when container size changes
  useEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        const rect = containerRef.current.getBoundingClientRect();
        setChartDimensions({
          width: rect.width,
          height: rect.height - 60, // Account for toolbar
        });
      }
    };

    updateDimensions();
    window.addEventListener('resize', updateDimensions);
    return () => window.removeEventListener('resize', updateDimensions);
  }, []);

  const handleAnnotationAdd = (annotation: Annotation) => {
    addAnnotation(annotation);
    setIsDrawingMode(false);
  };

  const handleAnnotationUpdate = (id: string, updates: Partial<Annotation>) => {
    updateAnnotation(id, updates);
  };

  const handleAnnotationDelete = (id: string) => {
    deleteAnnotation(id);
  };

  const handleDrawingModeChange = (mode: boolean) => {
    setIsDrawingMode(mode);
    if (mode) {
      setSelectedAnnotation(null);
    }
  };

  const handleSaveChart = () => {
    // Save chart as image
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    if (!ctx || !containerRef.current) return;

    canvas.width = chartDimensions.width;
    canvas.height = chartDimensions.height;

    // Draw chart background
    ctx.fillStyle = '#ffffff';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Convert chart to image (simplified - in real implementation, you'd capture the actual chart)
    const chartElement = containerRef.current.querySelector('.recharts-wrapper');
    if (chartElement) {
      // This is a simplified approach - in practice you'd use html2canvas or similar
      ctx.fillStyle = '#f8fafc';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    // Download the image
    const link = document.createElement('a');
    link.download = `${pair}-chart-${Date.now()}.png`;
    link.href = canvas.toDataURL();
    link.click();
  };

  const handleLoadAnnotations = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      loadAnnotations(file);
    }
  };

  const exportChartData = () => {
    const csvContent = [
      'Timestamp,Open,High,Low,Close,Volume',
      ...data.map(point => 
        `${new Date(point.timestamp).toISOString()},${point.open},${point.high},${point.low},${point.close},${point.volume}`
      ).join('\n')
    ].join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `${pair}-data-${Date.now()}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className={`bg-white rounded-lg shadow-sm border border-gray-200 ${className}`}>
      {/* Header */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <h3 className="text-lg font-medium text-gray-900">{pair} Chart</h3>
            <div className="flex items-center space-x-2">
              <button
                onClick={() => setShowAnnotations(!showAnnotations)}
                className={`p-2 rounded-lg transition-colors ${
                  showAnnotations ? 'bg-primary-100 text-primary-600' : 'text-gray-600 hover:bg-gray-100'
                }`}
                title={showAnnotations ? 'Hide Annotations' : 'Show Annotations'}
              >
                {showAnnotations ? <Eye className="w-4 h-4" /> : <EyeOff className="w-4 h-4" />}
              </button>
              
              <button
                onClick={() => setShowAnnotationPanel(!showAnnotationPanel)}
                className={`p-2 rounded-lg transition-colors ${
                  showAnnotationPanel ? 'bg-primary-100 text-primary-600' : 'text-gray-600 hover:bg-gray-100'
                }`}
                title="Annotation Tools"
              >
                <Layers className="w-4 h-4" />
              </button>
            </div>
          </div>

          <div className="flex items-center space-x-2">
            <button
              onClick={exportChartData}
              className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
              title="Export Data"
            >
              <Download className="w-4 h-4" />
            </button>
            
            <button
              onClick={handleSaveChart}
              className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
              title="Save Chart"
            >
              <Save className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      {/* Chart Container */}
      <div ref={containerRef} className="relative">
        {/* Main Chart */}
        <div className="relative z-10">
          <AdvancedChart
            pair={pair}
            data={data}
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

        {/* Annotations Layer */}
        {showAnnotations && (
          <div className="absolute inset-0 z-20 pointer-events-none">
            <ChartAnnotations
              width={chartDimensions.width}
              height={chartDimensions.height}
              annotations={annotations}
              onAnnotationAdd={handleAnnotationAdd}
              onAnnotationUpdate={handleAnnotationUpdate}
              onAnnotationDelete={handleAnnotationDelete}
              onAnnotationSelect={setSelectedAnnotation}
              selectedAnnotation={selectedAnnotation}
              isDrawingMode={isDrawingMode}
              onDrawingModeChange={handleDrawingModeChange}
            />
          </div>
        )}
      </div>

      {/* Annotation Panel */}
      {showAnnotationPanel && (
        <div className="border-t border-gray-200 p-4 bg-gray-50">
          <div className="flex items-center justify-between mb-4">
            <h4 className="text-sm font-medium text-gray-900">Annotation Tools</h4>
            <div className="flex items-center space-x-2">
              <input
                type="file"
                accept=".json"
                onChange={handleLoadAnnotations}
                className="hidden"
                id="load-annotations"
              />
              <label
                htmlFor="load-annotations"
                className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg cursor-pointer"
                title="Load Annotations"
              >
                <Upload className="w-4 h-4" />
              </label>
              
              <button
                onClick={saveAnnotations}
                className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
                title="Save Annotations"
              >
                <Save className="w-4 h-4" />
              </button>
              
              <button
                onClick={clearAllAnnotations}
                className="p-2 text-red-600 hover:text-red-800 hover:bg-red-100 rounded-lg"
                title="Clear All Annotations"
              >
                <RotateCcw className="w-4 h-4" />
              </button>
            </div>
          </div>

          {/* Annotation List */}
          <div className="space-y-2 max-h-32 overflow-y-auto">
            {annotations.length === 0 ? (
              <p className="text-sm text-gray-500 text-center py-4">
                No annotations yet. Use the drawing tools to add annotations.
              </p>
            ) : (
              annotations.map((annotation) => (
                <div
                  key={annotation.id}
                  className={`flex items-center justify-between p-2 rounded-lg border ${
                    selectedAnnotation === annotation.id
                      ? 'border-primary-500 bg-primary-50'
                      : 'border-gray-200 bg-white'
                  }`}
                >
                  <div className="flex items-center space-x-2">
                    <div
                      className="w-3 h-3 rounded-full"
                      style={{ backgroundColor: annotation.color }}
                    />
                    <span className="text-sm text-gray-900 capitalize">
                      {annotation.type}
                    </span>
                    {annotation.text && (
                      <span className="text-sm text-gray-500">- {annotation.text}</span>
                    )}
                  </div>
                  
                  <div className="flex items-center space-x-1">
                    <button
                      onClick={() => setSelectedAnnotation(annotation.id)}
                      className="p-1 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded"
                      title="Select"
                    >
                      <Eye className="w-3 h-3" />
                    </button>
                    
                    <button
                      onClick={() => deleteAnnotation(annotation.id)}
                      className="p-1 text-red-600 hover:text-red-800 hover:bg-red-100 rounded"
                      title="Delete"
                    >
                      <RotateCcw className="w-3 h-3" />
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>

          {/* Annotation Statistics */}
          {annotations.length > 0 && (
            <div className="mt-4 pt-4 border-t border-gray-200">
              <div className="grid grid-cols-3 gap-4 text-center">
                <div>
                  <div className="text-lg font-semibold text-gray-900">{annotations.length}</div>
                  <div className="text-xs text-gray-500">Total</div>
                </div>
                <div>
                  <div className="text-lg font-semibold text-gray-900">
                    {annotations.filter(a => a.type === 'trendline').length}
                  </div>
                  <div className="text-xs text-gray-500">Trend Lines</div>
                </div>
                <div>
                  <div className="text-lg font-semibold text-gray-900">
                    {annotations.filter(a => a.type === 'fibonacci').length}
                  </div>
                  <div className="text-xs text-gray-500">Fibonacci</div>
                </div>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

// Hook for managing enhanced chart state
export const useEnhancedChart = () => {
  const [showAnnotations, setShowAnnotations] = useState(true);
  const [isDrawingMode, setIsDrawingMode] = useState(false);
  const [showAnnotationPanel, setShowAnnotationPanel] = useState(false);
  const [chartSettings, setChartSettings] = useState({
    showVolume: true,
    showMA: true,
    showBB: false,
    showRSI: false,
  });

  const {
    annotations,
    selectedAnnotation,
    setSelectedAnnotation,
    addAnnotation,
    updateAnnotation,
    deleteAnnotation,
    clearAllAnnotations,
    saveAnnotations,
    loadAnnotations,
  } = useChartAnnotations();

  const toggleAnnotationVisibility = () => {
    setShowAnnotations(!showAnnotations);
  };

  const toggleDrawingMode = () => {
    setIsDrawingMode(!isDrawingMode);
    if (!isDrawingMode) {
      setSelectedAnnotation(null);
    }
  };

  const toggleAnnotationPanel = () => {
    setShowAnnotationPanel(!showAnnotationPanel);
  };

  const updateChartSettings = (settings: Partial<typeof chartSettings>) => {
    setChartSettings(prev => ({ ...prev, ...settings }));
  };

  return {
    // State
    showAnnotations,
    isDrawingMode,
    showAnnotationPanel,
    chartSettings,
    annotations,
    selectedAnnotation,
    
    // Actions
    setShowAnnotations,
    setIsDrawingMode,
    setShowAnnotationPanel,
    setSelectedAnnotation,
    updateChartSettings,
    
    // Annotation actions
    addAnnotation,
    updateAnnotation,
    deleteAnnotation,
    clearAllAnnotations,
    saveAnnotations,
    loadAnnotations,
    
    // Convenience methods
    toggleAnnotationVisibility,
    toggleDrawingMode,
    toggleAnnotationPanel,
  };
}; 