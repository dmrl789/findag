import { 
  NetworkMetrics, 
  NodeMetrics, 
  Block, 
  Transaction, 
  Asset, 
  PricePoint, 
  TradingPair, 
  Trade, 
  OrderBook, 
  OrderBookEntry 
} from '../types';

// Demo Network Metrics
export const demoNetworkMetrics: NetworkMetrics = {
  totalNodes: 12,
  activeNodes: 11,
  totalTPS: 1250000,
  averageLatency: 45,
  totalTransactions: 4567890123,
  finalizedBlocks: 1234567,
  currentRound: 45678,
};

// Demo Node Metrics
export const demoNodeMetrics: NodeMetrics[] = [
  {
    nodeId: 'node-001',
    uptime: 86400,
    tps: 125000,
    latency: 42,
    memoryUsage: 2147483648,
    cpuUsage: 0.65,
    connectedPeers: 8,
    lastBlockTime: Date.now() - 1000,
  },
  {
    nodeId: 'node-002',
    uptime: 86400,
    tps: 118000,
    latency: 48,
    memoryUsage: 1879048192,
    cpuUsage: 0.58,
    connectedPeers: 7,
    lastBlockTime: Date.now() - 1200,
  },
  {
    nodeId: 'node-003',
    uptime: 86400,
    tps: 132000,
    latency: 38,
    memoryUsage: 2415919104,
    cpuUsage: 0.72,
    connectedPeers: 9,
    lastBlockTime: Date.now() - 800,
  },
];

// Demo Assets
export const demoAssets: Asset[] = [
  {
    id: 'btc',
    symbol: 'BTC',
    name: 'Bitcoin',
    decimals: 8,
    totalSupply: 21000000,
    circulatingSupply: 19500000,
    price: 43250.75,
    priceChange24h: 1250.50,
    priceChangePercent24h: 2.98,
    volume24h: 28456789012,
    marketCap: 843456789012,
    lastUpdated: Date.now(),
  },
  {
    id: 'eth',
    symbol: 'ETH',
    name: 'Ethereum',
    decimals: 18,
    totalSupply: 120000000,
    circulatingSupply: 119000000,
    price: 2650.25,
    priceChange24h: -45.75,
    priceChangePercent24h: -1.70,
    volume24h: 15678901234,
    marketCap: 315678901234,
    lastUpdated: Date.now(),
  },
  {
    id: 'usdt',
    symbol: 'USDT',
    name: 'Tether',
    decimals: 6,
    totalSupply: 95000000000,
    circulatingSupply: 95000000000,
    price: 1.0001,
    priceChange24h: 0.0001,
    priceChangePercent24h: 0.01,
    volume24h: 45678901234,
    marketCap: 95000000000,
    lastUpdated: Date.now(),
  },
];

// Demo Trading Pairs
export const demoTradingPairs: TradingPair[] = [
  {
    baseAsset: 'BTC',
    quoteAsset: 'USD',
    symbol: 'BTC/USD',
    price: 43250.75,
    priceChange24h: 1250.50,
    volume24h: 28456789012,
    lastTrade: {
      id: 'trade-001',
      pair: 'BTC/USD',
      price: 43250.75,
      amount: 0.5,
      side: 'buy',
      timestamp: Date.now() - 5000,
      maker: 'user-001',
      taker: 'user-002',
      fee: 0.001,
    },
  },
  {
    baseAsset: 'ETH',
    quoteAsset: 'USD',
    symbol: 'ETH/USD',
    price: 2650.25,
    priceChange24h: -45.75,
    volume24h: 15678901234,
    lastTrade: {
      id: 'trade-002',
      pair: 'ETH/USD',
      price: 2650.25,
      amount: 2.5,
      side: 'sell',
      timestamp: Date.now() - 3000,
      maker: 'user-003',
      taker: 'user-004',
      fee: 0.001,
    },
  },
];

// Demo Price History
export const demoPriceHistory: PricePoint[] = Array.from({ length: 100 }, (_, i) => {
  const basePrice = 43250;
  const timeOffset = i * 60000; // 1 minute intervals
  const priceVariation = Math.sin(i * 0.1) * 500 + Math.random() * 200;
  const volume = 1000000 + Math.random() * 5000000;
  
  return {
    timestamp: Date.now() - (100 - i) * 60000,
    price: basePrice + priceVariation,
    volume,
    high: basePrice + priceVariation + Math.random() * 100,
    low: basePrice + priceVariation - Math.random() * 100,
    open: basePrice + priceVariation - Math.random() * 50,
    close: basePrice + priceVariation + Math.random() * 50,
  };
});

