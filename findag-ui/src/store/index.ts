import { create } from 'zustand';
import { finDAGApi } from '../services/api';
import {
  NetworkMetrics,
  NodeMetrics,
  Block,
  Transaction,
  Round,
  Validator,
  UIState,
} from '../types';

// Initial UI state
const initialUIState: UIState = {
  selectedTimeRange: '24h',
  autoRefresh: true,
  theme: 'light',
  chartType: 'line',
};

interface AppState {
  // UI State
  ui: UIState;
  
  // Connection State
  isConnected: boolean;
  connectionStatus: 'connected' | 'disconnected' | 'connecting';
  connectionCheckInterval?: any;
  
  // Real-time Data
  networkMetrics: NetworkMetrics | null;
  nodeMetrics: NodeMetrics[];
  recentBlocks: Block[];
  recentTransactions: Transaction[];
  currentRound: Round | null;
  validators: Validator[];
  
  // Loading States
  isLoading: {
    networkMetrics: boolean;
    nodeMetrics: boolean;
    blocks: boolean;
    transactions: boolean;
    validators: boolean;
  };
  
  // Error States
  errors: {
    networkMetrics?: string;
    nodeMetrics?: string;
    blocks?: string;
    transactions?: string;
    validators?: string;
  };
  
  // Actions
  setUI: (ui: Partial<UIState>) => void;
  setConnectionStatus: (status: 'connected' | 'disconnected' | 'connecting') => void;
  setNetworkMetrics: (metrics: NetworkMetrics) => void;
  setNodeMetrics: (metrics: NodeMetrics[]) => void;
  addBlock: (block: Block) => void;
  addTransaction: (transaction: Transaction) => void;
  setCurrentRound: (round: Round) => void;
  setValidators: (validators: Validator[]) => void;
  setLoading: (key: keyof AppState['isLoading'], loading: boolean) => void;
  setError: (key: keyof AppState['errors'], error?: string) => void;
  clearErrors: () => void;
  
  // API Actions
  fetchNetworkMetrics: () => Promise<void>;
  fetchNodeMetrics: () => Promise<void>;
  fetchRecentBlocks: () => Promise<void>;
  fetchRecentTransactions: () => Promise<void>;
  fetchValidators: () => Promise<void>;
  connectWebSocket: () => void;
  disconnectWebSocket: () => void;
}

