import React, { useState, useEffect } from 'react';
import { 
  Wallet, 
  Link, 
  Unlink, 
  RefreshCw,
  AlertTriangle,
  CheckCircle,
  DollarSign,
  ArrowUpDown
} from 'lucide-react';
import { finDAGApi } from '../../services/api';
import { useNotifications, createNotification } from '../Common/NotificationSystem';

interface WalletConnectionProps {
  className?: string;
}

export const WalletConnection: React.FC<WalletConnectionProps> = ({ className = '' }) => {
  const [isConnected, setIsConnected] = useState(false);
  const [walletAddress, setWalletAddress] = useState<string>('');
  const [balance, setBalance] = useState<{ [asset: string]: number }>({});
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const { addNotification } = useNotifications();

  // Check if wallet is already connected
  useEffect(() => {
    const checkWalletConnection = async () => {
      try {
        const balanceResponse = await finDAGApi.getWalletBalance();
        if (balanceResponse.balances.length > 0) {
          setIsConnected(true);
          setBalance(balanceResponse.balances.reduce((acc, bal) => {
            acc[bal.asset] = bal.amount;
            return acc;
          }, {} as { [asset: string]: number }));
        }
      } catch (error) {
        // Wallet not connected
        setIsConnected(false);
      }
    };

    checkWalletConnection();
  }, []);

  const handleConnectWallet = async () => {
    setLoading(true);
    setError(null);

    try {
      const response = await finDAGApi.connectWallet();
      setWalletAddress(response.address);
      setIsConnected(true);
      
      // Fetch initial balance
      const balanceResponse = await finDAGApi.getWalletBalance();
      setBalance(balanceResponse.balances.reduce((acc, bal) => {
        acc[bal.asset] = bal.amount;
        return acc;
      }, {} as { [asset: string]: number }));

      addNotification(createNotification.success(
        'Wallet Connected',
        `Wallet connected successfully: ${response.address.slice(0, 8)}...${response.address.slice(-6)}`,
        { category: 'trading' }
      ));
    } catch (error: any) {
      const errorMessage = error.message || 'Failed to connect wallet';
      setError(errorMessage);
      
      addNotification(createNotification.error(
        'Connection Failed',
        errorMessage,
        { category: 'trading' }
      ));
    } finally {
      setLoading(false);
    }
  };

  const handleDisconnectWallet = () => {
    setIsConnected(false);
    setWalletAddress('');
    setBalance({});
    
    addNotification(createNotification.info(
      'Wallet Disconnected',
      'Wallet has been disconnected',
      { category: 'trading' }
    ));
  };

  const handleRefreshBalance = async () => {
    if (!isConnected) return;

    setLoading(true);
    try {
      const balanceResponse = await finDAGApi.getWalletBalance();
      setBalance(balanceResponse.balances.reduce((acc, bal) => {
        acc[bal.asset] = bal.amount;
        return acc;
      }, {} as { [asset: string]: number }));

      addNotification(createNotification.success(
        'Balance Updated',
        'Wallet balance refreshed successfully',
        { category: 'trading' }
      ));
    } catch (error: any) {
      addNotification(createNotification.error(
        'Balance Error',
        error.message || 'Failed to refresh balance',
        { category: 'trading' }
      ));
    } finally {
      setLoading(false);
    }
  };

  const formatAddress = (address: string) => {
    return `${address.slice(0, 8)}...${address.slice(-6)}`;
  };

  const formatBalance = (amount: number, asset: string) => {
    return `${amount.toFixed(6)} ${asset}`;
  };

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 ${className}`}>
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center space-x-3">
          <div className="p-2 bg-primary-100 rounded-lg">
            <Wallet className="w-6 h-6 text-primary-600" />
          </div>
          <div>
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
              Wallet Connection
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Connect your wallet to start trading
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-2">
          {isConnected && (
            <button
              onClick={handleRefreshBalance}
              disabled={loading}
              className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
            >
              {loading ? (
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
          )}
        </div>
      </div>

      {error && (
        <div className="mb-4 rounded-md bg-danger-50 p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <AlertTriangle className="h-5 w-5 text-danger-400" />
            </div>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-danger-800">
                {error}
              </h3>
            </div>
          </div>
        </div>
      )}

      {!isConnected ? (
        <div className="text-center py-8">
          <div className="mb-4">
            <Wallet className="w-16 h-16 mx-auto text-gray-300" />
          </div>
          <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
            Connect Your Wallet
          </h3>
          <p className="text-gray-500 dark:text-gray-400 mb-6">
            Connect your wallet to access trading features and manage your assets
          </p>
          <button
            onClick={handleConnectWallet}
            disabled={loading}
            className="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
          >
            {loading ? (
              <>
                <RefreshCw className="w-5 h-5 mr-2 animate-spin" />
                Connecting...
              </>
            ) : (
              <>
                <Link className="w-5 h-5 mr-2" />
                Connect Wallet
              </>
            )}
          </button>
        </div>
      ) : (
        <div className="space-y-6">
          {/* Connection Status */}
          <div className="flex items-center justify-between p-4 bg-success-50 rounded-lg">
            <div className="flex items-center space-x-3">
              <CheckCircle className="w-5 h-5 text-success-600" />
              <div>
                <p className="text-sm font-medium text-success-800">Wallet Connected</p>
                <p className="text-xs text-success-600">{formatAddress(walletAddress)}</p>
              </div>
            </div>
            <button
              onClick={handleDisconnectWallet}
              className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            >
              <Unlink className="w-4 h-4 mr-2" />
              Disconnect
            </button>
          </div>

          {/* Balance Display */}
          <div>
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
              Wallet Balance
            </h3>
            <div className="space-y-3">
              {Object.entries(balance).map(([asset, amount]) => (
                <div
                  key={asset}
                  className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
                >
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center">
                      <span className="text-xs font-bold text-gray-700">
                        {asset.slice(0, 2).toUpperCase()}
                      </span>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-gray-900 dark:text-white">{asset}</p>
                      <p className="text-xs text-gray-500 dark:text-gray-400">Available</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="text-sm font-medium text-gray-900 dark:text-white">
                      {formatBalance(amount, asset)}
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Quick Actions */}
          <div>
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
              Quick Actions
            </h3>
            <div className="grid grid-cols-2 gap-3">
              <button className="flex items-center justify-center space-x-2 p-3 bg-primary-50 hover:bg-primary-100 rounded-lg transition-colors">
                <DollarSign className="w-5 h-5 text-primary-600" />
                <span className="text-sm font-medium text-primary-700">Deposit</span>
              </button>
              <button className="flex items-center justify-center space-x-2 p-3 bg-primary-50 hover:bg-primary-100 rounded-lg transition-colors">
                <ArrowUpDown className="w-5 h-5 text-primary-600" />
                <span className="text-sm font-medium text-primary-700">Withdraw</span>
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 