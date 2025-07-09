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
  connectionCheckInterval?: number;
  
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

  // API Actions - Real Backend Integration
  fetchNetworkMetrics: async () => {
    const { setLoading, setError, setNetworkMetrics } = get();
    
    setLoading('networkMetrics', true);
    setError('networkMetrics');
    
    try {
      const metrics = await finDAGApi.getNetworkMetrics();
      setNetworkMetrics(metrics);
    } catch (error: any) {
      console.error('Failed to fetch network metrics:', error);
      setError('networkMetrics', error.message || 'Failed to fetch network metrics');
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
      console.error('Failed to fetch node metrics:', error);
      setError('nodeMetrics', error.message || 'Failed to fetch node metrics');
    } finally {
      setLoading('nodeMetrics', false);
    }
  },

  fetchRecentBlocks: async () => {
    const { setLoading, setError, addBlock } = get();
    
    setLoading('blocks', true);
    setError('blocks');
    
    try {
      const response = await finDAGApi.getBlocks(1, 10);
      // Clear existing blocks and add new ones
      set({ recentBlocks: response.data });
    } catch (error: any) {
      console.error('Failed to fetch recent blocks:', error);
      setError('blocks', error.message || 'Failed to fetch recent blocks');
    } finally {
      setLoading('blocks', false);
    }
  },

  fetchRecentTransactions: async () => {
    const { setLoading, setError, addTransaction } = get();
    
    setLoading('transactions', true);
    setError('transactions');
    
    try {
      const response = await finDAGApi.getTransactions(1, 10);
      // Clear existing transactions and add new ones
      set({ recentTransactions: response.data });
    } catch (error: any) {
      console.error('Failed to fetch recent transactions:', error);
      setError('transactions', error.message || 'Failed to fetch recent transactions');
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
      console.error('Failed to fetch validators:', error);
      setError('validators', error.message || 'Failed to fetch validators');
    } finally {
      setLoading('validators', false);
    }
  },

  connectWebSocket: () => {
    const { setConnectionStatus } = get();
    
    setConnectionStatus('connecting');
    
    try {
      finDAGApi.connectWebSocket();
      
      // Listen for connection status updates
      finDAGApi.addEventListener('connection_status', (event) => {
        const status = event.data.status;
        setConnectionStatus(status === 'connected' ? 'connected' : 'disconnected');
        set({ isConnected: status === 'connected' });
      });

      // Listen for real-time data updates
      finDAGApi.addEventListener('block', (event) => {
        get().addBlock(event.data);
      });

      finDAGApi.addEventListener('transaction', (event) => {
        get().addTransaction(event.data);
      });

      finDAGApi.addEventListener('round', (event) => {
        get().setCurrentRound(event.data);
      });

      finDAGApi.addEventListener('metrics', (event) => {
        // Update node metrics with real-time data
        const { nodeMetrics } = get();
        const updatedMetrics = nodeMetrics.map(metric => 
          metric.nodeId === event.data.nodeId ? { ...metric, ...event.data } : metric
        );
        set({ nodeMetrics: updatedMetrics });
      });

      // Set up periodic health checks
      const checkConnection = () => {
        const isConnected = finDAGApi.isConnected();
        const status = finDAGApi.getConnectionStatus();
        set({ isConnected, connectionStatus: status });
        
        if (!isConnected && status === 'disconnected') {
          // Attempt to reconnect
          setTimeout(() => {
            finDAGApi.connectWebSocket();
          }, 5000);
        }
      };

      // Check connection every 30 seconds
      const interval = setInterval(checkConnection, 30000);
      
      // Store interval for cleanup
      set({ connectionCheckInterval: interval });

    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
      setConnectionStatus('disconnected');
    }
  },

  disconnectWebSocket: () => {
    const { setConnectionStatus } = get();
    
    try {
      finDAGApi.disconnectWebSocket();
      setConnectionStatus('disconnected');
      set({ isConnected: false });
      
      // Clear connection check interval
      const { connectionCheckInterval } = get();
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