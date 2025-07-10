import React, { useRef, useEffect, useState } from 'react';

interface ChartData {
  timestamp: number;
  value: number;
  label?: string;
}

interface ChartConfig {
  type: 'line' | 'bar' | 'area' | 'candlestick';
  title: string;
  data: ChartData[];
  color: string;
  yAxisLabel: string;
  xAxisLabel: string;
}

interface Annotation {
  id: string;
  x: number;
  y: number;
  text: string;
  color: string;
  type: 'point' | 'line' | 'text';
}

interface AdvancedChartProps {
  config: ChartConfig;
  annotations: Annotation[];
  onAnnotationAdd: (annotation: Annotation) => void;
  onAnnotationRemove: (id: string) => void;
}

const AdvancedChart: React.FC<AdvancedChartProps> = ({
  config,
  annotations,
  onAnnotationAdd,
  onAnnotationRemove,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isDrawing, setIsDrawing] = useState(false);
  const [tooltip, setTooltip] = useState<{ x: number; y: number; text: string } | null>(null);

  useEffect(() => {
    drawChart();
  }, [config, annotations]);

  const drawChart = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const { width, height } = canvas.getBoundingClientRect();
    canvas.width = width;
    canvas.height = height;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    if (config.data.length === 0) {
      // Draw empty state
      ctx.fillStyle = '#9CA3AF';
      ctx.font = '16px Arial';
      ctx.textAlign = 'center';
      ctx.fillText('No data available', width / 2, height / 2);
      return;
    }

    // Calculate scales
    const padding = 60;
    const chartWidth = width - 2 * padding;
    const chartHeight = height - 2 * padding;

    const minValue = Math.min(...config.data.map(d => d.value));
    const maxValue = Math.max(...config.data.map(d => d.value));
    const valueRange = maxValue - minValue || 1;

    const minTime = Math.min(...config.data.map(d => d.timestamp));
    const maxTime = Math.max(...config.data.map(d => d.timestamp));
    const timeRange = maxTime - minTime || 1;

    // Draw grid
    ctx.strokeStyle = '#E5E7EB';
    ctx.lineWidth = 1;
    
    // Vertical grid lines
    for (let i = 0; i <= 5; i++) {
      const x = padding + (i / 5) * chartWidth;
      ctx.beginPath();
      ctx.moveTo(x, padding);
      ctx.lineTo(x, height - padding);
      ctx.stroke();
    }

    // Horizontal grid lines
    for (let i = 0; i <= 5; i++) {
      const y = padding + (i / 5) * chartHeight;
      ctx.beginPath();
      ctx.moveTo(padding, y);
      ctx.lineTo(width - padding, y);
      ctx.stroke();
    }

    // Draw axes
    ctx.strokeStyle = '#374151';
    ctx.lineWidth = 2;
    
    // Y-axis
    ctx.beginPath();
    ctx.moveTo(padding, padding);
    ctx.lineTo(padding, height - padding);
    ctx.stroke();

    // X-axis
    ctx.beginPath();
    ctx.moveTo(padding, height - padding);
    ctx.lineTo(width - padding, height - padding);
    ctx.stroke();

    // Draw axis labels
    ctx.fillStyle = '#6B7280';
    ctx.font = '12px Arial';
    ctx.textAlign = 'right';
    
    // Y-axis labels
    for (let i = 0; i <= 5; i++) {
      const value = minValue + (i / 5) * valueRange;
      const y = height - padding - (i / 5) * chartHeight;
      ctx.fillText(value.toFixed(1), padding - 10, y + 4);
    }

    // X-axis labels
    ctx.textAlign = 'center';
    for (let i = 0; i <= 5; i++) {
      const time = minTime + (i / 5) * timeRange;
      const x = padding + (i / 5) * chartWidth;
      const date = new Date(time);
      ctx.fillText(date.toLocaleTimeString(), x, height - padding + 20);
    }

    // Draw chart data
    ctx.strokeStyle = config.color;
    ctx.fillStyle = config.color;
    ctx.lineWidth = 2;

    if (config.type === 'line' || config.type === 'area') {
      ctx.beginPath();
      config.data.forEach((point, index) => {
        const x = padding + ((point.timestamp - minTime) / timeRange) * chartWidth;
        const y = height - padding - ((point.value - minValue) / valueRange) * chartHeight;
        
        if (index === 0) {
          ctx.moveTo(x, y);
        } else {
          ctx.lineTo(x, y);
        }
      });
      ctx.stroke();

      if (config.type === 'area') {
        ctx.lineTo(width - padding, height - padding);
        ctx.lineTo(padding, height - padding);
        ctx.closePath();
        ctx.fillStyle = config.color + '20';
        ctx.fill();
      }
    } else if (config.type === 'bar') {
      const barWidth = chartWidth / config.data.length * 0.8;
      config.data.forEach((point) => {
        const x = padding + ((point.timestamp - minTime) / timeRange) * chartWidth - barWidth / 2;
        const y = height - padding - ((point.value - minValue) / valueRange) * chartHeight;
        const barHeight = height - padding - y;
        
        ctx.fillRect(x, y, barWidth, barHeight);
      });
    }

    // Draw data points
    ctx.fillStyle = config.color;
    config.data.forEach((point) => {
      const x = padding + ((point.timestamp - minTime) / timeRange) * chartWidth;
      const y = height - padding - ((point.value - minValue) / valueRange) * chartHeight;
      
      ctx.beginPath();
      ctx.arc(x, y, 3, 0, 2 * Math.PI);
      ctx.fill();
    });

    // Draw annotations
    annotations.forEach((annotation) => {
      const x = padding + ((annotation.x - minTime) / timeRange) * chartWidth;
      const y = height - padding - ((annotation.y - minValue) / valueRange) * chartHeight;
      
      ctx.fillStyle = annotation.color;
      ctx.strokeStyle = annotation.color;
      
      if (annotation.type === 'point') {
        ctx.beginPath();
        ctx.arc(x, y, 6, 0, 2 * Math.PI);
        ctx.fill();
        ctx.lineWidth = 2;
        ctx.stroke();
      } else if (annotation.type === 'line') {
        ctx.beginPath();
        ctx.moveTo(x, padding);
        ctx.lineTo(x, height - padding);
        ctx.stroke();
      }
      
      // Draw annotation text
      ctx.fillStyle = annotation.color;
      ctx.font = '12px Arial';
      ctx.textAlign = 'center';
      ctx.fillText(annotation.text, x, y - 10);
    });
  };

  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const padding = 60;
    const chartWidth = canvas.width - 2 * padding;
    const chartHeight = canvas.height - 2 * padding;

    const minTime = Math.min(...config.data.map(d => d.timestamp));
    const maxTime = Math.max(...config.data.map(d => d.timestamp));
    const timeRange = maxTime - minTime || 1;

    const minValue = Math.min(...config.data.map(d => d.value));
    const maxValue = Math.max(...config.data.map(d => d.value));
    const valueRange = maxValue - minValue || 1;

    // Find closest data point
    let closestPoint: ChartData | null = null;
    let minDistance = Infinity;

    config.data.forEach((point) => {
      const pointX = padding + ((point.timestamp - minTime) / timeRange) * chartWidth;
      const pointY = canvas.height - padding - ((point.value - minValue) / valueRange) * chartHeight;
      
      const distance = Math.sqrt((x - pointX) ** 2 + (y - pointY) ** 2);
      if (distance < minDistance && distance < 20) {
        minDistance = distance;
        closestPoint = point;
      }
    });

    if (closestPoint) {
      setTooltip({
        x: e.clientX,
        y: e.clientY,
        text: `${closestPoint.label || new Date(closestPoint.timestamp).toLocaleTimeString()}: ${closestPoint.value.toFixed(2)}`,
      });
    } else {
      setTooltip(null);
    }
  };

  const handleMouseLeave = () => {
    setTooltip(null);
  };

  const handleDoubleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const padding = 60;
    const chartWidth = canvas.width - 2 * padding;
    const chartHeight = canvas.height - 2 * padding;

    const minTime = Math.min(...config.data.map(d => d.timestamp));
    const maxTime = Math.max(...config.data.map(d => d.timestamp));
    const timeRange = maxTime - minTime || 1;

    const minValue = Math.min(...config.data.map(d => d.value));
    const maxValue = Math.max(...config.data.map(d => d.value));
    const valueRange = maxValue - minValue || 1;

    const timestamp = minTime + ((x - padding) / chartWidth) * timeRange;
    const value = minValue + ((canvas.height - padding - y) / chartHeight) * valueRange;

    const annotation: Annotation = {
      id: Date.now().toString(),
      x: timestamp,
      y: value,
      text: `Annotation ${Date.now()}`,
      color: '#EF4444',
      type: 'point',
    };

    onAnnotationAdd(annotation);
  };

  return (
    <div className="relative">
      <canvas
        ref={canvasRef}
        className="w-full h-full cursor-crosshair"
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        onDoubleClick={handleDoubleClick}
      />
      
      {tooltip && (
        <div
          className="absolute bg-gray-900 text-white px-2 py-1 rounded text-sm pointer-events-none z-10"
          style={{
            left: tooltip.x + 10,
            top: tooltip.y - 10,
          }}
        >
          {tooltip.text}
        </div>
      )}
    </div>
  );
};

export default AdvancedChart; 