import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { PerformanceMonitor, usePerformanceMonitoring, withPerformanceTracking } from '../PerformanceMonitor';
import { cache } from '../../../utils/cache';
import { performanceMonitor } from '../../../utils/cache';

// Mock the cache and performance monitor
jest.mock('../../../utils/cache', () => ({
  cache: {
    getStats: jest.fn(),
  },
  performanceMonitor: {
    getMetrics: jest.fn(),
    getAlerts: jest.fn(),
    clearOldAlerts: jest.fn(),
  },
}));

describe('PerformanceMonitor', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    
    // Mock cache stats
    (cache.getStats as jest.Mock).mockReturnValue({
      size: 5,
      maxSize: 100,
      hitRate: 0.85,
      totalHits: 85,
      totalMisses: 15,
    });
    
    // Mock performance metrics
    (performanceMonitor.getMetrics as jest.Mock).mockReturnValue({
      '/api/test': {
        avg: 150,
        min: 100,
        max: 200,
        count: 10,
      },
      '/api/slow': {
        avg: 800,
        min: 600,
        max: 1000,
        count: 5,
      },
    });
    
    // Mock alerts
    (performanceMonitor.getAlerts as jest.Mock).mockReturnValue([
      {
        type: 'performance',
        message: 'Slow API response detected: /api/slow (800ms)',
        timestamp: Date.now(),
      },
    ]);
  });

  describe('Component Rendering', () => {
    test('should render floating button when not visible', () => {
      render(<PerformanceMonitor />);
      
      const button = screen.getByRole('button', { name: /performance monitor/i });
      expect(button).toBeInTheDocument();
      expect(button).toHaveClass('bg-blue-600');
    });

    test('should render monitor panel when visible', () => {
      render(<PerformanceMonitor />);
      
      // Click to open
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Should show monitor panel
      expect(screen.getByText('Performance Monitor')).toBeInTheDocument();
      expect(screen.getByText('Cache Statistics')).toBeInTheDocument();
      expect(screen.getByText('API Performance')).toBeInTheDocument();
    });

    test('should display cache statistics correctly', () => {
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Check cache stats
      expect(screen.getByText('5/100')).toBeInTheDocument(); // Size
      expect(screen.getByText('85.0%')).toBeInTheDocument(); // Hit rate
      expect(screen.getByText('85')).toBeInTheDocument(); // Total hits
      expect(screen.getByText('15')).toBeInTheDocument(); // Total misses
    });

    test('should display API performance metrics', () => {
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Check API metrics
      expect(screen.getByText('/api/test')).toBeInTheDocument();
      expect(screen.getByText('150ms')).toBeInTheDocument(); // Average
      expect(screen.getByText('Min: 100ms')).toBeInTheDocument();
      expect(screen.getByText('Max: 200ms')).toBeInTheDocument();
      expect(screen.getByText('Calls: 10')).toBeInTheDocument();
    });

    test('should display performance alerts', () => {
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Check alerts
      expect(screen.getByText('Performance Alerts')).toBeInTheDocument();
      expect(screen.getByText(/Slow API response detected/)).toBeInTheDocument();
    });
  });

  describe('User Interactions', () => {
    test('should toggle visibility when button is clicked', () => {
      render(<PerformanceMonitor />);
      
      const button = screen.getByRole('button', { name: /performance monitor/i });
      
      // Initially should show button
      expect(button).toBeInTheDocument();
      
      // Click to open
      fireEvent.click(button);
      expect(screen.getByText('Performance Monitor')).toBeInTheDocument();
      
      // Click to close
      const closeButton = screen.getByText('×');
      fireEvent.click(closeButton);
      
      // Should show button again
      expect(screen.getByRole('button', { name: /performance monitor/i })).toBeInTheDocument();
    });

    test('should toggle auto-refresh', () => {
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Find auto-refresh toggle
      const autoRefreshButton = screen.getByText('Auto');
      expect(autoRefreshButton).toHaveClass('bg-green-100');
      
      // Toggle to manual
      fireEvent.click(autoRefreshButton);
      expect(autoRefreshButton).toHaveClass('bg-gray-100');
      expect(autoRefreshButton).toHaveTextContent('Manual');
    });

    test('should clear alerts when clear button is clicked', () => {
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Click clear alerts
      const clearButton = screen.getByText('Clear Alerts');
      fireEvent.click(clearButton);
      
      // Should call clearOldAlerts
      expect(performanceMonitor.clearOldAlerts).toHaveBeenCalled();
    });
  });

  describe('Performance Color Coding', () => {
    test('should show green for fast API calls', () => {
      (performanceMonitor.getMetrics as jest.Mock).mockReturnValue({
        '/api/fast': { avg: 50, min: 30, max: 70, count: 5 },
      });
      
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Should show green color for fast response
      const fastElement = screen.getByText('50ms');
      expect(fastElement).toHaveClass('text-green-600');
    });

    test('should show yellow for moderate API calls', () => {
      (performanceMonitor.getMetrics as jest.Mock).mockReturnValue({
        '/api/moderate': { avg: 250, min: 200, max: 300, count: 5 },
      });
      
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Should show yellow color for moderate response
      const moderateElement = screen.getByText('250ms');
      expect(moderateElement).toHaveClass('text-yellow-600');
    });

    test('should show red for slow API calls', () => {
      (performanceMonitor.getMetrics as jest.Mock).mockReturnValue({
        '/api/slow': { avg: 800, min: 600, max: 1000, count: 5 },
      });
      
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Should show red color for slow response
      const slowElement = screen.getByText('800ms');
      expect(slowElement).toHaveClass('text-red-600');
    });
  });

  describe('Auto-refresh Functionality', () => {
    beforeEach(() => {
      jest.useFakeTimers();
    });

    afterEach(() => {
      jest.useRealTimers();
    });

    test('should auto-refresh metrics every 5 seconds', async () => {
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Initial call
      expect(performanceMonitor.getMetrics).toHaveBeenCalledTimes(1);
      
      // Fast-forward 5 seconds
      jest.advanceTimersByTime(5000);
      
      await waitFor(() => {
        expect(performanceMonitor.getMetrics).toHaveBeenCalledTimes(2);
      });
      
      // Fast-forward another 5 seconds
      jest.advanceTimersByTime(5000);
      
      await waitFor(() => {
        expect(performanceMonitor.getMetrics).toHaveBeenCalledTimes(3);
      });
    });

    test('should stop auto-refresh when toggled to manual', async () => {
      render(<PerformanceMonitor />);
      
      // Open monitor
      const button = screen.getByRole('button', { name: /performance monitor/i });
      fireEvent.click(button);
      
      // Toggle to manual
      const autoRefreshButton = screen.getByText('Auto');
      fireEvent.click(autoRefreshButton);
      
      // Fast-forward 5 seconds
      jest.advanceTimersByTime(5000);
      
      // Should not have called getMetrics again
      expect(performanceMonitor.getMetrics).toHaveBeenCalledTimes(1);
    });
  });
});

