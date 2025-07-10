import React, { useState, useEffect } from 'react';
import { 
  BarChart3, 
  TrendingUp, 
  TrendingDown, 
  Bell,
  History,
  PieChart,
  Layers
} from 'lucide-react';
import { OptimizedChart } from '../Charts/OptimizedChart';
import { AdvancedOrderForm } from './AdvancedOrderForm';
import { PortfolioTracker } from './PortfolioTracker';
import { TradingHistory } from './TradingHistory';
import { TradingAlerts } from './TradingAlerts';
import { MarketDepth } from './MarketDepth';
import { OrderBook } from './OrderBook';
import { RecentTrades } from './RecentTrades';
import { UserBalance } from './UserBalance';
import { UserOrders } from './UserOrders';
import { PricePoint, Trade, OrderBook as OrderBookType } from '../../types';
import { useNotifications, createNotification } from '../Common/NotificationSystem';
import { finDAGApi } from '../../services/api';

interface AdvancedTradingViewProps {
  pair: string;
  className?: string;
}

export const AdvancedTradingView: React.FC<AdvancedTradingViewProps> = ({ 
  pair, 
  className = '' 
}) => {
  const [activeTab, setActiveTab] = useState<'trading' | 'portfolio' | 'history' | 'alerts' | 'depth'>('trading');
  const [showAdvancedOrder, setShowAdvancedOrder] = useState(false);
  const [priceData, setPriceData] = useState<PricePoint[]>([]);
  const [trades, setTrades] = useState<Trade[]>([]);
  const [orderBook, setOrderBook] = useState<OrderBookType | null>(null);
  const [loading, setLoading] = useState(true);
  const [timeFrame, setTimeFrame] = useState<'1m' | '5m' | '15m' | '1h' | '4h' | '1d' | '1w'>('1h');
  const [chartType, setChartType] = useState<'line' | 'candlestick' | 'area' | 'volume' | 'technical'>('candlestick');

  const { addNotification } = useNotifications();

  // Load data from backend API
  useEffect(() => {
    const loadData = async () => {
      setLoading(true);

      try {
        // Load data from backend API
        const [priceHistory, recentTrades, orderBookData] = await Promise.all([
          finDAGApi.getPriceHistory(pair, timeFrame, 1000),
          finDAGApi.getRecentTrades(pair, 100),
          finDAGApi.getOrderBook(pair, 20)
        ]);

        setPriceData(priceHistory);
        setTrades(recentTrades);
        setOrderBook(orderBookData);

        // Add notification for data loaded
        addNotification(createNotification.info(
          'Trading Data Loaded',
          `Successfully loaded trading data for ${pair}`,
          { category: 'trading' }
        ));
      } catch (error: any) {
        const errorMessage = error.message || 'Failed to load trading data';
        
        addNotification(createNotification.error(
          'Data Load Error',
          errorMessage,
          { category: 'trading' }
        ));
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [pair, timeFrame, addNotification]);

  const handleTimeFrameChange = (newTimeFrame: string) => {
    setTimeFrame(newTimeFrame as any);
    
    // Load new price data for the selected timeframe
    const loadPriceData = async () => {
      try {
        const newPriceData = await finDAGApi.getPriceHistory(pair, newTimeFrame, 1000);
        setPriceData(newPriceData);
        
        addNotification(createNotification.info(
          'Timeframe Changed',
          `Chart timeframe updated to ${newTimeFrame}`,
          { category: 'trading' }
        ));
      } catch (error: any) {
        addNotification(createNotification.error(
          'Timeframe Error',
          'Failed to load data for new timeframe',
          { category: 'trading' }
        ));
      }
    };

    loadPriceData();
  };

  const handleChartTypeChange = (newChartType: string) => {
    setChartType(newChartType as any);
    
    // Add notification for chart type change
    addNotification(createNotification.info(
      'Chart Type Changed',
      `Chart type updated to ${newChartType}`,
      { category: 'trading' }
    ));
  };

  const handleOrderPlaced = (order: any) => {
    console.log('Order placed:', order);
    setShowAdvancedOrder(false);
    
    // Add notification for order placement
    addNotification(createNotification.order(
      'Order Placed',
      `${order.side.toUpperCase()} order for ${order.amount} ${pair} at ${order.price || 'market price'}`,
      {
        category: 'order',
        priority: 'high',
        actions: [
          {
            label: 'View Order',
            action: () => {
              // Navigate to orders tab or open order details
              setActiveTab('history');
            },
            variant: 'primary'
          },
          {
            label: 'Cancel',
            action: () => {
              // Cancel order logic
              addNotification(createNotification.warning(
                'Order Cancelled',
                'Order cancellation requested',
                { category: 'order' }
              ));
            },
            variant: 'danger'
          }
        ]
      }
    ));
  };

  const handlePriceAlert = (alert: any) => {
    // Add notification for price alert
    addNotification(createNotification.price(
      'Price Alert',
      `${pair} price ${alert.condition} ${alert.value}`,
      {
        category: 'price',
        priority: 'medium',
        actions: [
          {
            label: 'View Chart',
            action: () => setActiveTab('trading'),
            variant: 'primary'
          }
        ]
      }
    ));
  };

  const handleTradeExecuted = (trade: Trade) => {
    // Add notification for trade execution
    addNotification(createNotification.trade(
      'Trade Executed',
      `${trade.side.toUpperCase()} ${trade.amount} ${pair} at ${trade.price}`,
      {
        category: 'trading',
        priority: 'medium',
        data: { trade }
      }
    ));
  };

  const tabs = [
    {
      id: 'trading',
      label: 'Trading',
      icon: TrendingUp,
      description: 'Advanced trading interface',
    },
    {
      id: 'portfolio',
      label: 'Portfolio',
      icon: PieChart,
      description: 'Portfolio tracking and analysis',
    },
    {
      id: 'history',
      label: 'History',
      icon: History,
      description: 'Trading history and analytics',
    },
    {
      id: 'alerts',
      label: 'Alerts',
      icon: Bell,
      description: 'Price alerts and notifications',
    },
    {
      id: 'depth',
      label: 'Market Depth',
      icon: Layers,
      description: 'Order book depth analysis',
    },
  ];

  if (loading) {
    return (
      <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 ${className}`}>
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/4 mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3, 4].map(i => (
              <div key={i} className="h-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
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
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
              Advanced Trading - {pair}
            </h1>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Professional trading interface with advanced features
            </p>
          </div>
          <button
            onClick={() => setShowAdvancedOrder(true)}
            className="flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
          >
            <TrendingUp className="w-4 h-4 mr-2" />
            Advanced Order
          </button>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="border-b border-gray-200 dark:border-gray-700">
        <nav className="flex space-x-8 px-6">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`flex items-center py-4 px-1 border-b-2 font-medium text-sm transition-colors ${
                  activeTab === tab.id
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300'
                }`}
              >
                <Icon className="w-4 h-4 mr-2" />
                {tab.label}
              </button>
            );
          })}
        </nav>
      </div>

      {/* Tab Content */}
      <div className="p-6">
        {activeTab === 'trading' && (
          <div className="space-y-6">
            {/* Chart Section */}
            <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
                  Price Chart
                </h2>
                <div className="flex items-center space-x-2">
                  <select
                    value={timeFrame}
                    onChange={(e) => handleTimeFrameChange(e.target.value)}
                    className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                  >
                    <option value="1m">1m</option>
                    <option value="5m">5m</option>
                    <option value="15m">15m</option>
                    <option value="1h">1h</option>
                    <option value="4h">4h</option>
                    <option value="1d">1d</option>
                    <option value="1w">1w</option>
                  </select>
                  <select
                    value={chartType}
                    onChange={(e) => handleChartTypeChange(e.target.value)}
                    className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                  >
                    <option value="line">Line</option>
                    <option value="candlestick">Candlestick</option>
                    <option value="area">Area</option>
                    <option value="volume">Volume</option>
                    <option value="technical">Technical</option>
                  </select>
                </div>
              </div>
              <div className="h-96">
                <OptimizedChart
                  pair={pair}
                  data={priceData}
                  timeFrame={timeFrame}
                  chartType={chartType}
                  onTimeFrameChange={handleTimeFrameChange}
                  onChartTypeChange={handleChartTypeChange}
                  loading={loading}
                  showVolume={true}
                  showMA={true}
                  showBB={false}
                  showRSI={false}
                  enableOptimizations={true}
                />
              </div>
            </div>

            {/* Trading Interface */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              {/* Order Book */}
              <div className="lg:col-span-1">
                <OrderBook data={orderBook} pair={pair} />
              </div>

              {/* Recent Trades */}
              <div className="lg:col-span-1">
                <RecentTrades trades={trades} pair={pair} />
              </div>

              {/* User Data */}
              <div className="lg:col-span-1 space-y-4">
                <UserBalance />
                <UserOrders />
              </div>
            </div>
          </div>
        )}

        {activeTab === 'portfolio' && (
          <PortfolioTracker />
        )}

        {activeTab === 'history' && (
          <TradingHistory />
        )}

        {activeTab === 'alerts' && (
          <TradingAlerts />
        )}

        {activeTab === 'depth' && (
          <MarketDepth orderBook={orderBook} pair={pair} />
        )}
      </div>

      {/* Advanced Order Modal */}
      {showAdvancedOrder && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
            <AdvancedOrderForm
              pair={pair}
              currentPrice={priceData[priceData.length - 1]?.price || 0}
              onOrderPlaced={handleOrderPlaced}
              onCancel={() => setShowAdvancedOrder(false)}
            />
          </div>
        </div>
      )}
    </div>
  );
}; 