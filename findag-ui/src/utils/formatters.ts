/**
 * Format a number with appropriate suffixes (K, M, B)
 */
export const formatNumber = (num: number): string => {
  if (num >= 1e9) {
    return (num / 1e9).toFixed(1) + 'B';
  }
  if (num >= 1e6) {
    return (num / 1e6).toFixed(1) + 'M';
  }
  if (num >= 1e3) {
    return (num / 1e3).toFixed(1) + 'K';
  }
  return num.toFixed(0);
};

/**
 * Format price with appropriate decimal places
 */
export const formatPrice = (price: number, decimals: number = 6): string => {
  if (price === 0) return '0.00';
  
  // For very small prices, show more decimals
  if (price < 0.0001) {
    return price.toExponential(2);
  }
  
  // For regular prices, show appropriate decimals
  if (price < 1) {
    return price.toFixed(6);
  }
  if (price < 10) {
    return price.toFixed(4);
  }
  if (price < 100) {
    return price.toFixed(3);
  }
  if (price < 1000) {
    return price.toFixed(2);
  }
  
  // For large prices, use number formatting
  return price.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
};

/**
 * Format latency in milliseconds with appropriate units
 */
export const formatLatency = (latencyMs: number): string => {
  if (latencyMs < 1) {
    return `${(latencyMs * 1000).toFixed(0)}μs`;
  }
  if (latencyMs < 1000) {
    return `${latencyMs.toFixed(1)}ms`;
  }
  return `${(latencyMs / 1000).toFixed(2)}s`;
};

/**
 * Format a timestamp for display
 */
export const formatTimestamp = (timestamp: number): string => {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  
  // If less than 1 minute ago
  if (diffMs < 60000) {
    return 'Just now';
  }
  
  // If less than 1 hour ago
  if (diffMs < 3600000) {
    const minutes = Math.floor(diffMs / 60000);
    return `${minutes}m ago`;
  }
  
  // If less than 24 hours ago
  if (diffMs < 86400000) {
    const hours = Math.floor(diffMs / 3600000);
    return `${hours}h ago`;
  }
  
  // If less than 7 days ago
  if (diffMs < 604800000) {
    const days = Math.floor(diffMs / 86400000);
    return `${days}d ago`;
  }
  
  // Otherwise show full date
  return date.toLocaleDateString();
};

/**
 * Format file size in bytes with appropriate units
 */
export const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

/**
 * Format percentage with 2 decimal places
 */
export const formatPercentage = (value: number, total?: number): string => {
  if (total !== undefined && total === 0) return '0%';
  if (total !== undefined) {
    return `${((value / total) * 100).toFixed(2)}%`;
  }
  return `${value.toFixed(2)}%`;
};

/**
 * Format currency values
 */
export const formatCurrency = (amount: number, currency: string = 'USD'): string => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 6,
  }).format(amount);
};

/**
 * Format an address for display (truncate middle)
 */
export const formatAddress = (address: string, startChars: number = 6, endChars: number = 4): string => {
  if (address.length <= startChars + endChars) {
    return address;
  }
  return `${address.slice(0, startChars)}...${address.slice(-endChars)}`;
};

/**
 * Format a hash for display (truncate middle)
 */
export const formatHash = (hash: string, startChars: number = 8, endChars: number = 8): string => {
  if (hash.length <= startChars + endChars) {
    return hash;
  }
  return `${hash.slice(0, startChars)}...${hash.slice(-endChars)}`;
};

/**
 * Format memory usage in bytes to human readable format
 */
export const formatMemoryUsage = (bytes: number): string => {
  return formatFileSize(bytes);
};

/**
 * Format CPU usage as percentage
 */
export const formatCpuUsage = (usage: number): string => {
  return `${(usage * 100).toFixed(1)}%`;
};

/**
 * Format uptime in seconds to human readable format
 */
export const formatUptime = (seconds: number): string => {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  
  if (days > 0) {
    return `${days}d ${hours}h ${minutes}m`;
  }
  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  }
  return `${minutes}m`;
};

/**
 * Format a duration in milliseconds to human readable format
 */
export const formatDuration = (ms: number): string => {
  const seconds = Math.floor(ms / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);
  
  if (days > 0) {
    return `${days}d ${hours % 24}h ${minutes % 60}m`;
  }
  if (hours > 0) {
    return `${hours}h ${minutes % 60}m ${seconds % 60}s`;
  }
  if (minutes > 0) {
    return `${minutes}m ${seconds % 60}s`;
  }
  return `${seconds}s`;
}; 