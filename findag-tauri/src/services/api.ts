import { invoke } from '@tauri-apps/api/tauri';

// Types for backend communication
export interface NodeStatus {
  is_running: boolean;
  is_connected: boolean;
  uptime: number;
  peers: number;
  tps: number;
  blocks_per_second: number;
  mempool_size: number;
  last_block_hash?: string;
  last_round_number?: number;
  version: string;
}

export interface NodeConfig {
  port: number;
  peers: string[];
  data_directory: string;
  max_block_size: number;
  block_interval: number;
  round_interval: number;
}

export interface WalletInfo {
  address: string;
  public_key: string;
  handle?: string;
  total_balance: number;
  transaction_count: number;
  last_activity: number;
}

export interface Balance {
  asset: string;
  amount: number;
  available: number;
  locked: number;
  last_updated: number;
}

export interface Transaction {
  id: string;
  transaction_type: string;
  asset: string;
  amount: number;
  fee: number;
  status: string;
  timestamp: number;
  block_hash?: string;
  round_number?: number;
  from_address?: string;
  to_address?: string;
  memo?: string;
}

export interface Order {
  id: string;
  symbol: string;
  side: string;
  order_type: string;
  quantity: number;
  price?: number;
  stop_price?: number;
  status: string;
  timestamp: number;
  filled_quantity?: number;
  average_price?: number;
}

export interface MarketData {
  symbol: string;
  last_price: number;
  bid: number;
  ask: number;
  volume: number;
  change: number;
  change_percent: number;
  high: number;
  low: number;
  timestamp: number;
}

export interface Position {
  symbol: string;
  quantity: number;
  average_price: number;
  unrealized_pnl: number;
  realized_pnl: number;
  timestamp: number;
}

export interface Peer {
  id: string;
  address: string;
  port: number;
  status: string;
  latency: number;
  last_seen: number;
  version: string;
  user_agent: string;
}

export interface NetworkStats {
  total_peers: number;
  connected_peers: number;
  total_bandwidth: number;
  average_latency: number;
  uptime: number;
  blocks_received: number;
  blocks_sent: number;
}

export interface SystemInfo {
  platform: string;
  version: string;
  memory_total: number;
  memory_available: number;
  cpu_cores: number;
  cpu_usage: number;
  disk_total: number;
  disk_available: number;
  node_id: string;
  architecture: string;
}

export interface SystemConfig {
  port: number;
  max_block_size: number;
  block_interval: number;
  round_interval: number;
  peers: string[];
  validator_address: string;
  validator_public_key: string;
}

export interface SystemStats {
  cpu_usage: number;
  memory_usage: number;
  disk_usage: number;
  uptime: number;
  network_connections: number;
  active_processes: number;
}

export interface AppSettings {
  theme: 'light' | 'dark' | 'auto';
  language: string;
  notifications: boolean;
  auto_refresh: boolean;
  refresh_interval: number;
  debug_mode: boolean;
}

export interface LogEntry {
  id: string;
  timestamp: number;
  level: string;
  component: string;
  message: string;
  details?: any;
}

export interface Validator {
  address: string;
  public_key: string;
  stake: number;
  status: string;
  uptime: number;
  last_activity: number;
  last_active: number;
  performance: number;
  blocks_produced: number;
  metadata?: string;
}

export interface ValidatorStats {
  total_validators: number;
  active_validators: number;
  inactive_validators: number;
  total_stake: number;
  average_uptime: number;
  average_performance: number;
  slashed_validators: number;
  blocks_produced: number;
  votes_cast: number;
}

// Node Management API
export const nodeAPI = {
  startNode: async (): Promise<void> => {
    return invoke('start_findag_node');
  },

  stopNode: async (): Promise<void> => {
    return invoke('stop_findag_node');
  },

  getStatus: async (): Promise<NodeStatus> => {
    return invoke('get_node_status');
  },

  getConfig: async (): Promise<NodeConfig> => {
    return invoke('get_node_config');
  },

  updateConfig: async (config: NodeConfig): Promise<void> => {
    return invoke('update_node_config', { config });
  },
};

// Wallet API
export const walletAPI = {
  createWallet: async (): Promise<WalletInfo> => {
    return invoke('create_wallet');
  },

  importWallet: async (privateKey: string): Promise<WalletInfo> => {
    return invoke('import_wallet', { privateKey });
  },

  getBalance: async (walletAddress: string): Promise<Balance[]> => {
    return invoke('get_wallet_balance', { walletAddress });
  },

  sendTransaction: async (
    toAddress: string,
    asset: string,
    amount: number,
    memo?: string
  ): Promise<Transaction> => {
    return invoke('send_transaction', { toAddress, asset, amount, memo });
  },

  getTransactionHistory: async (walletAddress?: string): Promise<Transaction[]> => {
    return invoke('get_transaction_history', { walletAddress });
  },

  backupWallet: async (walletAddress: string): Promise<string> => {
    return invoke('backup_wallet', { walletAddress });
  },
};

