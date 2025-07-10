import { cache } from '../cache';

describe('Simple Cache Test', () => {
  beforeEach(() => {
    cache.clear();
  });

  test('should set and get cache values', () => {
    const key = 'test-key';
    const value = { data: 'test-value' };
    
    cache.set(key, value, 5000);
    const result = cache.get(key);
    
    expect(result).toEqual(value);
  });

  test('should return null for non-existent keys', () => {
    const result = cache.get('non-existent');
    expect(result).toBeNull();
  });

  test('should clear all cache entries', () => {
    cache.set('key1', 'value1', 5000);
    cache.set('key2', 'value2', 5000);
    
    expect(cache.get('key1')).toBe('value1');
    expect(cache.get('key2')).toBe('value2');
    
    cache.clear();
    
    expect(cache.get('key1')).toBeNull();
    expect(cache.get('key2')).toBeNull();
  });
}); 