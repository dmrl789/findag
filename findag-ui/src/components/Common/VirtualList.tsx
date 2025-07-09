import React, { useState, useEffect, useRef, useCallback, useMemo } from 'react';

interface VirtualListProps<T> {
  items: T[];
  height: number;
  itemHeight: number;
  renderItem: (item: T, index: number) => React.ReactNode;
  overscan?: number;
  className?: string;
  onScroll?: (scrollTop: number) => void;
  getItemKey?: (item: T, index: number) => string | number;
}

export function VirtualList<T>({
  items,
  height,
  itemHeight,
  renderItem,
  overscan = 5,
  className = '',
  onScroll,
  getItemKey = (_, index) => index,
}: VirtualListProps<T>) {
  const [scrollTop, setScrollTop] = useState(0);
  const containerRef = useRef<HTMLDivElement>(null);

  // Calculate visible range
  const visibleRange = useMemo(() => {
    const start = Math.floor(scrollTop / itemHeight);
    const visibleCount = Math.ceil(height / itemHeight);
    const end = Math.min(start + visibleCount + overscan, items.length);
    const startIndex = Math.max(0, start - overscan);
    
    return { start: startIndex, end };
  }, [scrollTop, itemHeight, height, overscan, items.length]);

  // Calculate total height and offset
  const totalHeight = items.length * itemHeight;
  const offsetY = visibleRange.start * itemHeight;

  // Handle scroll
  const handleScroll = useCallback((event: React.UIEvent<HTMLDivElement>) => {
    const newScrollTop = event.currentTarget.scrollTop;
    setScrollTop(newScrollTop);
    onScroll?.(newScrollTop);
  }, [onScroll]);

  // Scroll to item
  const scrollToItem = useCallback((index: number, behavior: ScrollBehavior = 'smooth') => {
    if (containerRef.current) {
      const targetScrollTop = index * itemHeight;
      containerRef.current.scrollTo({
        top: targetScrollTop,
        behavior,
      });
    }
  }, [itemHeight]);

  // Scroll to top
  const scrollToTop = useCallback(() => {
    scrollToItem(0);
  }, [scrollToItem]);

  // Get visible items
  const visibleItems = useMemo(() => {
    return items.slice(visibleRange.start, visibleRange.end);
  }, [items, visibleRange.start, visibleRange.end]);

  return (
    <div
      ref={containerRef}
      className={`overflow-auto ${className}`}
      style={{ height }}
      onScroll={handleScroll}
    >
      <div style={{ height: totalHeight, position: 'relative' }}>
        <div
          style={{
            position: 'absolute',
            top: offsetY,
            left: 0,
            right: 0,
          }}
        >
          {visibleItems.map((item, index) => {
            const actualIndex = visibleRange.start + index;
            return (
              <div
                key={getItemKey(item, actualIndex)}
                style={{ height: itemHeight }}
              >
                {renderItem(item, actualIndex)}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}

// Hook for virtual list state
export function useVirtualList<T>(
  items: T[],
  itemHeight: number,
  containerHeight: number,
  overscan: number = 5
) {
  const [scrollTop, setScrollTop] = useState(0);

  const visibleRange = useMemo(() => {
    const start = Math.floor(scrollTop / itemHeight);
    const visibleCount = Math.ceil(containerHeight / itemHeight);
    const end = Math.min(start + visibleCount + overscan, items.length);
    const startIndex = Math.max(0, start - overscan);
    
    return { start: startIndex, end };
  }, [scrollTop, itemHeight, containerHeight, overscan, items.length]);

  const totalHeight = items.length * itemHeight;
  const offsetY = visibleRange.start * itemHeight;
  const visibleItems = items.slice(visibleRange.start, visibleRange.end);

  return {
    scrollTop,
    setScrollTop,
    visibleRange,
    totalHeight,
    offsetY,
    visibleItems,
  };
}

// Virtual table component
interface VirtualTableProps<T> {
  items: T[];
  height: number;
  rowHeight: number;
  columns: {
    key: string;
    header: string;
    width?: number | string;
    render: (item: T, index: number) => React.ReactNode;
  }[];
  className?: string;
  getItemKey?: (item: T, index: number) => string | number;
}

export function VirtualTable<T>({
  items,
  height,
  rowHeight,
  columns,
  className = '',
  getItemKey = (_, index) => index,
}: VirtualTableProps<T>) {
  const [scrollTop, setScrollTop] = useState(0);
  const containerRef = useRef<HTMLDivElement>(null);

  const visibleRange = useMemo(() => {
    const start = Math.floor(scrollTop / rowHeight);
    const visibleCount = Math.ceil(height / rowHeight);
    const end = Math.min(start + visibleCount + 5, items.length);
    const startIndex = Math.max(0, start - 5);
    
    return { start: startIndex, end };
  }, [scrollTop, rowHeight, height, items.length]);

  const totalHeight = items.length * rowHeight;
  const offsetY = visibleRange.start * rowHeight;
  const visibleItems = items.slice(visibleRange.start, visibleRange.end);

  const handleScroll = useCallback((event: React.UIEvent<HTMLDivElement>) => {
    setScrollTop(event.currentTarget.scrollTop);
  }, []);

  return (
    <div className={`border border-gray-200 rounded-lg overflow-hidden ${className}`}>
      {/* Header */}
      <div className="bg-gray-50 border-b border-gray-200">
        <div className="flex">
          {columns.map((column) => (
            <div
              key={column.key}
              className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              style={{ width: column.width || 'auto', flex: column.width ? 'none' : 1 }}
            >
              {column.header}
            </div>
          ))}
        </div>
      </div>

      {/* Virtual body */}
      <div
        ref={containerRef}
        className="overflow-auto"
        style={{ height: height - 48 }} // Subtract header height
        onScroll={handleScroll}
      >
        <div style={{ height: totalHeight, position: 'relative' }}>
          <div
            style={{
              position: 'absolute',
              top: offsetY,
              left: 0,
              right: 0,
            }}
          >
            {visibleItems.map((item, index) => {
              const actualIndex = visibleRange.start + index;
              return (
                <div
                  key={getItemKey(item, actualIndex)}
                  className="flex border-b border-gray-200 hover:bg-gray-50"
                  style={{ height: rowHeight }}
                >
                  {columns.map((column) => (
                    <div
                      key={column.key}
                      className="px-4 py-3 text-sm text-gray-900 flex items-center"
                      style={{ width: column.width || 'auto', flex: column.width ? 'none' : 1 }}
                    >
                      {column.render(item, actualIndex)}
                    </div>
                  ))}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
} 