import { Cache, cache, performanceMonitor, createCacheKey, createUserCacheKey, trackApiCall } from '../cache';

// Mock window.setTimeout and window.setInterval
const mockSetTimeout = jest.fn();
const mockSetInterval = jest.fn();
const mockClearTimeout = jest.fn();
const mockClearInterval = jest.fn();

Object.defineProperty(window, 'setTimeout', {
  value: mockSetTimeout,
});

Object.defineProperty(window, 'setInterval', {
  value: mockSetInterval,
});

Object.defineProperty(window, 'clearTimeout', {
  value: mockClearTimeout,
});

Object.defineProperty(window, 'clearInterval', {
  value: mockClearInterval,
});

describe('Cache System', () => {
  let testCache: Cache;

  beforeEach(() => {
    jest.clearAllMocks();
    testCache = new Cache({
      maxSize: 10,
      defaultTTL: 1000,
      cleanupInterval: 500,
      compressionEnabled: true,
      compressionThreshold: 100,
      batchTimeout: 50,
      maxBatchSize: 5,
    });
  });

  afterEach(() => {
    testCache.clear();
    testCache.stopCleanup();
  });

  describe('Basic Cache Operations', () => {
    test('should set and get cache entry', async () => {
      const testData = { id: 1, name: 'test' };
      await testCache.set('test-key', testData);
      
      const result = await testCache.get('test-key');
      expect(result).toEqual(testData);
    });

    test('should handle cache miss', async () => {
      const result = await testCache.get('non-existent-key');
      expect(result).toBeNull();
    });

    test('should check if key exists', () => {
      testCache.set('test-key', 'test-value');
      expect(testCache.has('test-key')).toBe(true);
      expect(testCache.has('non-existent-key')).toBe(false);
    });

    test('should delete cache entry', async () => {
      await testCache.set('test-key', 'test-value');
      expect(testCache.delete('test-key')).toBe(true);
      expect(await testCache.get('test-key')).toBeNull();
    });

    test('should clear all cache entries', async () => {
      await testCache.set('key1', 'value1');
      await testCache.set('key2', 'value2');
      
      testCache.clear();
      
      expect(await testCache.get('key1')).toBeNull();
      expect(await testCache.get('key2')).toBeNull();
    });
  });

  describe('TTL and Expiration', () => {
    test('should expire cache entries after TTL', async () => {
      await testCache.set('test-key', 'test-value', 100);
      
      // Wait for expiration
      await new Promise(resolve => setTimeout(resolve, 150));
      
      const result = await testCache.get('test-key');
      expect(result).toBeNull();
    });

    test('should not expire cache entries before TTL', async () => {
      await testCache.set('test-key', 'test-value', 1000);
      
      const result = await testCache.get('test-key');
      expect(result).toBe('test-value');
    });

    test('should cleanup expired entries', async () => {
      await testCache.set('expired-key', 'expired-value', 50);
      await testCache.set('valid-key', 'valid-value', 1000);
      
      // Wait for expiration
      await new Promise(resolve => setTimeout(resolve, 100));
      
      // Trigger cleanup
      testCache['cleanup']();
      
      expect(await testCache.get('expired-key')).toBeNull();
      expect(await testCache.get('valid-key')).toBe('valid-value');
    });
  });

  describe('Compression', () => {
    test('should compress large data', async () => {
      const largeData = 'x'.repeat(200); // Larger than compression threshold
      await testCache.set('large-key', largeData);
      
      const stats = testCache.getStats();
      expect(stats.compressedEntries).toBe(1);
    });

    test('should not compress small data', async () => {
      const smallData = 'small'; // Smaller than compression threshold
      await testCache.set('small-key', smallData);
      
      const stats = testCache.getStats();
      expect(stats.compressedEntries).toBe(0);
    });

    test('should decompress data correctly', async () => {
      const originalData = { large: 'x'.repeat(200) };
      await testCache.set('compressed-key', originalData);
      
      const result = await testCache.get('compressed-key');
      expect(result).toEqual(originalData);
    });

    test('should handle compression errors gracefully', async () => {
      // Mock compression to fail
      const originalCompress = testCache['compressionUtils']?.compress;
      if (originalCompress) {
        testCache['compressionUtils'].compress = jest.fn().mockRejectedValue(new Error('Compression failed'));
      }
      
      const testData = 'x'.repeat(200);
      await testCache.set('error-key', testData);
      
      const result = await testCache.get('error-key');
      expect(result).toBe(testData); // Should store uncompressed
    });
  });

  describe('Cache Statistics', () => {
    test('should track cache statistics correctly', async () => {
      await testCache.set('key1', 'value1');
      await testCache.set('key2', 'value2');
      
      const stats = testCache.getStats();
      
      expect(stats.size).toBe(2);
      expect(stats.maxSize).toBe(10);
      expect(stats.totalHits).toBe(0); // No gets yet
      expect(stats.compressedEntries).toBe(0);
    });

    test('should track cache hits and misses', async () => {
      await testCache.set('key1', 'value1');
      
      // Hit
      await testCache.get('key1');
      
      // Miss
      await testCache.get('non-existent');
      
      const stats = testCache.getStats();
      expect(stats.totalHits).toBe(1);
    });

    test('should calculate hit rate correctly', async () => {
      await testCache.set('key1', 'value1');
      
      // 2 hits, 1 miss
      await testCache.get('key1');
      await testCache.get('key1');
      await testCache.get('non-existent');
      
      const stats = testCache.getStats();
      expect(stats.hitRate).toBe(2 / 3);
    });
  });

  describe('LRU Eviction', () => {
    test('should evict least recently used when cache is full', async () => {
      // Fill cache to capacity
      for (let i = 0; i < 10; i++) {
        await testCache.set(`key${i}`, `value${i}`);
      }
      
      // Access some keys to update LRU
      await testCache.get('key0');
      await testCache.get('key1');
      
      // Add one more to trigger eviction
      await testCache.set('new-key', 'new-value');
      
      // key2 should be evicted (least recently used)
      expect(await testCache.get('key2')).toBeNull();
      expect(await testCache.get('key0')).toBe('value0'); // Still there
      expect(await testCache.get('new-key')).toBe('new-value');
    });
  });

  describe('Request Deduplication', () => {
    test('should deduplicate concurrent requests', async () => {
      const deduplicator = testCache.getDeduplicator();
      
      let callCount = 0;
      const requestFn = jest.fn().mockImplementation(async () => {
        callCount++;
        return { data: 'test' };
      });
      
      // Make concurrent requests with same key
      const promises = [
        deduplicator.deduplicate('test-key', requestFn),
        deduplicator.deduplicate('test-key', requestFn),
        deduplicator.deduplicate('test-key', requestFn),
      ];
      
      const results = await Promise.all(promises);
      
      // Should only call the function once
      expect(callCount).toBe(1);
      expect(results).toEqual([
        { data: 'test' },
        { data: 'test' },
        { data: 'test' },
      ]);
    });

    test('should handle different keys separately', async () => {
      const deduplicator = testCache.getDeduplicator();
      
      let callCount = 0;
      const requestFn = jest.fn().mockImplementation(async () => {
        callCount++;
        return { data: 'test' };
      });
      
      // Make requests with different keys
      const promises = [
        deduplicator.deduplicate('key1', requestFn),
        deduplicator.deduplicate('key2', requestFn),
        deduplicator.deduplicate('key3', requestFn),
      ];
      
      await Promise.all(promises);
      
      // Should call the function for each key
      expect(callCount).toBe(3);
    });

    test('should cleanup old requests', async () => {
      const deduplicator = testCache.getDeduplicator();
      
      const stats = deduplicator.getStats();
      expect(stats.pendingRequests).toBe(0);
    });
  });

  describe('Request Batching', () => {
    test('should batch requests within timeout window', async () => {
      const batcher = testCache.getBatcher();
      
      const promises = [
        batcher.batchRequest('batch-key', '/api/test1', 'GET'),
        batcher.batchRequest('batch-key', '/api/test2', 'GET'),
        batcher.batchRequest('batch-key', '/api/test3', 'GET'),
      ];
      
      // Wait for batch timeout
      await new Promise(resolve => setTimeout(resolve, 100));
      
      const stats = batcher.getStats();
      expect(stats.activeBatches).toBe(0);
    });

    test('should execute batch when full', async () => {
      const batcher = testCache.getBatcher();
      
      // Make requests up to max batch size
      const promises = [];
      for (let i = 0; i < 6; i++) {
        promises.push(batcher.batchRequest('batch-key', `/api/test${i}`, 'GET'));
      }
      
      // Wait for execution
      await new Promise(resolve => setTimeout(resolve, 100));
      
      const stats = batcher.getStats();
      expect(stats.activeBatches).toBe(1); // One batch should still be active
    });
  });

  describe('Performance Monitoring', () => {
    test('should track API call performance', () => {
      trackApiCall('/api/test', 150);
      trackApiCall('/api/test', 250);
      trackApiCall('/api/test', 100);
      
      const metrics = performanceMonitor.getMetrics();
      expect(metrics['/api/test']).toBeDefined();
      expect(metrics['/api/test'].count).toBe(3);
      expect(metrics['/api/test'].avg).toBe(166.67);
      expect(metrics['/api/test'].min).toBe(100);
      expect(metrics['/api/test'].max).toBe(250);
    });

    test('should generate performance alerts for slow requests', () => {
      trackApiCall('/api/slow', 1500); // Above slow threshold
      trackApiCall('/api/very-slow', 6000); // Above very slow threshold
      
      const alerts = performanceMonitor.getAlerts();
      expect(alerts.length).toBeGreaterThan(0);
      expect(alerts.some(alert => alert.message.includes('slow response'))).toBe(true);
    });

    test('should calculate performance score', () => {
      trackApiCall('/api/fast', 50);
      trackApiCall('/api/medium', 300);
      
      const score = performanceMonitor.getPerformanceScore();
      expect(score).toBeGreaterThan(0);
      expect(score).toBeLessThanOrEqual(100);
    });

    test('should clear old alerts', () => {
      // Add some old alerts
      performanceMonitor['alerts'] = [
        { type: 'warning', message: 'Old alert', timestamp: Date.now() - 7200000 }, // 2 hours ago
        { type: 'warning', message: 'Recent alert', timestamp: Date.now() - 1800000 }, // 30 minutes ago
      ];
      
      performanceMonitor.clearOldAlerts();
      
      const alerts = performanceMonitor.getAlerts();
      expect(alerts.length).toBe(1); // Only recent alert should remain
    });
  });

  describe('Utility Functions', () => {
    test('should create cache keys correctly', () => {
      const key1 = createCacheKey('/api/test');
      expect(key1).toBe('/api/test:');
      
      const key2 = createCacheKey('/api/test', { page: 1, limit: 10 });
      expect(key2).toBe('/api/test:{"page":1,"limit":10}');
    });

    test('should create user cache keys correctly', () => {
      const key = createUserCacheKey('user123', '/api/profile', { include: 'details' });
      expect(key).toBe('user:user123:/api/profile:{"include":"details"}');
    });
  });

  describe('Singleton Instance', () => {
    test('should return same instance', () => {
      const instance1 = Cache.getInstance();
      const instance2 = Cache.getInstance();
      expect(instance1).toBe(instance2);
    });

    test('should use default configuration', () => {
      const instance = Cache.getInstance();
      const stats = instance.getStats();
      expect(stats.maxSize).toBe(1000); // Default value
    });
  });

  describe('Error Handling', () => {
    test('should handle compression errors gracefully', async () => {
      const largeData = 'x'.repeat(200);
      
      // Mock compression to fail
      const originalCompress = testCache['compressionUtils']?.compress;
      if (originalCompress) {
        testCache['compressionUtils'].compress = jest.fn().mockRejectedValue(new Error('Compression failed'));
      }
      
      await testCache.set('error-key', largeData);
      
      const result = await testCache.get('error-key');
      expect(result).toBe(largeData); // Should store uncompressed
    });

    test('should handle decompression errors gracefully', async () => {
      const testData = 'test-value';
      await testCache.set('test-key', testData);
      
      // Mock decompression to fail
      const originalDecompress = testCache['compressionUtils']?.decompress;
      if (originalDecompress) {
        testCache['compressionUtils'].decompress = jest.fn().mockRejectedValue(new Error('Decompression failed'));
      }
      
      const result = await testCache.get('test-key');
      expect(result).toBeNull(); // Should return null on decompression error
    });
  });
}); 