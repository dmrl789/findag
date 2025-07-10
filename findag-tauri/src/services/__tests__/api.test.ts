import { nodeAPI, walletAPI, tradingAPI, systemAPI } from '../api';

// Mock Tauri invoke
jest.mock('@tauri-apps/api/tauri', () => ({
  invoke: jest.fn(),
}));

describe('API Service', () => {
  const mockInvoke = require('@tauri-apps/api/tauri').invoke;

  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('Node API', () => {
    it('should start node successfully', async () => {
      mockInvoke.mockResolvedValueOnce(undefined);

      await nodeAPI.startNode();

      expect(mockInvoke).toHaveBeenCalledWith('start_findag_node');
    });

    it('should stop node successfully', async () => {
      mockInvoke.mockResolvedValueOnce(undefined);

      await nodeAPI.stopNode();

      expect(mockInvoke).toHaveBeenCalledWith('stop_findag_node');
    });

    it('should get node status', async () => {
      const mockStatus = {
        is_running: true,
        is_connected: true,
        uptime: 3600,
        peers: 5,
        tps: 1000,
        blocks_per_second: 10,
        mempool_size: 50,
        last_block_hash: 'abc123',
        last_round_number: 100,
        version: '1.0.0',
      };

      mockInvoke.mockResolvedValueOnce(mockStatus);

      const result = await nodeAPI.getStatus();

      expect(mockInvoke).toHaveBeenCalledWith('get_node_status');
      expect(result).toEqual(mockStatus);
    });

    it('should get node config', async () => {
      const mockConfig = {
        port: 8080,
        peers: ['peer1', 'peer2'],
        data_directory: '/data',
        max_block_size: 1024,
        block_interval: 1000,
        round_interval: 5000,
      };

      mockInvoke.mockResolvedValueOnce(mockConfig);

      const result = await nodeAPI.getConfig();

      expect(mockInvoke).toHaveBeenCalledWith('get_node_config');
      expect(result).toEqual(mockConfig);
    });

    it('should update node config', async () => {
      const config = {
        port: 9090,
        peers: ['peer3'],
        data_directory: '/new-data',
        max_block_size: 2048,
        block_interval: 2000,
        round_interval: 10000,
      };

      mockInvoke.mockResolvedValueOnce(undefined);

      await nodeAPI.updateConfig(config);

      expect(mockInvoke).toHaveBeenCalledWith('update_node_config', { config });
    });
  });

  describe('Wallet API', () => {
    it('should create wallet successfully', async () => {
      const mockWallet = {
        address: 'wallet123',
        public_key: 'pubkey123',
        handle: 'my-wallet',
        total_balance: 1000.0,
        transaction_count: 5,
        last_activity: Date.now(),
      };

      mockInvoke.mockResolvedValueOnce(mockWallet);

      const result = await walletAPI.createWallet();

      expect(mockInvoke).toHaveBeenCalledWith('create_wallet');
      expect(result).toEqual(mockWallet);
    });

    it('should import wallet successfully', async () => {
      const privateKey = 'private-key-123';
      const mockWallet = {
        address: 'wallet456',
        public_key: 'pubkey456',
        handle: 'imported-wallet',
        total_balance: 500.0,
        transaction_count: 3,
        last_activity: Date.now(),
      };

      mockInvoke.mockResolvedValueOnce(mockWallet);

      const result = await walletAPI.importWallet(privateKey);

      expect(mockInvoke).toHaveBeenCalledWith('import_wallet', { privateKey });
      expect(result).toEqual(mockWallet);
    });

    it('should get wallet balance', async () => {
      const walletAddress = 'wallet123';
      const mockBalances = [
        {
          asset: 'EUR',
          amount: 1000.0,
          available: 950.0,
          locked: 50.0,
          last_updated: Date.now(),
        },
      ];

      mockInvoke.mockResolvedValueOnce(mockBalances);

      const result = await walletAPI.getBalance(walletAddress);

      expect(mockInvoke).toHaveBeenCalledWith('get_wallet_balance', { walletAddress });
      expect(result).toEqual(mockBalances);
    });

    it('should send transaction successfully', async () => {
      const transactionData = {
        toAddress: 'recipient123',
        asset: 'EUR',
        amount: 100.0,
        memo: 'Payment',
      };

      const mockTransaction = {
        id: 'tx123',
        transaction_type: 'transfer',
        asset: 'EUR',
        amount: 100.0,
        fee: 0.1,
        status: 'pending',
        timestamp: Date.now(),
        from_address: 'sender123',
        to_address: 'recipient123',
        memo: 'Payment',
      };

      mockInvoke.mockResolvedValueOnce(mockTransaction);

      const result = await walletAPI.sendTransaction(
        transactionData.toAddress,
        transactionData.asset,
        transactionData.amount,
        transactionData.memo
      );

      expect(mockInvoke).toHaveBeenCalledWith('send_transaction', {
        toAddress: transactionData.toAddress,
        asset: transactionData.asset,
        amount: transactionData.amount,
        memo: transactionData.memo,
      });
      expect(result).toEqual(mockTransaction);
    });
  });

  describe('Trading API', () => {
    it('should get trading pairs', async () => {
      const mockPairs = ['EUR/USD', 'GBP/USD', 'USD/JPY'];

      mockInvoke.mockResolvedValueOnce(mockPairs);

      const result = await tradingAPI.getTradingPairs();

      expect(mockInvoke).toHaveBeenCalledWith('get_trading_pairs');
      expect(result).toEqual(mockPairs);
    });

    it('should get market data', async () => {
      const symbol = 'EUR/USD';
      const mockMarketData = {
        symbol: 'EUR/USD',
        last_price: 1.0850,
        bid: 1.0848,
        ask: 1.0852,
        volume: 1000000,
        change: 0.0020,
        change_percent: 0.18,
        high: 1.0870,
        low: 1.0830,
        timestamp: Date.now(),
      };

      mockInvoke.mockResolvedValueOnce(mockMarketData);

      const result = await tradingAPI.getMarketData(symbol);

      expect(mockInvoke).toHaveBeenCalledWith('get_market_data', { symbol });
      expect(result).toEqual(mockMarketData);
    });

    it('should place order successfully', async () => {
      const orderData = {
        symbol: 'EUR/USD',
        side: 'buy',
        orderType: 'limit',
        quantity: 1000,
        price: 1.0850,
        stopPrice: undefined,
      };

      const mockOrder = {
        id: 'order123',
        symbol: 'EUR/USD',
        side: 'buy',
        order_type: 'limit',
        quantity: 1000,
        price: 1.0850,
        status: 'pending',
        timestamp: Date.now(),
      };

      mockInvoke.mockResolvedValueOnce(mockOrder);

      const result = await tradingAPI.placeOrder(
        orderData.symbol,
        orderData.side,
        orderData.orderType,
        orderData.quantity,
        orderData.price,
        orderData.stopPrice
      );

      expect(mockInvoke).toHaveBeenCalledWith('place_order', {
        symbol: orderData.symbol,
        side: orderData.side,
        orderType: orderData.orderType,
        quantity: orderData.quantity,
        price: orderData.price,
        stopPrice: orderData.stopPrice,
      });
      expect(result).toEqual(mockOrder);
    });
  });

  describe('System API', () => {
    it('should get system info', async () => {
      const mockSystemInfo = {
        platform: 'Windows',
        version: '10.0.19044',
        memory_total: 16777216,
        memory_available: 8388608,
        cpu_cores: 8,
        cpu_usage: 45.2,
        disk_total: 1000000000000,
        disk_available: 500000000000,
        node_id: 'node123',
        architecture: 'x86_64',
      };

      mockInvoke.mockResolvedValueOnce(mockSystemInfo);

      const result = await systemAPI.getSystemInfo();

      expect(mockInvoke).toHaveBeenCalledWith('get_system_info');
      expect(result).toEqual(mockSystemInfo);
    });

    it('should get system stats', async () => {
      const mockSystemStats = {
        cpu_usage: 45.2,
        memory_usage: 65.8,
        disk_usage: 50.0,
        uptime: 86400,
        network_connections: 25,
        active_processes: 150,
      };

      mockInvoke.mockResolvedValueOnce(mockSystemStats);

      const result = await systemAPI.getSystemStats();

      expect(mockInvoke).toHaveBeenCalledWith('get_system_stats');
      expect(result).toEqual(mockSystemStats);
    });
  });

  describe('Error Handling', () => {
    it('should handle API errors gracefully', async () => {
      const error = new Error('Network error');
      mockInvoke.mockRejectedValueOnce(error);

      await expect(nodeAPI.getStatus()).rejects.toThrow('Network error');
    });

    it('should handle invalid responses', async () => {
      mockInvoke.mockResolvedValueOnce(null);

      const result = await nodeAPI.getStatus();
      expect(result).toBeNull();
    });
  });
}); 