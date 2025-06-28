import axios, { AxiosInstance } from 'axios';
import { io, Socket } from 'socket.io-client';
import {
  Block,
  Transaction,
  Validator,
  Round,
  NodeMetrics,
  NetworkMetrics,
  Asset,
  PricePoint,
  TradingPair,
  Trade,
  OrderBook,
  MarketOrder,
  ApiResponse,
  PaginatedResponse,
  WebSocketEvent,
  BlockEvent,
  TransactionEvent,
  RoundEvent,
  MetricsEvent,
  PriceEvent,
  TradeEvent,
  OrderBookEvent,
} from '../types';

class FinDAGApi {
  private http: AxiosInstance;
  private ws: Socket | null = null;
  private eventListeners: Map<string, ((event: WebSocketEvent) => void)[]> = new Map();

  constructor(baseURL: string = 'http://localhost:8080') {
    this.http = axios.create({
      baseURL: `${baseURL}/api`,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add response interceptor for error handling
    this.http.interceptors.response.use(
      (response) => response,
      (error) => {
        console.error('API Error:', error);
        return Promise.reject(error);
      }
    );
  }

  // HTTP API Methods - Core Blockchain
  async getNetworkMetrics(): Promise<NetworkMetrics> {
    const response = await this.http.get<ApiResponse<NetworkMetrics>>('/metrics/network');
    return response.data.data!;
  }

  async getNodeMetrics(nodeId?: string): Promise<NodeMetrics[]> {
    const url = nodeId ? `/metrics/node/${nodeId}` : '/metrics/nodes';
    const response = await this.http.get<ApiResponse<NodeMetrics[]>>(url);
    return response.data.data!;
  }

  async getBlocks(page: number = 1, limit: number = 50): Promise<PaginatedResponse<Block>> {
    const response = await this.http.get<ApiResponse<PaginatedResponse<Block>>>('/blocks', {
      params: { page, limit },
    });
    return response.data.data!;
  }

  async getBlock(blockId: string): Promise<Block> {
    const response = await this.http.get<ApiResponse<Block>>(`/blocks/${blockId}`);
    return response.data.data!;
  }

  async getTransactions(page: number = 1, limit: number = 50): Promise<PaginatedResponse<Transaction>> {
    const response = await this.http.get<ApiResponse<PaginatedResponse<Transaction>>>('/transactions', {
      params: { page, limit },
    });
    return response.data.data!;
  }

  async getTransaction(txId: string): Promise<Transaction> {
    const response = await this.http.get<ApiResponse<Transaction>>(`/transactions/${txId}`);
    return response.data.data!;
  }

  async getValidators(): Promise<Validator[]> {
    const response = await this.http.get<ApiResponse<Validator[]>>('/validators');
    return response.data.data!;
  }

  async getRounds(page: number = 1, limit: number = 20): Promise<PaginatedResponse<Round>> {
    const response = await this.http.get<ApiResponse<PaginatedResponse<Round>>>('/rounds', {
      params: { page, limit },
    });
    return response.data.data!;
  }

  async getCurrentRound(): Promise<Round> {
    const response = await this.http.get<ApiResponse<Round>>('/rounds/current');
    return response.data.data!;
  }

  async submitTransaction(transaction: Omit<Transaction, 'id' | 'timestamp' | 'status'>): Promise<Transaction> {
    const response = await this.http.post<ApiResponse<Transaction>>('/transactions', transaction);
    return response.data.data!;
  }

  // HTTP API Methods - Trading & Assets
  async getAssets(): Promise<Asset[]> {
    const response = await this.http.get<ApiResponse<Asset[]>>('/assets');
    return response.data.data!;
  }

  async getAsset(assetId: string): Promise<Asset> {
    const response = await this.http.get<ApiResponse<Asset>>(`/assets/${assetId}`);
    return response.data.data!;
  }

  async getTradingPairs(): Promise<TradingPair[]> {
    const response = await this.http.get<ApiResponse<TradingPair[]>>('/trading/pairs');
    return response.data.data!;
  }

  async getTradingPair(pair: string): Promise<TradingPair> {
    const response = await this.http.get<ApiResponse<TradingPair>>(`/trading/pairs/${pair}`);
    return response.data.data!;
  }

  async getPriceHistory(
    pair: string,
    timeFrame: string = '1h',
    limit: number = 1000
  ): Promise<PricePoint[]> {
    const response = await this.http.get<ApiResponse<PricePoint[]>>(`/trading/price-history/${pair}`, {
      params: { timeFrame, limit },
    });
    return response.data.data!;
  }

  async getRecentTrades(pair: string, limit: number = 100): Promise<Trade[]> {
    const response = await this.http.get<ApiResponse<Trade[]>>(`/trading/trades/${pair}`, {
      params: { limit },
    });
    return response.data.data!;
  }

  async getOrderBook(pair: string, depth: number = 20): Promise<OrderBook> {
    const response = await this.http.get<ApiResponse<OrderBook>>(`/trading/orderbook/${pair}`, {
      params: { depth },
    });
    return response.data.data!;
  }

  async getUserOrders(userId: string, status?: string): Promise<MarketOrder[]> {
    const response = await this.http.get<ApiResponse<MarketOrder[]>>(`/trading/orders/${userId}`, {
      params: { status },
    });
    return response.data.data!;
  }

  async getUserBalance(userId: string): Promise<{ [asset: string]: number }> {
    const response = await this.http.get<ApiResponse<{ [asset: string]: number }>>(`/trading/balance/${userId}`);
    return response.data.data!;
  }

  async placeOrder(order: Omit<MarketOrder, 'id' | 'timestamp' | 'status' | 'filledAmount' | 'averagePrice'>): Promise<MarketOrder> {
    const response = await this.http.post<ApiResponse<MarketOrder>>('/trading/orders', order);
    return response.data.data!;
  }

  async cancelOrder(orderId: string): Promise<{ success: boolean }> {
    const response = await this.http.delete<ApiResponse<{ success: boolean }>>(`/trading/orders/${orderId}`);
    return response.data.data!;
  }

  // WebSocket Methods
  connectWebSocket(baseURL: string = 'ws://localhost:8080'): void {
    if (this.ws) {
      this.ws.disconnect();
    }

    this.ws = io(baseURL, {
      transports: ['websocket'],
      autoConnect: true,
    });

    this.ws.on('connect', () => {
      console.log('WebSocket connected');
    });

    this.ws.on('disconnect', () => {
      console.log('WebSocket disconnected');
    });

    this.ws.on('error', (error) => {
      console.error('WebSocket error:', error);
    });

    // Handle different event types
    this.ws.on('block', (data: Block) => {
      this.emitEvent('block', { type: 'block', data, timestamp: Date.now() });
    });

    this.ws.on('transaction', (data: Transaction) => {
      this.emitEvent('transaction', { type: 'transaction', data, timestamp: Date.now() });
    });

    this.ws.on('round', (data: Round) => {
      this.emitEvent('round', { type: 'round', data, timestamp: Date.now() });
    });

    this.ws.on('metrics', (data: NodeMetrics) => {
      this.emitEvent('metrics', { type: 'metrics', data, timestamp: Date.now() });
    });

    // Trading events
    this.ws.on('price', (data: PricePoint) => {
      this.emitEvent('price', { type: 'price', data, timestamp: Date.now() });
    });

    this.ws.on('trade', (data: Trade) => {
      this.emitEvent('trade', { type: 'trade', data, timestamp: Date.now() });
    });

    this.ws.on('orderbook', (data: OrderBook) => {
      this.emitEvent('orderbook', { type: 'orderbook', data, timestamp: Date.now() });
    });
  }

  // Subscribe to specific trading pairs for real-time updates
  subscribeToPair(pair: string): void {
    if (this.ws && this.ws.connected) {
      this.ws.emit('subscribe', { channel: 'trading', pair });
    }
  }

  unsubscribeFromPair(pair: string): void {
    if (this.ws && this.ws.connected) {
      this.ws.emit('unsubscribe', { channel: 'trading', pair });
    }
  }

  disconnectWebSocket(): void {
    if (this.ws) {
      this.ws.disconnect();
      this.ws = null;
    }
  }

  // Event Listener Management
  addEventListener(eventType: string, callback: (event: WebSocketEvent) => void): void {
    if (!this.eventListeners.has(eventType)) {
      this.eventListeners.set(eventType, []);
    }
    this.eventListeners.get(eventType)!.push(callback);
  }

  removeEventListener(eventType: string, callback: (event: WebSocketEvent) => void): void {
    const listeners = this.eventListeners.get(eventType);
    if (listeners) {
      const index = listeners.indexOf(callback);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    }
  }

  private emitEvent(eventType: string, event: WebSocketEvent): void {
    const listeners = this.eventListeners.get(eventType);
    if (listeners) {
      listeners.forEach(callback => callback(event));
    }
  }

  // Utility Methods
  isConnected(): boolean {
    return this.ws?.connected || false;
  }

  getConnectionStatus(): 'connected' | 'disconnected' | 'connecting' {
    if (!this.ws) return 'disconnected';
    if (this.ws.connected) return 'connected';
    return 'connecting';
  }
}

// Export singleton instance
export const finDAGApi = new FinDAGApi();
export default finDAGApi; 