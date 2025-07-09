import React, { useState, useRef, useEffect } from 'react';
import { 
  MousePointer, 
  Type, 
  Minus, 
  Square, 
  Circle, 
  ArrowUpRight,
  Trash2,
  Save,
  RotateCcw,
  Settings
} from 'lucide-react';

export interface Annotation {
  id: string;
  type: 'trendline' | 'fibonacci' | 'support' | 'resistance' | 'text' | 'rectangle' | 'circle' | 'arrow';
  points: { x: number; y: number }[];
  text?: string;
  color: string;
  strokeWidth: number;
  opacity: number;
  timestamp: number;
  metadata?: Record<string, any>;
}

export interface ChartAnnotationsProps {
  width: number;
  height: number;
  annotations: Annotation[];
  onAnnotationAdd: (annotation: Annotation) => void;
  onAnnotationUpdate: (id: string, annotation: Partial<Annotation>) => void;
  onAnnotationDelete: (id: string) => void;
  onAnnotationSelect?: (id: string | null) => void;
  selectedAnnotation?: string | null;
  isDrawingMode: boolean;
  onDrawingModeChange: (mode: boolean) => void;
  className?: string;
}

const DRAWING_TOOLS = [
  { id: 'select', icon: MousePointer, label: 'Select', type: null },
  { id: 'trendline', icon: Minus, label: 'Trend Line', type: 'trendline' },
  { id: 'fibonacci', icon: ArrowUpRight, label: 'Fibonacci', type: 'fibonacci' },
  { id: 'support', icon: Minus, label: 'Support', type: 'support' },
  { id: 'resistance', icon: Minus, label: 'Resistance', type: 'resistance' },
  { id: 'text', icon: Type, label: 'Text', type: 'text' },
  { id: 'rectangle', icon: Square, label: 'Rectangle', type: 'rectangle' },
  { id: 'circle', icon: Circle, label: 'Circle', type: 'circle' },
  { id: 'arrow', icon: ArrowUpRight, label: 'Arrow', type: 'arrow' },
] as const;

const COLORS = [
  '#3B82F6', '#EF4444', '#10B981', '#F59E0B', '#8B5CF6',
  '#EC4899', '#06B6D4', '#84CC16', '#F97316', '#6366F1'
];

