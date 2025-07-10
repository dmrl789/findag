// Comprehensive caching system for performance optimization

interface CacheEntry<T> {
  data: T;
  timestamp: number;
  ttl: number;
  accessCount: number;
  lastAccessed: number;
  compressed?: boolean;
  size?: number;
}

interface CacheConfig {
  maxSize: number;
  defaultTTL: number;
  cleanupInterval: number;
  compressionEnabled: boolean;
  compressionThreshold: number; // Only compress data larger than this size
  batchTimeout: number;
  maxBatchSize: number;
}

interface RequestCache {
  [key: string]: CacheEntry<any>;
}

interface BatchRequest {
  id: string;
  requests: Array<{
    url: string;
    method: string;
    data?: any;
    resolve: (value: any) => void;
    reject: (error: any) => void;
  }>;
  timeout: number;
  createdAt: number;
}

// Cache configuration
const DEFAULT_CONFIG: CacheConfig = {
  maxSize: 1000,
  defaultTTL: 5 * 60 * 1000, // 5 minutes
  cleanupInterval: 60 * 1000, // 1 minute
  compressionEnabled: true,
  compressionThreshold: 1024, // 1KB
  batchTimeout: 50, // 50ms batch window
  maxBatchSize: 10
};

// Enhanced compression utilities
class CompressionUtils {
  static async compress(data: any): Promise<string> {
    if (typeof data === 'string') {
      return this.compressString(data);
    }
    
    const jsonString = JSON.stringify(data);
    return this.compressString(jsonString);
  }

  static async compressString(str: string): Promise<string> {
    // Use TextEncoder and compression
    const encoder = new TextEncoder();
    const data = encoder.encode(str);
    
    // Simple compression for demo - in production use proper compression
    if (data.length < 1024) {
      return str; // Don't compress small data
    }
    
    // For demo purposes, use a simple compression
    // In production, use proper compression libraries
    return btoa(str).substring(0, Math.min(str.length, str.length * 0.8));
  }

  static async decompress(compressed: string): Promise<any> {
    try {
      // Try to parse as JSON first
      return JSON.parse(compressed);
    } catch {
      // If not JSON, try to decompress
      return this.decompressString(compressed);
    }
  }

  static async decompressString(compressed: string): Promise<string> {
    try {
      // Simple decompression for demo
      return atob(compressed);
    } catch {
      return compressed; // Return as-is if decompression fails
    }
  }

  static getDataSize(data: any): number {
    if (typeof data === 'string') {
      return new Blob([data]).size;
    }
    return new Blob([JSON.stringify(data)]).size;
  }
}

// Enhanced request deduplication
class RequestDeduplicator {
  private pendingRequests: Map<string, Promise<any>> = new Map();
  private requestTimestamps: Map<string, number> = new Map();
  private maxRequestAge = 30000; // 30 seconds

  async deduplicate<T>(
    key: string,
    requestFn: () => Promise<T>
  ): Promise<T> {
    // Clean up old requests
    this.cleanupOldRequests();

    if (this.pendingRequests.has(key)) {
      return this.pendingRequests.get(key)!;
    }

    const promise = requestFn();
    this.pendingRequests.set(key, promise);
    this.requestTimestamps.set(key, Date.now());

    try {
      const result = await promise;
      return result;
    } finally {
      this.pendingRequests.delete(key);
      this.requestTimestamps.delete(key);
    }
  }

  private cleanupOldRequests(): void {
    const now = Date.now();
    for (const [key, timestamp] of this.requestTimestamps.entries()) {
      if (now - timestamp > this.maxRequestAge) {
        this.pendingRequests.delete(key);
        this.requestTimestamps.delete(key);
      }
    }
  }

  clear(): void {
    this.pendingRequests.clear();
    this.requestTimestamps.clear();
  }

  getStats(): { pendingRequests: number; oldestRequest: number } {
    const timestamps = Array.from(this.requestTimestamps.values());
    return {
      pendingRequests: this.pendingRequests.size,
      oldestRequest: timestamps.length > 0 ? Math.min(...timestamps) : 0
    };
  }
}

