import React, { useState, useRef, useEffect, useCallback, useMemo } from 'react';
import { ChevronUp, ChevronDown, ArrowUp, ArrowDown } from 'lucide-react';
import { useVirtualScroll } from './PerformanceOptimizer';

export interface VirtualScrollerProps<T> {
  items: T[];
  itemHeight: number;
  containerHeight: number;
  overscan?: number;
  renderItem: (item: T, index: number) => React.ReactNode;
  className?: string;
  showScrollIndicator?: boolean;
  enableSmoothScrolling?: boolean;
  onScroll?: (scrollTop: number) => void;
  onVisibleRangeChange?: (startIndex: number, endIndex: number) => void;
  loading?: boolean;
  loadingComponent?: React.ReactNode;
  emptyComponent?: React.ReactNode;
  getItemKey?: (item: T, index: number) => string | number;
}

export function VirtualScroller<T>({
  items,
  itemHeight,
  containerHeight,
  overscan = 5,
  renderItem,
  className = '',
  showScrollIndicator = true,
  enableSmoothScrolling = true,
  onScroll,
  onVisibleRangeChange,
  loading = false,
  loadingComponent,
  emptyComponent,
  getItemKey = (_, index) => index,
}: VirtualScrollerProps<T>) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const [isScrolling, setIsScrolling] = useState(false);
  const [scrollDirection, setScrollDirection] = useState<'up' | 'down' | null>(null);

  const {
    virtualItems,
    totalHeight,
    setScrollTop: setVirtualScrollTop,
  } = useVirtualScroll(items, itemHeight, containerHeight, overscan);

  // Handle scroll events
  const handleScroll = useCallback((event: React.UIEvent<HTMLDivElement>) => {
    const target = event.target as HTMLDivElement;
    const newScrollTop = target.scrollTop;
    
    setScrollTop(newScrollTop);
    setVirtualScrollTop?.(newScrollTop);
    
    // Determine scroll direction
    if (newScrollTop > scrollTop) {
      setScrollDirection('down');
    } else if (newScrollTop < scrollTop) {
      setScrollDirection('up');
    }
    
    setIsScrolling(true);
    onScroll?.(newScrollTop);
    
    // Clear scrolling state after a delay
    setTimeout(() => {
      setIsScrolling(false);
      setScrollDirection(null);
    }, 150);
  }, [scrollTop, setVirtualScrollTop, onScroll]);



  // Notify visible range changes
  useEffect(() => {
    if (virtualItems.length > 0) {
      const startIndex = virtualItems[0].index;
      const endIndex = virtualItems[virtualItems.length - 1].index;
      onVisibleRangeChange?.(startIndex, endIndex);
    }
  }, [virtualItems, onVisibleRangeChange]);

  // Scroll to specific item
  const scrollToItem = useCallback((index: number, behavior: ScrollBehavior = 'smooth') => {
    const targetScrollTop = index * itemHeight;
    containerRef.current?.scrollTo({
      top: targetScrollTop,
      behavior: enableSmoothScrolling ? behavior : 'auto',
    });
  }, [itemHeight, enableSmoothScrolling]);

  // Scroll to top
  const scrollToTop = useCallback(() => {
    scrollToItem(0);
  }, [scrollToItem]);

  // Scroll to bottom
  const scrollToBottom = useCallback(() => {
    scrollToItem(items.length - 1);
  }, [scrollToItem, items.length]);

  // Calculate scroll percentage
  const scrollPercentage = useMemo(() => {
    if (totalHeight <= containerHeight) return 0;
    return (scrollTop / (totalHeight - containerHeight)) * 100;
  }, [scrollTop, totalHeight, containerHeight]);

  // Show scroll to top button
  const showScrollToTop = scrollPercentage > 10;

  // Loading state
  if (loading && loadingComponent) {
    return (
      <div className={`flex items-center justify-center ${className}`} style={{ height: containerHeight }}>
        {loadingComponent}
      </div>
    );
  }

  // Empty state
  if (items.length === 0 && emptyComponent) {
    return (
      <div className={`flex items-center justify-center ${className}`} style={{ height: containerHeight }}>
        {emptyComponent}
      </div>
    );
  }

  return (
    <div className={`relative ${className}`}>
      {/* Main scroll container */}
      <div
        ref={containerRef}
        className="overflow-auto"
        style={{ height: containerHeight }}
        onScroll={handleScroll}
      >
        <div style={{ height: totalHeight, position: 'relative' }}>
          {virtualItems.map(({ item, index, offsetTop }) => (
            <div
              key={getItemKey(item, index)}
              style={{
                position: 'absolute',
                top: offsetTop,
                height: itemHeight,
                width: '100%',
              }}
              className={`transition-opacity duration-200 ${
                isScrolling ? 'opacity-75' : 'opacity-100'
              }`}
            >
              {renderItem(item, index)}
            </div>
          ))}
        </div>
      </div>

      {/* Scroll indicator */}
      {showScrollIndicator && items.length > 0 && (
        <div className="absolute right-2 top-1/2 transform -translate-y-1/2">
          <div className="bg-gray-800 bg-opacity-75 rounded-lg p-2 text-white text-xs">
            <div className="flex items-center space-x-1">
              {scrollDirection === 'up' && <ArrowUp className="w-3 h-3" />}
              {scrollDirection === 'down' && <ArrowDown className="w-3 h-3" />}
              <span>{Math.round(scrollPercentage)}%</span>
            </div>
          </div>
        </div>
      )}

      {/* Scroll to top button */}
      {showScrollToTop && (
        <button
          onClick={scrollToTop}
          className="absolute bottom-4 right-4 bg-primary-600 text-white p-3 rounded-full shadow-lg hover:bg-primary-700 transition-colors"
          title="Scroll to top"
        >
          <ChevronUp className="w-5 h-5" />
        </button>
      )}

      {/* Performance info */}
      <div className="absolute top-2 left-2 bg-black bg-opacity-75 text-white text-xs px-2 py-1 rounded">
        {virtualItems.length} / {items.length} items
      </div>
    </div>
  );
}

