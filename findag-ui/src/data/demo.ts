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
  OrderBookEntry,
  Validator
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

// Demo Assets - Traditional Financial Instruments
export const demoAssets: Asset[] = [
  {
    id: 'eur-usd',
    symbol: 'EUR/USD',
    name: 'Euro / US Dollar',
    decimals: 5,
    totalSupply: 0,
    circulatingSupply: 0,
    price: 1.0850,
    priceChange24h: 0.0025,
    priceChangePercent24h: 0.23,
    volume24h: 125000000000,
    marketCap: 0,
    lastUpdated: Date.now(),
  },
  {
    id: 'gbp-usd',
    symbol: 'GBP/USD',
    name: 'British Pound / US Dollar',
    decimals: 5,
    totalSupply: 0,
    circulatingSupply: 0,
    price: 1.2650,
    priceChange24h: -0.0080,
    priceChangePercent24h: -0.63,
    volume24h: 89000000000,
    marketCap: 0,
    lastUpdated: Date.now(),
  },
  {
    id: 'usd-jpy',
    symbol: 'USD/JPY',
    name: 'US Dollar / Japanese Yen',
    decimals: 3,
    totalSupply: 0,
    circulatingSupply: 0,
    price: 148.25,
    priceChange24h: 0.45,
    priceChangePercent24h: 0.30,
    volume24h: 95000000000,
    marketCap: 0,
    lastUpdated: Date.now(),
  },
  {
    id: 'aapl',
    symbol: 'AAPL',
    name: 'Apple Inc.',
    decimals: 2,
    totalSupply: 15700000000,
    circulatingSupply: 15700000000,
    price: 185.50,
    priceChange24h: 2.75,
    priceChangePercent24h: 1.50,
    volume24h: 4500000000,
    marketCap: 2910000000000,
    lastUpdated: Date.now(),
  },
  {
    id: 'msft',
    symbol: 'MSFT',
    name: 'Microsoft Corporation',
    decimals: 2,
    totalSupply: 7500000000,
    circulatingSupply: 7500000000,
    price: 415.80,
    priceChange24h: -3.20,
    priceChangePercent24h: -0.76,
    volume24h: 3200000000,
    marketCap: 3110000000000,
    lastUpdated: Date.now(),
  },
  {
    id: 'gold',
    symbol: 'XAU/USD',
    name: 'Gold / US Dollar',
    decimals: 2,
    totalSupply: 0,
    circulatingSupply: 0,
    price: 2045.50,
    priceChange24h: 12.75,
    priceChangePercent24h: 0.63,
    volume24h: 85000000000,
    marketCap: 0,
    lastUpdated: Date.now(),
  },
  {
    id: 'oil',
    symbol: 'WTI/USD',
    name: 'West Texas Intermediate Crude Oil',
    decimals: 2,
    totalSupply: 0,
    circulatingSupply: 0,
    price: 78.25,
    priceChange24h: -1.50,
    priceChangePercent24h: -1.88,
    volume24h: 65000000000,
    marketCap: 0,
    lastUpdated: Date.now(),
  },
  {
    id: 'us10y',
    symbol: 'US10Y',
    name: 'US 10-Year Treasury Bond',
    decimals: 3,
    totalSupply: 0,
    circulatingSupply: 0,
    price: 4.125,
    priceChange24h: -0.025,
    priceChangePercent24h: -0.60,
    volume24h: 45000000000,
    marketCap: 0,
    lastUpdated: Date.now(),
  },
];

