import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

export interface Balance {
  asset: string;
  amount: number;
  available: number;
  locked: number;
  lastUpdated: number;
}

export interface Transaction {
  id: string;
  type: 'send' | 'receive' | 'trade' | 'fee';
  asset: string;
  amount: number;
  fee: number;
  status: 'pending' | 'confirmed' | 'failed';
  timestamp: number;
  blockHash?: string;
  roundNumber?: number;
  fromAddress?: string;
  toAddress?: string;
  memo?: string;
}

export interface WalletInfo {
  address: string;
  publicKey: string;
  handle?: string;
  totalBalance: number;
  transactionCount: number;
  lastActivity: number;
}

interface WalletContextType {
  walletInfo: WalletInfo | null;
  balances: Balance[];
  transactions: Transaction[];
  isLoading: boolean;
  error: string | null;
  createWallet: () => Promise<void>;
  importWallet: (privateKey: string) => Promise<void>;
  sendTransaction: (toAddress: string, asset: string, amount: number, memo?: string) => Promise<void>;
  getBalances: () => Promise<void>;
  getTransactions: () => Promise<void>;
  refreshWallet: () => Promise<void>;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

interface WalletProviderProps {
  children: ReactNode;
}

export const WalletProvider: React.FC<WalletProviderProps> = ({ children }) => {
  const [walletInfo, setWalletInfo] = useState<WalletInfo | null>(null);
  const [balances, setBalances] = useState<Balance[]>([]);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Check for existing wallet
    checkExistingWallet();
  }, []);

  const checkExistingWallet = async () => {
    try {
      const savedWallet = localStorage.getItem('findag-wallet');
      if (savedWallet) {
        const wallet = JSON.parse(savedWallet);
        setWalletInfo(wallet);
        await Promise.all([getBalances(), getTransactions()]);
      }
    } catch (err) {
      console.error('Failed to load existing wallet:', err);
    }
  };

  const createWallet = async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to create wallet
      console.log('Creating new wallet...');
      
      // Simulate wallet creation
      const mockWallet: WalletInfo = {
        address: 'fdg1q' + Math.random().toString(36).substr(2, 40),
        publicKey: 'mock-public-key-' + Date.now(),
        handle: '@user.' + Math.random().toString(36).substr(2, 8),
        totalBalance: 0,
        transactionCount: 0,
        lastActivity: Date.now(),
      };
      
      localStorage.setItem('findag-wallet', JSON.stringify(mockWallet));
      setWalletInfo(mockWallet);
      
      // Initialize with empty balances
      setBalances([]);
      setTransactions([]);
      
      setIsLoading(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create wallet');
      setIsLoading(false);
    }
  };

  const importWallet = async (_privateKey: string) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to import wallet
      console.log('Importing wallet...');
      
      // Simulate wallet import
      const mockWallet: WalletInfo = {
        address: 'fdg1q' + Math.random().toString(36).substr(2, 40),
        publicKey: 'imported-public-key-' + Date.now(),
        handle: '@imported.user',
        totalBalance: 1000,
        transactionCount: 5,
        lastActivity: Date.now(),
      };
      
      localStorage.setItem('findag-wallet', JSON.stringify(mockWallet));
      setWalletInfo(mockWallet);
      
      await Promise.all([getBalances(), getTransactions()]);
      
      setIsLoading(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to import wallet');
      setIsLoading(false);
    }
  };

  const sendTransaction = async (toAddress: string, asset: string, amount: number, memo?: string) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to send transaction
      console.log('Sending transaction...', { toAddress, asset, amount, memo });
      
      const newTransaction: Transaction = {
        id: `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'send',
        asset,
        amount: -amount,
        fee: 0.001,
        status: 'pending',
        timestamp: Date.now(),
        toAddress,
        memo,
      };
      
      setTransactions(prev => [newTransaction, ...prev]);
      
      // Simulate transaction confirmation
      setTimeout(() => {
        setTransactions(prev => 
          prev.map(tx => 
            tx.id === newTransaction.id 
              ? { ...tx, status: 'confirmed', blockHash: 'block_' + Date.now() }
              : tx
          )
        );
      }, 3000);
      
      setIsLoading(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to send transaction');
      setIsLoading(false);
    }
  };

  const getBalances = async () => {
    try {
      // TODO: Call backend to get balances
      // For now, using mock data
      const mockBalances: Balance[] = [
        {
          asset: 'EUR',
          amount: 10000,
          available: 9500,
          locked: 500,
          lastUpdated: Date.now(),
        },
        {
          asset: 'USD',
          amount: 5000,
          available: 4800,
          locked: 200,
          lastUpdated: Date.now(),
        },
        {
          asset: 'JPY',
          amount: 1000000,
          available: 990000,
          locked: 10000,
          lastUpdated: Date.now(),
        },
      ];
      
      setBalances(mockBalances);
    } catch (err) {
      console.error('Failed to get balances:', err);
    }
  };

  const getTransactions = async () => {
    try {
      // TODO: Call backend to get transactions
      // For now, using mock data
      const mockTransactions: Transaction[] = [
        {
          id: 'tx_1',
          type: 'receive',
          asset: 'EUR',
          amount: 1000,
          fee: 0,
          status: 'confirmed',
          timestamp: Date.now() - 3600000,
          blockHash: 'block_123',
          roundNumber: 1000,
          fromAddress: 'fdg1qsender123',
          toAddress: walletInfo?.address || '',
        },
        {
          id: 'tx_2',
          type: 'send',
          asset: 'USD',
          amount: -500,
          fee: 0.001,
          status: 'confirmed',
          timestamp: Date.now() - 1800000,
          blockHash: 'block_124',
          roundNumber: 1001,
          fromAddress: walletInfo?.address || '',
          toAddress: 'fdg1qrecipient456',
          memo: 'Payment for services',
        },
      ];
      
      setTransactions(mockTransactions);
    } catch (err) {
      console.error('Failed to get transactions:', err);
    }
  };

  const refreshWallet = async () => {
    try {
      await Promise.all([getBalances(), getTransactions()]);
    } catch (err) {
      console.error('Failed to refresh wallet:', err);
    }
  };

  const value: WalletContextType = {
    walletInfo,
    balances,
    transactions,
    isLoading,
    error,
    createWallet,
    importWallet,
    sendTransaction,
    getBalances,
    getTransactions,
    refreshWallet,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = (): WalletContextType => {
  const context = useContext(WalletContext);
  if (context === undefined) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
}; 