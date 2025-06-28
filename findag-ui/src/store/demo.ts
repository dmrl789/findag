import { create } from 'zustand';
import { 
  demoNetworkMetrics, 
  demoNodeMetrics, 
  demoBlocks, 
  demoTransactions,
  demoPriceHistory,
  demoRecentTrades,
  demoOrderBook,
  demoTradingPairs
} from '../data/demo';

interface DemoState {
  // Demo data
  networkMetrics: typeof demoNetworkMetrics;
  nodeMetrics: typeof demoNodeMetrics;
  recentBlocks: typeof demoBlocks;
  recentTransactions: typeof demoTransactions;
  priceHistory: typeof demoPriceHistory;
  recentTrades: typeof demoRecentTrades;
  orderBook: typeof demoOrderBook;
  tradingPairs: typeof demoTradingPairs;
  
  // Demo actions
  updatePrice: () => void;
  addDemoTrade: () => void;
  updateOrderBook: () => void;
}

export const useDemoStore = create<DemoState>((set, get) => ({
  // Initial demo data
  networkMetrics: demoNetworkMetrics,
  nodeMetrics: demoNodeMetrics,
  recentBlocks: demoBlocks,
  recentTransactions: demoTransactions,
  priceHistory: demoPriceHistory,
  recentTrades: demoRecentTrades,
  orderBook: demoOrderBook,
  tradingPairs: demoTradingPairs,

  // Demo actions
  updatePrice: () => {
    const { priceHistory } = get();
    const newPrice = priceHistory[priceHistory.length - 1].price + (Math.random() - 0.5) * 100;
    const newPricePoint = {
      timestamp: Date.now(),
      price: newPrice,
      volume: 1000000 + Math.random() * 5000000,
      high: newPrice + Math.random() * 50,
      low: newPrice - Math.random() * 50,
      open: newPrice - Math.random() * 25,
      close: newPrice + Math.random() * 25,
    };

    set((state) => ({
      priceHistory: [...state.priceHistory.slice(1), newPricePoint],
      tradingPairs: state.tradingPairs.map(pair => 
        pair.symbol === 'BTC/USD' 
          ? { ...pair, price: newPrice }
          : pair
      ),
    }));
  },

  addDemoTrade: () => {
    const { recentTrades, tradingPairs } = get();
    const currentPrice = tradingPairs.find(p => p.symbol === 'BTC/USD')?.price || 43250;
    
    const newTrade = {
      id: `trade-${String(recentTrades.length + 1).padStart(3, '0')}`,
      pair: 'BTC/USD',
      price: currentPrice + (Math.random() - 0.5) * 50,
      amount: 0.1 + Math.random() * 2,
      side: Math.random() > 0.5 ? 'buy' : 'sell',
      timestamp: Date.now(),
      maker: `user-${String(Math.floor(Math.random() * 100)).padStart(3, '0')}`,
      taker: `user-${String(Math.floor(Math.random() * 100)).padStart(3, '0')}`,
      fee: 0.001,
    };

    set((state) => ({
      recentTrades: [newTrade, ...state.recentTrades.slice(0, 49)],
    }));
  },

  updateOrderBook: () => {
    const { orderBook } = get();
    const currentPrice = orderBook.bids[0].price + 25;
    
    const newOrderBook = {
      ...orderBook,
      bids: orderBook.bids.map((bid, i) => ({
        ...bid,
        price: currentPrice - (i + 1) * 5,
        amount: bid.amount + (Math.random() - 0.5) * 0.5,
      })),
      asks: orderBook.asks.map((ask, i) => ({
        ...ask,
        price: currentPrice + (i + 1) * 5,
        amount: ask.amount + (Math.random() - 0.5) * 0.5,
      })),
      lastUpdateId: orderBook.lastUpdateId + 1,
    };

    // Recalculate totals
    newOrderBook.bids = newOrderBook.bids.map((bid, i, arr) => ({
      ...bid,
      total: arr.slice(0, i + 1).reduce((sum, e) => sum + e.amount, 0),
    }));
    newOrderBook.asks = newOrderBook.asks.map((ask, i, arr) => ({
      ...ask,
      total: arr.slice(0, i + 1).reduce((sum, e) => sum + e.amount, 0),
    }));

    set({ orderBook: newOrderBook });
  },
}));

// Start demo updates
export const startDemoUpdates = () => {
  const updateInterval = setInterval(() => {
    const { updatePrice, addDemoTrade, updateOrderBook } = useDemoStore.getState();
    
    // Update price every 2 seconds
    updatePrice();
    
    // Add trade every 3 seconds
    if (Math.random() > 0.7) {
      addDemoTrade();
    }
    
    // Update order book every 5 seconds
    if (Math.random() > 0.8) {
      updateOrderBook();
    }
  }, 2000);

  return () => clearInterval(updateInterval);
}; 