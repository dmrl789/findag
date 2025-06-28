// FinDAG Core Types
export interface FinDAGTime {
  timestamp: number; // 64-bit timestamp with 100ns resolution
  nodeId: string;
}

export interface Block {
  id: string;
  parentIds: string[];
  timestamp: FinDAGTime;
  transactions: Transaction[];
  validator: string;
  round: number;
  hash: string;
  signature: string;
}

export interface Transaction {
  id: string;
  from: string;
  to: string;
  amount: number;
  asset: string;
  timestamp: FinDAGTime;
  signature: string;
  status: 'pending' | 'confirmed' | 'finalized' | 'failed';
  type: 'transfer' | 'buy' | 'sell' | 'mint' | 'burn';
  price?: number; // Price for trading operations
  fee?: number;
}

// Asset and Trading Types
export interface Asset {
  id: string;
  symbol: string;
  name: string;
  decimals: number;
  totalSupply: number;
  circulatingSupply: number;
  price: number;
  priceChange24h: number;
  priceChangePercent24h: number;
  volume24h: number;
  marketCap: number;
  lastUpdated: number;
}

export interface PricePoint {
  timestamp: number;
  price: number;
  volume: number;
  high: number;
  low: number;
  open: number;
  close: number;
}

export interface TradingPair {
  baseAsset: string;
  quoteAsset: string;
  symbol: string;
  price: number;
  priceChange24h: number;
  volume24h: number;
  lastTrade: Trade | null;
}

export interface Trade {
  id: string;
  pair: string;
  price: number;
  amount: number;
  side: 'buy' | 'sell';
  timestamp: number;
  maker: string;
  taker: string;
  fee: number;
}

export interface OrderBook {
  pair: string;
  bids: OrderBookEntry[];
  asks: OrderBookEntry[];
  lastUpdateId: number;
}

export interface OrderBookEntry {
  price: number;
  amount: number;
  total: number;
}

export interface MarketOrder {
  id: string;
  pair: string;
  side: 'buy' | 'sell';
  amount: number;
  price?: number; // Market orders don't have price
  type: 'market' | 'limit' | 'stop';
  status: 'pending' | 'filled' | 'cancelled' | 'rejected';
  timestamp: number;
  user: string;
  filledAmount: number;
  averagePrice: number;
}

export interface Validator {
  id: string;
  address: string;
  publicKey: string;
  stake: number;
  status: 'active' | 'inactive' | 'slashed';
  lastSeen: number;
  pingLatency: number;
}

export interface Round {
  number: number;
  startTime: number;
  endTime?: number;
  validators: string[];
  finalizedBlocks: string[];
  status: 'active' | 'finalized' | 'failed';
}

export interface NodeMetrics {
  nodeId: string;
  uptime: number;
  tps: number;
  latency: number;
  memoryUsage: number;
  cpuUsage: number;
  connectedPeers: number;
  lastBlockTime: number;
}

export interface NetworkMetrics {
  totalNodes: number;
  activeNodes: number;
  totalTPS: number;
  averageLatency: number;
  totalTransactions: number;
  finalizedBlocks: number;
  currentRound: number;
}

// API Response Types
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: number;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
}

// WebSocket Event Types
export interface WebSocketEvent {
  type: string;
  data: any;
  timestamp: number;
}

export interface BlockEvent extends WebSocketEvent {
  type: 'block';
  data: Block;
}

export interface TransactionEvent extends WebSocketEvent {
  type: 'transaction';
  data: Transaction;
}

export interface RoundEvent extends WebSocketEvent {
  type: 'round';
  data: Round;
}

export interface MetricsEvent extends WebSocketEvent {
  type: 'metrics';
  data: NodeMetrics;
}

export interface PriceEvent extends WebSocketEvent {
  type: 'price';
  data: PricePoint;
}

export interface TradeEvent extends WebSocketEvent {
  type: 'trade';
  data: Trade;
}

export interface OrderBookEvent extends WebSocketEvent {
  type: 'orderbook';
  data: OrderBook;
}

// UI State Types
export interface UIState {
  selectedNode?: string;
  selectedTimeRange: '1h' | '6h' | '24h' | '7d' | '30d';
  autoRefresh: boolean;
  theme: 'light' | 'dark';
  selectedAsset?: string;
  selectedPair?: string;
  chartType: 'line' | 'candlestick' | 'volume';
}

export interface ChartDataPoint {
  timestamp: number;
  value: number;
  label?: string;
}

export interface DAGNode {
  id: string;
  label: string;
  level: number;
  timestamp: number;
  validator: string;
  transactionCount: number;
}

export interface DAGEdge {
  from: string;
  to: string;
  arrows: 'to';
}

export interface DAGData {
  nodes: DAGNode[];
  edges: DAGEdge[];
}

// Trading UI Types
export interface TradingViewState {
  selectedPair: string;
  timeFrame: '1m' | '5m' | '15m' | '1h' | '4h' | '1d' | '1w';
  priceHistory: PricePoint[];
  recentTrades: Trade[];
  orderBook: OrderBook | null;
  userOrders: MarketOrder[];
  userBalance: { [asset: string]: number };
} 