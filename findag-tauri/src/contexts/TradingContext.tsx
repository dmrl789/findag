import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

export interface Order {
  id: string;
  symbol: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit' | 'stop' | 'stop-limit';
  quantity: number;
  price?: number;
  stopPrice?: number;
  status: 'pending' | 'filled' | 'cancelled' | 'rejected';
  timestamp: number;
  filledQuantity?: number;
  averagePrice?: number;
}

export interface Position {
  symbol: string;
  quantity: number;
  averagePrice: number;
  unrealizedPnL: number;
  realizedPnL: number;
  timestamp: number;
}

export interface MarketData {
  symbol: string;
  lastPrice: number;
  bid: number;
  ask: number;
  volume: number;
  change: number;
  changePercent: number;
  high: number;
  low: number;
  timestamp: number;
}

interface TradingContextType {
  orders: Order[];
  positions: Position[];
  marketData: Record<string, MarketData>;
  isLoading: boolean;
  error: string | null;
  placeOrder: (order: Omit<Order, 'id' | 'status' | 'timestamp'>) => Promise<void>;
  cancelOrder: (orderId: string) => Promise<void>;
  getOrders: () => Promise<void>;
  getPositions: () => Promise<void>;
  getMarketData: (symbol: string) => Promise<void>;
  refreshAll: () => Promise<void>;
}

const TradingContext = createContext<TradingContextType | undefined>(undefined);

interface TradingProviderProps {
  children: ReactNode;
}

export const TradingProvider: React.FC<TradingProviderProps> = ({ children }) => {
  const [orders, setOrders] = useState<Order[]>([]);
  const [positions, setPositions] = useState<Position[]>([]);
  const [marketData, setMarketData] = useState<Record<string, MarketData>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Load initial data
    loadInitialData();
  }, []);

  const loadInitialData = async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      await Promise.all([
        getOrders(),
        getPositions(),
        getMarketData('EUR/USD'),
        getMarketData('USD/JPY'),
        getMarketData('GBP/USD'),
      ]);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load trading data');
    } finally {
      setIsLoading(false);
    }
  };

  const placeOrder = async (orderData: Omit<Order, 'id' | 'status' | 'timestamp'>) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to place order
      console.log('Placing order...', orderData);
      
      const newOrder: Order = {
        ...orderData,
        id: `order_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        status: 'pending',
        timestamp: Date.now(),
      };
      
      setOrders(prev => [newOrder, ...prev]);
      
      // Simulate order processing
      setTimeout(() => {
        setOrders(prev => 
          prev.map(order => 
            order.id === newOrder.id 
              ? { ...order, status: 'filled', filledQuantity: orderData.quantity, averagePrice: orderData.price || 1.0 }
              : order
          )
        );
      }, 2000);
      
      setIsLoading(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to place order');
      setIsLoading(false);
    }
  };

  const cancelOrder = async (orderId: string) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // TODO: Call backend to cancel order
      console.log('Cancelling order...', orderId);
      
      setOrders(prev => 
        prev.map(order => 
          order.id === orderId 
            ? { ...order, status: 'cancelled' }
            : order
        )
      );
      
      setIsLoading(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to cancel order');
      setIsLoading(false);
    }
  };

  const getOrders = async () => {
    try {
      // TODO: Call backend to get orders
      // For now, using mock data
      const mockOrders: Order[] = [
        {
          id: 'order_1',
          symbol: 'EUR/USD',
          side: 'buy',
          type: 'limit',
          quantity: 1000,
          price: 1.0850,
          status: 'filled',
          timestamp: Date.now() - 3600000,
          filledQuantity: 1000,
          averagePrice: 1.0850,
        },
        {
          id: 'order_2',
          symbol: 'USD/JPY',
          side: 'sell',
          type: 'market',
          quantity: 500,
          status: 'pending',
          timestamp: Date.now() - 1800000,
        },
      ];
      
      setOrders(mockOrders);
    } catch (err) {
      console.error('Failed to get orders:', err);
    }
  };

  const getPositions = async () => {
    try {
      // TODO: Call backend to get positions
      // For now, using mock data
      const mockPositions: Position[] = [
        {
          symbol: 'EUR/USD',
          quantity: 1000,
          averagePrice: 1.0850,
          unrealizedPnL: 25.50,
          realizedPnL: 0,
          timestamp: Date.now() - 3600000,
        },
      ];
      
      setPositions(mockPositions);
    } catch (err) {
      console.error('Failed to get positions:', err);
    }
  };

  const getMarketData = async (symbol: string) => {
    try {
      // TODO: Call backend to get market data
      // For now, using mock data
      const mockMarketData: MarketData = {
        symbol,
        lastPrice: 1.0850 + (Math.random() - 0.5) * 0.01,
        bid: 1.0848,
        ask: 1.0852,
        volume: Math.floor(Math.random() * 1000000) + 100000,
        change: (Math.random() - 0.5) * 0.01,
        changePercent: (Math.random() - 0.5) * 2,
        high: 1.0900,
        low: 1.0800,
        timestamp: Date.now(),
      };
      
      setMarketData(prev => ({
        ...prev,
        [symbol]: mockMarketData,
      }));
    } catch (err) {
      console.error('Failed to get market data:', err);
    }
  };

  const refreshAll = async () => {
    try {
      await Promise.all([
        getOrders(),
        getPositions(),
        ...Object.keys(marketData).map(symbol => getMarketData(symbol)),
      ]);
    } catch (err) {
      console.error('Failed to refresh trading data:', err);
    }
  };

  // Auto-refresh market data every 5 seconds
  useEffect(() => {
    const interval = setInterval(() => {
      Object.keys(marketData).forEach(symbol => {
        getMarketData(symbol);
      });
    }, 5000);
    
    return () => clearInterval(interval);
  }, [marketData]);

  const value: TradingContextType = {
    orders,
    positions,
    marketData,
    isLoading,
    error,
    placeOrder,
    cancelOrder,
    getOrders,
    getPositions,
    getMarketData,
    refreshAll,
  };

  return (
    <TradingContext.Provider value={value}>
      {children}
    </TradingContext.Provider>
  );
};

export const useTrading = (): TradingContextType => {
  const context = useContext(TradingContext);
  if (context === undefined) {
    throw new Error('useTrading must be used within a TradingProvider');
  }
  return context;
}; 