// Demo Trading Pairs - Traditional Financial Instruments
export const demoTradingPairs: TradingPair[] = [
  {
    baseAsset: 'EUR',
    quoteAsset: 'USD',
    symbol: 'EUR/USD',
    price: 1.0850,
    priceChange24h: 0.0025,
    volume24h: 125000000000,
    lastTrade: {
      id: 'trade-001',
      pair: 'EUR/USD',
      price: 1.0850,
      amount: 100000,
      side: 'buy',
      timestamp: Date.now() - 5000,
      maker: 'bank-001',
      taker: 'bank-002',
      fee: 0,
    },
  },
  {
    baseAsset: 'GBP',
    quoteAsset: 'USD',
    symbol: 'GBP/USD',
    price: 1.2650,
    priceChange24h: -0.0080,
    volume24h: 89000000000,
    lastTrade: {
      id: 'trade-002',
      pair: 'GBP/USD',
      price: 1.2650,
      amount: 50000,
      side: 'sell',
      timestamp: Date.now() - 3000,
      maker: 'bank-003',
      taker: 'bank-004',
      fee: 0,
    },
  },
  {
    baseAsset: 'AAPL',
    quoteAsset: 'USD',
    symbol: 'AAPL/USD',
    price: 185.50,
    priceChange24h: 2.75,
    volume24h: 4500000000,
    lastTrade: {
      id: 'trade-003',
      pair: 'AAPL/USD',
      price: 185.50,
      amount: 100,
      side: 'buy',
      timestamp: Date.now() - 2000,
      maker: 'broker-001',
      taker: 'broker-002',
      fee: 0,
    },
  },
  {
    baseAsset: 'XAU',
    quoteAsset: 'USD',
    symbol: 'XAU/USD',
    price: 2045.50,
    priceChange24h: 12.75,
    volume24h: 85000000000,
    lastTrade: {
      id: 'trade-004',
      pair: 'XAU/USD',
      price: 2045.50,
      amount: 100,
      side: 'buy',
      timestamp: Date.now() - 4000,
      maker: 'dealer-001',
      taker: 'dealer-002',
      fee: 0,
    },
  },
];

// Demo Price History - EUR/USD with realistic forex data
export const demoPriceHistory: PricePoint[] = Array.from({ length: 100 }, (_, i) => {
  const basePrice = 1.0850;
  const timeOffset = i * 60000; // 1 minute intervals
  const priceVariation = Math.sin(i * 0.1) * 0.002 + Math.random() * 0.001;
  const volume = 1000000 + Math.random() * 5000000;
  
  return {
    timestamp: Date.now() - (100 - i) * 60000,
    price: basePrice + priceVariation,
    volume,
    high: basePrice + priceVariation + Math.random() * 0.0005,
    low: basePrice + priceVariation - Math.random() * 0.0005,
    open: basePrice + priceVariation - Math.random() * 0.0002,
    close: basePrice + priceVariation + Math.random() * 0.0002,
  };
});

// Demo Recent Trades - EUR/USD trades
export const demoRecentTrades: Trade[] = Array.from({ length: 50 }, (_, i) => ({
  id: `trade-${String(i + 1).padStart(3, '0')}`,
  pair: 'EUR/USD',
  price: 1.0850 + (Math.random() - 0.5) * 0.002,
  amount: 10000 + Math.random() * 100000,
  side: Math.random() > 0.5 ? 'buy' : 'sell',
  timestamp: Date.now() - i * 30000, // 30 second intervals
  maker: `bank-${String(Math.floor(Math.random() * 10)).padStart(3, '0')}`,
  taker: `bank-${String(Math.floor(Math.random() * 10)).padStart(3, '0')}`,
  fee: 0,
}));

// Demo Order Book - EUR/USD
export const demoOrderBook: OrderBook = {
  pair: 'EUR/USD',
  bids: Array.from({ length: 20 }, (_, i) => ({
    price: 1.0845 - i * 0.0001,
    amount: 10000 + Math.random() * 100000,
    total: 0,
  })).map((entry, i, arr) => ({
    ...entry,
    total: arr.slice(0, i + 1).reduce((sum, e) => sum + e.amount, 0),
  })),
  asks: Array.from({ length: 20 }, (_, i) => ({
    price: 1.0855 + i * 0.0001,
    amount: 10000 + Math.random() * 100000,
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

// Demo Validators
export const demoValidators: Validator[] = Array.from({ length: 10 }, (_, i) => ({
  id: `validator-${String(i + 1).padStart(2, '0')}`,
  address: `0x${String(i + 1).padStart(40, '0')}`,
  name: `Validator ${i + 1}`,
  stake: Math.random() * 1000000 + 100000,
  commission: Math.random() * 0.1 + 0.05,
  uptime: Math.random() * 100,
  performance: Math.random() * 100,
  status: Math.random() > 0.1 ? 'active' : 'inactive',
  lastBlockTime: Date.now() - Math.random() * 60000,
  totalBlocks: Math.floor(Math.random() * 10000) + 1000,
  totalStake: Math.random() * 10000000 + 1000000,
})); 