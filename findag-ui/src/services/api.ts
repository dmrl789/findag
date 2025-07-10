import axios, { AxiosInstance, AxiosError } from 'axios';
import { io, Socket } from 'socket.io-client';
import { errorHandler, ErrorType } from '../utils/errorHandler';
import { SecurityHeaders, CSRFProtection, RateLimiter } from '../utils/security';
import { cache } from '../utils/cache';
import { withPerformanceTracking } from '../components/Common/PerformanceMonitor';
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

// API Configuration
const API_CONFIG = {
  baseURL: (import.meta as any).env?.VITE_API_URL || 'http://localhost:8080',
  timeout: 10000,
  retryAttempts: 3,
  retryDelay: 1000,
};

// Error types
export interface ApiError {
  message: string;
  status?: number;
  code?: string;
  details?: any;
}

class FinDAGApi {
  private http: AxiosInstance;
  private ws: Socket | null = null;
  private eventListeners: Map<string, ((event: WebSocketEvent) => void)[]> = new Map();
  private authToken: string | null = null;
  
  // Enhanced WebSocket management
  private reconnectAttempts: number = 0;
  private maxReconnectAttempts: number = 10;
  private reconnectDelay: number = 1000; // Start with 1 second
  private maxReconnectDelay: number = 30000; // Max 30 seconds
  private reconnectTimer: number | null = null;
  private connectionCheckInterval: number | null = null;
  private isManualDisconnect: boolean = false;
  private pendingSubscriptions: Set<string> = new Set();

  constructor() {
    this.http = axios.create({
      baseURL: API_CONFIG.baseURL,
      timeout: API_CONFIG.timeout,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add request interceptor for authentication
    this.http.interceptors.request.use(
      (config) => {
        if (this.authToken) {
          config.headers.Authorization = `Bearer ${this.authToken}`;
        }
        return config;
      },
      (error) => {
        return Promise.reject(error);
      }
    );

    // Add response interceptor for error handling and retry logic
    this.http.interceptors.response.use(
      (response) => response,
      async (error: AxiosError) => {
        const originalRequest = error.config as any;
        
        // Handle authentication errors
        if (error.response && error.response.status === 401) {
          // Token expired or invalid - redirect to login
          this.authToken = null;
          localStorage.removeItem('auth_token');
          window.location.href = '/login';
          return Promise.reject(this.createApiError(error, 'Authentication failed'));
        }

        // Handle rate limiting
        if (error.response && error.response.status === 429) {
          const retryAfter = error.response.headers['retry-after'];
          const delay = retryAfter ? parseInt(retryAfter) * 1000 : API_CONFIG.retryDelay;
          await new Promise(resolve => setTimeout(resolve, delay));
          return this.http.request(originalRequest);
        }

        // Handle server errors with retry logic
        if (error.response && error.response.status >= 500 && originalRequest && !originalRequest._retry) {
          originalRequest._retry = true;
          const retryCount = originalRequest._retryCount || 0;
          
          if (retryCount < API_CONFIG.retryAttempts) {
            originalRequest._retryCount = retryCount + 1;
            const delay = API_CONFIG.retryDelay * Math.pow(2, retryCount);
            await new Promise(resolve => setTimeout(resolve, delay));
            return this.http.request(originalRequest);
          }
        }

        console.error('API Error:', error);
        return Promise.reject(this.createApiError(error));
      }
    );
  }

  private createApiError(error: AxiosError, defaultMessage?: string): ApiError {
    const message = defaultMessage || 
      (error.response?.data as any)?.error || 
      error.message || 
      'An unexpected error occurred';
    
    return {
      message,
      status: error.response?.status,
      code: error.code,
      details: error.response?.data,
    };
  }

  // Authentication Methods
  async login(username: string, password: string): Promise<{ token: string; role: string }> {
    try {
      const response = await this.http.post('/auth/login', { username, password });
      const { token, role } = response.data;
      this.authToken = token;
      localStorage.setItem('auth_token', token);
      return { token, role };
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Login failed');
    }
  }

  async register(username: string, email: string, password: string, confirmPassword: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await this.http.post('/auth/register', { 
        username, 
        email, 
        password, 
        confirm_password: confirmPassword 
      });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Registration failed');
    }
  }

  async passwordReset(email: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await this.http.post('/auth/password-reset', { email });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Password reset failed');
    }
  }

