import { WebSocketEvent, Block, Transaction, Round, NodeMetrics, PricePoint, Trade, OrderBook } from '../types';

export interface WebSocketMessage {
  type: string;
  data: any;
  timestamp: number;
  id?: string;
}

export interface ValidationResult {
  isValid: boolean;
  error?: string;
  data?: any;
}

export class WebSocketMessageParser {
  private static readonly VALID_EVENT_TYPES = [
    'block', 'transaction', 'round', 'metrics', 'price', 'trade', 'orderbook',
    'connection_status', 'error', 'ping', 'pong'
  ];

  static parseMessage(rawMessage: any): ValidationResult {
    try {
      // Basic structure validation
      if (!rawMessage || typeof rawMessage !== 'object') {
        return { isValid: false, error: 'Invalid message format' };
      }

      const { type, data, timestamp, id } = rawMessage;

      // Validate required fields
      if (!type || typeof type !== 'string') {
        return { isValid: false, error: 'Missing or invalid message type' };
      }

      if (!this.VALID_EVENT_TYPES.includes(type)) {
        return { isValid: false, error: `Unknown event type: ${type}` };
      }

      if (timestamp === undefined || typeof timestamp !== 'number') {
        return { isValid: false, error: 'Missing or invalid timestamp' };
      }

      // Validate data based on type
      const dataValidation = this.validateDataByType(type, data);
      if (!dataValidation.isValid) {
        return dataValidation;
      }

      return {
        isValid: true,
        data: {
          type,
          data: dataValidation.data,
          timestamp,
          id
        }
      };
    } catch (error) {
      return {
        isValid: false,
        error: `Message parsing error: ${error instanceof Error ? error.message : 'Unknown error'}`
      };
    }
  }

  private static validateDataByType(type: string, data: any): ValidationResult {
    switch (type) {
      case 'block':
        return this.validateBlockData(data);
      case 'transaction':
        return this.validateTransactionData(data);
      case 'round':
        return this.validateRoundData(data);
      case 'metrics':
        return this.validateMetricsData(data);
      case 'price':
        return this.validatePriceData(data);
      case 'trade':
        return this.validateTradeData(data);
      case 'orderbook':
        return this.validateOrderBookData(data);
      case 'connection_status':
        return this.validateConnectionStatusData(data);
      case 'error':
        return this.validateErrorData(data);
      case 'ping':
      case 'pong':
        return { isValid: true, data: {} };
      default:
        return { isValid: false, error: `Unknown event type: ${type}` };
    }
  }

