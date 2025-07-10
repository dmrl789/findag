import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import AdvancedChart from '../AdvancedChart';

// Mock TradingView
jest.mock('react-ts-tradingview-widgets', () => {
  return function MockTradingViewWidget({ symbol }: { symbol: string }) {
    return <div data-testid="tradingview-widget" data-symbol={symbol} />;
  };
});

describe('AdvancedChart Performance', () => {
  const mockChartConfig = {
    type: 'line' as const,
    title: 'EUR/USD Price Chart',
    data: Array.from({ length: 1000 }, (_, i) => ({
      timestamp: Date.now() - (1000 - i) * 60000,
      value: 1.0850 + Math.random() * 0.01,
      label: `Point ${i}`,
    })),
    color: '#3B82F6',
    yAxisLabel: 'Price (EUR/USD)',
    xAxisLabel: 'Time',
  };

  const mockAnnotations = [
    {
      id: '1',
      x: Date.now() - 500000,
      y: 1.0850,
      text: 'Support Level',
      color: '#10B981',
      type: 'point' as const,
    },
  ];

  it('renders chart with large dataset efficiently', async () => {
    const startTime = performance.now();

    render(
      <AdvancedChart
        config={mockChartConfig}
        annotations={mockAnnotations}
        onAnnotationAdd={jest.fn()}
        onAnnotationRemove={jest.fn()}
      />
    );

    const endTime = performance.now();
    const renderTime = endTime - startTime;

    // Should render within 100ms for large datasets
    expect(renderTime).toBeLessThan(100);

    await waitFor(() => {
      expect(screen.getByRole('img')).toBeInTheDocument();
    });
  });

  it('handles rapid data updates without performance degradation', async () => {
    const { rerender } = render(
      <AdvancedChart
        config={mockChartConfig}
        annotations={mockAnnotations}
        onAnnotationAdd={jest.fn()}
        onAnnotationRemove={jest.fn()}
      />
    );

    // Simulate rapid data updates
    for (let i = 0; i < 10; i++) {
      const startTime = performance.now();

      const updatedConfig = {
        ...mockChartConfig,
        data: mockChartConfig.data.slice(0, 100 + i * 10),
      };

      rerender(
        <AdvancedChart
          config={updatedConfig}
          annotations={mockAnnotations}
          onAnnotationAdd={jest.fn()}
          onAnnotationRemove={jest.fn()}
        />
      );

      const endTime = performance.now();
      const updateTime = endTime - startTime;

      // Each update should be fast
      expect(updateTime).toBeLessThan(50);
    }
  });

  it('maintains performance with multiple annotations', async () => {
    const multipleAnnotations = Array.from({ length: 10 }, (_, i) => ({
      id: `annotation-${i}`,
      x: Date.now() - (i * 50000),
      y: 1.0850 + Math.random() * 0.01,
      text: `Annotation ${i}`,
      color: '#10B981',
      type: 'point' as const,
    }));

    const startTime = performance.now();

    render(
      <AdvancedChart
        config={mockChartConfig}
        annotations={multipleAnnotations}
        onAnnotationAdd={jest.fn()}
        onAnnotationRemove={jest.fn()}
      />
    );

    const endTime = performance.now();
    const renderTime = endTime - startTime;

    // Should render with multiple annotations within reasonable time
    expect(renderTime).toBeLessThan(150);

    await waitFor(() => {
      expect(screen.getByRole('img')).toBeInTheDocument();
    });
  });

  it('handles memory usage efficiently', async () => {
    const initialMemory = (performance as any).memory?.usedJSHeapSize || 0;

    const { unmount } = render(
      <AdvancedChart
        config={mockChartConfig}
        annotations={mockAnnotations}
        onAnnotationAdd={jest.fn()}
        onAnnotationRemove={jest.fn()}
      />
    );

    // Wait for component to fully render
    await waitFor(() => {
      expect(screen.getByRole('img')).toBeInTheDocument();
    });

    // Unmount component
    unmount();

    // Force garbage collection if available
    if (global.gc) {
      global.gc();
    }

    // Memory should be cleaned up after unmount
    if ((performance as any).memory) {
      const finalMemory = (performance as any).memory.usedJSHeapSize;
      const memoryIncrease = finalMemory - initialMemory;
      
      // Memory increase should be reasonable (less than 10MB)
      expect(memoryIncrease).toBeLessThan(10 * 1024 * 1024);
    }
  });

  it('handles concurrent chart renders efficiently', async () => {
    const renderPromises = Array.from({ length: 5 }, (_, index) => {
      return new Promise<number>((resolve) => {
        const startTime = performance.now();
        
        const { unmount } = render(
          <AdvancedChart
            config={{
              ...mockChartConfig,
              title: `Chart ${index}`,
              data: mockChartConfig.data.slice(0, 100),
            }}
            annotations={mockAnnotations}
            onAnnotationAdd={jest.fn()}
            onAnnotationRemove={jest.fn()}
          />
        );

        setTimeout(() => {
          const endTime = performance.now();
          unmount();
          resolve(endTime - startTime);
        }, 100);
      });
    });

    const renderTimes = await Promise.all(renderPromises);

    // All charts should render efficiently
    renderTimes.forEach(renderTime => {
      expect(renderTime).toBeLessThan(100);
    });
  });
}); 