// Hook for virtual scroller state
export const useVirtualScroller = <T,>(
  items: T[],
  itemHeight: number,
  containerHeight: number
) => {
  const [scrollTop, setScrollTop] = useState(0);
  const [visibleRange, setVisibleRange] = useState({ start: 0, end: 0 });
  const [isScrolling, setIsScrolling] = useState(false);

  const handleScroll = useCallback((newScrollTop: number) => {
    setScrollTop(newScrollTop);
  }, []);

  const handleVisibleRangeChange = useCallback((startIndex: number, endIndex: number) => {
    setVisibleRange({ start: startIndex, end: endIndex });
  }, []);

  const scrollToIndex = useCallback((index: number) => {
    const targetScrollTop = index * itemHeight;
    setScrollTop(targetScrollTop);
  }, [itemHeight]);

  return {
    scrollTop,
    visibleRange,
    isScrolling,
    handleScroll,
    handleVisibleRangeChange,
    scrollToIndex,
  };
};

// Virtual list component for simple use cases
interface VirtualListProps<T> {
  items: T[];
  height: number;
  itemHeight: number;
  renderItem: (item: T, index: number) => React.ReactNode;
  className?: string;
  loading?: boolean;
  emptyMessage?: string;
}

export function VirtualList<T>({
  items,
  height,
  itemHeight,
  renderItem,
  className = '',
  loading = false,
  emptyMessage = 'No items to display',
}: VirtualListProps<T>) {
  const loadingComponent = (
    <div className="flex items-center justify-center space-x-2">
      <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-primary-600"></div>
      <span className="text-gray-600">Loading...</span>
    </div>
  );

  const emptyComponent = (
    <div className="text-center text-gray-500">
      <div className="text-lg font-medium mb-2">No Data</div>
      <div className="text-sm">{emptyMessage}</div>
    </div>
  );

  return (
    <VirtualScroller
      items={items}
      itemHeight={itemHeight}
      containerHeight={height}
      renderItem={renderItem}
      className={className}
      loading={loading}
      loadingComponent={loadingComponent}
      emptyComponent={emptyComponent}
    />
  );
}

// Infinite scroll wrapper
interface InfiniteScrollProps<T> {
  items: T[];
  itemHeight: number;
  containerHeight: number;
  renderItem: (item: T, index: number) => React.ReactNode;
  onLoadMore: () => void;
  hasMore: boolean;
  loading: boolean;
  className?: string;
  threshold?: number;
}

export function InfiniteScroll<T>({
  items,
  itemHeight,
  containerHeight,
  renderItem,
  onLoadMore,
  hasMore,
  loading,
  className = '',
  threshold = 5,
}: InfiniteScrollProps<T>) {
  const handleVisibleRangeChange = useCallback((startIndex: number, endIndex: number) => {
    if (hasMore && !loading && endIndex >= items.length - threshold) {
      onLoadMore();
    }
  }, [hasMore, loading, items.length, threshold, onLoadMore]);

  const loadingComponent = (
    <div className="flex items-center justify-center space-x-2 py-4">
      <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-primary-600"></div>
      <span className="text-sm text-gray-600">Loading more...</span>
    </div>
  );

  return (
    <VirtualScroller
      items={items}
      itemHeight={itemHeight}
      containerHeight={containerHeight}
      renderItem={renderItem}
      className={className}
      onVisibleRangeChange={handleVisibleRangeChange}
      loading={loading && items.length === 0}
      loadingComponent={loadingComponent}
    />
  );
} 