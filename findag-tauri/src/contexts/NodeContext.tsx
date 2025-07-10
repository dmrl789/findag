import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

export interface NodeStatus {
  isRunning: boolean;
  isConnected: boolean;
  uptime: number;
  peers: number;
  tps: number;
  blocksPerSecond: number;
  mempoolSize: number;
  lastBlockHash?: string;
  lastRoundNumber?: number;
  version: string;
}

export interface NodeConfig {
  port: number;
  peers: string[];
  dataDirectory: string;
  maxBlockSize: number;
  blockInterval: number;
  roundInterval: number;
}

interface NodeContextType {
  status: NodeStatus;
  config: NodeConfig;
  isLoading: boolean;
  error: string | null;
  startNode: () => Promise<void>;
  stopNode: () => Promise<void>;
  restartNode: () => Promise<void>;
  updateConfig: (config: Partial<NodeConfig>) => Promise<void>;
  refreshStatus: () => Promise<void>;
}

const NodeContext = createContext<NodeContextType | undefined>(undefined);

interface NodeProviderProps {
  children: ReactNode;
}

export const NodeProvider: React.FC<NodeProviderProps> = ({ children }) => {
  const [status, setStatus] = useState<NodeStatus>({
    isRunning: false,
    isConnected: false,
    uptime: 0,
    peers: 0,
    tps: 0,
    blocksPerSecond: 0,
    mempoolSize: 0,
    version: '1.0.0',
  });

  const [config, setConfig] = useState<NodeConfig>({
    port: 8080,
    peers: [],
    dataDirectory: './data',
    maxBlockSize: 32768,
    blockInterval: 50,
    roundInterval: 250,
  });

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Load initial config and status
    loadInitialState();
  }, []);

  const loadInitialState = async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Load config from backend
      // For now, using default config
      
      // TODO: Get initial status from backend
      await refreshStatus();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load node state');
    } finally {
      setIsLoading(false);
    }
  };

  const startNode = async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to start node
      console.log('Starting FinDAG node...');
      
      // Simulate node start
      setTimeout(() => {
        setStatus(prev => ({
          ...prev,
          isRunning: true,
          isConnected: true,
          uptime: 0,
          peers: 0,
          tps: 0,
          blocksPerSecond: 0,
          mempoolSize: 0,
        }));
        setIsLoading(false);
      }, 2000);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to start node');
      setIsLoading(false);
    }
  };

  const stopNode = async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to stop node
      console.log('Stopping FinDAG node...');
      
      // Simulate node stop
      setTimeout(() => {
        setStatus(prev => ({
          ...prev,
          isRunning: false,
          isConnected: false,
          uptime: 0,
          peers: 0,
          tps: 0,
          blocksPerSecond: 0,
          mempoolSize: 0,
        }));
        setIsLoading(false);
      }, 1000);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to stop node');
      setIsLoading(false);
    }
  };

  const restartNode = async () => {
    try {
      await stopNode();
      await new Promise(resolve => setTimeout(resolve, 1000));
      await startNode();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to restart node');
    }
  };

  const updateConfig = async (newConfig: Partial<NodeConfig>) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to update config
      console.log('Updating node config...', newConfig);
      
      setConfig(prev => ({ ...prev, ...newConfig }));
      setIsLoading(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update config');
      setIsLoading(false);
    }
  };

  const refreshStatus = async () => {
    try {
      // TODO: Call backend to get current status
      // For now, simulate status updates
      if (status.isRunning) {
        setStatus(prev => ({
          ...prev,
          uptime: prev.uptime + 1,
          tps: Math.floor(Math.random() * 1000) + 100,
          blocksPerSecond: Math.floor(Math.random() * 20) + 5,
          mempoolSize: Math.floor(Math.random() * 1000) + 100,
          peers: Math.floor(Math.random() * 10) + 1,
        }));
      }
    } catch (err) {
      console.error('Failed to refresh status:', err);
    }
  };

  // Auto-refresh status every 5 seconds when node is running
  useEffect(() => {
    if (!status.isRunning) return;

    const interval = setInterval(refreshStatus, 5000);
    return () => clearInterval(interval);
  }, [status.isRunning]);

  const value: NodeContextType = {
    status,
    config,
    isLoading,
    error,
    startNode,
    stopNode,
    restartNode,
    updateConfig,
    refreshStatus,
  };

  return (
    <NodeContext.Provider value={value}>
      {children}
    </NodeContext.Provider>
  );
};

export const useNode = (): NodeContextType => {
  const context = useContext(NodeContext);
  if (context === undefined) {
    throw new Error('useNode must be used within a NodeProvider');
  }
  return context;
}; 