export const ChartAnnotations: React.FC<ChartAnnotationsProps> = ({
  width,
  height,
  annotations,
  onAnnotationAdd,
  onAnnotationUpdate,
  onAnnotationDelete,
  onAnnotationSelect,
  selectedAnnotation,
  isDrawingMode,
  onDrawingModeChange,
  className = '',
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [currentTool, setCurrentTool] = useState<string>('select');
  const [isDrawing, setIsDrawing] = useState(false);
  const [drawingPoints, setDrawingPoints] = useState<{ x: number; y: number }[]>([]);
  const [selectedColor, setSelectedColor] = useState(COLORS[0]);
  const [strokeWidth, setStrokeWidth] = useState(2);
  const [opacity, setOpacity] = useState(1);
  const [showSettings, setShowSettings] = useState(false);

  // Drawing state
  const [tempAnnotation, setTempAnnotation] = useState<Partial<Annotation> | null>(null);

  useEffect(() => {
    drawAnnotations();
  }, [annotations, selectedAnnotation, width, height]);

  const drawAnnotations = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    // Draw all annotations
    annotations.forEach(annotation => {
      drawAnnotation(ctx, annotation, annotation.id === selectedAnnotation);
    });

    // Draw temporary annotation while drawing
    if (tempAnnotation && drawingPoints.length > 0) {
      drawAnnotation(ctx, { ...tempAnnotation, points: drawingPoints } as Annotation, false);
    }
  };

  const drawAnnotation = (ctx: CanvasRenderingContext2D, annotation: Annotation, isSelected: boolean) => {
    ctx.save();
    ctx.globalAlpha = annotation.opacity;
    ctx.strokeStyle = isSelected ? '#FF6B6B' : annotation.color;
    ctx.fillStyle = annotation.color;
    ctx.lineWidth = isSelected ? annotation.strokeWidth + 2 : annotation.strokeWidth;

    switch (annotation.type) {
      case 'trendline':
        drawTrendLine(ctx, annotation);
        break;
      case 'fibonacci':
        drawFibonacci(ctx, annotation);
        break;
      case 'support':
      case 'resistance':
        drawHorizontalLine(ctx, annotation);
        break;
      case 'text':
        drawText(ctx, annotation);
        break;
      case 'rectangle':
        drawRectangle(ctx, annotation);
        break;
      case 'circle':
        drawCircle(ctx, annotation);
        break;
      case 'arrow':
        drawArrow(ctx, annotation);
        break;
    }

    ctx.restore();
  };

  const drawTrendLine = (ctx: CanvasRenderingContext2D, annotation: Annotation) => {
    if (annotation.points.length < 2) return;
    
    const [start, end] = annotation.points;
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();

    // Draw handles
    drawHandle(ctx, start.x, start.y);
    drawHandle(ctx, end.x, end.y);
  };

  const drawFibonacci = (ctx: CanvasRenderingContext2D, annotation: Annotation) => {
    if (annotation.points.length < 2) return;
    
    const [start, end] = annotation.points;
    const levels = [0, 0.236, 0.382, 0.5, 0.618, 0.786, 1];
    const range = end.y - start.y;
    
    levels.forEach((level, index) => {
      const y = start.y + range * level;
      ctx.beginPath();
      ctx.moveTo(start.x, y);
      ctx.lineTo(end.x, y);
      ctx.stroke();
      
      // Draw level label
      ctx.fillStyle = annotation.color;
      ctx.font = '12px Arial';
      ctx.fillText(`${(level * 100).toFixed(1)}%`, end.x + 5, y - 5);
    });
  };

  const drawHorizontalLine = (ctx: CanvasRenderingContext2D, annotation: Annotation) => {
    if (annotation.points.length < 1) return;
    
    const y = annotation.points[0].y;
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(width, y);
    ctx.stroke();
    
    // Draw label
    ctx.fillStyle = annotation.color;
    ctx.font = '12px Arial';
    ctx.fillText(annotation.text || `${y.toFixed(2)}`, width - 60, y - 5);
  };

  const drawText = (ctx: CanvasRenderingContext2D, annotation: Annotation) => {
    if (annotation.points.length < 1 || !annotation.text) return;
    
    const point = annotation.points[0];
    ctx.fillStyle = annotation.color;
    ctx.font = '14px Arial';
    ctx.fillText(annotation.text, point.x, point.y);
  };

  const drawRectangle = (ctx: CanvasRenderingContext2D, annotation: Annotation) => {
    if (annotation.points.length < 2) return;
    
    const [start, end] = annotation.points;
    const rectWidth = end.x - start.x;
    const rectHeight = end.y - start.y;
    
    ctx.strokeRect(start.x, start.y, rectWidth, rectHeight);
  };

  const drawCircle = (ctx: CanvasRenderingContext2D, annotation: Annotation) => {
    if (annotation.points.length < 2) return;
    
    const [start, end] = annotation.points;
    const radius = Math.sqrt(Math.pow(end.x - start.x, 2) + Math.pow(end.y - start.y, 2));
    
    ctx.beginPath();
    ctx.arc(start.x, start.y, radius, 0, 2 * Math.PI);
    ctx.stroke();
  };

  const drawArrow = (ctx: CanvasRenderingContext2D, annotation: Annotation) => {
    if (annotation.points.length < 2) return;
    
    const [start, end] = annotation.points;
    const headLength = 15;
    const angle = Math.atan2(end.y - start.y, end.x - start.x);
    
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();
    
    // Draw arrow head
    ctx.beginPath();
    ctx.moveTo(end.x, end.y);
    ctx.lineTo(
      end.x - headLength * Math.cos(angle - Math.PI / 6),
      end.y - headLength * Math.sin(angle - Math.PI / 6)
    );
    ctx.moveTo(end.x, end.y);
    ctx.lineTo(
      end.x - headLength * Math.cos(angle + Math.PI / 6),
      end.y - headLength * Math.sin(angle + Math.PI / 6)
    );
    ctx.stroke();
  };

  const drawHandle = (ctx: CanvasRenderingContext2D, x: number, y: number) => {
    ctx.fillStyle = '#FFFFFF';
    ctx.strokeStyle = '#3B82F6';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.arc(x, y, 4, 0, 2 * Math.PI);
    ctx.fill();
    ctx.stroke();
  };

  const handleMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDrawingMode) return;

    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    setIsDrawing(true);
    setDrawingPoints([{ x, y }]);

    const tool = DRAWING_TOOLS.find(t => t.id === currentTool);
    if (tool?.type) {
      setTempAnnotation({
        id: `temp-${Date.now()}`,
        type: tool.type as Annotation['type'],
        color: selectedColor,
        strokeWidth,
        opacity,
        timestamp: Date.now(),
      });
    }
  };

  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDrawing || !isDrawingMode) return;

    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    setDrawingPoints(prev => [...prev.slice(0, 1), { x, y }]);
  };

  const handleMouseUp = () => {
    if (!isDrawing || !tempAnnotation) return;

    setIsDrawing(false);
    
    if (drawingPoints.length >= 1) {
      const newAnnotation: Annotation = {
        ...tempAnnotation,
        id: `annotation-${Date.now()}`,
        points: [...drawingPoints],
        text: tempAnnotation.type === 'text' ? prompt('Enter text:') || 'Text' : undefined,
      } as Annotation;

      onAnnotationAdd(newAnnotation);
    }

    setDrawingPoints([]);
    setTempAnnotation(null);
  };

  const handleToolChange = (toolId: string) => {
    setCurrentTool(toolId);
    onDrawingModeChange(toolId !== 'select');
  };

  const handleAnnotationClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (isDrawingMode) return;

    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    // Find clicked annotation
    const clickedAnnotation = annotations.find(annotation => {
      return annotation.points.some(point => {
        const distance = Math.sqrt(Math.pow(point.x - x, 2) + Math.pow(point.y - y, 2));
        return distance < 10;
      });
    });

    onAnnotationSelect?.(clickedAnnotation?.id || null);
  };

  const handleDeleteSelected = () => {
    if (selectedAnnotation) {
      onAnnotationDelete(selectedAnnotation);
      onAnnotationSelect?.(null);
    }
  };

  const handleClearAll = () => {
    if (confirm('Are you sure you want to delete all annotations?')) {
      annotations.forEach(annotation => {
        onAnnotationDelete(annotation.id);
      });
    }
  };

  return (
    <div className={`relative ${className}`}>
      {/* Toolbar */}
      <div className="absolute top-4 left-4 z-10 bg-white rounded-lg shadow-lg border border-gray-200 p-2">
        <div className="flex flex-col space-y-2">
          {DRAWING_TOOLS.map((tool) => {
            const Icon = tool.icon;
            return (
              <button
                key={tool.id}
                onClick={() => handleToolChange(tool.id)}
                className={`p-2 rounded-lg transition-colors ${
                  currentTool === tool.id
                    ? 'bg-primary-100 text-primary-600'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
                title={tool.label}
              >
                <Icon className="w-4 h-4" />
              </button>
            );
          })}
        </div>
      </div>

      {/* Settings Panel */}
      {showSettings && (
        <div className="absolute top-4 right-4 z-10 bg-white rounded-lg shadow-lg border border-gray-200 p-4 w-64">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-sm font-medium text-gray-900">Annotation Settings</h3>
            <button
              onClick={() => setShowSettings(false)}
              className="text-gray-400 hover:text-gray-600"
            >
              ✕
            </button>
          </div>

          <div className="space-y-4">
            {/* Color Picker */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Color</label>
              <div className="grid grid-cols-5 gap-2">
                {COLORS.map((color) => (
                  <button
                    key={color}
                    onClick={() => setSelectedColor(color)}
                    className={`w-8 h-8 rounded border-2 ${
                      selectedColor === color ? 'border-gray-900' : 'border-gray-300'
                    }`}
                    style={{ backgroundColor: color }}
                  />
                ))}
              </div>
            </div>

            {/* Stroke Width */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Stroke Width: {strokeWidth}
              </label>
              <input
                type="range"
                min="1"
                max="10"
                value={strokeWidth}
                onChange={(e) => setStrokeWidth(Number(e.target.value))}
                className="w-full"
              />
            </div>

            {/* Opacity */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Opacity: {Math.round(opacity * 100)}%
              </label>
              <input
                type="range"
                min="0.1"
                max="1"
                step="0.1"
                value={opacity}
                onChange={(e) => setOpacity(Number(e.target.value))}
                className="w-full"
              />
            </div>
          </div>
        </div>
      )}

      {/* Action Buttons */}
      <div className="absolute top-4 left-20 z-10 bg-white rounded-lg shadow-lg border border-gray-200 p-2">
        <div className="flex space-x-2">
          <button
            onClick={() => setShowSettings(!showSettings)}
            className={`p-2 rounded-lg transition-colors ${
              showSettings ? 'bg-primary-100 text-primary-600' : 'text-gray-600 hover:bg-gray-100'
            }`}
            title="Settings"
          >
            <Settings className="w-4 h-4" />
          </button>
          
          {selectedAnnotation && (
            <button
              onClick={handleDeleteSelected}
              className="p-2 text-red-600 hover:bg-red-100 rounded-lg transition-colors"
              title="Delete Selected"
            >
              <Trash2 className="w-4 h-4" />
            </button>
          )}
          
          <button
            onClick={handleClearAll}
            className="p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
            title="Clear All"
          >
            <RotateCcw className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Canvas */}
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        className="absolute inset-0 cursor-crosshair"
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onClick={handleAnnotationClick}
      />

      {/* Instructions */}
      {isDrawingMode && (
        <div className="absolute bottom-4 left-4 bg-black bg-opacity-75 text-white px-3 py-2 rounded-lg text-sm">
          {currentTool === 'trendline' && 'Click and drag to draw trend line'}
          {currentTool === 'fibonacci' && 'Click and drag to draw Fibonacci retracement'}
          {currentTool === 'support' && 'Click to place support line'}
          {currentTool === 'resistance' && 'Click to place resistance line'}
          {currentTool === 'text' && 'Click to place text annotation'}
          {currentTool === 'rectangle' && 'Click and drag to draw rectangle'}
          {currentTool === 'circle' && 'Click and drag to draw circle'}
          {currentTool === 'arrow' && 'Click and drag to draw arrow'}
        </div>
      )}
    </div>
  );
};

// Hook for managing annotations
export const useChartAnnotations = () => {
  const [annotations, setAnnotations] = useState<Annotation[]>([]);
  const [selectedAnnotation, setSelectedAnnotation] = useState<string | null>(null);

  const addAnnotation = (annotation: Annotation) => {
    setAnnotations(prev => [...prev, annotation]);
  };

  const updateAnnotation = (id: string, updates: Partial<Annotation>) => {
    setAnnotations(prev => 
      prev.map(ann => ann.id === id ? { ...ann, ...updates } : ann)
    );
  };

  const deleteAnnotation = (id: string) => {
    setAnnotations(prev => prev.filter(ann => ann.id !== id));
    if (selectedAnnotation === id) {
      setSelectedAnnotation(null);
    }
  };

  const clearAllAnnotations = () => {
    setAnnotations([]);
    setSelectedAnnotation(null);
  };

  const saveAnnotations = () => {
    const data = JSON.stringify(annotations);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `chart-annotations-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const loadAnnotations = (file: File) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        const data = JSON.parse(e.target?.result as string);
        setAnnotations(data);
      } catch (error) {
        console.error('Failed to load annotations:', error);
      }
    };
    reader.readAsText(file);
  };

  return {
    annotations,
    selectedAnnotation,
    setSelectedAnnotation,
    addAnnotation,
    updateAnnotation,
    deleteAnnotation,
    clearAllAnnotations,
    saveAnnotations,
    loadAnnotations,
  };
}; 