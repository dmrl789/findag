import { create } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import {
  UIState,
  NetworkMetrics,
  NodeMetrics,
  Block,
  Transaction,
  Round,
  Validator,
  WebSocketEvent,
} from '../types';
import { finDAGApi } from '../services/api';

interface AppState {
  // UI State
  ui: UIState;
  
  // Connection State
  isConnected: boolean;
  connectionStatus: 'connected' | 'disconnected' | 'connecting';
  
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

const initialState = {
  ui: {
    selectedTimeRange: '1h' as const,
    autoRefresh: true,
    theme: 'light' as const,
  },
  isConnected: false,
  connectionStatus: 'disconnected' as const,
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
};

export const useAppStore = create<AppState>()(
  devtools(
    subscribeWithSelector((set, get) => ({
      ...initialState,
      
      // UI Actions
      setUI: (ui) => set((state) => ({ ui: { ...state.ui, ...ui } })),
      
      // Connection Actions
      setConnectionStatus: (status) => set({ connectionStatus: status, isConnected: status === 'connected' }),
      
      // Data Actions
      setNetworkMetrics: (metrics) => set({ networkMetrics: metrics }),
      setNodeMetrics: (metrics) => set({ nodeMetrics: metrics }),
      addBlock: (block) => set((state) => ({
        recentBlocks: [block, ...state.recentBlocks.slice(0, 99)], // Keep last 100 blocks
      })),
      addTransaction: (transaction) => set((state) => ({
        recentTransactions: [transaction, ...state.recentTransactions.slice(0, 99)], // Keep last 100 transactions
      })),
      setCurrentRound: (round) => set({ currentRound: round }),
      setValidators: (validators) => set({ validators }),
      
      // Loading Actions
      setLoading: (key, loading) => set((state) => ({
        isLoading: { ...state.isLoading, [key]: loading },
      })),
      
      // Error Actions
      setError: (key, error) => set((state) => ({
        errors: { ...state.errors, [key]: error },
      })),
      clearErrors: () => set({ errors: {} }),
      
      // API Actions
      fetchNetworkMetrics: async () => {
        const { setLoading, setError, setNetworkMetrics } = get();
        setLoading('networkMetrics', true);
        setError('networkMetrics');
        
        try {
          const metrics = await finDAGApi.getNetworkMetrics();
          setNetworkMetrics(metrics);
        } catch (error) {
          setError('networkMetrics', error instanceof Error ? error.message : 'Failed to fetch network metrics');
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
        } catch (error) {
          setError('nodeMetrics', error instanceof Error ? error.message : 'Failed to fetch node metrics');
        } finally {
          setLoading('nodeMetrics', false);
        }
      },
      
      fetchRecentBlocks: async () => {
        const { setLoading, setError, addBlock } = get();
        setLoading('blocks', true);
        setError('blocks');
        
        try {
          const response = await finDAGApi.getBlocks(1, 20);
          set({ recentBlocks: response.data });
        } catch (error) {
          setError('blocks', error instanceof Error ? error.message : 'Failed to fetch blocks');
        } finally {
          setLoading('blocks', false);
        }
      },
      
      fetchRecentTransactions: async () => {
        const { setLoading, setError } = get();
        setLoading('transactions', true);
        setError('transactions');
        
        try {
          const response = await finDAGApi.getTransactions(1, 20);
          set({ recentTransactions: response.data });
        } catch (error) {
          setError('transactions', error instanceof Error ? error.message : 'Failed to fetch transactions');
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
        } catch (error) {
          setError('validators', error instanceof Error ? error.message : 'Failed to fetch validators');
        } finally {
          setLoading('validators', false);
        }
      },
      
      connectWebSocket: () => {
        const { setConnectionStatus } = get();
        setConnectionStatus('connecting');
        
        finDAGApi.connectWebSocket();
        
        // Set up event listeners
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
          // Update specific node metrics
          const { nodeMetrics, setNodeMetrics } = get();
          const updatedMetrics = nodeMetrics.map(metric => 
            metric.nodeId === event.data.nodeId ? event.data : metric
          );
          setNodeMetrics(updatedMetrics);
        });
        
        // Check connection status periodically
        const checkConnection = () => {
          const status = finDAGApi.getConnectionStatus();
          setConnectionStatus(status);
          
          if (status === 'connected') {
            // Fetch initial data
            get().fetchNetworkMetrics();
            get().fetchNodeMetrics();
            get().fetchRecentBlocks();
            get().fetchRecentTransactions();
            get().fetchValidators();
          }
        };
        
        // Check immediately and then every 5 seconds
        checkConnection();
        const interval = setInterval(checkConnection, 5000);
        
        // Store interval for cleanup
        (window as any).__findag_connection_interval = interval;
      },
      
      disconnectWebSocket: () => {
        const { setConnectionStatus } = get();
        finDAGApi.disconnectWebSocket();
        setConnectionStatus('disconnected');
        
        // Clear interval
        if ((window as any).__findag_connection_interval) {
          clearInterval((window as any).__findag_connection_interval);
        }
      },
    })),
    {
      name: 'findag-store',
    }
  )
); 