describe('usePerformanceMonitoring Hook', () => {
  test('should provide performance monitoring functions', () => {
    const TestComponent = () => {
      const { trackApiCall, getMetrics, getAlerts, clearAlerts } = usePerformanceMonitoring();
      
      // Test that functions are available
      expect(typeof trackApiCall).toBe('function');
      expect(typeof getMetrics).toBe('function');
      expect(typeof getAlerts).toBe('function');
      expect(typeof clearAlerts).toBe('function');
      
      return <div>Test</div>;
    };
    
    render(<TestComponent />);
  });
});

describe('withPerformanceTracking HOC', () => {
  test('should wrap function with performance tracking', async () => {
    const mockFn = jest.fn().mockResolvedValue('test result');
    const trackedFn = withPerformanceTracking(mockFn, '/api/test');
    
    const result = await trackedFn();
    
    expect(result).toBe('test result');
    expect(mockFn).toHaveBeenCalled();
    
    // Should have tracked the API call
    const metrics = performanceMonitor.getMetrics();
    expect(metrics['/api/test']).toBeDefined();
  });

  test('should track errors in performance monitoring', async () => {
    const mockFn = jest.fn().mockRejectedValue(new Error('API Error'));
    const trackedFn = withPerformanceTracking(mockFn, '/api/error');
    
    await expect(trackedFn()).rejects.toThrow('API Error');
    
    // Should still track the API call even if it failed
    const metrics = performanceMonitor.getMetrics();
    expect(metrics['/api/error']).toBeDefined();
  });
}); 