  private static validateBlockData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Block data is required' };
    }

    const required = ['id', 'number', 'parentIds', 'timestamp', 'transactions', 'validator'];
    for (const field of required) {
      if (!(field in data)) {
        return { isValid: false, error: `Block missing required field: ${field}` };
      }
    }

    if (typeof data.id !== 'string' || data.id.length === 0) {
      return { isValid: false, error: 'Block ID must be a non-empty string' };
    }

    if (typeof data.number !== 'number' || data.number < 0) {
      return { isValid: false, error: 'Block number must be a non-negative number' };
    }

    if (!Array.isArray(data.parentIds)) {
      return { isValid: false, error: 'Block parentIds must be an array' };
    }

    if (!Array.isArray(data.transactions)) {
      return { isValid: false, error: 'Block transactions must be an array' };
    }

    return { isValid: true, data };
  }

  private static validateTransactionData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Transaction data is required' };
    }

    const required = ['id', 'hash', 'from', 'to', 'amount', 'asset', 'timestamp', 'status', 'type'];
    for (const field of required) {
      if (!(field in data)) {
        return { isValid: false, error: `Transaction missing required field: ${field}` };
      }
    }

    if (typeof data.id !== 'string' || data.id.length === 0) {
      return { isValid: false, error: 'Transaction ID must be a non-empty string' };
    }

    if (typeof data.hash !== 'string' || data.hash.length === 0) {
      return { isValid: false, error: 'Transaction hash must be a non-empty string' };
    }

    if (typeof data.amount !== 'number' || data.amount < 0) {
      return { isValid: false, error: 'Transaction amount must be a non-negative number' };
    }

    const validStatuses = ['pending', 'confirmed', 'finalized', 'failed'];
    if (!validStatuses.includes(data.status)) {
      return { isValid: false, error: `Invalid transaction status: ${data.status}` };
    }

    const validTypes = ['transfer', 'buy', 'sell', 'mint', 'burn'];
    if (!validTypes.includes(data.type)) {
      return { isValid: false, error: `Invalid transaction type: ${data.type}` };
    }

    return { isValid: true, data };
  }

  private static validateRoundData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Round data is required' };
    }

    const required = ['number', 'startTime', 'validators', 'finalizedBlocks', 'status'];
    for (const field of required) {
      if (!(field in data)) {
        return { isValid: false, error: `Round missing required field: ${field}` };
      }
    }

    if (typeof data.number !== 'number' || data.number < 0) {
      return { isValid: false, error: 'Round number must be a non-negative number' };
    }

    if (typeof data.startTime !== 'number' || data.startTime < 0) {
      return { isValid: false, error: 'Round startTime must be a non-negative number' };
    }

    if (!Array.isArray(data.validators)) {
      return { isValid: false, error: 'Round validators must be an array' };
    }

    if (!Array.isArray(data.finalizedBlocks)) {
      return { isValid: false, error: 'Round finalizedBlocks must be an array' };
    }

    const validStatuses = ['active', 'finalized', 'failed'];
    if (!validStatuses.includes(data.status)) {
      return { isValid: false, error: `Invalid round status: ${data.status}` };
    }

    return { isValid: true, data };
  }

  private static validateMetricsData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Metrics data is required' };
    }

    const required = ['nodeId', 'uptime', 'tps', 'latency', 'memoryUsage', 'cpuUsage', 'connectedPeers'];
    for (const field of required) {
      if (!(field in data)) {
        return { isValid: false, error: `Metrics missing required field: ${field}` };
      }
    }

    if (typeof data.nodeId !== 'string' || data.nodeId.length === 0) {
      return { isValid: false, error: 'Node ID must be a non-empty string' };
    }

    if (typeof data.uptime !== 'number' || data.uptime < 0) {
      return { isValid: false, error: 'Uptime must be a non-negative number' };
    }

    if (typeof data.tps !== 'number' || data.tps < 0) {
      return { isValid: false, error: 'TPS must be a non-negative number' };
    }

    if (typeof data.latency !== 'number' || data.latency < 0) {
      return { isValid: false, error: 'Latency must be a non-negative number' };
    }

    if (typeof data.cpuUsage !== 'number' || data.cpuUsage < 0 || data.cpuUsage > 1) {
      return { isValid: false, error: 'CPU usage must be between 0 and 1' };
    }

    return { isValid: true, data };
  }

  private static validatePriceData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Price data is required' };
    }

    const required = ['timestamp', 'price', 'volume', 'high', 'low', 'open', 'close'];
    for (const field of required) {
      if (!(field in data)) {
        return { isValid: false, error: `Price data missing required field: ${field}` };
      }
    }

    if (typeof data.timestamp !== 'number' || data.timestamp < 0) {
      return { isValid: false, error: 'Price timestamp must be a non-negative number' };
    }

    if (typeof data.price !== 'number' || data.price < 0) {
      return { isValid: false, error: 'Price must be a non-negative number' };
    }

    if (typeof data.volume !== 'number' || data.volume < 0) {
      return { isValid: false, error: 'Volume must be a non-negative number' };
    }

    return { isValid: true, data };
  }

  private static validateTradeData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Trade data is required' };
    }

    const required = ['id', 'pair', 'price', 'amount', 'side', 'timestamp'];
    for (const field of required) {
      if (!(field in data)) {
        return { isValid: false, error: `Trade missing required field: ${field}` };
      }
    }

    if (typeof data.id !== 'string' || data.id.length === 0) {
      return { isValid: false, error: 'Trade ID must be a non-empty string' };
    }

    if (typeof data.pair !== 'string' || data.pair.length === 0) {
      return { isValid: false, error: 'Trade pair must be a non-empty string' };
    }

    if (typeof data.price !== 'number' || data.price < 0) {
      return { isValid: false, error: 'Trade price must be a non-negative number' };
    }

    if (typeof data.amount !== 'number' || data.amount < 0) {
      return { isValid: false, error: 'Trade amount must be a non-negative number' };
    }

    if (!['buy', 'sell'].includes(data.side)) {
      return { isValid: false, error: `Invalid trade side: ${data.side}` };
    }

    return { isValid: true, data };
  }

  private static validateOrderBookData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Order book data is required' };
    }

    const required = ['pair', 'bids', 'asks', 'lastUpdateId'];
    for (const field of required) {
      if (!(field in data)) {
        return { isValid: false, error: `Order book missing required field: ${field}` };
      }
    }

    if (typeof data.pair !== 'string' || data.pair.length === 0) {
      return { isValid: false, error: 'Order book pair must be a non-empty string' };
    }

    if (!Array.isArray(data.bids)) {
      return { isValid: false, error: 'Order book bids must be an array' };
    }

    if (!Array.isArray(data.asks)) {
      return { isValid: false, error: 'Order book asks must be an array' };
    }

    if (typeof data.lastUpdateId !== 'number' || data.lastUpdateId < 0) {
      return { isValid: false, error: 'Last update ID must be a non-negative number' };
    }

    return { isValid: true, data };
  }

  private static validateConnectionStatusData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Connection status data is required' };
    }

    if (!('status' in data)) {
      return { isValid: false, error: 'Connection status missing status field' };
    }

    const validStatuses = ['connected', 'disconnected', 'connecting', 'error', 'failed'];
    if (!validStatuses.includes(data.status)) {
      return { isValid: false, error: `Invalid connection status: ${data.status}` };
    }

    return { isValid: true, data };
  }

  private static validateErrorData(data: any): ValidationResult {
    if (!data || typeof data !== 'object') {
      return { isValid: false, error: 'Error data is required' };
    }

    if (!('message' in data) || typeof data.message !== 'string') {
      return { isValid: false, error: 'Error data must have a message field' };
    }

    return { isValid: true, data };
  }
}