// Enhanced request batching
class RequestBatcher {
  private batches: Map<string, BatchRequest> = new Map();
  private config: CacheConfig;

  constructor(config: CacheConfig) {
    this.config = config;
  }

  async batchRequest<T>(
    batchKey: string,
    url: string,
    method: string,
    data?: any
  ): Promise<T> {
    return new Promise((resolve, reject) => {
      const request = {
        url,
        method,
        data,
        resolve,
        reject
      };

      if (!this.batches.has(batchKey)) {
        const batch: BatchRequest = {
          id: batchKey,
          requests: [request],
          timeout: window.setTimeout(() => this.executeBatch(batchKey), this.config.batchTimeout) as unknown as number,
          createdAt: Date.now()
        };
        this.batches.set(batchKey, batch);
      } else {
        const batch = this.batches.get(batchKey)!;
        
        // Check if batch is full
        if (batch.requests.length >= this.config.maxBatchSize) {
          // Execute current batch immediately
          clearTimeout(batch.timeout);
          this.executeBatch(batchKey);
          
          // Create new batch
          const newBatch: BatchRequest = {
            id: batchKey,
            requests: [request],
            timeout: window.setTimeout(() => this.executeBatch(batchKey), this.config.batchTimeout) as unknown as number,
            createdAt: Date.now()
          };
          this.batches.set(batchKey, newBatch);
        } else {
          batch.requests.push(request);
        }
      }
    });
  }

  private async executeBatch(batchKey: string): Promise<void> {
    const batch = this.batches.get(batchKey);
    if (!batch) return;

    this.batches.delete(batchKey);

    try {
      // Execute batch request (implementation depends on backend support)
      const results = await this.executeBatchRequest(batch.requests);
      
      // Distribute results to individual requests
      batch.requests.forEach((request, index) => {
        request.resolve(results[index]);
      });
    } catch (error) {
      batch.requests.forEach(request => {
        request.reject(error);
      });
    }
  }

  private async executeBatchRequest(requests: any[]): Promise<any[]> {
    // This would be implemented based on backend batch endpoint
    // For now, execute requests individually with improved error handling
    const results = await Promise.allSettled(
      requests.map(request => 
        fetch(request.url, {
          method: request.method,
          body: request.data ? JSON.stringify(request.data) : undefined,
          headers: {
            'Content-Type': 'application/json'
          }
        }).then(res => {
          if (!res.ok) {
            throw new Error(`HTTP ${res.status}: ${res.statusText}`);
          }
          return res.json();
        })
      )
    );

    return results.map((result, index) => {
      if (result.status === 'fulfilled') {
        return result.value;
      } else {
        throw result.reason;
      }
    });
  }

  clear(): void {
    this.batches.forEach(batch => {
      clearTimeout(batch.timeout);
    });
    this.batches.clear();
  }

  getStats(): { activeBatches: number; totalRequests: number } {
    let totalRequests = 0;
    this.batches.forEach(batch => {
      totalRequests += batch.requests.length;
    });

    return {
      activeBatches: this.batches.size,
      totalRequests
    };
  }
}

// Enhanced performance monitoring
class PerformanceMonitor {
  private metrics: Map<string, { avg: number; min: number; max: number; count: number; total: number }> = new Map();
  private alerts: Array<{ type: string; message: string; timestamp: number }> = [];
  private thresholds = {
    slowRequest: 1000, // 1 second
    verySlowRequest: 5000, // 5 seconds
    errorRate: 0.1, // 10%
    cacheHitRate: 0.5 // 50%
  };

  trackApiCall(endpoint: string, duration: number): void {
    const existing = this.metrics.get(endpoint);
    
    if (existing) {
      existing.count++;
      existing.total += duration;
      existing.avg = existing.total / existing.count;
      existing.min = Math.min(existing.min, duration);
      existing.max = Math.max(existing.max, duration);
    } else {
      this.metrics.set(endpoint, {
        avg: duration,
        min: duration,
        max: duration,
        count: 1,
        total: duration
      });
    }

    // Check for performance alerts
    this.checkPerformanceAlerts(endpoint, duration);
  }