// Trading API
export const tradingAPI = {
  getTradingPairs: async (): Promise<string[]> => {
    return invoke('get_trading_pairs');
  },

  getMarketData: async (symbol: string): Promise<MarketData> => {
    return invoke('get_market_data', { symbol });
  },

  placeOrder: async (
    symbol: string,
    side: string,
    orderType: string,
    quantity: number,
    price?: number,
    stopPrice?: number
  ): Promise<Order> => {
    return invoke('place_order', { symbol, side, orderType, quantity, price, stopPrice });
  },

  cancelOrder: async (orderId: string): Promise<void> => {
    return invoke('cancel_order', { orderId });
  },

  getOrderHistory: async (): Promise<Order[]> => {
    return invoke('get_order_history');
  },
};

// DAG API
export const dagAPI = {
  getStatus: async (): Promise<any> => {
    return invoke('get_dag_status');
  },

  getBlocks: async (limit?: number): Promise<any[]> => {
    return invoke('get_dag_blocks', { limit });
  },

  getTransactions: async (blockHash?: string): Promise<Transaction[]> => {
    return invoke('get_dag_transactions', { blockHash });
  },

  submitTransaction: async (transactionData: any): Promise<string> => {
    return invoke('submit_dag_transaction', { transactionData });
  },
};

// Network API
export const networkAPI = {
  getStatus: async (): Promise<NetworkStats> => {
    return invoke('get_network_status');
  },

  getPeerList: async (): Promise<Peer[]> => {
    return invoke('get_peer_list');
  },

  addPeer: async (address: string, port: number): Promise<void> => {
    return invoke('add_peer', { address, port });
  },

  removePeer: async (peerId: string): Promise<void> => {
    return invoke('remove_peer', { peerId });
  },
};

// System API
export const systemAPI = {
  getSystemInfo: async (): Promise<SystemInfo> => {
    return invoke('get_system_info');
  },

  getSystemConfig: async (): Promise<SystemConfig> => {
    return invoke('get_system_config');
  },

  getSystemStats: async (): Promise<SystemStats> => {
    return invoke('get_system_stats');
  },

  updateSystemConfig: async (config: SystemConfig): Promise<void> => {
    return invoke('update_system_config', { config });
  },

  updateAppSettings: async (settings: AppSettings): Promise<void> => {
    return invoke('update_app_settings', { settings });
  },

  restartNode: async (): Promise<void> => {
    return invoke('restart_node');
  },

  getLogs: async (
    level?: string,
    component?: string,
    limit?: number
  ): Promise<LogEntry[]> => {
    return invoke('get_logs', { level, component, limit });
  },

  exportData: async (dataType: string): Promise<string> => {
    return invoke('export_data', { dataType });
  },
};

// Validator API
export const validatorAPI = {
  getValidatorList: async (): Promise<Validator[]> => {
    return invoke('get_validator_list');
  },

  getValidatorStats: async (): Promise<ValidatorStats> => {
    return invoke('get_validator_stats');
  },

  addValidator: async (
    address: string,
    publicKey: string,
    metadata: string,
    adminToken: string
  ): Promise<void> => {
    return invoke('add_validator', { address, publicKey, metadata, adminToken });
  },

  removeValidator: async (address: string, adminToken: string): Promise<void> => {
    return invoke('remove_validator', { address, adminToken });
  },

  slashValidator: async (address: string, adminToken: string): Promise<void> => {
    return invoke('slash_validator', { address, adminToken });
  },
};

// Utility functions
export const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

export const formatUptime = (seconds: number): string => {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (days > 0) {
    return `${days}d ${hours}h ${minutes}m ${secs}s`;
  } else if (hours > 0) {
    return `${hours}h ${minutes}m ${secs}s`;
  } else if (minutes > 0) {
    return `${minutes}m ${secs}s`;
  } else {
    return `${secs}s`;
  }
};

export const formatLatency = (ms: number): string => {
  if (ms < 1000) {
    return `${ms}ms`;
  } else {
    return `${(ms / 1000).toFixed(2)}s`;
  }
};

export const formatTimestamp = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleString();
};

export const getStatusColor = (status: string): string => {
  switch (status.toLowerCase()) {
    case 'running':
    case 'connected':
    case 'confirmed':
    case 'filled':
      return 'text-green-600';
    case 'pending':
    case 'connecting':
      return 'text-yellow-600';
    case 'stopped':
    case 'disconnected':
    case 'cancelled':
    case 'failed':
      return 'text-red-600';
    default:
      return 'text-gray-600';
  }
};

export const getStatusBgColor = (status: string): string => {
  switch (status.toLowerCase()) {
    case 'running':
    case 'connected':
    case 'confirmed':
    case 'filled':
      return 'bg-green-100 text-green-800';
    case 'pending':
    case 'connecting':
      return 'bg-yellow-100 text-yellow-800';
    case 'stopped':
    case 'disconnected':
    case 'cancelled':
    case 'failed':
      return 'bg-red-100 text-red-800';
    default:
      return 'bg-gray-100 text-gray-800';
  }
}; 