  async passwordResetConfirm(token: string, newPassword: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await this.http.post(`/auth/password-reset/${token}`, { new_password: newPassword });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Password reset confirmation failed');
    }
  }

  async changePassword(currentPassword: string, newPassword: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await this.http.post('/auth/change-password', { 
        current_password: currentPassword, 
        new_password: newPassword 
      });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Password change failed');
    }
  }

  async setup2FA(): Promise<{ secret: string; qr_url: string; message: string }> {
    try {
      const response = await this.http.post('/auth/2fa/setup');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, '2FA setup failed');
    }
  }

  async enable2FA(secret: string, code: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await this.http.post('/auth/2fa/enable', { secret, code });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, '2FA enable failed');
    }
  }

  async disable2FA(code: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await this.http.post('/auth/2fa/disable', { code });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, '2FA disable failed');
    }
  }

  async verify2FA(secret: string, code: string): Promise<{ valid: boolean; message: string }> {
    try {
      const response = await this.http.post('/auth/2fa/verify', { secret, code });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, '2FA verification failed');
    }
  }

  logout(): void {
    this.authToken = null;
    localStorage.removeItem('auth_token');
    this.disconnectWebSocket();
  }

  setAuthToken(token: string): void {
    this.authToken = token;
    localStorage.setItem('auth_token', token);
  }

  // HTTP API Methods - Core Blockchain (Updated to match backend)
  async getNetworkMetrics(): Promise<NetworkMetrics> {
    const cacheKey = 'network_metrics';
    const cached = cache.get(cacheKey) as NetworkMetrics | null;
    if (cached) {
      return cached;
    }

    const trackedMethod = withPerformanceTracking(
      async () => {
        try {
          // Get validators and health status from backend
          const [validators, health] = await Promise.all([
            this.getValidators(),
            this.getHealth(),
          ]);

          // Create network metrics from available data
          const networkMetrics: NetworkMetrics = {
            totalNodes: validators.length,
            activeNodes: validators.filter(v => v.status === 'active').length,
            totalTPS: 0, // Will be updated via WebSocket
            averageLatency: 0, // Will be updated via WebSocket
            totalTransactions: 0, // Will be updated via WebSocket
            finalizedBlocks: 0, // Will be updated via WebSocket
            currentRound: 0, // Will be updated via WebSocket
            activeValidators: validators.filter(v => v.status === 'active').length,
            transactionGrowth: 0,
            validatorGrowth: 0,
            hashRate: 0,
            hashRateGrowth: 0,
            averageBlockTime: 0,
            blockTimeChange: 0,
          };

          return networkMetrics;
        } catch (error) {
          throw this.createApiError(error as AxiosError, 'Failed to fetch network metrics');
        }
      },
      'GET /network/metrics'
    );

    try {
      const data = await trackedMethod();
      cache.set(cacheKey, data, 30000); // Cache for 30 seconds
      return data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch network metrics');
    }
  }

  async getNodeMetrics(nodeId?: string): Promise<NodeMetrics[]> {
    try {
      // Get validators from backend and convert to node metrics
      const validators = await this.getValidators();
      
      return validators.map((validator, index) => ({
        nodeId: validator.id || `node-${index}`,
        uptime: Math.floor(Math.random() * 86400) + 3600, // 1-24 hours (mock for now)
        tps: Math.floor(Math.random() * 100000) + 50000, // 50k-150k TPS (mock for now)
        latency: Math.floor(Math.random() * 50) + 20, // 20-70ms (mock for now)
        memoryUsage: Math.floor(Math.random() * 2147483648) + 1073741824, // 1-3GB (mock for now)
        cpuUsage: Math.random() * 0.8 + 0.2, // 20-100% (mock for now)
        connectedPeers: Math.floor(Math.random() * 10) + 5, // 5-15 peers (mock for now)
        lastBlockTime: Date.now() - Math.floor(Math.random() * 60000), // Last minute (mock for now)
      }));
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch node metrics');
    }
  }

  async getBlocks(page: number = 1, limit: number = 50): Promise<PaginatedResponse<Block>> {
    try {
      // Since the backend doesn't have a dedicated blocks endpoint,
      // we'll create mock data for now
      const mockBlocks: Block[] = Array.from({ length: limit }, (_, i) => ({
        id: `block-${page * limit + i}`,
        number: page * limit + i,
        parentIds: [`block-${page * limit + i - 1}`],
        timestamp: { timestamp: Date.now() - i * 1000, nodeId: `validator-${i % 3}` },
        transactions: [],
        transactionCount: Math.floor(Math.random() * 100) + 50,
        validator: `validator-${i % 3}`,
        round: Math.floor(i / 10),
        hash: `hash-${page * limit + i}`,
        signature: `sig-${page * limit + i}`,
      }));

      return {
        data: mockBlocks,
        total: 1000,
        page,
        pageSize: limit,
        hasMore: page * limit < 1000,
      };
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch blocks');
    }
  }

  async getBlock(blockId: string): Promise<Block> {
    try {
      // Since the backend doesn't have a dedicated block endpoint,
      // we'll create mock data for now
      const block: Block = {
        id: blockId,
        number: parseInt(blockId.split('-')[1]) || 0,
        parentIds: [`block-${parseInt(blockId.split('-')[1]) - 1}`],
        timestamp: { timestamp: Date.now(), nodeId: 'validator-01' },
        transactions: [],
        transactionCount: Math.floor(Math.random() * 100) + 50,
        validator: 'validator-01',
        round: Math.floor(parseInt(blockId.split('-')[1]) / 10),
        hash: `hash-${blockId}`,
        signature: `sig-${blockId}`,
      };

      return block;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch block');
    }
  }

  async getTransactions(page: number = 1, limit: number = 50): Promise<PaginatedResponse<Transaction>> {
    try {
      // Since the backend doesn't have a dedicated transactions endpoint,
      // we'll create mock data for now
      const mockTransactions: Transaction[] = Array.from({ length: limit }, (_, i) => ({
        id: `tx-${page * limit + i}`,
        hash: `hash-${page * limit + i}`,
        from: `address-${i % 10}`,
        to: `address-${(i + 1) % 10}`,
        amount: Math.floor(Math.random() * 1000) + 1,
        asset: 'USD',
        timestamp: { timestamp: Date.now() - i * 1000, nodeId: 'validator-01' },
        signature: `sig-${page * limit + i}`,
        status: ['pending', 'confirmed', 'finalized', 'failed'][Math.floor(Math.random() * 4)] as any,
        type: ['transfer', 'buy', 'sell', 'mint', 'burn'][Math.floor(Math.random() * 5)] as any,
        price: Math.random() * 100,
        fee: Math.random() * 10,
      }));

      return {
        data: mockTransactions,
        total: 10000,
        page,
        pageSize: limit,
        hasMore: page * limit < 10000,
      };
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch transactions');
    }
  }

  async getTransaction(txId: string): Promise<Transaction> {
    try {
      // Since the backend doesn't have a dedicated transaction endpoint,
      // we'll create mock data for now
      const transaction: Transaction = {
        id: txId,
        hash: `hash-${txId}`,
        from: 'address-01',
        to: 'address-02',
        amount: Math.floor(Math.random() * 1000) + 1,
        asset: 'USD',
        timestamp: { timestamp: Date.now(), nodeId: 'validator-01' },
        signature: `sig-${txId}`,
        status: 'finalized',
        type: 'transfer',
        price: Math.random() * 100,
        fee: Math.random() * 10,
      };

      return transaction;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch transaction');
    }
  }

  async getValidators(): Promise<Validator[]> {
    try {
      const response = await this.http.get('/validators');
      const validators = response.data;
      
      // Convert backend validator format to frontend format
      return validators.map((validator: any, index: number) => ({
        id: validator.address || `validator-${index}`,
        address: validator.address || `address-${index}`,
        publicKey: validator.public_key || `pubkey-${index}`,
        stake: validator.stake || Math.floor(Math.random() * 1000000) + 100000,
        status: validator.status || 'active',
        lastSeen: validator.last_seen || Date.now(),
        pingLatency: validator.ping_latency || Math.floor(Math.random() * 100) + 20,
      }));
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch validators');
    }
  }

  async getRounds(page: number = 1, limit: number = 20): Promise<PaginatedResponse<Round>> {
    try {
      // Since the backend doesn't have a dedicated rounds endpoint,
      // we'll create mock data for now
      const mockRounds: Round[] = Array.from({ length: limit }, (_, i) => ({
        number: page * limit + i,
        startTime: Date.now() - (page * limit + i) * 60000,
        endTime: Date.now() - (page * limit + i - 1) * 60000,
        validators: [`validator-${i % 3}`, `validator-${(i + 1) % 3}`],
        finalizedBlocks: [`block-${i * 10}`, `block-${i * 10 + 1}`],
        transactionCount: Math.floor(Math.random() * 1000) + 500,
        status: 'finalized',
      }));

      return {
        data: mockRounds,
        total: 1000,
        page,
        pageSize: limit,
        hasMore: page * limit < 1000,
      };
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch rounds');
    }
  }

  async getCurrentRound(): Promise<Round> {
    try {
      // Since the backend doesn't have a dedicated current round endpoint,
      // we'll create mock data for now
      const currentRound: Round = {
        number: 45678,
        startTime: Date.now() - 30000,
        endTime: undefined, // Active round
        validators: ['validator-01', 'validator-02', 'validator-03'],
        finalizedBlocks: ['block-001', 'block-002', 'block-003'],
        transactionCount: 750,
        status: 'active',
      };

      return currentRound;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch current round');
    }
  }

  async submitTransaction(transaction: Omit<Transaction, 'id' | 'timestamp' | 'status'>): Promise<Transaction> {
    try {
      const response = await this.http.post('/tx', transaction);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to submit transaction');
    }
  }

  async getHealth(): Promise<{ status: string; timestamp: string; version: string }> {
    try {
      const response = await this.http.get('/health');
      return {
        status: response.data === 'OK' ? 'healthy' : 'unhealthy',
        timestamp: new Date().toISOString(),
        version: '1.0.0',
      };
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch health status');
    }
  }

  async getAssets(): Promise<Asset[]> {
    try {
      const response = await this.http.get('/assets');
      const assetSymbols = response.data;
      
      // Convert backend asset format to frontend format
      return assetSymbols.map((symbol: string, index: number) => ({
        id: symbol,
        symbol,
        name: symbol,
        decimals: 6,
        totalSupply: Math.floor(Math.random() * 1000000000) + 100000000,
        circulatingSupply: Math.floor(Math.random() * 1000000000) + 100000000,
        price: Math.random() * 1000 + 1,
        priceChange24h: (Math.random() - 0.5) * 100,
        priceChangePercent24h: (Math.random() - 0.5) * 20,
        volume24h: Math.floor(Math.random() * 1000000000) + 10000000,
        marketCap: Math.floor(Math.random() * 10000000000) + 1000000000,
        lastUpdated: Date.now(),
      }));
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch assets');
    }
  }

  async getAsset(assetId: string): Promise<Asset> {
    try {
      const assets = await this.getAssets();
      const asset = assets.find(a => a.id === assetId);
      if (!asset) {
        throw new Error(`Asset ${assetId} not found`);
      }
      return asset;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch asset');
    }
  }

  // Trading API Methods (Mock implementations for now)
  async getTradingPairs(): Promise<TradingPair[]> {
    try {
      // Mock trading pairs since backend doesn't have trading endpoints yet
      const pairs = ['BTC/USD', 'ETH/USD', 'EUR/USD', 'GBP/USD'];
      return pairs.map(pair => ({
        baseAsset: pair.split('/')[0],
        quoteAsset: pair.split('/')[1],
        symbol: pair,
        price: Math.random() * 1000 + 1,
        priceChange24h: (Math.random() - 0.5) * 100,
        volume24h: Math.floor(Math.random() * 1000000000) + 10000000,
        lastTrade: null,
      }));
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch trading pairs');
    }
  }

  async getTradingPair(pair: string): Promise<TradingPair> {
    try {
      const pairs = await this.getTradingPairs();
      const tradingPair = pairs.find(p => p.symbol === pair);
      if (!tradingPair) {
        throw new Error(`Trading pair ${pair} not found`);
      }
      return tradingPair;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch trading pair');
    }
  }

  async getPriceHistory(
    pair: string,
    timeFrame: string = '1h',
    limit: number = 1000
  ): Promise<PricePoint[]> {
    try {
      // Mock price history since backend doesn't have trading endpoints yet
      const now = Date.now();
      const interval = timeFrame === '1h' ? 3600000 : 60000; // 1 hour or 1 minute
      
      return Array.from({ length: limit }, (_, i) => ({
        timestamp: now - (limit - i) * interval,
        price: Math.random() * 1000 + 1,
        volume: Math.floor(Math.random() * 1000000) + 100000,
        high: Math.random() * 1000 + 1,
        low: Math.random() * 1000 + 1,
        open: Math.random() * 1000 + 1,
        close: Math.random() * 1000 + 1,
      }));
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch price history');
    }
  }

  async getRecentTrades(pair: string, limit: number = 100): Promise<Trade[]> {
    try {
      // Mock recent trades since backend doesn't have trading endpoints yet
      const now = Date.now();
      return Array.from({ length: limit }, (_, i) => ({
        id: `trade-${i}`,
        pair,
        price: Math.random() * 1000 + 1,
        amount: Math.random() * 100 + 1,
        side: Math.random() > 0.5 ? 'buy' : 'sell',
        timestamp: now - i * 1000,
        maker: `address-${i % 10}`,
        taker: `address-${(i + 1) % 10}`,
        fee: Math.random() * 10,
      }));
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch recent trades');
    }
  }

  async getOrderBook(pair: string, depth: number = 20): Promise<OrderBook> {
    try {
      // Mock order book since backend doesn't have trading endpoints yet
      const basePrice = Math.random() * 1000 + 1;
      const bids = Array.from({ length: depth }, (_, i) => ({
        price: basePrice - (i * 0.1),
        amount: Math.random() * 100 + 1,
        total: Math.random() * 1000 + 100,
      }));
      
      const asks = Array.from({ length: depth }, (_, i) => ({
        price: basePrice + (i * 0.1),
        amount: Math.random() * 100 + 1,
        total: Math.random() * 1000 + 100,
      }));

      return {
        pair,
        bids,
        asks,
        lastUpdateId: Date.now(),
      };
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch order book');
    }
  }

  async getUserOrders(userId: string, status?: string): Promise<MarketOrder[]> {
    try {
      // Mock user orders since backend doesn't have trading endpoints yet
      return Array.from({ length: 10 }, (_, i) => ({
        id: `order-${i}`,
        pair: 'BTC/USD',
        side: Math.random() > 0.5 ? 'buy' : 'sell',
        amount: Math.random() * 10 + 1,
        price: Math.random() * 1000 + 1,
        type: Math.random() > 0.5 ? 'market' : 'limit',
        status: ['pending', 'filled', 'cancelled', 'rejected'][Math.floor(Math.random() * 4)] as any,
        timestamp: Date.now() - i * 60000,
        user: userId,
        filledAmount: Math.random() * 10,
        averagePrice: Math.random() * 1000 + 1,
      }));
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch user orders');
    }
  }

  async getUserBalance(userId: string): Promise<{ [asset: string]: number }> {
    try {
      // Mock user balance since backend doesn't have trading endpoints yet
      return {
        'BTC': Math.random() * 10 + 1,
        'ETH': Math.random() * 100 + 10,
        'USD': Math.random() * 10000 + 1000,
        'EUR': Math.random() * 10000 + 1000,
      };
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch user balance');
    }
  }

  // Wallet Integration Methods
  async connectWallet(): Promise<{ address: string; message: string }> {
    try {
      const response = await this.http.post('/wallet/connect');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Wallet connection failed');
    }
  }

  async getWalletBalance(): Promise<{ balances: Array<{ asset: string; amount: number }>; message: string }> {
    try {
      const response = await this.http.get('/wallet/balance');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch wallet balance');
    }
  }

  async getTransactionHistory(params?: { page?: number; limit?: number; asset?: string }): Promise<{
    transactions: Array<{
      tx_hash: string;
      from: string;
      to: string;
      amount: number;
      asset: string;
      timestamp: number;
      status: string;
      fee: number;
    }>;
    total: number;
    message: string;
  }> {
    try {
      const queryParams = new URLSearchParams();
      if (params?.page) queryParams.append('page', params.page.toString());
      if (params?.limit) queryParams.append('limit', params.limit.toString());
      if (params?.asset) queryParams.append('asset', params.asset);
      
      const response = await this.http.get(`/wallet/transactions?${queryParams.toString()}`);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch transaction history');
    }
  }

  async depositFunds(amount: number, asset: string, externalAddress?: string): Promise<{
    tx_hash: string;
    status: string;
    message: string;
  }> {
    try {
      const response = await this.http.post('/wallet/deposit', {
        amount,
        asset,
        external_address: externalAddress
      });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Deposit failed');
    }
  }

  async withdrawFunds(amount: number, asset: string, externalAddress: string): Promise<{
    tx_hash: string;
    status: string;
    message: string;
  }> {
    try {
      const response = await this.http.post('/wallet/withdraw', {
        amount,
        asset,
        external_address: externalAddress
      });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Withdrawal failed');
    }
  }

  async getWalletAddresses(): Promise<{
    addresses: Array<{ address: string; label: string; is_active: boolean }>;
    message: string;
  }> {
    try {
      const response = await this.http.get('/wallet/addresses');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch wallet addresses');
    }
  }

  async generateWalletAddress(label?: string): Promise<{ address: string; message: string }> {
    try {
      const response = await this.http.post('/wallet/addresses', { label });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to generate wallet address');
    }
  }

  // Trading Operations Methods
  async placeOrder(order: {
    symbol: string;
    side: string;
    order_type: string;
    quantity: number;
    price?: number;
    client_order_id?: string;
    currency?: string;
  }): Promise<{ order_id: string; status: string; message: string }> {
    try {
      const response = await this.http.post('/orders', order);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to place order');
    }
  }

  async cancelOrder(orderId: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await this.http.delete(`/orders/${orderId}`);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to cancel order');
    }
  }

  async getOrderHistory(params?: { page?: number; limit?: number; status?: string }): Promise<{
    orders: Array<{
      order_id: string;
      symbol: string;
      side: string;
      order_type: string;
      quantity: number;
      price?: number;
      status: string;
      filled_quantity: number;
      average_price: number;
      created_at: number;
      updated_at: number;
    }>;
    total: number;
    message: string;
  }> {
    try {
      const queryParams = new URLSearchParams();
      if (params?.page) queryParams.append('page', params.page.toString());
      if (params?.limit) queryParams.append('limit', params.limit.toString());
      if (params?.status) queryParams.append('status', params.status);
      
      const response = await this.http.get(`/orders?${queryParams.toString()}`);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch order history');
    }
  }

  async getTradeHistory(params?: { page?: number; limit?: number; symbol?: string }): Promise<{
    trades: Array<{
      trade_id: string;
      order_id: string;
      symbol: string;
      side: string;
      quantity: number;
      price: number;
      fee: number;
      timestamp: number;
    }>;
    total: number;
    message: string;
  }> {
    try {
      const queryParams = new URLSearchParams();
      if (params?.page) queryParams.append('page', params.page.toString());
      if (params?.limit) queryParams.append('limit', params.limit.toString());
      if (params?.symbol) queryParams.append('symbol', params.symbol);
      
      const response = await this.http.get(`/trades?${queryParams.toString()}`);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch trade history');
    }
  }

  async getPositions(): Promise<{
    positions: Array<{
      symbol: string;
      side: string;
      quantity: number;
      average_price: number;
      current_price: number;
      unrealized_pnl: number;
      realized_pnl: number;
      margin_used: number;
      leverage: number;
    }>;
    message: string;
  }> {
    try {
      const response = await this.http.get('/positions');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch positions');
    }
  }

  // DAG Operations Methods
  async submitDagTransaction(transaction: {
    from: string;
    to: string;
    amount: number;
    asset: string;
    purpose?: string;
    shard_id?: number;
  }): Promise<{ tx_hash: string; block_id: string; status: string; message: string }> {
    try {
      const response = await this.http.post('/dag/submit-transaction', transaction);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to submit DAG transaction');
    }
  }

  // Analytics & Reporting Methods
  async getTradingAnalytics(): Promise<{
    total_volume: number;
    total_trades: number;
    win_rate: number;
    profit_loss: number;
    message: string;
  }> {
    try {
      const response = await this.http.get('/analytics/trading');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch trading analytics');
    }
  }

  async getPerformanceMetrics(): Promise<{
    avg_latency_ms: number;
    max_throughput: number;
    uptime_hours: number;
    error_rate: number;
    message: string;
  }> {
    try {
      const response = await this.http.get('/analytics/performance');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch performance metrics');
    }
  }

  async getPerformanceMetricsTimeSeries(timeRange: string = '24h'): Promise<{
    tps: Array<{ timestamp: number; value: number }>;
    latency: Array<{ timestamp: number; value: number }>;
    nodes: Array<{ timestamp: number; value: number }>;
    blocks: Array<{ timestamp: number; value: number }>;
    message: string;
  }> {
    try {
      const response = await this.http.get(`/analytics/performance/timeseries?range=${timeRange}`);
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch time-series performance metrics');
    }
  }

  async getRiskAnalysis(): Promise<{
    value_at_risk: number;
    max_drawdown: number;
    exposure: number;
    message: string;
  }> {
    try {
      const response = await this.http.get('/analytics/risk');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch risk analysis');
    }
  }

  async getPortfolioReport(): Promise<{
    holdings: Array<{ asset: string; amount: number; value: number }>;
    total_value: number;
    returns_pct: number;
    message: string;
  }> {
    try {
      const response = await this.http.get('/analytics/portfolio');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch portfolio report');
    }
  }

  async getMarketAnalysis(): Promise<{
    price_trend: string;
    volatility: number;
    liquidity: number;
    message: string;
  }> {
    try {
      const response = await this.http.get('/analytics/market');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch market analysis');
    }
  }

  // Real-time Data Methods
  async subscribeRealtime(channels: string[]): Promise<{
    success: boolean;
    channels: string[];
    message: string;
  }> {
    try {
      const response = await this.http.get('/realtime/subscribe', {
        params: { channels: channels.join(',') }
      });
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to subscribe to real-time data');
    }
  }

  async getRealtimeStatus(): Promise<{
    active_connections: number;
    uptime_seconds: number;
    status: string;
    message: string;
  }> {
    try {
      const response = await this.http.get('/realtime/status');
      return response.data;
    } catch (error) {
      throw this.createApiError(error as AxiosError, 'Failed to fetch real-time status');
    }
  }

  // Enhanced WebSocket Methods
  connectWebSocket(baseURL?: string): void {
    if (this.isManualDisconnect) {
      console.log('Manual disconnect in effect, not reconnecting');
      return;
    }

    if (this.ws) {
      this.ws.disconnect();
    }

    const wsURL = baseURL || API_CONFIG.baseURL.replace('http', 'ws');
    console.log(`Connecting to WebSocket: ${wsURL}`);
    
    this.ws = io(wsURL, {
      transports: ['websocket'],
      auth: {
        token: this.authToken,
      },
      timeout: 10000, // 10 second timeout
      forceNew: true,
      reconnection: false, // We'll handle reconnection manually
    });

    this.ws.on('connect', () => {
      console.log('WebSocket connected successfully');
      this.reconnectAttempts = 0;
      this.reconnectDelay = 1000;
      
      this.emitEvent('connection_status', {
        type: 'connection_status',
        data: { status: 'connected' },
        timestamp: Date.now(),
      });

      // Resubscribe to any pending subscriptions
      this.pendingSubscriptions.forEach(pair => {
        this.subscribeToPair(pair);
      });
      this.pendingSubscriptions.clear();

      // Start connection health check
      this.startConnectionHealthCheck();
    });

    this.ws.on('disconnect', (reason) => {
      console.log('WebSocket disconnected:', reason);
      this.stopConnectionHealthCheck();
      
      this.emitEvent('connection_status', {
        type: 'connection_status',
        data: { status: 'disconnected', reason },
        timestamp: Date.now(),
      });

      // Attempt reconnection unless manually disconnected
      if (!this.isManualDisconnect && reason !== 'io client disconnect') {
        this.scheduleReconnection();
      }
    });

    this.ws.on('connect_error', (error) => {
      console.error('WebSocket connection error:', error);
      this.stopConnectionHealthCheck();
      
      this.emitEvent('connection_status', {
        type: 'connection_status',
        data: { status: 'error', error: error.message },
        timestamp: Date.now(),
      });

      // Attempt reconnection unless manually disconnected
      if (!this.isManualDisconnect) {
        this.scheduleReconnection();
      }
    });

    // Enhanced event handling with validation
    this.ws.on('block', (data) => {
      if (this.validateBlockData(data)) {
        this.emitEvent('block', {
          type: 'block',
          data,
          timestamp: Date.now(),
        });
      }
    });

    this.ws.on('transaction', (data) => {
      if (this.validateTransactionData(data)) {
        this.emitEvent('transaction', {
          type: 'transaction',
          data,
          timestamp: Date.now(),
        });
      }
    });

    this.ws.on('round', (data) => {
      if (this.validateRoundData(data)) {
        this.emitEvent('round', {
          type: 'round',
          data,
          timestamp: Date.now(),
        });
      }
    });

    this.ws.on('metrics', (data) => {
      if (this.validateMetricsData(data)) {
        this.emitEvent('metrics', {
          type: 'metrics',
          data,
          timestamp: Date.now(),
        });
      }
    });

    this.ws.on('price', (data) => {
      if (this.validatePriceData(data)) {
        this.emitEvent('price', {
          type: 'price',
          data,
          timestamp: Date.now(),
        });
      }
    });

    this.ws.on('trade', (data) => {
      if (this.validateTradeData(data)) {
        this.emitEvent('trade', {
          type: 'trade',
          data,
          timestamp: Date.now(),
        });
      }
    });

    this.ws.on('orderbook', (data) => {
      if (this.validateOrderBookData(data)) {
        this.emitEvent('orderbook', {
          type: 'orderbook',
          data,
          timestamp: Date.now(),
        });
      }
    });

    // Handle connection timeout
    setTimeout(() => {
      if (this.ws && !this.ws.connected) {
        console.warn('WebSocket connection timeout');
        this.ws.disconnect();
      }
    }, 15000); // 15 second timeout
  }

  private scheduleReconnection(): void {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error('Max reconnection attempts reached');
      this.emitEvent('connection_status', {
        type: 'connection_status',
        data: { 
          status: 'failed', 
          error: 'Max reconnection attempts reached',
          attempts: this.reconnectAttempts 
        },
        timestamp: Date.now(),
      });
      return;
    }

    this.reconnectAttempts++;
    const delay = Math.min(this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1), this.maxReconnectDelay);
    
    console.log(`Scheduling reconnection attempt ${this.reconnectAttempts} in ${delay}ms`);
    
    this.reconnectTimer = setTimeout(() => {
      console.log(`Attempting reconnection ${this.reconnectAttempts}/${this.maxReconnectAttempts}`);
      this.connectWebSocket();
    }, delay);
  }

  private startConnectionHealthCheck(): void {
    this.connectionCheckInterval = setInterval(() => {
      if (this.ws && this.ws.connected) {
        // Send ping to check connection health
        this.ws.emit('ping');
      }
    }, 30000); // Check every 30 seconds
  }

  private stopConnectionHealthCheck(): void {
    if (this.connectionCheckInterval) {
      clearInterval(this.connectionCheckInterval);
      this.connectionCheckInterval = null;
    }
  }

  private validateBlockData(data: any): boolean {
    return data && 
           typeof data.id === 'string' && 
           typeof data.number === 'number' &&
           Array.isArray(data.parentIds) &&
           typeof data.timestamp === 'object';
  }

  private validateTransactionData(data: any): boolean {
    return data && 
           typeof data.id === 'string' && 
           typeof data.hash === 'string' &&
           typeof data.from === 'string' &&
           typeof data.to === 'string' &&
           typeof data.amount === 'number';
  }

  private validateRoundData(data: any): boolean {
    return data && 
           typeof data.number === 'number' && 
           typeof data.startTime === 'number' &&
           Array.isArray(data.validators) &&
           Array.isArray(data.finalizedBlocks);
  }

  private validateMetricsData(data: any): boolean {
    return data && 
           typeof data.nodeId === 'string' && 
           typeof data.uptime === 'number' &&
           typeof data.tps === 'number' &&
           typeof data.latency === 'number';
  }

  private validatePriceData(data: any): boolean {
    return data && 
           typeof data.timestamp === 'number' && 
           typeof data.price === 'number' &&
           typeof data.volume === 'number';
  }

  private validateTradeData(data: any): boolean {
    return data && 
           typeof data.id === 'string' && 
           typeof data.pair === 'string' &&
           typeof data.price === 'number' &&
           typeof data.amount === 'number' &&
           ['buy', 'sell'].includes(data.side);
  }

  private validateOrderBookData(data: any): boolean {
    return data && 
           typeof data.pair === 'string' && 
           Array.isArray(data.bids) &&
           Array.isArray(data.asks) &&
           typeof data.lastUpdateId === 'number';
  }

  subscribeToPair(pair: string): void {
    if (this.ws && this.ws.connected) {
      console.log(`Subscribing to pair: ${pair}`);
      this.ws.emit('subscribe', { pair });
    } else {
      console.log(`Adding ${pair} to pending subscriptions`);
      this.pendingSubscriptions.add(pair);
    }
  }

  unsubscribeFromPair(pair: string): void {
    if (this.ws && this.ws.connected) {
      console.log(`Unsubscribing from pair: ${pair}`);
      this.ws.emit('unsubscribe', { pair });
    }
    this.pendingSubscriptions.delete(pair);
  }

  disconnectWebSocket(): void {
    console.log('Manually disconnecting WebSocket');
    this.isManualDisconnect = true;
    
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    
    this.stopConnectionHealthCheck();
    
    if (this.ws) {
      this.ws.disconnect();
      this.ws = null;
    }
    
    this.emitEvent('connection_status', {
      type: 'connection_status',
      data: { status: 'disconnected', reason: 'manual_disconnect' },
      timestamp: Date.now(),
    });
  }

  // Reset manual disconnect flag to allow reconnections
  enableReconnections(): void {
    this.isManualDisconnect = false;
    this.reconnectAttempts = 0;
    this.reconnectDelay = 1000;
  }

  addEventListener(eventType: string, callback: (event: WebSocketEvent) => void): void {
    if (!this.eventListeners.has(eventType)) {
      this.eventListeners.set(eventType, []);
    }
    this.eventListeners.get(eventType)!.push(callback);
  }

  removeEventListener(eventType: string, callback: (event: WebSocketEvent) => void): void {
    if (this.eventListeners.has(eventType)) {
      const listeners = this.eventListeners.get(eventType)!;
      const index = listeners.indexOf(callback);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    }
  }

  private emitEvent(eventType: string, event: WebSocketEvent): void {
    if (this.eventListeners.has(eventType)) {
      this.eventListeners.get(eventType)!.forEach(callback => {
        try {
          callback(event);
        } catch (error) {
          console.error(`Error in event listener for ${eventType}:`, error);
        }
      });
    }
  }

  isConnected(): boolean {
    return this.ws?.connected || false;
  }

  getConnectionStatus(): 'connected' | 'disconnected' | 'connecting' {
    if (!this.ws) return 'disconnected';
    if (this.ws.connected) return 'connected';
    return 'connecting';
  }

  isAuthenticated(): boolean {
    return !!this.authToken;
  }

  getAuthToken(): string | null {
    return this.authToken;
  }
}

// Export singleton instance
export const finDAGApi = new FinDAGApi(); 