  private checkPerformanceAlerts(endpoint: string, duration: number): void {
    if (duration > this.thresholds.verySlowRequest) {
      this.addAlert('error', `${endpoint} took ${duration}ms - very slow response`);
    } else if (duration > this.thresholds.slowRequest) {
      this.addAlert('warning', `${endpoint} took ${duration}ms - slow response`);
    }
  }

  private addAlert(type: string, message: string): void {
    this.alerts.push({
      type,
      message,
      timestamp: Date.now()
    });

    // Keep only last 100 alerts
    if (this.alerts.length > 100) {
      this.alerts = this.alerts.slice(-100);
    }
  }

  getMetrics(): { [endpoint: string]: { avg: number; min: number; max: number; count: number } } {
    const result: { [endpoint: string]: { avg: number; min: number; max: number; count: number } } = {};
    
    this.metrics.forEach((value, key) => {
      result[key] = {
        avg: value.avg,
        min: value.min,
        max: value.max,
        count: value.count
      };
    });

    return result;
  }

  getAlerts(): Array<{ type: string; message: string; timestamp: number }> {
    return [...this.alerts];
  }

  clearOldAlerts(): void {
    const oneHourAgo = Date.now() - (60 * 60 * 1000);
    this.alerts = this.alerts.filter(alert => alert.timestamp > oneHourAgo);
  }

  clearMetrics(): void {
    this.metrics.clear();
    this.alerts = [];
  }

  getPerformanceScore(): number {
    if (this.metrics.size === 0) return 100;

    let totalScore = 0;
    let endpointCount = 0;

    this.metrics.forEach((metric) => {
      let score = 100;
      
      // Deduct points for slow average response times
      if (metric.avg > 1000) score -= 30;
      else if (metric.avg > 500) score -= 15;
      else if (metric.avg > 200) score -= 5;

      // Deduct points for high max response times
      if (metric.max > 5000) score -= 20;
      else if (metric.max > 2000) score -= 10;

      totalScore += score;
      endpointCount++;
    });

    return Math.max(0, Math.min(100, totalScore / endpointCount));
  }
}

// Main cache class
export class Cache {
  private static instance: Cache;
  private cache: RequestCache = {};
  private config: CacheConfig;
  private deduplicator: RequestDeduplicator;
  private batcher: RequestBatcher;
  private performanceMonitor: PerformanceMonitor;
  private cleanupTimer: number | null = null;

  constructor(config: Partial<CacheConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.deduplicator = new RequestDeduplicator();
    this.batcher = new RequestBatcher(this.config);
    this.performanceMonitor = new PerformanceMonitor();
    this.startCleanup();
  }

  static getInstance(config?: Partial<CacheConfig>): Cache {
    if (!Cache.instance) {
      Cache.instance = new Cache(config);
    }
    return Cache.instance;
  }

  // Set cache entry with compression
  async set<T>(key: string, data: T, ttl?: number): Promise<void> {
    const dataSize = CompressionUtils.getDataSize(data);
    let processedData: any = data;
    let compressed = false;

    // Compress if enabled and data is large enough
    if (this.config.compressionEnabled && dataSize > this.config.compressionThreshold) {
      try {
        processedData = await CompressionUtils.compress(JSON.stringify(data)); // always a string
        compressed = true;
      } catch (error) {
        console.warn('Compression failed, storing uncompressed data:', error);
      }
    }

    const entry: CacheEntry<T> = {
      data: processedData,
      timestamp: Date.now(),
      ttl: ttl || this.config.defaultTTL,
      accessCount: 0,
      lastAccessed: Date.now(),
      compressed,
      size: dataSize
    };

    // Check cache size limit
    if (Object.keys(this.cache).length >= this.config.maxSize) {
      this.evictLRU();
    }

    this.cache[key] = entry;
  }

