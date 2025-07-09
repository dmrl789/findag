import React, { useState, useEffect } from 'react';
import { 
  Wallet, 
  TrendingUp, 
  TrendingDown, 
  RefreshCw,
  AlertTriangle,
  Eye,
  EyeOff
} from 'lucide-react';
import { useTradingStore } from '../../store/trading';
import { useAuthStore } from '../../store/auth';
import { Asset } from '../../types';
import { formatNumber, formatCurrency } from '../../utils/formatters';
import { LoadingSpinner } from '../Common/LoadingSpinner';

export const UserBalance: React.FC = () => {
  const { user } = useAuthStore();
  const {
    userBalance,
    availableAssets,
    isLoading,
    errors,
    fetchUserBalance,
    fetchAvailableAssets,
  } = useTradingStore();
  
  const [showBalances, setShowBalances] = useState(true);
  const [selectedAsset, setSelectedAsset] = useState<Asset | null>(null);

  useEffect(() => {
    if (user?.username) {
      fetchUserBalance(user.username);
      fetchAvailableAssets();
    }
  }, [user?.username, fetchUserBalance, fetchAvailableAssets]);

  const handleRefresh = () => {
    if (user?.username) {
      fetchUserBalance(user.username);
      fetchAvailableAssets();
    }
  };

  const getAssetInfo = (assetSymbol: string): Asset | undefined => {
    return availableAssets.find(asset => asset.symbol === assetSymbol);
  };

  const calculateTotalValue = (): number => {
    return Object.entries(userBalance).reduce((total, [asset, amount]) => {
      const assetInfo = getAssetInfo(asset);
      if (assetInfo && assetInfo.price) {
        return total + (amount * assetInfo.price);
      }
      return total;
    }, 0);
  };

  const getAssetChangeColor = (asset: Asset) => {
    if (asset.priceChangePercent24h > 0) return 'text-success-600';
    if (asset.priceChangePercent24h < 0) return 'text-danger-600';
    return 'text-gray-600';
  };

  const getAssetChangeIcon = (asset: Asset) => {
    if (asset.priceChangePercent24h > 0) return <TrendingUp className="w-4 h-4" />;
    if (asset.priceChangePercent24h < 0) return <TrendingDown className="w-4 h-4" />;
    return null;
  };

  const totalValue = calculateTotalValue();
  const nonZeroBalances = Object.entries(userBalance).filter(([_, amount]) => amount > 0);

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">My Balance</h3>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => setShowBalances(!showBalances)}
            className="text-gray-600 hover:text-gray-900"
            title={showBalances ? 'Hide balances' : 'Show balances'}
          >
            {showBalances ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
          </button>
          <button
            onClick={handleRefresh}
            disabled={isLoading.userBalance}
            className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
          >
            {isLoading.userBalance ? (
              <>
                <RefreshCw className="w-4 h-4 mr-2 animate-spin" />
                Refreshing...
              </>
            ) : (
              <>
                <RefreshCw className="w-4 h-4 mr-2" />
                Refresh
              </>
            )}
          </button>
        </div>
      </div>

      {errors.userBalance && (
        <div className="mb-4 rounded-md bg-danger-50 p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <AlertTriangle className="h-5 w-5 text-danger-400" />
            </div>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-danger-800">
                {errors.userBalance}
              </h3>
            </div>
          </div>
        </div>
      )}

      {/* Total Value */}
      <div className="mb-6 p-4 bg-primary-50 rounded-lg">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm text-primary-600">Total Portfolio Value</p>
            <p className="text-2xl font-bold text-primary-900">
              {showBalances ? formatCurrency(totalValue) : '••••••'}
            </p>
          </div>
          <div className="p-2 bg-primary-100 rounded-lg">
            <Wallet className="w-6 h-6 text-primary-600" />
          </div>
        </div>
      </div>

      {isLoading.userBalance ? (
        <div className="flex items-center justify-center py-8">
          <LoadingSpinner size="lg" text="Loading balance..." />
        </div>
      ) : nonZeroBalances.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          <Wallet className="w-12 h-12 mx-auto mb-4 text-gray-300" />
          <p>No assets found</p>
          <p className="text-sm">Your asset balances will appear here</p>
        </div>
      ) : (
        <div className="space-y-3">
          {nonZeroBalances.map(([asset, amount]) => {
            const assetInfo = getAssetInfo(asset);
            const value = assetInfo?.price ? amount * assetInfo.price : 0;
            
            return (
              <div
                key={asset}
                className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 cursor-pointer"
                onClick={() => assetInfo && setSelectedAsset(assetInfo)}
              >
                <div className="flex-1">
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center">
                      <span className="text-xs font-bold text-gray-700">
                        {asset.slice(0, 2).toUpperCase()}
                      </span>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-gray-900">{asset}</p>
                      {assetInfo && (
                        <p className="text-xs text-gray-500">{assetInfo.name}</p>
                      )}
                    </div>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-sm font-medium text-gray-900">
                    {showBalances ? formatNumber(amount) : '••••••'}
                  </p>
                  {showBalances && value > 0 && (
                    <p className="text-xs text-gray-500">
                      {formatCurrency(value)}
                    </p>
                  )}
                  {assetInfo && (
                    <div className={`flex items-center space-x-1 text-xs ${getAssetChangeColor(assetInfo)}`}>
                      {getAssetChangeIcon(assetInfo)}
                      <span>
                        {assetInfo.priceChangePercent24h >= 0 ? '+' : ''}
                        {assetInfo.priceChangePercent24h.toFixed(2)}%
                      </span>
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      )}

      {/* Asset Details Modal */}
      {selectedAsset && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4 max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">{selectedAsset.name}</h2>
              <button
                onClick={() => setSelectedAsset(null)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-500">Symbol</label>
                  <p className="text-sm text-gray-900">{selectedAsset.symbol}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Price</label>
                  <p className="text-sm text-gray-900">{formatCurrency(selectedAsset.price)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">24h Change</label>
                  <div className={`flex items-center space-x-1 text-sm ${getAssetChangeColor(selectedAsset)}`}>
                    {getAssetChangeIcon(selectedAsset)}
                    <span>
                      {selectedAsset.priceChangePercent24h >= 0 ? '+' : ''}
                      {selectedAsset.priceChangePercent24h.toFixed(2)}%
                    </span>
                  </div>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">24h Volume</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedAsset.volume24h)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Market Cap</label>
                  <p className="text-sm text-gray-900">{formatCurrency(selectedAsset.marketCap)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Circulating Supply</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedAsset.circulatingSupply)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Total Supply</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedAsset.totalSupply)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Decimals</label>
                  <p className="text-sm text-gray-900">{selectedAsset.decimals}</p>
                </div>
              </div>
              
              {userBalance[selectedAsset.symbol] && (
                <div className="pt-4 border-t border-gray-200">
                  <h4 className="text-sm font-medium text-gray-900 mb-2">Your Balance</h4>
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <label className="block text-xs text-gray-500">Amount</label>
                      <p className="text-sm font-medium text-gray-900">
                        {formatNumber(userBalance[selectedAsset.symbol])} {selectedAsset.symbol}
                      </p>
                    </div>
                    <div>
                      <label className="block text-xs text-gray-500">Value</label>
                      <p className="text-sm font-medium text-gray-900">
                        {formatCurrency(userBalance[selectedAsset.symbol] * selectedAsset.price)}
                      </p>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 