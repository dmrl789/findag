import React, { useState, useEffect } from 'react';
import { 
  TrendingUp, 
  TrendingDown, 
  DollarSign, 
  PieChart, 
  BarChart3,
  Calendar,
  RefreshCw,
  Download,
  Eye,
  EyeOff
} from 'lucide-react';
import { Asset, Trade } from '../../types';

export interface PortfolioAsset {
  asset: string;
  symbol: string;
  name: string;
  quantity: number;
  averagePrice: number;
  currentPrice: number;
  marketValue: number;
  unrealizedPnL: number;
  unrealizedPnLPercent: number;
  costBasis: number;
  lastUpdated: number;
}

export interface PortfolioMetrics {
  totalValue: number;
  totalPnL: number;
  totalPnLPercent: number;
  totalCostBasis: number;
  bestPerformer: PortfolioAsset | null;
  worstPerformer: PortfolioAsset | null;
  totalTrades: number;
  winRate: number;
  averageReturn: number;
}

interface PortfolioTrackerProps {
  className?: string;
}

export const PortfolioTracker: React.FC<PortfolioTrackerProps> = ({ className = '' }) => {
  const [assets, setAssets] = useState<PortfolioAsset[]>([]);
  const [metrics, setMetrics] = useState<PortfolioMetrics | null>(null);
  const [loading, setLoading] = useState(true);
  const [showHidden, setShowHidden] = useState(false);
  const [timeRange, setTimeRange] = useState<'1d' | '7d' | '30d' | '90d' | '1y'>('7d');
  const [sortBy, setSortBy] = useState<'value' | 'pnl' | 'pnlPercent' | 'name'>('value');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  // Mock data - in real app this would come from API
  useEffect(() => {
    const mockAssets: PortfolioAsset[] = [
      {
        asset: 'BTC',
        symbol: 'BTC',
        name: 'Bitcoin',
        quantity: 2.5,
        averagePrice: 45000,
        currentPrice: 52000,
        marketValue: 130000,
        unrealizedPnL: 17500,
        unrealizedPnLPercent: 15.56,
        costBasis: 112500,
        lastUpdated: Date.now(),
      },
      {
        asset: 'ETH',
        symbol: 'ETH',
        name: 'Ethereum',
        quantity: 15.8,
        averagePrice: 3200,
        currentPrice: 3800,
        marketValue: 60040,
        unrealizedPnL: 9480,
        unrealizedPnLPercent: 18.75,
        costBasis: 50560,
        lastUpdated: Date.now(),
      },
      {
        asset: 'ADA',
        symbol: 'ADA',
        name: 'Cardano',
        quantity: 5000,
        averagePrice: 0.45,
        currentPrice: 0.42,
        marketValue: 2100,
        unrealizedPnL: -150,
        unrealizedPnLPercent: -6.67,
        costBasis: 2250,
        lastUpdated: Date.now(),
      },
      {
        asset: 'DOT',
        symbol: 'DOT',
        name: 'Polkadot',
        quantity: 200,
        averagePrice: 18.5,
        currentPrice: 22.3,
        marketValue: 4460,
        unrealizedPnL: 760,
        unrealizedPnLPercent: 20.54,
        costBasis: 3700,
        lastUpdated: Date.now(),
      },
    ];

    setAssets(mockAssets);
    calculateMetrics(mockAssets);
    setLoading(false);
  }, []);

  const calculateMetrics = (portfolioAssets: PortfolioAsset[]) => {
    const totalValue = portfolioAssets.reduce((sum, asset) => sum + asset.marketValue, 0);
    const totalCostBasis = portfolioAssets.reduce((sum, asset) => sum + asset.costBasis, 0);
    const totalPnL = totalValue - totalCostBasis;
    const totalPnLPercent = totalCostBasis > 0 ? (totalPnL / totalCostBasis) * 100 : 0;

    const bestPerformer = portfolioAssets.reduce((best, asset) => 
      asset.unrealizedPnLPercent > best.unrealizedPnLPercent ? asset : best
    );

    const worstPerformer = portfolioAssets.reduce((worst, asset) => 
      asset.unrealizedPnLPercent < worst.unrealizedPnLPercent ? asset : worst
    );

    setMetrics({
      totalValue,
      totalPnL,
      totalPnLPercent,
      totalCostBasis,
      bestPerformer,
      worstPerformer,
      totalTrades: 156, // Mock data
      winRate: 68.5, // Mock data
      averageReturn: 12.3, // Mock data
    });
  };

  const sortedAssets = [...assets].sort((a, b) => {
    let aValue: number;
    let bValue: number;

    switch (sortBy) {
      case 'value':
        aValue = a.marketValue;
        bValue = b.marketValue;
        break;
      case 'pnl':
        aValue = a.unrealizedPnL;
        bValue = b.unrealizedPnL;
        break;
      case 'pnlPercent':
        aValue = a.unrealizedPnLPercent;
        bValue = b.unrealizedPnLPercent;
        break;
      case 'name':
        aValue = a.name.localeCompare(b.name);
        bValue = 0;
        break;
      default:
        aValue = a.marketValue;
        bValue = b.marketValue;
    }

    if (sortBy === 'name') {
      return sortOrder === 'asc' ? aValue : -aValue;
    }

    return sortOrder === 'asc' ? aValue - bValue : bValue - aValue;
  });

  const handleRefresh = async () => {
    setLoading(true);
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    setLoading(false);
  };

  const handleExport = () => {
    const csvContent = [
      ['Asset', 'Symbol', 'Quantity', 'Average Price', 'Current Price', 'Market Value', 'Unrealized P&L', 'P&L %'],
      ...assets.map(asset => [
        asset.name,
        asset.symbol,
        asset.quantity.toString(),
        asset.averagePrice.toString(),
        asset.currentPrice.toString(),
        asset.marketValue.toString(),
        asset.unrealizedPnL.toString(),
        asset.unrealizedPnLPercent.toString(),
      ])
    ].map(row => row.join(',')).join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `portfolio_${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    window.URL.revokeObjectURL(url);
  };

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
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
              Portfolio Tracker
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Track your investments and performance
            </p>
          </div>
          <div className="flex items-center space-x-2">
            <button
              onClick={() => setShowHidden(!showHidden)}
              className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              {showHidden ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
            </button>
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

      {/* Portfolio Summary */}
      {metrics && (
        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
              <div className="flex items-center">
                <DollarSign className="w-8 h-8 text-blue-500" />
                <div className="ml-3">
                  <p className="text-sm text-gray-500 dark:text-gray-400">Total Value</p>
                  <p className="text-lg font-semibold text-gray-900 dark:text-white">
                    ${metrics.totalValue.toLocaleString()}
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
              <div className="flex items-center">
                <TrendingUp className="w-8 h-8 text-green-500" />
                <div className="ml-3">
                  <p className="text-sm text-gray-500 dark:text-gray-400">Total P&L</p>
                  <p className={`text-lg font-semibold ${
                    metrics.totalPnL >= 0 ? 'text-green-600' : 'text-red-600'
                  }`}>
                    ${metrics.totalPnL.toLocaleString()} ({metrics.totalPnLPercent.toFixed(2)}%)
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
              <div className="flex items-center">
                <BarChart3 className="w-8 h-8 text-purple-500" />
                <div className="ml-3">
                  <p className="text-sm text-gray-500 dark:text-gray-400">Win Rate</p>
                  <p className="text-lg font-semibold text-gray-900 dark:text-white">
                    {metrics.winRate.toFixed(1)}%
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
              <div className="flex items-center">
                <Calendar className="w-8 h-8 text-orange-500" />
                <div className="ml-3">
                  <p className="text-sm text-gray-500 dark:text-gray-400">Total Trades</p>
                  <p className="text-lg font-semibold text-gray-900 dark:text-white">
                    {metrics.totalTrades}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Controls */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-4 sm:space-y-0">
          <div className="flex items-center space-x-4">
            <select
              value={timeRange}
              onChange={(e) => setTimeRange(e.target.value as any)}
              className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            >
              <option value="1d">1 Day</option>
              <option value="7d">7 Days</option>
              <option value="30d">30 Days</option>
              <option value="90d">90 Days</option>
              <option value="1y">1 Year</option>
            </select>
          </div>

          <div className="flex items-center space-x-2">
            <span className="text-sm text-gray-500 dark:text-gray-400">Sort by:</span>
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as any)}
              className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            >
              <option value="value">Value</option>
              <option value="pnl">P&L</option>
              <option value="pnlPercent">P&L %</option>
              <option value="name">Name</option>
            </select>
            <button
              onClick={() => setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')}
              className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              {sortOrder === 'asc' ? '↑' : '↓'}
            </button>
          </div>
        </div>
      </div>

      {/* Assets Table */}
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Asset
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Quantity
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Avg Price
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Current Price
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Market Value
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Unrealized P&L
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                P&L %
              </th>
            </tr>
          </thead>
          <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
            {sortedAssets.map((asset) => (
              <tr key={asset.asset} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                <td className="px-6 py-4 whitespace-nowrap">
                  <div>
                    <div className="text-sm font-medium text-gray-900 dark:text-white">
                      {asset.name}
                    </div>
                    <div className="text-sm text-gray-500 dark:text-gray-400">
                      {asset.symbol}
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                  {asset.quantity.toFixed(6)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                  ${asset.averagePrice.toLocaleString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                  ${asset.currentPrice.toLocaleString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                  ${asset.marketValue.toLocaleString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className={`text-sm font-medium ${
                    asset.unrealizedPnL >= 0 ? 'text-green-600' : 'text-red-600'
                  }`}>
                    ${asset.unrealizedPnL.toLocaleString()}
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className={`text-sm font-medium ${
                    asset.unrealizedPnLPercent >= 0 ? 'text-green-600' : 'text-red-600'
                  }`}>
                    {asset.unrealizedPnLPercent >= 0 ? '+' : ''}{asset.unrealizedPnLPercent.toFixed(2)}%
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Empty State */}
      {sortedAssets.length === 0 && (
        <div className="p-12 text-center">
          <PieChart className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">No assets found</h3>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Start trading to build your portfolio.
          </p>
        </div>
      )}
    </div>
  );
}; 