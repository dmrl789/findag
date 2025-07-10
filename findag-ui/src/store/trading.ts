import { create } from 'zustand';
import { finDAGApi } from '../services/api';
import {
  TradingPair,
  PricePoint,
  Trade,
  OrderBook,
  MarketOrder,
  Asset,
} from '../types';

interface TradingState {
  // Trading data
  tradingPairs: TradingPair[];
  selectedPair: string | null;
  priceHistory: PricePoint[];
  recentTrades: Trade[];
  orderBook: OrderBook | null;
  
  // User data
  userOrders: MarketOrder[];
  userBalance: { [asset: string]: number };
  availableAssets: Asset[];
  
  // Loading states
  isLoading: {
    tradingPairs: boolean;
    priceHistory: boolean;
    recentTrades: boolean;
    orderBook: boolean;
    userOrders: boolean;
    userBalance: boolean;
    placingOrder: boolean;
  };
  
  // Error states
  errors: {
    tradingPairs?: string;
    priceHistory?: string;
    recentTrades?: string;
    orderBook?: string;
    userOrders?: string;
    userBalance?: string;
    placingOrder?: string;
  };
  
  // Actions
  setSelectedPair: (pair: string) => void;
  setTradingPairs: (pairs: TradingPair[]) => void;
  setPriceHistory: (history: PricePoint[]) => void;
  setRecentTrades: (trades: Trade[]) => void;
  setOrderBook: (orderBook: OrderBook) => void;
  setUserOrders: (orders: MarketOrder[]) => void;
  setUserBalance: (balance: { [asset: string]: number }) => void;
  setAvailableAssets: (assets: Asset[]) => void;
  
  // Loading actions
  setLoading: (key: keyof TradingState['isLoading'], loading: boolean) => void;
  setError: (key: keyof TradingState['errors'], error?: string) => void;
  clearErrors: () => void;
  
  // API actions
  fetchTradingPairs: () => Promise<void>;
  fetchPriceHistory: (pair: string, timeFrame?: string) => Promise<void>;
  fetchRecentTrades: (pair: string) => Promise<void>;
  fetchOrderBook: (pair: string) => Promise<void>;
  fetchUserOrders: (userId: string) => Promise<void>;
  fetchUserBalance: (userId: string) => Promise<void>;
  fetchAvailableAssets: () => Promise<void>;
  placeOrder: (order: Omit<MarketOrder, 'id' | 'timestamp' | 'status' | 'filledAmount' | 'averagePrice'>) => Promise<MarketOrder>;
  cancelOrder: (orderId: string) => Promise<void>;
  
  // Real-time updates
  addTrade: (trade: Trade) => void;
  updateOrderBook: (orderBook: OrderBook) => void;
  updatePrice: (pricePoint: PricePoint) => void;
  updateOrderStatus: (orderId: string, status: MarketOrder['status']) => void;
}

