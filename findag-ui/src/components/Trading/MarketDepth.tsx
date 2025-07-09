import React, { useState, useEffect } from 'react';
import { 
  BarChart3, 
  TrendingUp, 
  TrendingDown, 
  DollarSign, 
  Percent,
  RefreshCw,
  Settings,
  Eye,
  EyeOff
} from 'lucide-react';
import { OrderBook, OrderBookEntry } from '../../types';

interface MarketDepthProps {
  orderBook: OrderBook | null;
  pair: string;
  className?: string;
}

export const MarketDepth: React.FC<MarketDepthProps> = ({ 
  orderBook, 
  pair, 
  className = '' 
}) => {
  const [depth, setDepth] = useState(20);
  const [showSettings, setShowSettings] = useState(false);
  const [showCumulative, setShowCumulative] = useState(true);
  const [priceRange, setPriceRange] = useState(5); // percentage
  const [loading, setLoading] = useState(false);

  // Mock data - in real app this would come from API
  useEffect(() => {
    if (!orderBook) {
      // Generate mock order book data
      const mockOrderBook: OrderBook = {
        pair: pair,
        bids: [],
        asks: [],
        lastUpdateId: Date.now(),
      };

      // Generate bids (buy orders)
      const basePrice = 50000; // Mock base price
      for (let i = 0; i < 50; i++) {
        const price = basePrice - (i * 10);
        const amount = Math.random() * 2 + 0.1;
        const total = mockOrderBook.bids.length > 0 
          ? mockOrderBook.bids[mockOrderBook.bids.length - 1].total + amount
          : amount;
        
        mockOrderBook.bids.push({
          price,
          amount,
          total,
        });
      }

      // Generate asks (sell orders)
      for (let i = 0; i < 50; i++) {
        const price = basePrice + (i * 10);
        const amount = Math.random() * 2 + 0.1;
        const total = mockOrderBook.asks.length > 0 
          ? mockOrderBook.asks[mockOrderBook.asks.length - 1].total + amount
          : amount;
        
        mockOrderBook.asks.push({
          price,
          amount,
          total,
        });
      }

      // Sort bids in descending order and asks in ascending order
      mockOrderBook.bids.sort((a, b) => b.price - a.price);
      mockOrderBook.asks.sort((a, b) => a.price - b.price);

      setOrderBook(mockOrderBook);
    }
  }, [orderBook, pair]);

  const [currentOrderBook, setOrderBook] = useState<OrderBook | null>(orderBook);

  const handleRefresh = async () => {
    setLoading(true);
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    setLoading(false);
  };

  const getSpread = () => {
    if (!currentOrderBook || currentOrderBook.bids.length === 0 || currentOrderBook.asks.length === 0) {
      return { spread: 0, spreadPercent: 0 };
    }

    const bestBid = currentOrderBook.bids[0].price;
    const bestAsk = currentOrderBook.asks[0].price;
    const spread = bestAsk - bestBid;
    const spreadPercent = (spread / bestBid) * 100;

    return { spread, spreadPercent };
  };

  const getMidPrice = () => {
    if (!currentOrderBook || currentOrderBook.bids.length === 0 || currentOrderBook.asks.length === 0) {
      return 0;
    }

    const bestBid = currentOrderBook.bids[0].price;
    const bestAsk = currentOrderBook.asks[0].price;
    return (bestBid + bestAsk) / 2;
  };

  const getFilteredOrders = (orders: OrderBookEntry[], isBid: boolean) => {
    if (!currentOrderBook) return [];

    const midPrice = getMidPrice();
    const priceThreshold = midPrice * (priceRange / 100);

    return orders
      .filter(order => {
        if (isBid) {
          return order.price >= midPrice - priceThreshold;
        } else {
          return order.price <= midPrice + priceThreshold;
        }
      })
      .slice(0, depth);
  };

  const getMaxTotal = () => {
    const bids = getFilteredOrders(currentOrderBook?.bids || [], true);
    const asks = getFilteredOrders(currentOrderBook?.asks || [], false);
    
    const maxBidTotal = bids.length > 0 ? Math.max(...bids.map(b => b.total)) : 0;
    const maxAskTotal = asks.length > 0 ? Math.max(...asks.map(a => a.total)) : 0;
    
    return Math.max(maxBidTotal, maxAskTotal);
  };

  const formatPrice = (price: number) => {
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(price);
  };

  const formatAmount = (amount: number) => {
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 4,
      maximumFractionDigits: 4,
    }).format(amount);
  };

  const getBarWidth = (total: number, maxTotal: number) => {
    return Math.max((total / maxTotal) * 100, 1);
  };

  const { spread, spreadPercent } = getSpread();
  const midPrice = getMidPrice();
  const maxTotal = getMaxTotal();
  const filteredBids = getFilteredOrders(currentOrderBook?.bids || [], true);
  const filteredAsks = getFilteredOrders(currentOrderBook?.asks || [], false);

  if (loading) {
    return (
      <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 ${className}`}>
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/4 mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3, 4, 5].map(i => (
              <div key={i} className="h-8 bg-gray-200 dark:bg-gray-700 rounded"></div>
            ))}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg ${className}`}>
      {/* Header */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
              Market Depth
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              {pair} - Order Book Analysis
            </p>
          </div>
          <div className="flex items-center space-x-2">
            <button
              onClick={() => setShowCumulative(!showCumulative)}
              className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              {showCumulative ? <Eye className="w-5 h-5" /> : <EyeOff className="w-5 h-5" />}
            </button>
            <button
              onClick={() => setShowSettings(!showSettings)}
              className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              <Settings className="w-5 h-5" />
            </button>
            <button
              onClick={handleRefresh}
              className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              <RefreshCw className="w-5 h-5" />
            </button>
          </div>
        </div>
      </div>

      {/* Market Statistics */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <DollarSign className="w-6 h-6 text-blue-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Mid Price</p>
                <p className="text-lg font-semibold text-gray-900 dark:text-white">
                  ${formatPrice(midPrice)}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <TrendingUp className="w-6 h-6 text-green-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Best Bid</p>
                <p className="text-lg font-semibold text-green-600">
                  ${currentOrderBook?.bids[0] ? formatPrice(currentOrderBook.bids[0].price) : '0.00'}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <TrendingDown className="w-6 h-6 text-red-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Best Ask</p>
                <p className="text-lg font-semibold text-red-600">
                  ${currentOrderBook?.asks[0] ? formatPrice(currentOrderBook.asks[0].price) : '0.00'}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <Percent className="w-6 h-6 text-purple-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Spread</p>
                <p className="text-lg font-semibold text-gray-900 dark:text-white">
                  ${formatPrice(spread)} ({spreadPercent.toFixed(3)}%)
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Settings Panel */}
      {showSettings && (
        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Depth Level
              </label>
              <select
                value={depth}
                onChange={(e) => setDepth(parseInt(e.target.value))}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
              >
                <option value={10}>10 levels</option>
                <option value={20}>20 levels</option>
                <option value={50}>50 levels</option>
                <option value={100}>100 levels</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Price Range (%)
              </label>
              <select
                value={priceRange}
                onChange={(e) => setPriceRange(parseInt(e.target.value))}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
              >
                <option value={1}>1%</option>
                <option value={2}>2%</option>
                <option value={5}>5%</option>
                <option value={10}>10%</option>
                <option value={20}>20%</option>
              </select>
            </div>

            <div className="flex items-end">
              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={showCumulative}
                  onChange={(e) => setShowCumulative(e.target.checked)}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                  Show Cumulative Volume
                </span>
              </label>
            </div>
          </div>
        </div>
      )}

      {/* Market Depth Visualization */}
      <div className="p-6">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Asks (Sell Orders) */}
          <div>
            <h3 className="text-lg font-semibold text-red-600 mb-4 flex items-center">
              <TrendingDown className="w-5 h-5 mr-2" />
              Asks (Sell Orders)
            </h3>
            <div className="space-y-2">
              {filteredAsks.map((ask, index) => (
                <div key={index} className="flex items-center space-x-2">
                  <div className="flex-1 flex items-center space-x-2">
                    <div 
                      className="bg-red-100 dark:bg-red-900/20 h-6 rounded transition-all duration-200"
                      style={{ width: `${getBarWidth(showCumulative ? ask.total : ask.amount, maxTotal)}%` }}
                    />
                    <span className="text-sm font-medium text-red-600 min-w-[80px]">
                      ${formatPrice(ask.price)}
                    </span>
                  </div>
                  <span className="text-sm text-gray-600 dark:text-gray-400 min-w-[80px] text-right">
                    {formatAmount(ask.amount)}
                  </span>
                  {showCumulative && (
                    <span className="text-sm text-gray-500 dark:text-gray-500 min-w-[80px] text-right">
                      {formatAmount(ask.total)}
                    </span>
                  )}
                </div>
              ))}
            </div>
          </div>

          {/* Bids (Buy Orders) */}
          <div>
            <h3 className="text-lg font-semibold text-green-600 mb-4 flex items-center">
              <TrendingUp className="w-5 h-5 mr-2" />
              Bids (Buy Orders)
            </h3>
            <div className="space-y-2">
              {filteredBids.map((bid, index) => (
                <div key={index} className="flex items-center space-x-2">
                  <div className="flex-1 flex items-center space-x-2">
                    <div 
                      className="bg-green-100 dark:bg-green-900/20 h-6 rounded transition-all duration-200"
                      style={{ width: `${getBarWidth(showCumulative ? bid.total : bid.amount, maxTotal)}%` }}
                    />
                    <span className="text-sm font-medium text-green-600 min-w-[80px]">
                      ${formatPrice(bid.price)}
                    </span>
                  </div>
                  <span className="text-sm text-gray-600 dark:text-gray-400 min-w-[80px] text-right">
                    {formatAmount(bid.amount)}
                  </span>
                  {showCumulative && (
                    <span className="text-sm text-gray-500 dark:text-gray-500 min-w-[80px] text-right">
                      {formatAmount(bid.total)}
                    </span>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Legend */}
        <div className="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2">
                <div className="w-4 h-4 bg-red-100 dark:bg-red-900/20 rounded"></div>
                <span>Asks</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-4 h-4 bg-green-100 dark:bg-green-900/20 rounded"></div>
                <span>Bids</span>
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <span>Price</span>
              <span>Amount</span>
              {showCumulative && <span>Total</span>}
            </div>
          </div>
        </div>
      </div>

      {/* Liquidity Analysis */}
      <div className="p-6 border-t border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Liquidity Analysis
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
            <div className="flex items-center">
              <BarChart3 className="w-6 h-6 text-blue-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Total Bid Liquidity</p>
                <p className="text-lg font-semibold text-blue-600">
                  {formatAmount(filteredBids.reduce((sum, bid) => sum + bid.amount, 0))}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-purple-50 dark:bg-purple-900/20 rounded-lg p-4">
            <div className="flex items-center">
              <BarChart3 className="w-6 h-6 text-purple-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Total Ask Liquidity</p>
                <p className="text-lg font-semibold text-purple-600">
                  {formatAmount(filteredAsks.reduce((sum, ask) => sum + ask.amount, 0))}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <BarChart3 className="w-6 h-6 text-gray-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Liquidity Ratio</p>
                <p className="text-lg font-semibold text-gray-900 dark:text-white">
                  {(filteredBids.reduce((sum, bid) => sum + bid.amount, 0) / 
                    filteredAsks.reduce((sum, ask) => sum + ask.amount, 0)).toFixed(2)}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}; 