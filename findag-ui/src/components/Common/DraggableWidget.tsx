import React, { useState, useRef, useEffect } from 'react';
import { GripVertical, X } from 'lucide-react';

interface DraggableWidgetProps {
  id: string;
  children: React.ReactNode;
  isEditMode: boolean;
  onRemove?: () => void;
  onDragStart?: (e: React.DragEvent, id: string) => void;
  onDragOver?: (e: React.DragEvent, id: string) => void;
  onDrop?: (e: React.DragEvent, id: string) => void;
  className?: string;
  size?: string;
}

export const DraggableWidget: React.FC<DraggableWidgetProps> = ({
  id,
  children,
  isEditMode,
  onRemove,
  onDragStart,
  onDragOver,
  onDrop,
  className = '',
  size = 'col-span-1',
}) => {
  const [isDragging, setIsDragging] = useState(false);
  const [isDragOver, setIsDragOver] = useState(false);
  const widgetRef = useRef<HTMLDivElement>(null);

  const handleDragStart = (e: React.DragEvent) => {
    if (!isEditMode) return;
    
    setIsDragging(true);
    e.dataTransfer.setData('text/plain', id);
    e.dataTransfer.effectAllowed = 'move';
    
    // Add dragging class to body
    document.body.classList.add('dragging');
    
    onDragStart?.(e, id);
  };

  const handleDragEnd = () => {
    setIsDragging(false);
    setIsDragOver(false);
    document.body.classList.remove('dragging');
  };

  const handleDragOver = (e: React.DragEvent) => {
    if (!isEditMode) return;
    
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    setIsDragOver(true);
    onDragOver?.(e, id);
  };

  const handleDragLeave = () => {
    setIsDragOver(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    if (!isEditMode) return;
    
    e.preventDefault();
    setIsDragOver(false);
    onDrop?.(e, id);
  };

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      document.body.classList.remove('dragging');
    };
  }, []);

  return (
    <div
      ref={widgetRef}
      className={`relative ${size} ${className} ${
        isEditMode ? 'cursor-move' : ''
      } ${
        isDragging ? 'opacity-50 scale-95' : ''
      } ${
        isDragOver ? 'ring-2 ring-primary-500 ring-opacity-50' : ''
      } transition-all duration-200`}
      draggable={isEditMode}
      onDragStart={handleDragStart}
      onDragEnd={handleDragEnd}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
    >
      {/* Edit mode overlay */}
      {isEditMode && (
        <div className="absolute inset-0 pointer-events-none">
          {/* Drag handle */}
          <div className="absolute top-2 left-2 z-20 pointer-events-auto">
            <div className="p-1 bg-gray-800 bg-opacity-75 text-white rounded cursor-move hover:bg-opacity-90 transition-colors">
              <GripVertical className="w-3 h-3" />
            </div>
          </div>

          {/* Remove button */}
          {onRemove && (
            <div className="absolute top-2 right-2 z-20 pointer-events-auto">
              <button
                onClick={onRemove}
                className="p-1 bg-red-500 text-white rounded-full hover:bg-red-600 transition-colors"
                title="Remove widget"
              >
                <X className="w-3 h-3" />
              </button>
            </div>
          )}

          {/* Widget border */}
          <div className="absolute inset-0 border-2 border-dashed border-gray-300 rounded-lg pointer-events-none" />
        </div>
      )}

      {/* Widget content */}
      <div className={`${isEditMode ? 'pointer-events-none' : ''}`}>
        {children}
      </div>

      {/* Drop indicator */}
      {isDragOver && (
        <div className="absolute inset-0 bg-primary-500 bg-opacity-10 border-2 border-primary-500 rounded-lg pointer-events-none z-10" />
      )}
    </div>
  );
};

// Drag and drop context provider
interface DragDropContextProps {
  children: React.ReactNode;
  onWidgetMove?: (fromId: string, toId: string) => void;
}

export const DragDropContext: React.FC<DragDropContextProps> = ({
  children,
  onWidgetMove,
}) => {
  const [draggedId, setDraggedId] = useState<string | null>(null);

  const handleDragStart = (id: string) => {
    setDraggedId(id);
  };

  const handleDrop = (targetId: string) => {
    if (draggedId && draggedId !== targetId) {
      onWidgetMove?.(draggedId, targetId);
    }
    setDraggedId(null);
  };

  return (
    <div className="drag-drop-context">
      {React.Children.map(children, (child) => {
        if (React.isValidElement(child)) {
          return React.cloneElement(child, {
            onDragStart: (e: React.DragEvent, id: string) => handleDragStart(id),
            onDrop: (e: React.DragEvent, id: string) => handleDrop(id),
          } as any);
        }
        return child;
      })}
    </div>
  );
};

// Widget grid container
interface WidgetGridProps {
  children: React.ReactNode;
  layout: string;
  className?: string;
  onWidgetMove?: (fromId: string, toId: string) => void;
}

export const WidgetGrid: React.FC<WidgetGridProps> = ({
  children,
  layout,
  className = '',
  onWidgetMove,
}) => {
  return (
    <DragDropContext onWidgetMove={onWidgetMove}>
      <div className={`grid gap-6 ${layout} ${className}`}>
        {children}
      </div>
    </DragDropContext>
  );
};

// Widget placeholder for empty slots
interface WidgetPlaceholderProps {
  onAddWidget?: () => void;
  className?: string;
}

export const WidgetPlaceholder: React.FC<WidgetPlaceholderProps> = ({
  onAddWidget,
  className = '',
}) => {
  return (
    <div
      className={`border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:border-primary-400 hover:bg-primary-50 transition-colors ${className}`}
      onClick={onAddWidget}
    >
      <div className="text-gray-400 mb-2">
        <svg className="w-8 h-8 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
        </svg>
      </div>
      <p className="text-sm text-gray-500">Click to add a widget</p>
    </div>
  );
}; 