export const useTradingStore = create<TradingState>((set, get) => ({
  // Initial state
  tradingPairs: [],
  selectedPair: null,
  priceHistory: [],
  recentTrades: [],
  orderBook: null,
  userOrders: [],
  userBalance: {},
  availableAssets: [],
  isLoading: {
    tradingPairs: false,
    priceHistory: false,
    recentTrades: false,
    orderBook: false,
    userOrders: false,
    userBalance: false,
    placingOrder: false,
  },
  errors: {},

  // Data setters
  setSelectedPair: (pair) => set({ selectedPair: pair }),
  setTradingPairs: (pairs) => set({ tradingPairs: pairs }),
  setPriceHistory: (history) => set({ priceHistory: history }),
  setRecentTrades: (trades) => set({ recentTrades: trades }),
  setOrderBook: (orderBook) => set({ orderBook }),
  setUserOrders: (orders) => set({ userOrders: orders }),
  setUserBalance: (balance) => set({ userBalance: balance }),
  setAvailableAssets: (assets) => set({ availableAssets: assets }),

  // Loading actions
  setLoading: (key, loading) => set((state) => ({
    isLoading: { ...state.isLoading, [key]: loading }
  })),

  // Error actions
  setError: (key, error) => set((state) => ({
    errors: { ...state.errors, [key]: error }
  })),
  clearErrors: () => set({ errors: {} }),

  // API actions - Real Backend Integration
  fetchTradingPairs: async () => {
    const { setLoading, setError, setTradingPairs } = get();
    
    setLoading('tradingPairs', true);
    setError('tradingPairs');
    
    try {
      const pairs = await finDAGApi.getTradingPairs();
      setTradingPairs(pairs);
    } catch (error: any) {
      console.error('Failed to fetch trading pairs:', error);
      setError('tradingPairs', error.message || 'Failed to fetch trading pairs');
    } finally {
      setLoading('tradingPairs', false);
    }
  },

  fetchPriceHistory: async (pair: string, timeFrame: string = '1h') => {
    const { setLoading, setError, setPriceHistory } = get();
    
    setLoading('priceHistory', true);
    setError('priceHistory');
    
    try {
      const history = await finDAGApi.getPriceHistory(pair, timeFrame);
      setPriceHistory(history);
    } catch (error: any) {
      console.error('Failed to fetch price history:', error);
      setError('priceHistory', error.message || 'Failed to fetch price history');
    } finally {
      setLoading('priceHistory', false);
    }
  },

  fetchRecentTrades: async (pair: string) => {
    const { setLoading, setError, setRecentTrades } = get();
    
    setLoading('recentTrades', true);
    setError('recentTrades');
    
    try {
      const trades = await finDAGApi.getRecentTrades(pair);
      setRecentTrades(trades);
    } catch (error: any) {
      console.error('Failed to fetch recent trades:', error);
      setError('recentTrades', error.message || 'Failed to fetch recent trades');
    } finally {
      setLoading('recentTrades', false);
    }
  },

  fetchOrderBook: async (pair: string) => {
    const { setLoading, setError, setOrderBook } = get();
    
    setLoading('orderBook', true);
    setError('orderBook');
    
    try {
      const orderBook = await finDAGApi.getOrderBook(pair);
      setOrderBook(orderBook);
    } catch (error: any) {
      console.error('Failed to fetch order book:', error);
      setError('orderBook', error.message || 'Failed to fetch order book');
    } finally {
      setLoading('orderBook', false);
    }
  },

  fetchUserOrders: async (userId: string) => {
    const { setLoading, setError, setUserOrders } = get();
    
    setLoading('userOrders', true);
    setError('userOrders');
    
    try {
      const orders = await finDAGApi.getUserOrders(userId);
      setUserOrders(orders);
    } catch (error: any) {
      console.error('Failed to fetch user orders:', error);
      setError('userOrders', error.message || 'Failed to fetch user orders');
    } finally {
      setLoading('userOrders', false);
    }
  },

  fetchUserBalance: async (userId: string) => {
    const { setLoading, setError, setUserBalance } = get();
    
    setLoading('userBalance', true);
    setError('userBalance');
    
    try {
      const balance = await finDAGApi.getUserBalance(userId);
      setUserBalance(balance);
    } catch (error: any) {
      console.error('Failed to fetch user balance:', error);
      setError('userBalance', error.message || 'Failed to fetch user balance');
    } finally {
      setLoading('userBalance', false);
    }
  },

  fetchAvailableAssets: async () => {
    const { setLoading, setError, setAvailableAssets } = get();
    
    setLoading('userBalance', true);
    setError('userBalance');
    
    try {
      const assets = await finDAGApi.getAssets();
      setAvailableAssets(assets);
    } catch (error: any) {
      console.error('Failed to fetch available assets:', error);
      setError('userBalance', error.message || 'Failed to fetch available assets');
    } finally {
      setLoading('userBalance', false);
    }
  },

  placeOrder: async (order) => {
    const { setLoading, setError, fetchUserOrders } = get();
    
    setLoading('placingOrder', true);
    setError('placingOrder');
    
    try {
      // Convert MarketOrder to API format
      const apiOrder = {
        symbol: order.pair,
        side: order.side,
        order_type: order.type,
        quantity: order.amount,
        price: order.price,
        client_order_id: `order-${Date.now()}`,
        currency: 'USD'
      };
      
      const response = await finDAGApi.placeOrder(apiOrder);
      
      // Convert response back to MarketOrder format
      const placedOrder: MarketOrder = {
        id: response.order_id,
        pair: order.pair,
        side: order.side,
        amount: order.amount,
        price: order.price,
        type: order.type,
        status: response.status as any,
        timestamp: Date.now(),
        user: order.user || 'current-user',
        filledAmount: 0,
        averagePrice: 0,
      };
      
      // Refresh user orders after placing new order
      if (order.user) {
        await fetchUserOrders(order.user);
      }
      
      return placedOrder;
    } catch (error: any) {
      console.error('Failed to place order:', error);
      setError('placingOrder', error.message || 'Failed to place order');
      throw error;
    } finally {
      setLoading('placingOrder', false);
    }
  },

  cancelOrder: async (orderId: string) => {
    const { setError, fetchUserOrders } = get();
    
    try {
      await finDAGApi.cancelOrder(orderId);
      
      // Refresh user orders after cancellation
      const { userOrders } = get();
      if (userOrders.length > 0) {
        await fetchUserOrders(userOrders[0].user);
      }
    } catch (error: any) {
      console.error('Failed to cancel order:', error);
      setError('userOrders', error.message || 'Failed to cancel order');
      throw error;
    }
  },

  // Real-time updates
  addTrade: (trade) => set((state) => ({
    recentTrades: [trade, ...state.recentTrades.slice(0, 99)]
  })),

  updateOrderBook: (orderBook) => set({ orderBook }),

  updatePrice: (pricePoint) => set((state) => ({
    priceHistory: [...state.priceHistory.slice(-999), pricePoint]
  })),

  updateOrderStatus: (orderId, status) => set((state) => ({
    userOrders: state.userOrders.map(order =>
      order.id === orderId ? { ...order, status } : order
    )
  })),
})); 