export const useAppStore = create<AppState>((set, get) => ({
  // Initial State
  ui: initialUIState,
  isConnected: false,
  connectionStatus: 'disconnected',
  networkMetrics: null,
  nodeMetrics: [],
  recentBlocks: [],
  recentTransactions: [],
  currentRound: null,
  validators: [],
  isLoading: {
    networkMetrics: false,
    nodeMetrics: false,
    blocks: false,
    transactions: false,
    validators: false,
  },
  errors: {},

  // UI Actions
  setUI: (ui) => set((state) => ({ ui: { ...state.ui, ...ui } })),

  // Connection Actions
  setConnectionStatus: (status) => set({ connectionStatus: status }),

  // Data Actions
  setNetworkMetrics: (metrics) => set({ networkMetrics: metrics }),
  setNodeMetrics: (metrics) => set({ nodeMetrics: metrics }),
  addBlock: (block) => set((state) => ({ 
    recentBlocks: [block, ...state.recentBlocks.slice(0, 99)] 
  })),
  addTransaction: (transaction) => set((state) => ({ 
    recentTransactions: [transaction, ...state.recentTransactions.slice(0, 99)] 
  })),
  setCurrentRound: (round) => set({ currentRound: round }),
  setValidators: (validators) => set({ validators }),

  // Loading Actions
  setLoading: (key, loading) => set((state) => ({
    isLoading: { ...state.isLoading, [key]: loading }
  })),

  // Error Actions
  setError: (key, error) => set((state) => ({
    errors: { ...state.errors, [key]: error }
  })),
  clearErrors: () => set({ errors: {} }),

  // API Actions - Real Backend Integration with Demo Fallback
  fetchNetworkMetrics: async () => {
    const { setLoading, setError, setNetworkMetrics } = get();
    
    setLoading('networkMetrics', true);
    setError('networkMetrics');
    
    try {
      const metrics = await finDAGApi.getNetworkMetrics();
      setNetworkMetrics(metrics);
    } catch (error: any) {
      console.error('Failed to fetch network metrics, using demo data:', error);
      // Use a simple demo metrics object
      setNetworkMetrics({
        totalNodes: 12,
        activeNodes: 11,
        totalTPS: 1250000,
        averageLatency: 45,
        totalTransactions: 4567890123,
        finalizedBlocks: 1234567,
        currentRound: 45678,
        activeValidators: 10,
        transactionGrowth: 2.5,
        validatorGrowth: 1.2,
        hashRate: 1000000,
        averageBlockTime: 2.5,
        blockTimeChange: -0.1,
        hashRateGrowth: 5.0
      });
    } finally {
      setLoading('networkMetrics', false);
    }
  },

  fetchNodeMetrics: async () => {
    const { setLoading, setError, setNodeMetrics } = get();
    
    setLoading('nodeMetrics', true);
    setError('nodeMetrics');
    
    try {
      const metrics = await finDAGApi.getNodeMetrics();
      setNodeMetrics(metrics);
    } catch (error: any) {
      console.error('Failed to fetch node metrics, using demo data:', error);
      setNodeMetrics([
        {
          nodeId: 'node-001',
          uptime: 86400,
          tps: 125000,
          latency: 42,
          memoryUsage: 2147483648,
          cpuUsage: 0.65,
          connectedPeers: 8,
          lastBlockTime: Date.now() - 1000,
        }
      ]);
    } finally {
      setLoading('nodeMetrics', false);
    }
  },

  fetchRecentBlocks: async () => {
    const { setLoading, setError } = get();
    
    setLoading('blocks', true);
    setError('blocks');
    
    try {
      const response = await finDAGApi.getBlocks(1, 10);
      set({ recentBlocks: response.data });
    } catch (error: any) {
      console.error('Failed to fetch recent blocks, using demo data:', error);
      set({ recentBlocks: [] });
    } finally {
      setLoading('blocks', false);
    }
  },

  fetchRecentTransactions: async () => {
    const { setLoading, setError } = get();
    
    setLoading('transactions', true);
    setError('transactions');
    
    try {
      const response = await finDAGApi.getTransactions(1, 10);
      set({ recentTransactions: response.data });
    } catch (error: any) {
      console.error('Failed to fetch recent transactions, using demo data:', error);
      set({ recentTransactions: [] });
    } finally {
      setLoading('transactions', false);
    }
  },

  fetchValidators: async () => {
    const { setLoading, setError, setValidators } = get();
    
    setLoading('validators', true);
    setError('validators');
    
    try {
      const validators = await finDAGApi.getValidators();
      setValidators(validators);
    } catch (error: any) {
      console.error('Failed to fetch validators, using demo data:', error);
      setValidators([]);
    } finally {
      setLoading('validators', false);
    }
  },

  connectWebSocket: () => {
    const { setConnectionStatus } = get();
    
    setConnectionStatus('connecting');
    
    try {
      finDAGApi.connectWebSocket();
      setConnectionStatus('connected');
      set({ isConnected: true });
      
      // Set up connection health check
      const interval = setInterval(() => {
        const isConnected = finDAGApi.isConnected();
        if (!isConnected) {
          setConnectionStatus('disconnected');
          set({ isConnected: false });
          clearInterval(interval);
        }
      }, 5000);
      
      set({ connectionCheckInterval: interval });
    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
      setConnectionStatus('disconnected');
      set({ isConnected: false });
    }
  },

  disconnectWebSocket: () => {
    const { setConnectionStatus, connectionCheckInterval } = get();
    
    try {
      finDAGApi.disconnectWebSocket();
      setConnectionStatus('disconnected');
      set({ isConnected: false });
      
      if (connectionCheckInterval) {
        clearInterval(connectionCheckInterval);
        set({ connectionCheckInterval: undefined });
      }
    } catch (error) {
      console.error('Failed to disconnect WebSocket:', error);
    }
  },
}));

// Export all stores
export { useAuthStore } from './auth';
export { useDemoStore } from './demo';
export { useTradingStore } from './trading';