// Demo Recent Trades
export const demoRecentTrades: Trade[] = Array.from({ length: 50 }, (_, i) => ({
  id: `trade-${String(i + 1).padStart(3, '0')}`,
  pair: 'BTC/USD',
  price: 43250 + (Math.random() - 0.5) * 100,
  amount: 0.1 + Math.random() * 2,
  side: Math.random() > 0.5 ? 'buy' : 'sell',
  timestamp: Date.now() - i * 30000, // 30 second intervals
  maker: `user-${String(Math.floor(Math.random() * 100)).padStart(3, '0')}`,
  taker: `user-${String(Math.floor(Math.random() * 100)).padStart(3, '0')}`,
  fee: 0.001,
}));

// Demo Order Book
export const demoOrderBook: OrderBook = {
  pair: 'BTC/USD',
  bids: Array.from({ length: 20 }, (_, i) => ({
    price: 43200 - i * 5,
    amount: 0.1 + Math.random() * 2,
    total: 0,
  })).map((entry, i, arr) => ({
    ...entry,
    total: arr.slice(0, i + 1).reduce((sum, e) => sum + e.amount, 0),
  })),
  asks: Array.from({ length: 20 }, (_, i) => ({
    price: 43250 + i * 5,
    amount: 0.1 + Math.random() * 2,
    total: 0,
  })).map((entry, i, arr) => ({
    ...entry,
    total: arr.slice(0, i + 1).reduce((sum, e) => sum + e.amount, 0),
  })),
  lastUpdateId: 123456789,
};

// Demo Blocks
export const demoBlocks: Block[] = Array.from({ length: 10 }, (_, i) => ({
  id: `block-${String(i + 1).padStart(6, '0')}`,
  parentIds: [`block-${String(i).padStart(6, '0')}`],
  timestamp: {
    timestamp: Date.now() - i * 1000,
    nodeId: `node-${String(Math.floor(Math.random() * 3) + 1).padStart(3, '0')}`,
  },
  transactions: Array.from({ length: Math.floor(Math.random() * 100) + 50 }, (_, j) => ({
    id: `tx-${String(i * 100 + j).padStart(8, '0')}`,
    from: `user-${String(Math.floor(Math.random() * 1000)).padStart(3, '0')}`,
    to: `user-${String(Math.floor(Math.random() * 1000)).padStart(3, '0')}`,
    amount: Math.random() * 1000,
    asset: Math.random() > 0.5 ? 'BTC' : 'ETH',
    timestamp: {
      timestamp: Date.now() - i * 1000 - j * 10,
      nodeId: `node-${String(Math.floor(Math.random() * 3) + 1).padStart(3, '0')}`,
    },
    signature: `sig-${String(i * 100 + j).padStart(8, '0')}`,
    status: Math.random() > 0.1 ? 'finalized' : 'confirmed',
    type: Math.random() > 0.7 ? 'transfer' : (Math.random() > 0.5 ? 'buy' : 'sell'),
    price: Math.random() > 0.7 ? Math.random() * 50000 : undefined,
    fee: Math.random() * 0.01,
  })),
  validator: `validator-${String(Math.floor(Math.random() * 10) + 1).padStart(2, '0')}`,
  round: 45678 - i,
  hash: `hash-${String(i + 1).padStart(6, '0')}`,
  signature: `block-sig-${String(i + 1).padStart(6, '0')}`,
}));

// Demo Transactions
export const demoTransactions: Transaction[] = Array.from({ length: 20 }, (_, i) => ({
  id: `tx-${String(i + 1).padStart(8, '0')}`,
  from: `user-${String(Math.floor(Math.random() * 1000)).padStart(3, '0')}`,
  to: `user-${String(Math.floor(Math.random() * 1000)).padStart(3, '0')}`,
  amount: Math.random() * 1000,
  asset: Math.random() > 0.5 ? 'BTC' : 'ETH',
  timestamp: {
    timestamp: Date.now() - i * 60000,
    nodeId: `node-${String(Math.floor(Math.random() * 3) + 1).padStart(3, '0')}`,
  },
  signature: `sig-${String(i + 1).padStart(8, '0')}`,
  status: Math.random() > 0.1 ? 'finalized' : 'confirmed',
  type: Math.random() > 0.7 ? 'transfer' : (Math.random() > 0.5 ? 'buy' : 'sell'),
  price: Math.random() > 0.7 ? Math.random() * 50000 : undefined,
  fee: Math.random() * 0.01,
})); 