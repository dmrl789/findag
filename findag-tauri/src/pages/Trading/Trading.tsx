import React, { useState, useEffect, useCallback } from 'react';
import { tradingAPI, MarketData, Order } from '../../services/api';
import { showNotification } from '../../components/Common/NotificationSystem';
import LoadingSpinner from '../../components/Common/LoadingSpinner';

interface OrderForm {
  symbol: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit' | 'stop' | 'stop-limit';
  quantity: number;
  price?: number;
  stopPrice?: number;
}

const Trading: React.FC = () => {
  const [marketData, setMarketData] = useState<Record<string, MarketData>>({});
  const [orders, setOrders] = useState<Order[]>([]);
  const [tradingPairs, setTradingPairs] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedSymbol, setSelectedSymbol] = useState('EUR/USD');
  const [orderForm, setOrderForm] = useState<OrderForm>({
    symbol: 'EUR/USD',
    side: 'buy',
    type: 'limit',
    quantity: 0,
    price: 0,
  });
  const [activeTab, setActiveTab] = useState<'trading' | 'portfolio' | 'history'>('trading');
  const [error, setError] = useState<string | null>(null);

  const fetchTradingData = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [pairs, orderHistory] = await Promise.all([
        tradingAPI.getTradingPairs(),
        tradingAPI.getOrderHistory(),
      ]);
      setTradingPairs(pairs);
      setOrders(orderHistory);
      
      // Fetch market data for all pairs
      const marketDataPromises = pairs.map(async (pair) => {
        try {
          const data = await tradingAPI.getMarketData(pair);
          return [pair, data];
        } catch (err) {
          console.error(`Failed to fetch market data for ${pair}:`, err);
          return [pair, null];
        }
      });
      
      const marketDataResults = await Promise.all(marketDataPromises);
      const marketDataMap: Record<string, MarketData> = {};
      marketDataResults.forEach(([pair, data]) => {
        if (data) {
          marketDataMap[pair as string] = data as MarketData;
        }
      });
      setMarketData(marketDataMap);
    } catch (err) {
      setError('Failed to fetch trading data');
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchTradingData();
    const interval = setInterval(() => {
      fetchTradingData();
    }, 30000); // Refresh every 30 seconds
    return () => clearInterval(interval);
  }, [fetchTradingData]);

  const handleOrderSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!orderForm.quantity || orderForm.quantity <= 0) {
      showNotification({
        type: 'error',
        title: 'Invalid Order',
        message: 'Please enter a valid quantity',
      });
      return;
    }

    if (orderForm.type !== 'market' && (!orderForm.price || orderForm.price <= 0)) {
      showNotification({
        type: 'error',
        title: 'Invalid Order',
        message: 'Please enter a valid price',
      });
      return;
    }

    setIsLoading(true);
    try {
      await tradingAPI.placeOrder(
        orderForm.symbol,
        orderForm.side,
        orderForm.type,
        orderForm.quantity,
        orderForm.price,
        orderForm.stopPrice
      );

      showNotification({
        type: 'success',
        title: 'Order Placed',
        message: `${orderForm.side.toUpperCase()} ${orderForm.quantity} ${orderForm.symbol}`,
      });

      // Reset form
      setOrderForm({
        symbol: selectedSymbol,
        side: 'buy',
        type: 'limit',
        quantity: 0,
        price: marketData[selectedSymbol]?.last_price || 0,
      });
      
      // Refresh orders
      fetchTradingData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Order Failed',
        message: 'Failed to place order',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleCancelOrder = async (orderId: string) => {
    setIsLoading(true);
    try {
      await tradingAPI.cancelOrder(orderId);
      showNotification({
        type: 'success',
        title: 'Order Cancelled',
        message: 'Order has been cancelled',
      });
      fetchTradingData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Cancel Failed',
        message: 'Failed to cancel order',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const currentMarketData = marketData[selectedSymbol];
  // const pendingOrders = orders.filter(order => order.status === 'pending');
  // const filledOrders = orders.filter(order => order.status === 'filled');

  if (isLoading && !currentMarketData) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64 text-red-600">{error}</div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Trading</h1>
          <p className="text-gray-600 dark:text-gray-400">Advanced trading interface</p>
        </div>
        
        {/* Symbol Selector */}
        <div className="flex items-center space-x-4">
          <select
            value={selectedSymbol}
            onChange={(e) => {
              setSelectedSymbol(e.target.value);
              setOrderForm(prev => ({
                ...prev,
                symbol: e.target.value,
                price: marketData[e.target.value]?.last_price || 0,
              }));
            }}
            className="p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
          >
            {tradingPairs.map(symbol => (
              <option key={symbol} value={symbol}>{symbol}</option>
            ))}
          </select>
        </div>
      </div>

      {/* Market Data */}
      {currentMarketData && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Last Price</div>
              <div className="text-2xl font-bold text-gray-900 dark:text-white">
                {currentMarketData.last_price.toFixed(4)}
              </div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Change</div>
              <div className={`text-lg font-bold ${currentMarketData.change_percent >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                {currentMarketData.change_percent >= 0 ? '+' : ''}{currentMarketData.change_percent.toFixed(2)}%
              </div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Volume</div>
              <div className="text-lg font-bold text-gray-900 dark:text-white">
                {currentMarketData.volume.toLocaleString()}
              </div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">High/Low</div>
              <div className="text-sm text-gray-900 dark:text-white">
                {currentMarketData.high.toFixed(4)} / {currentMarketData.low.toFixed(4)}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Tabs */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
        <div className="border-b border-gray-200 dark:border-gray-700">
          <nav className="flex space-x-8 px-6">
            {(['trading', 'portfolio', 'history'] as const).map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={`py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === tab
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                }`}
              >
                {tab.charAt(0).toUpperCase() + tab.slice(1)}
              </button>
            ))}
          </nav>
        </div>

        <div className="p-6">
          {activeTab === 'trading' && (
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {/* Order Form */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Place Order</h3>
                <form onSubmit={handleOrderSubmit} className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Symbol</label>
                    <select
                      value={orderForm.symbol}
                      onChange={(e) => setOrderForm(prev => ({ ...prev, symbol: e.target.value }))}
                      className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                    >
                      {tradingPairs.map(symbol => (
                        <option key={symbol} value={symbol}>{symbol}</option>
                      ))}
                    </select>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Side</label>
                      <select
                        value={orderForm.side}
                        onChange={(e) => setOrderForm(prev => ({ ...prev, side: e.target.value as 'buy' | 'sell' }))}
                        className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                      >
                        <option value="buy">Buy</option>
                        <option value="sell">Sell</option>
                      </select>
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Type</label>
                      <select
                        value={orderForm.type}
                        onChange={(e) => setOrderForm(prev => ({ ...prev, type: e.target.value as any }))}
                        className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                      >
                        <option value="market">Market</option>
                        <option value="limit">Limit</option>
                        <option value="stop">Stop</option>
                        <option value="stop-limit">Stop Limit</option>
                      </select>
                    </div>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Quantity</label>
                    <input
                      type="number"
                      value={orderForm.quantity || ''}
                      onChange={(e) => setOrderForm(prev => ({ ...prev, quantity: parseFloat(e.target.value) || 0 }))}
                      className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                      placeholder="Enter quantity"
                      step="0.01"
                      min="0"
                    />
                  </div>

                  {orderForm.type !== 'market' && (
                    <div>
                      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Price</label>
                      <input
                        type="number"
                        value={orderForm.price || ''}
                        onChange={(e) => setOrderForm(prev => ({ ...prev, price: parseFloat(e.target.value) || 0 }))}
                        className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                        placeholder="Enter price"
                        step="0.0001"
                        min="0"
                      />
                    </div>
                  )}

                  {(orderForm.type === 'stop' || orderForm.type === 'stop-limit') && (
                    <div>
                      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Stop Price</label>
                      <input
                        type="number"
                        value={orderForm.stopPrice || ''}
                        onChange={(e) => setOrderForm(prev => ({ ...prev, stopPrice: parseFloat(e.target.value) || 0 }))}
                        className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                        placeholder="Enter stop price"
                        step="0.0001"
                        min="0"
                      />
                    </div>
                  )}

                  <button
                    type="submit"
                    className="w-full btn btn-primary"
                    disabled={isLoading}
                  >
                    {isLoading ? <LoadingSpinner size="sm" /> : 'Place Order'}
                  </button>
                </form>
              </div>

              {/* Order Book */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Order Book</h3>
                <div className="space-y-2">
                  <div className="text-sm text-gray-500 dark:text-gray-400">Coming soon...</div>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'portfolio' && (
            <div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Portfolio</h3>
              <div className="text-sm text-gray-500 dark:text-gray-400">Portfolio view coming soon...</div>
            </div>
          )}

          {activeTab === 'history' && (
            <div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Order History</h3>
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                  <thead>
                    <tr>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">ID</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Symbol</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Side</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Type</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Quantity</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Price</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Status</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {orders.length > 0 ? orders.map(order => (
                      <tr key={order.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                        <td className="px-4 py-2 font-mono text-xs text-blue-600 break-all">{order.id}</td>
                        <td className="px-4 py-2">{order.symbol}</td>
                        <td className="px-4 py-2">{order.side}</td>
                        <td className="px-4 py-2">{order.order_type}</td>
                        <td className="px-4 py-2">{order.quantity}</td>
                        <td className="px-4 py-2">{order.price || '-'}</td>
                        <td className="px-4 py-2">{order.status}</td>
                        <td className="px-4 py-2">
                          {order.status === 'pending' && (
                            <button
                              onClick={() => handleCancelOrder(order.id)}
                              className="text-red-600 hover:text-red-800 text-sm"
                            >
                              Cancel
                            </button>
                          )}
                        </td>
                      </tr>
                    )) : (
                      <tr>
                        <td colSpan={8} className="text-center text-gray-500 dark:text-gray-400 py-8">No orders found</td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default Trading; 