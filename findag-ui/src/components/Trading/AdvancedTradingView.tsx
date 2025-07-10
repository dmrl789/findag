import React, { useState, useEffect, useCallback, useRef } from 'react';
import { 
  TrendingUp, 
  TrendingDown, 
  Bell,
  History,
  PieChart,
  Layers,
  DollarSign,
  Percent,
  Clock,
  ArrowUp,
  ArrowDown,
  Plus,
  Minus,
  Settings,
  RefreshCw
} from 'lucide-react';
import { marketDataService } from '../../services/marketData';
import { DataSourceConfig } from '../Common/DataSourceConfig';

interface AdvancedTradingViewProps {
  pair: string;
  className?: string;
}

export const AdvancedTradingView: React.FC<AdvancedTradingViewProps> = ({ 
  pair, 
  className = '' 
}) => {
  const [activeTab, setActiveTab] = useState<'trading' | 'portfolio' | 'history' | 'alerts' | 'depth'>('trading');
  const [orderType, setOrderType] = useState<'buy' | 'sell'>('buy');
  const [orderAmount, setOrderAmount] = useState('');
  const [orderPrice, setOrderPrice] = useState('');
  const [realData, setRealData] = useState<any>(null);
  const [isLoadingData, setIsLoadingData] = useState(false);
  const [showDataSourceConfig, setShowDataSourceConfig] = useState(false);
  const [dataSourceStatus, setDataSourceStatus] = useState<string>('demo');
  const [timeFrame, setTimeFrame] = useState<string>('1W'); // 1m, 1h, 3h, 1D, 1W, 1M, 1Y, 5Y
  const [lastPriceUpdate, setLastPriceUpdate] = useState<number>(Date.now());
  const [currentPrice, setCurrentPrice] = useState<number>(0);
  const [priceChange, setPriceChange] = useState<number>(0);
  const [priceChangePercent, setPriceChangePercent] = useState<number>(0);
  const [isRealTimeEnabled, setIsRealTimeEnabled] = useState<boolean>(false);
  
  // Refs for real-time updates
  const updateIntervalRef = useRef<NodeJS.Timeout | null>(null);
  const lastDataRef = useRef<any>(null);

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
      description: 'Portfolio management',
    },
    {
      id: 'history',
      label: 'History',
      icon: History,
      description: 'Trading history',
    },
    {
      id: 'alerts',
      label: 'Alerts',
      icon: Bell,
      description: 'Price alerts',
    },
    {
      id: 'depth',
      label: 'Market Depth',
      icon: Layers,
      description: 'Order book depth',
    },
  ];

  // Real-time price update function
  const updatePriceData = useCallback(async () => {
    if (!isRealTimeEnabled) return;
    
    try {
      let newPrice: number;
      let newData: any;
      
      // Get fresh price data
      if (pair.includes('/')) {
        // Forex - get current rate
        const finnhubKey = localStorage.getItem('findag_api_keys') ? 
          JSON.parse(localStorage.getItem('findag_api_keys')!).finnhub : null;
        
        if (finnhubKey) {
          const response = await fetch(
            `https://finnhub.io/api/v1/forex/rates?base=${pair.split('/')[0]}&token=${finnhubKey}`
          );
          const data = await response.json();
          newPrice = data.quote?.[pair.split('/')[1]] || currentPrice;
        } else {
          // Simulate price movement for demo
          const volatility = 0.0001; // 0.01% movement
          const randomChange = (Math.random() - 0.5) * 2 * volatility;
          newPrice = currentPrice * (1 + randomChange);
        }
      } else {
        // Stock/Commodity - simulate price movement
        const volatility = 0.001; // 0.1% movement
        const randomChange = (Math.random() - 0.5) * 2 * volatility;
        newPrice = currentPrice * (1 + randomChange);
      }
      
      // Update price data
      const newPriceChange = newPrice - currentPrice;
      const newPriceChangePercent = currentPrice > 0 ? (newPriceChange / currentPrice) * 100 : 0;
      
      setCurrentPrice(newPrice);
      setPriceChange(newPriceChange);
      setPriceChangePercent(newPriceChangePercent);
      setLastPriceUpdate(Date.now());
      
      // Update chart data if we have existing data
      if (lastDataRef.current && lastDataRef.current.length > 0) {
        const updatedData = [...lastDataRef.current];
        const lastPoint = updatedData[updatedData.length - 1];
        
        // Update the last data point with new price
        const newPoint = {
          ...lastPoint,
          timestamp: Date.now(),
          close: newPrice,
          price: newPrice,
          high: Math.max(lastPoint.high, newPrice),
          low: Math.min(lastPoint.low, newPrice),
        };
        
        updatedData[updatedData.length - 1] = newPoint;
        setRealData(updatedData);
        lastDataRef.current = updatedData;
      }
      
      console.log(`Real-time update for ${pair}: ${newPrice} (${newPriceChangePercent.toFixed(4)}%)`);
    } catch (error) {
      console.error('Error updating real-time price:', error);
    }
  }, [currentPrice, isRealTimeEnabled, pair]);

  // Start/stop real-time updates
  useEffect(() => {
    if (isRealTimeEnabled && currentPrice > 0) {
      updateIntervalRef.current = setInterval(updatePriceData, 5000); // Update every 5 seconds
      console.log(`Started real-time updates for ${pair}`);
    } else if (updateIntervalRef.current) {
      clearInterval(updateIntervalRef.current);
      updateIntervalRef.current = null;
      console.log(`Stopped real-time updates for ${pair}`);
    }
    
    return () => {
      if (updateIntervalRef.current) {
        clearInterval(updateIntervalRef.current);
      }
    };
  }, [isRealTimeEnabled, currentPrice, updatePriceData]);

  // Get current price data (real or demo)
  const getCurrentPriceData = () => {
    if (realData && realData.length > 0) {
      const latest = realData[realData.length - 1];
      const previous = realData[realData.length - 2] || latest;
      const priceChange = latest.price - previous.price;
      const priceChangePercent = (priceChange / previous.price) * 100;
      
      // Update current price state if not set
      if (currentPrice === 0) {
        setCurrentPrice(latest.price);
        setPriceChange(priceChange);
        setPriceChangePercent(priceChangePercent);
      }
      
      console.log(`Using REAL data for ${pair}:`, {
        currentPrice: latest.price,
        dataPoints: realData.length,
        timeRange: `${new Date(realData[0].timestamp).toLocaleDateString()} - ${new Date(latest.timestamp).toLocaleDateString()}`,
        priceVariation: realData.map((p: any) => p.price).slice(-5) // Last 5 prices
      });
      
      return {
        currentPrice: latest.price,
        priceChange,
        priceChangePercent,
        volume24h: latest.volume || 125000000000,
        high24h: latest.high || latest.price,
        low24h: latest.low || latest.price,
      };
    }
    
    // Fallback to demo data
    console.log(`Using DEMO data for ${pair} (no real data available)`);
    const demoPrice = 1.0850;
    const demoChange = 0.0025;
    const demoChangePercent = 0.23;
    
    // Update current price state if not set
    if (currentPrice === 0) {
      setCurrentPrice(demoPrice);
      setPriceChange(demoChange);
      setPriceChangePercent(demoChangePercent);
    }
    
    return {
      currentPrice: demoPrice,
      priceChange: demoChange,
      priceChangePercent: demoChangePercent,
      volume24h: 125000000000,
      high24h: 1.0875,
      low24h: 1.0825,
    };
  };

  const priceData = getCurrentPriceData();

  const portfolioData = [
    { asset: 'EUR/USD', amount: 100000, value: 108500, change: 0.23 },
    { asset: 'GBP/USD', amount: 50000, value: 63250, change: -0.63 },
    { asset: 'AAPL', amount: 100, value: 18550, change: 1.50 },
    { asset: 'XAU/USD', amount: 100, value: 204550, change: 0.63 },
  ];

  const recentTrades = [
    { id: 1, type: 'buy', amount: 100000, price: 1.0850, time: '2 min ago' },
    { id: 2, type: 'sell', amount: 50000, price: 1.0845, time: '5 min ago' },
    { id: 3, type: 'buy', amount: 75000, price: 1.0855, time: '8 min ago' },
    { id: 4, type: 'sell', amount: 25000, price: 1.0840, time: '12 min ago' },
  ];

  const orderBook = {
    bids: [
      { price: 1.0845, amount: 100000, total: 108450 },
      { price: 1.0840, amount: 75000, total: 81300 },
      { price: 1.0835, amount: 125000, total: 135437.5 },
      { price: 1.0830, amount: 50000, total: 54150 },
      { price: 1.0825, amount: 100000, total: 108250 },
    ],
    asks: [
      { price: 1.0855, amount: 80000, total: 86840 },
      { price: 1.0860, amount: 150000, total: 162900 },
      { price: 1.0865, amount: 100000, total: 108650 },
      { price: 1.0870, amount: 120000, total: 130440 },
      { price: 1.0875, amount: 90000, total: 97875 },
    ],
  };

  // Fetch real market data
  const fetchRealData = async () => {
    setIsLoadingData(true);
    try {
      let data;
      let dataType: 'forex' | 'stock' | 'commodity';
      
      if (pair.includes('/')) {
        // Forex pair
        dataType = 'forex';
        data = await marketDataService.getForexData(pair, timeFrame);
      } else if (pair === 'AAPL' || pair === 'MSFT') {
        // Stock
        dataType = 'stock';
        data = await marketDataService.getStockData(pair, timeFrame);
      } else {
        // Commodity
        dataType = 'commodity';
        data = await marketDataService.getCommodityData(pair);
      }
      
      console.log(`Fetched ${dataType} data for ${pair} (${timeFrame}):`, data.length, 'data points');
      setRealData(data);
      lastDataRef.current = data;
      setDataSourceStatus('real');
      setLastPriceUpdate(Date.now());
      
      // Enable real-time updates if we have real data
      if (data.length > 0) {
        setIsRealTimeEnabled(true);
      }
    } catch (error) {
      console.error('Error fetching real data:', error);
      setDataSourceStatus('demo');
    } finally {
      setIsLoadingData(false);
    }
  };

  // Force refresh data
  const forceRefreshData = async () => {
    setIsLoadingData(true);
    try {
      let data;
      let dataType: 'forex' | 'stock' | 'commodity';
      
      if (pair.includes('/')) {
        dataType = 'forex';
        data = await marketDataService.forceRefreshData(pair, 'forex');
      } else if (pair === 'AAPL' || pair === 'MSFT') {
        dataType = 'stock';
        data = await marketDataService.forceRefreshData(pair, 'stock');
      } else {
        dataType = 'commodity';
        data = await marketDataService.forceRefreshData(pair, 'commodity');
      }
      
      console.log(`Force refreshed ${dataType} data for ${pair} (${timeFrame}):`, data.length, 'data points');
      setRealData(data);
      lastDataRef.current = data;
      setDataSourceStatus('real');
      setLastPriceUpdate(Date.now());
      
      // Enable real-time updates
      if (data.length > 0) {
        setIsRealTimeEnabled(true);
      }
    } catch (error) {
      console.error('Error force refreshing data:', error);
      setDataSourceStatus('demo');
    } finally {
      setIsLoadingData(false);
    }
  };

  // Initial data fetch
  useEffect(() => {
    fetchRealData();
  }, [pair, timeFrame]);

  // Toggle real-time updates
  const toggleRealTime = () => {
    setIsRealTimeEnabled(!isRealTimeEnabled);
  };

  // Check if API keys are configured
  useEffect(() => {
    const hasApiKey = marketDataService.isApiKeyConfigured('alphaVantage') ||
                     marketDataService.isApiKeyConfigured('polygon') ||
                     marketDataService.isApiKeyConfigured('finnhub');
    
    console.log('API key check:', {
      alphaVantage: marketDataService.isApiKeyConfigured('alphaVantage'),
      polygon: marketDataService.isApiKeyConfigured('polygon'),
      finnhub: marketDataService.isApiKeyConfigured('finnhub'),
      hasAnyKey: hasApiKey
    });
    
    if (hasApiKey) {
      fetchRealData();
    } else {
      console.log('No API keys configured, using demo data');
      setDataSourceStatus('demo');
    }
  }, [pair]);

  // Auto-refresh data every 15 seconds if using real data, every 5 seconds for demo data
  useEffect(() => {
    if (dataSourceStatus === 'real') {
      const interval = setInterval(fetchRealData, 15000);
      return () => clearInterval(interval);
    } else {
      // For demo data, refresh more frequently to show price movements
      const interval = setInterval(fetchRealData, 5000);
      return () => clearInterval(interval);
    }
  }, [dataSourceStatus]);

  return (
    <div className={`p-6 space-y-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Institutional Trading</h1>
          <p className="text-gray-600">Professional trading interface for {pair} - No fees, institutional-grade</p>
          <div className="flex items-center space-x-4 mt-2">
            <div className="flex items-center space-x-2">
              <div className={`w-2 h-2 rounded-full ${
                dataSourceStatus === 'real' ? 'bg-green-500' : 'bg-yellow-500'
              }`}></div>
              <span className="text-sm text-gray-600">
                {dataSourceStatus === 'real' ? 'Real-time Data' : 'Demo Data'}
                {isLoadingData && ' (Updating...)'}
              </span>
            </div>
            {isLoadingData && (
              <div className="flex items-center space-x-2">
                <RefreshCw className="w-4 h-4 animate-spin text-blue-600" />
                <span className="text-sm text-blue-600">Updating...</span>
              </div>
            )}
            <div className="text-xs text-gray-500">
              {marketDataService.isApiKeyConfigured('finnhub') && 'Finnhub ✓'}
              {marketDataService.isApiKeyConfigured('alphaVantage') && ' AlphaVantage ✓'}
              {!marketDataService.isApiKeyConfigured('finnhub') && !marketDataService.isApiKeyConfigured('alphaVantage') && 'No API keys'}
            </div>
            <div className="flex items-center space-x-2">
              <button
                onClick={toggleRealTime}
                className={`px-3 py-1 rounded-md text-xs font-medium border ${
                  isRealTimeEnabled 
                    ? 'bg-green-100 text-green-700 border-green-300' 
                    : 'bg-gray-100 text-gray-700 border-gray-300'
                }`}
                title={isRealTimeEnabled ? 'Disable real-time updates' : 'Enable real-time updates'}
              >
                <div className={`w-2 h-2 rounded-full inline-block mr-1 ${
                  isRealTimeEnabled ? 'bg-green-500 animate-pulse' : 'bg-gray-400'
                }`}></div>
                {isRealTimeEnabled ? 'Live' : 'Static'}
              </button>
            </div>
          </div>
        </div>
        <div className="flex items-center space-x-3">
          {/* Time Frame Selector */}
          <div className="flex items-center space-x-2">
            <span className="text-sm text-gray-600">Time Frame:</span>
            <select
              value={timeFrame}
              onChange={(e) => {
                setTimeFrame(e.target.value);
                fetchRealData(); // Refresh data with new time frame
              }}
              className="px-3 py-1 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="1m">1 Minute</option>
              <option value="1h">1 Hour</option>
              <option value="3h">3 Hours</option>
              <option value="1D">1 Day</option>
              <option value="1W">1 Week</option>
              <option value="1M">1 Month</option>
              <option value="1Y">1 Year</option>
              <option value="5Y">5 Years</option>
            </select>
          </div>
          
          <button 
            onClick={() => setShowDataSourceConfig(!showDataSourceConfig)}
            className="btn-secondary flex items-center space-x-2"
          >
            <Settings className="w-4 h-4" />
            <span>Data Sources</span>
          </button>
          <button 
            onClick={fetchRealData}
            disabled={isLoadingData}
            className="btn-secondary flex items-center space-x-2"
          >
            <RefreshCw className={`w-4 h-4 ${isLoadingData ? 'animate-spin' : ''}`} />
            <span>Refresh</span>
          </button>
          <button 
            onClick={forceRefreshData}
            disabled={isLoadingData}
            className="btn-secondary flex items-center space-x-2"
            title="Force refresh (bypass cache)"
          >
            <RefreshCw className={`w-4 h-4 ${isLoadingData ? 'animate-spin' : ''}`} />
            <span>Force Refresh</span>
          </button>
          <button className="btn-secondary flex items-center space-x-2">
            <Bell className="w-4 h-4" />
            <span>Price Alerts</span>
          </button>
        </div>
      </div>

      {/* Data Source Configuration */}
      {showDataSourceConfig && (
        <div className="mb-6">
          <DataSourceConfig onConfigChange={fetchRealData} />
        </div>
      )}

      {/* Price Overview */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold text-gray-900">{pair}</h3>
              {/* Data Source Status */}
              <div className="flex items-center space-x-2 mt-1">
                <div className={`w-2 h-2 rounded-full ${
                  dataSourceStatus === 'real' ? 'bg-green-500' : 'bg-yellow-500'
                }`} />
                <span className="text-xs text-gray-600">
                  {dataSourceStatus === 'real' ? 'Real-time Data' : 'Demo Data'}
                  {isLoadingData && ' (Loading...)'}
                </span>
              </div>
            </div>
            <div className="flex items-center space-x-2">
              {(priceChangePercent > 0 ? priceChangePercent : priceData.priceChangePercent) > 0 ? (
                <ArrowUp className="w-4 h-4 text-green-600" />
              ) : (
                <ArrowDown className="w-4 h-4 text-red-600" />
              )}
              <span className={`text-sm font-medium ${
                (priceChangePercent > 0 ? priceChangePercent : priceData.priceChangePercent) > 0 ? 'text-green-600' : 'text-red-600'
              }`}>
                {(priceChangePercent > 0 ? priceChangePercent : priceData.priceChangePercent) > 0 ? '+' : ''}{(priceChangePercent > 0 ? priceChangePercent : priceData.priceChangePercent).toFixed(2)}%
              </span>
            </div>
          </div>
          <div className="space-y-2">
            <div className="text-3xl font-bold text-gray-900 flex items-center space-x-2">
              ${currentPrice > 0 ? currentPrice.toFixed(4) : priceData.currentPrice.toFixed(4)}
              <div className={`w-2 h-2 rounded-full ${
                Date.now() - lastPriceUpdate < 10000 ? 'bg-green-500 animate-pulse' : 'bg-gray-300'
              }`} title="Price update indicator"></div>
            </div>
            <div className="text-sm text-gray-600">
              {priceChange > 0 ? '+' : ''}${priceChange > 0 ? priceChange.toFixed(4) : priceData.priceChange.toFixed(4)}
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">24h Stats</h3>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-sm text-gray-600">Volume</span>
              <span className="text-sm font-medium">${(priceData.volume24h / 1000000000).toFixed(1)}B</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-gray-600">High</span>
              <span className="text-sm font-medium">${priceData.high24h.toFixed(4)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-gray-600">Low</span>
              <span className="text-sm font-medium">${priceData.low24h.toFixed(4)}</span>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-900">Institutional Trade</h3>
            <button
              onClick={forceRefreshData}
              disabled={isLoadingData}
              className="flex items-center space-x-1 px-2 py-1 bg-blue-100 text-blue-700 rounded text-xs hover:bg-blue-200 disabled:opacity-50"
            >
              <RefreshCw className={`w-3 h-3 ${isLoadingData ? 'animate-spin' : ''}`} />
              <span>Refresh</span>
            </button>
          </div>
          <div className="space-y-4">
            <div className="flex space-x-2">
              <button
                onClick={() => setOrderType('buy')}
                className={`flex-1 py-2 px-4 rounded-md text-sm font-medium ${
                  orderType === 'buy'
                    ? 'bg-green-600 text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                Buy
              </button>
              <button
                onClick={() => setOrderType('sell')}
                className={`flex-1 py-2 px-4 rounded-md text-sm font-medium ${
                  orderType === 'sell'
                    ? 'bg-red-600 text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                Sell
              </button>
            </div>
            <input
              type="number"
              placeholder="Amount"
              value={orderAmount}
              onChange={(e) => setOrderAmount(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md"
            />
            <input
              type="number"
              placeholder="Price"
              value={orderPrice}
              onChange={(e) => setOrderPrice(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md"
            />
            <button className="w-full py-2 px-4 bg-primary-600 text-white rounded-md hover:bg-primary-700">
              Place Order
            </button>
          </div>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`py-2 px-1 border-b-2 font-medium text-sm flex items-center space-x-2 ${
                  activeTab === tab.id
                    ? 'border-primary-500 text-primary-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                <Icon className="w-4 h-4" />
                <span>{tab.label}</span>
              </button>
            );
          })}
        </nav>
      </div>

      {/* Tab Content */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        {activeTab === 'trading' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {/* Price Chart Placeholder */}
              <div className="bg-gray-50 rounded-lg p-6 h-64 flex items-center justify-center">
                <div className="text-center">
                  <TrendingUp className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                  <h3 className="text-lg font-medium text-gray-900 mb-2">Price Chart</h3>
                  <p className="text-gray-600">Interactive price chart with technical indicators</p>
                </div>
              </div>

              {/* Recent Trades */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Institutional Trades</h3>
                <div className="space-y-2">
                  {recentTrades.map((trade) => (
                    <div key={trade.id} className="flex items-center justify-between py-2 border-b border-gray-100">
                      <div className="flex items-center space-x-2">
                        <div className={`w-2 h-2 rounded-full ${
                          trade.type === 'buy' ? 'bg-green-500' : 'bg-red-500'
                        }`} />
                        <span className="text-sm font-medium">{trade.type.toUpperCase()}</span>
                      </div>
                      <div className="text-right">
                        <div className="text-sm font-medium">{trade.amount} BTC</div>
                        <div className="text-xs text-gray-500">${trade.price.toLocaleString()}</div>
                      </div>
                      <div className="text-xs text-gray-500">{trade.time}</div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'portfolio' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {/* Portfolio Summary */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Institutional Portfolio</h3>
                <div className="space-y-4">
                  {portfolioData.map((item) => (
                    <div key={item.asset} className="flex items-center justify-between p-4 border border-gray-200 rounded-lg">
                      <div>
                        <div className="font-medium text-gray-900">{item.asset}</div>
                        <div className="text-sm text-gray-500">{item.amount} {item.asset}</div>
                      </div>
                      <div className="text-right">
                        <div className="font-medium text-gray-900">${item.value.toLocaleString()}</div>
                        <div className={`text-sm ${
                          item.change > 0 ? 'text-green-600' : 'text-red-600'
                        }`}>
                          {item.change > 0 ? '+' : ''}{item.change}%
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Portfolio Chart */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Allocation</h3>
                <div className="bg-gray-50 rounded-lg p-6 h-64 flex items-center justify-center">
                  <div className="text-center">
                    <PieChart className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                    <h3 className="text-lg font-medium text-gray-900 mb-2">Portfolio Chart</h3>
                    <p className="text-gray-600">Visual representation of asset allocation</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'history' && (
          <div className="space-y-6">
            <div className="text-center py-12">
              <History className="w-12 h-12 text-gray-400 mx-auto mb-4" />
              <h3 className="text-lg font-medium text-gray-900 mb-2">Trading History</h3>
              <p className="text-gray-600">View your complete trading history with detailed analytics</p>
            </div>
          </div>
        )}

        {activeTab === 'alerts' && (
          <div className="space-y-6">
            <div className="text-center py-12">
              <Bell className="w-12 h-12 text-gray-400 mx-auto mb-4" />
              <h3 className="text-lg font-medium text-gray-900 mb-2">Price Alerts</h3>
              <p className="text-gray-600">Set up price alerts and notifications</p>
            </div>
          </div>
        )}

        {activeTab === 'depth' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {/* Order Book */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Institutional Order Book</h3>
                <div className="space-y-2">
                  {/* Asks */}
                  <div className="text-sm font-medium text-red-600 mb-2">Asks</div>
                  {orderBook.asks.map((ask, index) => (
                    <div key={index} className="flex items-center justify-between py-1 text-sm">
                      <span className="text-red-600">{ask.price.toLocaleString()}</span>
                      <span className="text-gray-600">{ask.amount}</span>
                      <span className="text-gray-500">{ask.total.toLocaleString()}</span>
                    </div>
                  ))}
                  
                  {/* Current Price */}
                  <div className="text-center py-2 bg-gray-100 rounded my-2">
                    <span className="text-lg font-bold text-gray-900">${currentPrice.toLocaleString()}</span>
                  </div>
                  
                  {/* Bids */}
                  <div className="text-sm font-medium text-green-600 mb-2">Bids</div>
                  {orderBook.bids.map((bid, index) => (
                    <div key={index} className="flex items-center justify-between py-1 text-sm">
                      <span className="text-green-600">{bid.price.toLocaleString()}</span>
                      <span className="text-gray-600">{bid.amount}</span>
                      <span className="text-gray-500">{bid.total.toLocaleString()}</span>
                    </div>
                  ))}
                </div>
              </div>

              {/* Market Depth Chart */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Market Depth</h3>
                <div className="bg-gray-50 rounded-lg p-6 h-64 flex items-center justify-center">
                  <div className="text-center">
                    <Layers className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                    <h3 className="text-lg font-medium text-gray-900 mb-2">Depth Chart</h3>
                    <p className="text-gray-600">Visual representation of market depth</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}; 