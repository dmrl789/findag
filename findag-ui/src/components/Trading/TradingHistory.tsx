import React, { useState, useEffect } from 'react';
import { 
  TrendingUp, 
  TrendingDown, 
  Filter, 
  Download, 
  Calendar,
  DollarSign,
  BarChart3,
  RefreshCw,
  Search,
  Clock
} from 'lucide-react';
import { Trade, MarketOrder } from '../../types';

export interface TradingHistoryItem {
  id: string;
  type: 'trade' | 'order';
  pair: string;
  side: 'buy' | 'sell';
  amount: number;
  price: number;
  total: number;
  fee: number;
  timestamp: number;
  status: 'completed' | 'pending' | 'cancelled' | 'failed';
  orderType?: 'market' | 'limit' | 'stop' | 'stop-limit';
  tradeId?: string;
  orderId?: string;
}

interface TradingHistoryProps {
  className?: string;
}

export const TradingHistory: React.FC<TradingHistoryProps> = ({ className = '' }) => {
  const [history, setHistory] = useState<TradingHistoryItem[]>([]);
  const [filteredHistory, setFilteredHistory] = useState<TradingHistoryItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedPair, setSelectedPair] = useState<string>('all');
  const [selectedType, setSelectedType] = useState<'all' | 'trade' | 'order'>('all');
  const [selectedSide, setSelectedSide] = useState<'all' | 'buy' | 'sell'>('all');
  const [selectedStatus, setSelectedStatus] = useState<'all' | 'completed' | 'pending' | 'cancelled' | 'failed'>('all');
  const [dateRange, setDateRange] = useState<'1d' | '7d' | '30d' | '90d' | '1y'>('30d');
  const [sortBy, setSortBy] = useState<'timestamp' | 'amount' | 'price' | 'total'>('timestamp');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  // Mock data - in real app this would come from API
  useEffect(() => {
    const mockHistory: TradingHistoryItem[] = [
      {
        id: '1',
        type: 'trade',
        pair: 'BTC/USD',
        side: 'buy',
        amount: 0.5,
        price: 52000,
        total: 26000,
        fee: 26,
        timestamp: Date.now() - 3600000, // 1 hour ago
        status: 'completed',
        tradeId: 'trade_001',
      },
      {
        id: '2',
        type: 'order',
        pair: 'ETH/USD',
        side: 'sell',
        amount: 2.5,
        price: 3800,
        total: 9500,
        fee: 9.5,
        timestamp: Date.now() - 7200000, // 2 hours ago
        status: 'completed',
        orderType: 'limit',
        orderId: 'order_002',
      },
      {
        id: '3',
        type: 'trade',
        pair: 'ADA/USD',
        side: 'buy',
        amount: 1000,
        price: 0.42,
        total: 420,
        fee: 0.42,
        timestamp: Date.now() - 10800000, // 3 hours ago
        status: 'completed',
        tradeId: 'trade_003',
      },
      {
        id: '4',
        type: 'order',
        pair: 'BTC/USD',
        side: 'sell',
        amount: 0.25,
        price: 52500,
        total: 13125,
        fee: 13.125,
        timestamp: Date.now() - 14400000, // 4 hours ago
        status: 'cancelled',
        orderType: 'limit',
        orderId: 'order_004',
      },
      {
        id: '5',
        type: 'trade',
        pair: 'DOT/USD',
        side: 'buy',
        amount: 50,
        price: 22.3,
        total: 1115,
        fee: 1.115,
        timestamp: Date.now() - 18000000, // 5 hours ago
        status: 'completed',
        tradeId: 'trade_005',
      },
      {
        id: '6',
        type: 'order',
        pair: 'ETH/USD',
        side: 'buy',
        amount: 1.5,
        price: 3750,
        total: 5625,
        fee: 5.625,
        timestamp: Date.now() - 21600000, // 6 hours ago
        status: 'pending',
        orderType: 'stop-limit',
        orderId: 'order_006',
      },
    ];

    setHistory(mockHistory);
    setFilteredHistory(mockHistory);
    setLoading(false);
  }, []);

  // Filter and sort history
  useEffect(() => {
    let filtered = [...history];

    // Search filter
    if (searchTerm) {
      filtered = filtered.filter(item =>
        item.pair.toLowerCase().includes(searchTerm.toLowerCase()) ||
        item.id.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }

    // Pair filter
    if (selectedPair !== 'all') {
      filtered = filtered.filter(item => item.pair === selectedPair);
    }

    // Type filter
    if (selectedType !== 'all') {
      filtered = filtered.filter(item => item.type === selectedType);
    }

    // Side filter
    if (selectedSide !== 'all') {
      filtered = filtered.filter(item => item.side === selectedSide);
    }

    // Status filter
    if (selectedStatus !== 'all') {
      filtered = filtered.filter(item => item.status === selectedStatus);
    }

    // Date range filter
    const now = Date.now();
    const ranges = {
      '1d': 24 * 60 * 60 * 1000,
      '7d': 7 * 24 * 60 * 60 * 1000,
      '30d': 30 * 24 * 60 * 60 * 1000,
      '90d': 90 * 24 * 60 * 60 * 1000,
      '1y': 365 * 24 * 60 * 60 * 1000,
    };
    filtered = filtered.filter(item => now - item.timestamp <= ranges[dateRange]);

    // Sort
    filtered.sort((a, b) => {
      let aValue: number;
      let bValue: number;

      switch (sortBy) {
        case 'timestamp':
          aValue = a.timestamp;
          bValue = b.timestamp;
          break;
        case 'amount':
          aValue = a.amount;
          bValue = b.amount;
          break;
        case 'price':
          aValue = a.price;
          bValue = b.price;
          break;
        case 'total':
          aValue = a.total;
          bValue = b.total;
          break;
        default:
          aValue = a.timestamp;
          bValue = b.timestamp;
      }

      return sortOrder === 'asc' ? aValue - bValue : bValue - aValue;
    });

    setFilteredHistory(filtered);
  }, [history, searchTerm, selectedPair, selectedType, selectedSide, selectedStatus, dateRange, sortBy, sortOrder]);

  const getAvailablePairs = () => {
    const pairs = new Set(history.map(item => item.pair));
    return Array.from(pairs);
  };

  const getStatistics = () => {
    const completed = filteredHistory.filter(item => item.status === 'completed');
    const totalVolume = completed.reduce((sum, item) => sum + item.total, 0);
    const totalFees = completed.reduce((sum, item) => sum + item.fee, 0);
    const buyVolume = completed.filter(item => item.side === 'buy').reduce((sum, item) => sum + item.total, 0);
    const sellVolume = completed.filter(item => item.side === 'sell').reduce((sum, item) => sum + item.total, 0);
    const totalTrades = completed.length;

    return {
      totalVolume,
      totalFees,
      buyVolume,
      sellVolume,
      totalTrades,
      averageTradeSize: totalTrades > 0 ? totalVolume / totalTrades : 0,
    };
  };

  const handleRefresh = async () => {
    setLoading(true);
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    setLoading(false);
  };

  const handleExport = () => {
    const csvContent = [
      ['Date', 'Type', 'Pair', 'Side', 'Amount', 'Price', 'Total', 'Fee', 'Status'],
      ...filteredHistory.map(item => [
        new Date(item.timestamp).toISOString(),
        item.type,
        item.pair,
        item.side,
        item.amount.toString(),
        item.price.toString(),
        item.total.toString(),
        item.fee.toString(),
        item.status,
      ])
    ].map(row => row.join(',')).join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `trading_history_${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    window.URL.revokeObjectURL(url);
  };

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - timestamp;

    if (diff < 60000) return 'Just now';
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    if (diff < 604800000) return `${Math.floor(diff / 86400000)}d ago`;
    
    return date.toLocaleDateString();
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'completed':
        return 'text-green-600 bg-green-100 dark:bg-green-900/20';
      case 'pending':
        return 'text-yellow-600 bg-yellow-100 dark:bg-yellow-900/20';
      case 'cancelled':
        return 'text-gray-600 bg-gray-100 dark:bg-gray-900/20';
      case 'failed':
        return 'text-red-600 bg-red-100 dark:bg-red-900/20';
      default:
        return 'text-gray-600 bg-gray-100 dark:bg-gray-900/20';
    }
  };

  const statistics = getStatistics();
  const availablePairs = getAvailablePairs();

  if (loading) {
    return (
      <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 ${className}`}>
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/4 mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3, 4, 5].map(i => (
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
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
              Trading History
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              View your trading activity and performance
            </p>
          </div>
          <div className="flex items-center space-x-2">
            <button
              onClick={handleRefresh}
              className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              <RefreshCw className="w-5 h-5" />
            </button>
            <button
              onClick={handleExport}
              className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              <Download className="w-5 h-5" />
            </button>
          </div>
        </div>
      </div>

      {/* Statistics */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <DollarSign className="w-8 h-8 text-blue-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Total Volume</p>
                <p className="text-lg font-semibold text-gray-900 dark:text-white">
                  ${statistics.totalVolume.toLocaleString()}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <BarChart3 className="w-8 h-8 text-green-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Total Trades</p>
                <p className="text-lg font-semibold text-gray-900 dark:text-white">
                  {statistics.totalTrades}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <TrendingUp className="w-8 h-8 text-purple-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Buy Volume</p>
                <p className="text-lg font-semibold text-gray-900 dark:text-white">
                  ${statistics.buyVolume.toLocaleString()}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div className="flex items-center">
              <TrendingDown className="w-8 h-8 text-orange-500" />
              <div className="ml-3">
                <p className="text-sm text-gray-500 dark:text-gray-400">Sell Volume</p>
                <p className="text-lg font-semibold text-gray-900 dark:text-white">
                  ${statistics.sellVolume.toLocaleString()}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Filters */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search trades..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>

          {/* Pair Filter */}
          <select
            value={selectedPair}
            onChange={(e) => setSelectedPair(e.target.value)}
            className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="all">All Pairs</option>
            {availablePairs.map(pair => (
              <option key={pair} value={pair}>{pair}</option>
            ))}
          </select>

          {/* Type Filter */}
          <select
            value={selectedType}
            onChange={(e) => setSelectedType(e.target.value as any)}
            className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="all">All Types</option>
            <option value="trade">Trades</option>
            <option value="order">Orders</option>
          </select>

          {/* Side Filter */}
          <select
            value={selectedSide}
            onChange={(e) => setSelectedSide(e.target.value as any)}
            className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="all">All Sides</option>
            <option value="buy">Buy</option>
            <option value="sell">Sell</option>
          </select>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
          {/* Status Filter */}
          <select
            value={selectedStatus}
            onChange={(e) => setSelectedStatus(e.target.value as any)}
            className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="all">All Statuses</option>
            <option value="completed">Completed</option>
            <option value="pending">Pending</option>
            <option value="cancelled">Cancelled</option>
            <option value="failed">Failed</option>
          </select>

          {/* Date Range */}
          <select
            value={dateRange}
            onChange={(e) => setDateRange(e.target.value as any)}
            className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="1d">Last 24 Hours</option>
            <option value="7d">Last 7 Days</option>
            <option value="30d">Last 30 Days</option>
            <option value="90d">Last 90 Days</option>
            <option value="1y">Last Year</option>
          </select>

          {/* Sort */}
          <div className="flex space-x-2">
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as any)}
              className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            >
              <option value="timestamp">Date</option>
              <option value="amount">Amount</option>
              <option value="price">Price</option>
              <option value="total">Total</option>
            </select>
            <button
              onClick={() => setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')}
              className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              {sortOrder === 'asc' ? '↑' : '↓'}
            </button>
          </div>
        </div>
      </div>

      {/* History Table */}
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Date
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Type
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Pair
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Side
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Amount
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Price
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Total
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Fee
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Status
              </th>
            </tr>
          </thead>
          <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
            {filteredHistory.map((item) => (
              <tr key={item.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                  <div className="flex items-center">
                    <Clock className="w-4 h-4 text-gray-400 mr-2" />
                    {formatTimestamp(item.timestamp)}
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${
                    item.type === 'trade' 
                      ? 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-300'
                      : 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-300'
                  }`}>
                    {item.type}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                  {item.pair}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`inline-flex items-center px-2 py-1 text-xs font-semibold rounded-full ${
                    item.side === 'buy'
                      ? 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-300'
                      : 'bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-300'
                  }`}>
                    {item.side === 'buy' ? <TrendingUp className="w-3 h-3 mr-1" /> : <TrendingDown className="w-3 h-3 mr-1" />}
                    {item.side.toUpperCase()}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                  {item.amount.toFixed(6)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                  ${item.price.toLocaleString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                  ${item.total.toLocaleString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                  ${item.fee.toFixed(2)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(item.status)}`}>
                    {item.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Empty State */}
      {filteredHistory.length === 0 && (
        <div className="p-12 text-center">
          <BarChart3 className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">No trading history found</h3>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Try adjusting your filters or start trading to see your history.
          </p>
        </div>
      )}

      {/* Results Count */}
      {filteredHistory.length > 0 && (
        <div className="p-4 border-t border-gray-200 dark:border-gray-700">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Showing {filteredHistory.length} of {history.length} items
          </p>
        </div>
      )}
    </div>
  );
}; 