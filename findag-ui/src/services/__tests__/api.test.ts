import { FinDAGApi } from '../api';
import axios from 'axios';
import { io } from 'socket.io-client';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

// Mock socket.io-client
jest.mock('socket.io-client');
const mockedIo = io as jest.MockedFunction<typeof io>;

// Mock localStorage
const localStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
});

describe('FinDAGApi', () => {
  let api: FinDAGApi;
  let mockAxiosInstance: any;

  beforeEach(() => {
    // Reset all mocks
    jest.clearAllMocks();
    
    // Create mock axios instance
    mockAxiosInstance = {
      post: jest.fn(),
      get: jest.fn(),
      put: jest.fn(),
      delete: jest.fn(),
      interceptors: {
        request: { use: jest.fn() },
        response: { use: jest.fn() },
      },
    };
    
    mockedAxios.create.mockReturnValue(mockAxiosInstance);
    
    // Mock socket instance
    const mockSocket = {
      on: jest.fn(),
      emit: jest.fn(),
      connect: jest.fn(),
      disconnect: jest.fn(),
      connected: true,
    };
    mockedIo.mockReturnValue(mockSocket as any);
    
    api = new FinDAGApi();
  });

  describe('Authentication', () => {
    test('should login successfully', async () => {
      const mockResponse = {
        data: {
          token: 'test-token',
          role: 'user',
        },
      };
      mockAxiosInstance.post.mockResolvedValue(mockResponse);

      const result = await api.login('testuser', 'password');

      expect(result).toEqual({
        token: 'test-token',
        role: 'user',
      });
      expect(mockAxiosInstance.post).toHaveBeenCalledWith('/auth/login', {
        username: 'testuser',
        password: 'password',
      });
      expect(localStorageMock.setItem).toHaveBeenCalledWith('auth_token', 'test-token');
    });

    test('should handle login failure', async () => {
      const mockError = {
        response: {
          status: 401,
          data: { error: 'Invalid credentials' },
        },
      };
      mockAxiosInstance.post.mockRejectedValue(mockError);

      await expect(api.login('testuser', 'wrong-password')).rejects.toThrow('Login failed');
    });

    test('should register successfully', async () => {
      const mockResponse = {
        data: {
          success: true,
          message: 'Registration successful',
        },
      };
      mockAxiosInstance.post.mockResolvedValue(mockResponse);

      const result = await api.register('testuser', 'test@example.com', 'password', 'password');

      expect(result).toEqual({
        success: true,
        message: 'Registration successful',
      });
      expect(mockAxiosInstance.post).toHaveBeenCalledWith('/auth/register', {
        username: 'testuser',
        email: 'test@example.com',
        password: 'password',
        confirm_password: 'password',
      });
    });

    test('should setup 2FA successfully', async () => {
      const mockResponse = {
        data: {
          secret: 'test-secret',
          qr_url: 'test-qr-url',
          message: '2FA setup successful',
        },
      };
      mockAxiosInstance.post.mockResolvedValue(mockResponse);

      const result = await api.setup2FA();

      expect(result).toEqual({
        secret: 'test-secret',
        qr_url: 'test-qr-url',
        message: '2FA setup successful',
      });
    });

    test('should logout successfully', () => {
      api.logout();

      expect(localStorageMock.removeItem).toHaveBeenCalledWith('auth_token');
    });
  });

  describe('Network Operations', () => {
    test('should get network metrics', async () => {
      const mockResponse = {
        data: {
          total_nodes: 10,
          active_nodes: 8,
          total_transactions: 1000,
          network_hashrate: 5000,
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getNetworkMetrics();

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/network/metrics');
    });

    test('should get blocks with pagination', async () => {
      const mockResponse = {
        data: {
          data: [
            { id: '1', hash: 'hash1', timestamp: 1234567890 },
            { id: '2', hash: 'hash2', timestamp: 1234567891 },
          ],
          total: 2,
          page: 1,
          limit: 50,
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getBlocks(1, 50);

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/blocks?page=1&limit=50');
    });

    test('should get transactions with pagination', async () => {
      const mockResponse = {
        data: {
          data: [
            { id: '1', from: 'addr1', to: 'addr2', amount: 100 },
            { id: '2', from: 'addr3', to: 'addr4', amount: 200 },
          ],
          total: 2,
          page: 1,
          limit: 50,
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getTransactions(1, 50);

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/transactions?page=1&limit=50');
    });
  });

  describe('Trading Operations', () => {
    test('should place order successfully', async () => {
      const mockResponse = {
        data: {
          order_id: 'order-123',
          status: 'pending',
          message: 'Order placed successfully',
        },
      };
      mockAxiosInstance.post.mockResolvedValue(mockResponse);

      const order = {
        symbol: 'BTC/USD',
        side: 'buy',
        order_type: 'limit',
        quantity: 1.5,
        price: 50000,
      };

      const result = await api.placeOrder(order);

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.post).toHaveBeenCalledWith('/trading/orders', order);
    });

    test('should cancel order successfully', async () => {
      const mockResponse = {
        data: {
          success: true,
          message: 'Order cancelled successfully',
        },
      };
      mockAxiosInstance.delete.mockResolvedValue(mockResponse);

      const result = await api.cancelOrder('order-123');

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.delete).toHaveBeenCalledWith('/trading/orders/order-123');
    });

    test('should get order history', async () => {
      const mockResponse = {
        data: {
          orders: [
            {
              order_id: 'order-1',
              symbol: 'BTC/USD',
              side: 'buy',
              order_type: 'limit',
              quantity: 1.0,
              price: 50000,
              status: 'filled',
              filled_quantity: 1.0,
              average_price: 50000,
              created_at: 1234567890,
              updated_at: 1234567891,
            },
          ],
          total: 1,
          message: 'Orders retrieved successfully',
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getOrderHistory({ page: 1, limit: 10 });

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/trading/orders?page=1&limit=10');
    });
  });

  describe('Wallet Operations', () => {
    test('should connect wallet successfully', async () => {
      const mockResponse = {
        data: {
          address: '0x1234567890abcdef',
          message: 'Wallet connected successfully',
        },
      };
      mockAxiosInstance.post.mockResolvedValue(mockResponse);

      const result = await api.connectWallet();

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.post).toHaveBeenCalledWith('/wallet/connect');
    });

    test('should get wallet balance', async () => {
      const mockResponse = {
        data: {
          balances: [
            { asset: 'BTC', amount: 1.5 },
            { asset: 'ETH', amount: 10.0 },
          ],
          message: 'Balance retrieved successfully',
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getWalletBalance();

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/wallet/balance');
    });

    test('should get transaction history', async () => {
      const mockResponse = {
        data: {
          transactions: [
            {
              tx_hash: '0xabc123',
              from: '0x1234567890abcdef',
              to: '0xfedcba0987654321',
              amount: 1.0,
              asset: 'BTC',
              timestamp: 1234567890,
              status: 'confirmed',
              fee: 0.001,
            },
          ],
          total: 1,
          message: 'Transactions retrieved successfully',
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getTransactionHistory({ page: 1, limit: 10 });

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/wallet/transactions?page=1&limit=10');
    });
  });

  describe('Analytics Operations', () => {
    test('should get trading analytics', async () => {
      const mockResponse = {
        data: {
          total_volume: 1000000,
          total_trades: 500,
          win_rate: 0.65,
          profit_loss: 50000,
          message: 'Analytics retrieved successfully',
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getTradingAnalytics();

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/analytics/trading');
    });

    test('should get performance metrics', async () => {
      const mockResponse = {
        data: {
          avg_latency_ms: 150,
          max_throughput: 1000,
          uptime_hours: 720,
          error_rate: 0.01,
          message: 'Performance metrics retrieved successfully',
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getPerformanceMetrics();

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/analytics/performance');
    });

    test('should get performance metrics time series', async () => {
      const mockResponse = {
        data: {
          tps: [{ timestamp: 1234567890, value: 100 }],
          latency: [{ timestamp: 1234567890, value: 150 }],
          nodes: [{ timestamp: 1234567890, value: 10 }],
          blocks: [{ timestamp: 1234567890, value: 1000 }],
          message: 'Time series data retrieved successfully',
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getPerformanceMetricsTimeSeries('24h');

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/analytics/performance/timeseries?timeRange=24h');
    });
  });

  describe('WebSocket Operations', () => {
    test('should connect WebSocket successfully', () => {
      const mockSocket = {
        on: jest.fn(),
        emit: jest.fn(),
        connect: jest.fn(),
        disconnect: jest.fn(),
        connected: true,
      };
      mockedIo.mockReturnValue(mockSocket as any);

      api.connectWebSocket('ws://localhost:8080');

      expect(mockedIo).toHaveBeenCalledWith('ws://localhost:8080', {
        transports: ['websocket'],
        autoConnect: false,
      });
      expect(mockSocket.connect).toHaveBeenCalled();
    });

    test('should subscribe to real-time channels', async () => {
      const mockResponse = {
        data: {
          success: true,
          channels: ['price_updates', 'trade_feed'],
          message: 'Subscribed successfully',
        },
      };
      mockAxiosInstance.post.mockResolvedValue(mockResponse);

      const result = await api.subscribeRealtime(['price_updates', 'trade_feed']);

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.post).toHaveBeenCalledWith('/realtime/subscribe', {
        channels: ['price_updates', 'trade_feed'],
      });
    });

    test('should get real-time status', async () => {
      const mockResponse = {
        data: {
          active_connections: 100,
          uptime_seconds: 3600,
          status: 'connected',
          message: 'Status retrieved successfully',
        },
      };
      mockAxiosInstance.get.mockResolvedValue(mockResponse);

      const result = await api.getRealtimeStatus();

      expect(result).toEqual(mockResponse.data);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/realtime/status');
    });
  });

  describe('Error Handling', () => {
    test('should handle network errors', async () => {
      const mockError = {
        code: 'NETWORK_ERROR',
        message: 'Network error',
      };
      mockAxiosInstance.get.mockRejectedValue(mockError);

      await expect(api.getNetworkMetrics()).rejects.toThrow('Network error');
    });

    test('should handle server errors with retry', async () => {
      const mockError = {
        response: {
          status: 500,
          data: { error: 'Internal server error' },
        },
      };
      mockAxiosInstance.get.mockRejectedValue(mockError);

      await expect(api.getNetworkMetrics()).rejects.toThrow('Internal server error');
    });

    test('should handle rate limiting', async () => {
      const mockError = {
        response: {
          status: 429,
          headers: { 'retry-after': '5' },
          data: { error: 'Rate limit exceeded' },
        },
      };
      mockAxiosInstance.get.mockRejectedValue(mockError);

      await expect(api.getNetworkMetrics()).rejects.toThrow('Rate limit exceeded');
    });
  });

  describe('Token Management', () => {
    test('should set auth token', () => {
      api.setAuthToken('new-token');

      expect(api.getAuthToken()).toBe('new-token');
    });

    test('should check authentication status', () => {
      api.setAuthToken('test-token');
      expect(api.isAuthenticated()).toBe(true);

      api.logout();
      expect(api.isAuthenticated()).toBe(false);
    });
  });
}); 