  // Get cache entry with decompression
  async get<T>(key: string): Promise<T | null> {
    const entry = this.cache[key] as CacheEntry<T>;
    
    if (!entry) return null;

    // Check if expired
    if (Date.now() - entry.timestamp > entry.ttl) {
      delete this.cache[key];
      return null;
    }

    // Update access statistics
    entry.accessCount++;
    entry.lastAccessed = Date.now();

    // Decompress if needed
    if (entry.compressed) {
      try {
        const decompressed = await CompressionUtils.decompress(entry.data as string);
        return JSON.parse(decompressed) as T;
      } catch (error) {
        console.warn('Decompression failed:', error);
        delete this.cache[key];
        return null;
      }
    }

    return entry.data;
  }

  has(key: string): boolean {
    const entry = this.cache[key];
    if (!entry) return false;

    // Check if expired
    if (Date.now() - entry.timestamp > entry.ttl) {
      delete this.cache[key];
      return false;
    }

    return true;
  }

  delete(key: string): boolean {
    if (this.cache[key]) {
      delete this.cache[key];
      return true;
    }
    return false;
  }

  clear(): void {
    this.cache = {};
    this.deduplicator.clear();
    this.batcher.clear();
  }

  getStats(): {
    size: number;
    maxSize: number;
    hitRate: number;
    totalHits: number;
    totalMisses: number;
    compressedEntries: number;
    totalSize: number;
    performanceScore: number;
  } {
    let totalHits = 0;
    let totalMisses = 0;
    let compressedEntries = 0;
    let totalSize = 0;

    Object.values(this.cache).forEach(entry => {
      totalHits += entry.accessCount;
      if (entry.compressed) compressedEntries++;
      if (entry.size) totalSize += entry.size;
    });

    const hitRate = totalHits + totalMisses > 0 ? totalHits / (totalHits + totalMisses) : 0;

    return {
      size: Object.keys(this.cache).length,
      maxSize: this.config.maxSize,
      hitRate,
      totalHits,
      totalMisses,
      compressedEntries,
      totalSize,
      performanceScore: this.performanceMonitor.getPerformanceScore()
    };
  }

  private evictLRU(): void {
    let oldestKey: string | null = null;
    let oldestTime = Date.now();

    for (const [key, entry] of Object.entries(this.cache)) {
      if (entry.lastAccessed < oldestTime) {
        oldestTime = entry.lastAccessed;
        oldestKey = key;
      }
    }

    if (oldestKey) {
      delete this.cache[oldestKey];
    }
  }

  private cleanup(): void {
    const now = Date.now();
    const keysToDelete: string[] = [];

    for (const [key, entry] of Object.entries(this.cache)) {
      if (now - entry.timestamp > entry.ttl) {
        keysToDelete.push(key);
      }
    }

    keysToDelete.forEach(key => delete this.cache[key]);
  }

  private startCleanup(): void {
    this.cleanupTimer = window.setInterval(() => {
      this.cleanup();
    }, this.config.cleanupInterval);
  }

  stopCleanup(): void {
    if (this.cleanupTimer) {
      clearInterval(this.cleanupTimer);
      this.cleanupTimer = null;
    }
  }

  getDeduplicator(): RequestDeduplicator {
    return this.deduplicator;
  }

  getBatcher(): RequestBatcher {
    return this.batcher;
  }

  getPerformanceMonitor(): PerformanceMonitor {
    return this.performanceMonitor;
  }
}

// Export singleton instances
export const cache = Cache.getInstance();
export const performanceMonitor = cache.getPerformanceMonitor();

// Utility functions
export const createCacheKey = (endpoint: string, params?: Record<string, any>): string => {
  const paramString = params ? JSON.stringify(params) : '';
  return `${endpoint}:${paramString}`;
};

export const createUserCacheKey = (userId: string, endpoint: string, params?: Record<string, any>): string => {
  const paramString = params ? JSON.stringify(params) : '';
  return `user:${userId}:${endpoint}:${paramString}`;
};

// Enhanced performance tracking
export const trackApiCall = (endpoint: string, duration: number): void => {
  performanceMonitor.trackApiCall(endpoint, duration);
};

// Performance monitoring utilities
export const getPerformanceMetrics = () => performanceMonitor.getMetrics();
export const getPerformanceAlerts = () => performanceMonitor.getAlerts();
export const getPerformanceScore = () => performanceMonitor.getPerformanceScore();
export const clearPerformanceData = () => performanceMonitor.clearMetrics(); 