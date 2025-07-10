import React, { useState, useEffect } from 'react';
import { Send, Activity, Shield, CheckCircle, AlertCircle, Loader } from 'lucide-react';
import { finDAGApi } from '../../services/api';
import { useNotifications, createNotification } from '../Common/NotificationSystem';

interface DAGOperationsProps {
  className?: string;
}

interface NetworkStatus {
  status: string;
  timestamp: string;
  version: string;
}

interface ValidatorInfo {
  id: string;
  address: string;
  publicKey: string;
  stake: number;
  status: string;
  lastSeen: number;
  pingLatency: number;
}

export const DAGOperations: React.FC<DAGOperationsProps> = ({ className = '' }) => {
  const [networkStatus, setNetworkStatus] = useState<NetworkStatus | null>(null);
  const [validators, setValidators] = useState<ValidatorInfo[]>([]);
  const [loadingStatus, setLoadingStatus] = useState(false);
  const [loadingValidators, setLoadingValidators] = useState(false);
  const [submittingTransaction, setSubmittingTransaction] = useState(false);
  
  // Transaction form state
  const [transactionForm, setTransactionForm] = useState({
    from: '',
    to: '',
    amount: '',
    asset: 'USD',
    purpose: '',
    shardId: '',
  });

  const { addNotification } = useNotifications();

  // Load network status
  const loadNetworkStatus = async () => {
    setLoadingStatus(true);
    try {
      const status = await finDAGApi.getHealth();
      setNetworkStatus(status);
      
      addNotification(createNotification.success(
        'Network Status Updated',
        `Network is ${status.status}`,
        { category: 'system' }
      ));
    } catch (error: any) {
      addNotification(createNotification.error(
        'Network Error',
        error.message || 'Failed to load network status',
        { category: 'system' }
      ));
    } finally {
      setLoadingStatus(false);
    }
  };

  // Load validators
  const loadValidators = async () => {
    setLoadingValidators(true);
    try {
      const validatorList = await finDAGApi.getValidators();
      setValidators(validatorList);
      
      addNotification(createNotification.success(
        'Validators Loaded',
        `Loaded ${validatorList.length} validators`,
        { category: 'system' }
      ));
    } catch (error: any) {
      addNotification(createNotification.error(
        'Validator Error',
        error.message || 'Failed to load validators',
        { category: 'system' }
      ));
    } finally {
      setLoadingValidators(false);
    }
  };

  // Submit DAG transaction
  const handleSubmitTransaction = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmittingTransaction(true);
    
    try {
      const response = await finDAGApi.submitDagTransaction({
        from: transactionForm.from,
        to: transactionForm.to,
        amount: parseFloat(transactionForm.amount),
        asset: transactionForm.asset,
        purpose: transactionForm.purpose || undefined,
        shard_id: transactionForm.shardId ? parseInt(transactionForm.shardId) : undefined,
      });
      
      addNotification(createNotification.success(
        'Transaction Submitted',
        `Transaction ${response.tx_hash} submitted successfully`,
        { category: 'trading' }
      ));
      
      // Reset form
      setTransactionForm({
        from: '',
        to: '',
        amount: '',
        asset: 'USD',
        purpose: '',
        shardId: '',
      });
    } catch (error: any) {
      addNotification(createNotification.error(
        'Transaction Error',
        error.message || 'Failed to submit transaction',
        { category: 'trading' }
      ));
    } finally {
      setSubmittingTransaction(false);
    }
  };

  // Load initial data
  useEffect(() => {
    loadNetworkStatus();
    loadValidators();
  }, []);

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Network Status */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-gray-900">Network Status</h3>
          <button
            onClick={loadNetworkStatus}
            disabled={loadingStatus}
            className="btn-secondary flex items-center space-x-2"
          >
            {loadingStatus ? (
              <Loader className="w-4 h-4 animate-spin" />
            ) : (
              <Activity className="w-4 h-4" />
            )}
            <span>Refresh</span>
          </button>
        </div>
        
        {networkStatus && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="flex items-center space-x-3">
              <div className={`w-3 h-3 rounded-full ${
                networkStatus.status === 'healthy' ? 'bg-green-500' : 'bg-red-500'
              }`} />
              <div>
                <div className="text-sm font-medium text-gray-900">Status</div>
                <div className="text-sm text-gray-500 capitalize">{networkStatus.status}</div>
              </div>
            </div>
            
            <div>
              <div className="text-sm font-medium text-gray-900">Version</div>
              <div className="text-sm text-gray-500">{networkStatus.version}</div>
            </div>
            
            <div>
              <div className="text-sm font-medium text-gray-900">Last Updated</div>
              <div className="text-sm text-gray-500">
                {new Date(networkStatus.timestamp).toLocaleString()}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Validators */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-gray-900">Validators</h3>
          <button
            onClick={loadValidators}
            disabled={loadingValidators}
            className="btn-secondary flex items-center space-x-2"
          >
            {loadingValidators ? (
              <Loader className="w-4 h-4 animate-spin" />
            ) : (
              <Shield className="w-4 h-4" />
            )}
            <span>Refresh</span>
          </button>
        </div>
        
        {validators.length > 0 && (
          <div className="space-y-3">
            {validators.map((validator) => (
              <div key={validator.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                <div className="flex items-center space-x-3">
                  <div className={`w-2 h-2 rounded-full ${
                    validator.status === 'active' ? 'bg-green-500' : 'bg-red-500'
                  }`} />
                  <div>
                    <div className="text-sm font-medium text-gray-900">
                      {validator.address.substring(0, 8)}...{validator.address.substring(validator.address.length - 8)}
                    </div>
                    <div className="text-xs text-gray-500">
                      Stake: {validator.stake.toLocaleString()} | Latency: {validator.pingLatency}ms
                    </div>
                  </div>
                </div>
                <div className="text-xs text-gray-500">
                  {validator.status}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Transaction Submission */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Submit DAG Transaction</h3>
        
        <form onSubmit={handleSubmitTransaction} className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                From Address
              </label>
              <input
                type="text"
                value={transactionForm.from}
                onChange={(e) => setTransactionForm(prev => ({ ...prev, from: e.target.value }))}
                placeholder="0x..."
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                required
              />
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                To Address
              </label>
              <input
                type="text"
                value={transactionForm.to}
                onChange={(e) => setTransactionForm(prev => ({ ...prev, to: e.target.value }))}
                placeholder="0x..."
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                required
              />
            </div>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Amount
              </label>
              <input
                type="number"
                step="0.000001"
                value={transactionForm.amount}
                onChange={(e) => setTransactionForm(prev => ({ ...prev, amount: e.target.value }))}
                placeholder="0.00"
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                required
              />
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Asset
              </label>
              <select
                value={transactionForm.asset}
                onChange={(e) => setTransactionForm(prev => ({ ...prev, asset: e.target.value }))}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="USD">USD</option>
                <option value="EUR">EUR</option>
                <option value="GBP">GBP</option>
                <option value="BTC">BTC</option>
                <option value="ETH">ETH</option>
              </select>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Shard ID (Optional)
              </label>
              <input
                type="number"
                value={transactionForm.shardId}
                onChange={(e) => setTransactionForm(prev => ({ ...prev, shardId: e.target.value }))}
                placeholder="0"
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Purpose (Optional)
            </label>
            <input
              type="text"
              value={transactionForm.purpose}
              onChange={(e) => setTransactionForm(prev => ({ ...prev, purpose: e.target.value }))}
              placeholder="Transaction purpose..."
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={submittingTransaction}
              className="btn-primary flex items-center space-x-2"
            >
              {submittingTransaction ? (
                <Loader className="w-4 h-4 animate-spin" />
              ) : (
                <Send className="w-4 h-4" />
              )}
              <span>{submittingTransaction ? 'Submitting...' : 'Submit Transaction'}</span>
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}; 