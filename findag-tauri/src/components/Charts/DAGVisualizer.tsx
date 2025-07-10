import React, { useRef, useEffect, useState } from 'react';

interface DAGNode {
  id: string;
  hash: string;
  timestamp: number;
  parentHashes: string[];
  childrenHashes: string[];
  transactionCount: number;
  roundNumber: number;
  validator: string;
  x?: number;
  y?: number;
}

interface DAGVisualizerProps {
  nodes: DAGNode[];
  selectedNode?: string;
  onNodeSelect?: (nodeId: string) => void;
  onNodeHover?: (nodeId: string | null) => void;
}

const DAGVisualizer: React.FC<DAGVisualizerProps> = ({
  nodes,
  selectedNode,
  onNodeSelect,
  onNodeHover,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [zoom, setZoom] = useState(1);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const [hoveredNode, setHoveredNode] = useState<string | null>(null);

  useEffect(() => {
    drawDAG();
  }, [nodes, selectedNode, zoom, pan, hoveredNode]);

  const drawDAG = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const { width, height } = canvas.getBoundingClientRect();
    canvas.width = width;
    canvas.height = height;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    if (nodes.length === 0) {
      ctx.fillStyle = '#9CA3AF';
      ctx.font = '16px Arial';
      ctx.textAlign = 'center';
      ctx.fillText('No DAG data available', width / 2, height / 2);
      return;
    }

    // Apply zoom and pan
    ctx.save();
    ctx.translate(pan.x, pan.y);
    ctx.scale(zoom, zoom);

    // Calculate node positions using a simple layout algorithm
    const nodePositions = calculateNodePositions(nodes, width, height);

    // Draw connections first (so they appear behind nodes)
    drawConnections(ctx, nodes, nodePositions);

    // Draw nodes
    drawNodes(ctx, nodes, nodePositions);

    ctx.restore();
  };

  const calculateNodePositions = (nodes: DAGNode[], width: number, height: number) => {
    const positions = new Map<string, { x: number; y: number }>();
    
    // Group nodes by round number
    const rounds = new Map<number, DAGNode[]>();
    nodes.forEach(node => {
      if (!rounds.has(node.roundNumber)) {
        rounds.set(node.roundNumber, []);
      }
      rounds.get(node.roundNumber)!.push(node);
    });

    const roundNumbers = Array.from(rounds.keys()).sort((a, b) => a - b);
    const maxRound = Math.max(...roundNumbers);
    const minRound = Math.min(...roundNumbers);

    // Calculate positions
    roundNumbers.forEach((roundNumber, roundIndex) => {
      const roundNodes = rounds.get(roundNumber)!;
      const y = (roundIndex / (roundNumbers.length - 1)) * (height - 200) + 100;
      
      roundNodes.forEach((node, nodeIndex) => {
        const x = (nodeIndex / (roundNodes.length - 1)) * (width - 200) + 100;
        positions.set(node.id, { x, y });
      });
    });

    return positions;
  };

  const drawConnections = (ctx: CanvasRenderingContext2D, nodes: DAGNode[], positions: Map<string, { x: number; y: number }>) => {
    ctx.strokeStyle = '#E5E7EB';
    ctx.lineWidth = 1;

    nodes.forEach(node => {
      const nodePos = positions.get(node.id);
      if (!nodePos) return;

      node.parentHashes.forEach(parentHash => {
        const parentNode = nodes.find(n => n.hash === parentHash);
        if (parentNode) {
          const parentPos = positions.get(parentNode.id);
          if (parentPos) {
            ctx.beginPath();
            ctx.moveTo(nodePos.x, nodePos.y);
            ctx.lineTo(parentPos.x, parentPos.y);
            ctx.stroke();
          }
        }
      });
    });
  };

  const drawNodes = (ctx: CanvasRenderingContext2D, nodes: DAGNode[], positions: Map<string, { x: number; y: number }>) => {
    nodes.forEach(node => {
      const pos = positions.get(node.id);
      if (!pos) return;

      const isSelected = selectedNode === node.id;
      const isHovered = hoveredNode === node.id;

      // Node background
      ctx.fillStyle = isSelected ? '#3B82F6' : isHovered ? '#60A5FA' : '#6B7280';
      ctx.beginPath();
      ctx.arc(pos.x, pos.y, 20, 0, 2 * Math.PI);
      ctx.fill();

      // Node border
      ctx.strokeStyle = isSelected ? '#1D4ED8' : '#374151';
      ctx.lineWidth = isSelected ? 3 : 1;
      ctx.stroke();

      // Node text
      ctx.fillStyle = '#FFFFFF';
      ctx.font = '12px Arial';
      ctx.textAlign = 'center';
      ctx.fillText(node.transactionCount.toString(), pos.x, pos.y + 4);

      // Node info on hover
      if (isHovered) {
        const info = `Round: ${node.roundNumber}\nTxs: ${node.transactionCount}\nHash: ${node.hash.slice(0, 8)}...`;
        drawTooltip(ctx, pos.x, pos.y, info);
      }
    });
  };

  const drawTooltip = (ctx: CanvasRenderingContext2D, x: number, y: number, text: string) => {
    const lines = text.split('\n');
    const lineHeight = 16;
    const padding = 8;
    const maxWidth = Math.max(...lines.map(line => ctx.measureText(line).width));
    const tooltipWidth = maxWidth + padding * 2;
    const tooltipHeight = lines.length * lineHeight + padding * 2;

    // Tooltip background
    ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
    ctx.fillRect(x + 25, y - tooltipHeight / 2, tooltipWidth, tooltipHeight);

    // Tooltip text
    ctx.fillStyle = '#FFFFFF';
    ctx.font = '12px Arial';
    ctx.textAlign = 'left';
    lines.forEach((line, index) => {
      ctx.fillText(line, x + 25 + padding, y - tooltipHeight / 2 + padding + (index + 1) * lineHeight);
    });
  };

  const getNodeAtPosition = (x: number, y: number): string | null => {
    const canvas = canvasRef.current;
    if (!canvas) return null;

    const rect = canvas.getBoundingClientRect();
    const adjustedX = (x - rect.left - pan.x) / zoom;
    const adjustedY = (y - rect.top - pan.y) / zoom;

    // Find the closest node
    let closestNode: string | null = null;
    let minDistance = Infinity;

    nodes.forEach(node => {
      const pos = { x: node.x || 0, y: node.y || 0 };
      const distance = Math.sqrt((adjustedX - pos.x) ** 2 + (adjustedY - pos.y) ** 2);
      if (distance < minDistance && distance < 25) {
        minDistance = distance;
        closestNode = node.id;
      }
    });

    return closestNode;
  };

  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const nodeId = getNodeAtPosition(e.clientX, e.clientY);
    setHoveredNode(nodeId);
    onNodeHover?.(nodeId);
  };

  const handleMouseLeave = () => {
    setHoveredNode(null);
    onNodeHover?.(null);
  };

  const handleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const nodeId = getNodeAtPosition(e.clientX, e.clientY);
    if (nodeId) {
      onNodeSelect?.(nodeId);
    }
  };

  const handleWheel = (e: React.WheelEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    setZoom(prev => Math.max(0.1, Math.min(3, prev * delta)));
  };

  const handleMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    setIsDragging(true);
    setDragStart({ x: e.clientX - pan.x, y: e.clientY - pan.y });
  };

  const handleMouseMovePan = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (isDragging) {
      setPan({
        x: e.clientX - dragStart.x,
        y: e.clientY - dragStart.y,
      });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  return (
    <div className="relative">
      <canvas
        ref={canvasRef}
        className="w-full h-full cursor-grab active:cursor-grabbing"
        onMouseMove={isDragging ? handleMouseMovePan : handleMouseMove}
        onMouseLeave={handleMouseLeave}
        onClick={handleClick}
        onWheel={handleWheel}
        onMouseDown={handleMouseDown}
        onMouseUp={handleMouseUp}
      />
      
      {/* Zoom Controls */}
      <div className="absolute top-4 right-4 flex flex-col space-y-2">
        <button
          onClick={() => setZoom(prev => Math.min(3, prev * 1.2))}
          className="w-8 h-8 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded shadow-sm hover:bg-gray-50 dark:hover:bg-gray-700"
        >
          +
        </button>
        <button
          onClick={() => setZoom(prev => Math.max(0.1, prev * 0.8))}
          className="w-8 h-8 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded shadow-sm hover:bg-gray-50 dark:hover:bg-gray-700"
        >
          −
        </button>
        <button
          onClick={() => { setZoom(1); setPan({ x: 0, y: 0 }); }}
          className="w-8 h-8 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded shadow-sm hover:bg-gray-50 dark:hover:bg-gray-700 text-xs"
        >
          ⌂
        </button>
      </div>

      {/* Info Panel */}
      <div className="absolute bottom-4 left-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-3 shadow-sm">
        <div className="text-sm text-gray-600 dark:text-gray-400">
          <div>Nodes: {nodes.length}</div>
          <div>Zoom: {Math.round(zoom * 100)}%</div>
          {selectedNode && (
            <div className="mt-2 text-xs">
              Selected: {selectedNode.slice(0, 8)}...
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default DAGVisualizer; 