export class WebSocketErrorHandler {
  static handleError(error: any, context: string): WebSocketEvent {
    const errorMessage = this.formatError(error, context);
    console.error(`WebSocket Error [${context}]:`, errorMessage);
    
    // Emit error event for UI handling
    const errorEvent: WebSocketEvent = {
      type: 'error',
      data: {
        message: errorMessage,
        context,
        timestamp: Date.now(),
        originalError: error
      },
      timestamp: Date.now()
    };

    // This would be handled by the API class
    return errorEvent;
  }

  private static formatError(error: any, context: string): string {
    if (error instanceof Error) {
      return `${context}: ${error.message}`;
    }
    
    if (typeof error === 'string') {
      return `${context}: ${error}`;
    }
    
    if (error && typeof error === 'object' && error.message) {
      return `${context}: ${error.message}`;
    }
    
    return `${context}: Unknown error occurred`;
  }
}

export class WebSocketReconnectionManager {
  private static readonly MAX_RECONNECT_ATTEMPTS = 10;
  private static readonly BASE_DELAY = 1000;
  private static readonly MAX_DELAY = 30000;

  static calculateDelay(attempt: number): number {
    const delay = this.BASE_DELAY * Math.pow(2, attempt - 1);
    return Math.min(delay, this.MAX_DELAY);
  }

  static shouldAttemptReconnect(attempt: number, reason?: string): boolean {
    if (attempt >= this.MAX_RECONNECT_ATTEMPTS) {
      return false;
    }

    // Don't reconnect on manual disconnect
    if (reason === 'io client disconnect') {
      return false;
    }

    return true;
  }

  static getReconnectMessage(attempt: number, maxAttempts: number): string {
    if (attempt >= maxAttempts) {
      return `Max reconnection attempts (${maxAttempts}) reached. Please check your connection and try again.`;
    }
    
    return `Reconnection attempt ${attempt}/${maxAttempts}`;
  }
} 