import React, { useState, useEffect, useCallback } from 'react';
import { dagAPI, Transaction } from '../../services/api';
import { showNotification } from '../../components/Common/NotificationSystem';
import LoadingSpinner from '../../components/Common/LoadingSpinner';

interface DAGStatus {
  total_blocks: number;
  total_rounds: number;
  current_round: number;
  dag_height: number;
  last_block_hash: string;
  last_round_hash: string;
}

interface DAGBlock {
  hash: string;
  round: number;
  timestamp: number;
  transactions: number;
  size: number;
}

interface TransactionForm {
  to_address: string;
  asset: string;
  amount: number;
  memo: string;
}

const DAGExplorer: React.FC = () => {
  const [dagStatus, setDagStatus] = useState<DAGStatus | null>(null);
  const [blocks, setBlocks] = useState<DAGBlock[]>([]);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<'overview' | 'blocks' | 'transactions' | 'submit'>('overview');
  const [selectedBlock, setSelectedBlock] = useState<string | null>(null);
  const [transactionForm, setTransactionForm] = useState<TransactionForm>({
    to_address: '',
    asset: 'EUR',
    amount: 0,
    memo: '',
  });
  const [error, setError] = useState<string | null>(null);

  const fetchDAGData = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [status, blocksData] = await Promise.all([
        dagAPI.getStatus(),
        dagAPI.getBlocks(20), // Get last 20 blocks
      ]);
      setDagStatus(status as DAGStatus);
      setBlocks(blocksData as DAGBlock[]);
    } catch (err) {
      setError('Failed to fetch DAG data');
    } finally {
      setIsLoading(false);
    }
  }, []);

  const fetchTransactions = useCallback(async (blockHash?: string) => {
    try {
      const txs = await dagAPI.getTransactions(blockHash);
      setTransactions(txs);
    } catch (err) {
      console.error('Failed to fetch transactions:', err);
    }
  }, []);

  useEffect(() => {
    fetchDAGData();
    const interval = setInterval(() => {
      fetchDAGData();
    }, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, [fetchDAGData]);

  useEffect(() => {
    if (selectedBlock) {
      fetchTransactions(selectedBlock);
    } else {
      fetchTransactions();
    }
  }, [selectedBlock, fetchTransactions]);

  const handleSubmitTransaction = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!transactionForm.to_address.trim()) {
      showNotification({
        type: 'error',
        title: 'Invalid Address',
        message: 'Please enter a valid recipient address',
      });
      return;
    }

    if (!transactionForm.amount || transactionForm.amount <= 0) {
      showNotification({
        type: 'error',
        title: 'Invalid Amount',
        message: 'Please enter a valid amount',
      });
      return;
    }

    setIsLoading(true);
    try {
      const transactionData = {
        to_address: transactionForm.to_address,
        asset: transactionForm.asset,
        amount: transactionForm.amount,
        memo: transactionForm.memo,
      };

      const txId = await dagAPI.submitTransaction(transactionData);
      
      setTransactionForm({
        to_address: '',
        asset: 'EUR',
        amount: 0,
        memo: '',
      });

      showNotification({
        type: 'success',
        title: 'Transaction Submitted',
        message: `Transaction ${txId} submitted to DAG`,
      });

      // Refresh DAG data
      fetchDAGData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Submission Failed',
        message: 'Failed to submit transaction to DAG',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const formatHash = (hash: string) => {
    return hash.length > 20 ? `${hash.substring(0, 10)}...${hash.substring(hash.length - 10)}` : hash;
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  if (isLoading && !dagStatus) {
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
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">DAG Explorer</h1>
          <p className="text-gray-600 dark:text-gray-400">BlockDAG visualization and exploration</p>
        </div>
        <div className="flex space-x-2">
          <button
            onClick={() => fetchDAGData()}
            disabled={isLoading}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition disabled:opacity-50"
          >
            {isLoading ? <LoadingSpinner size="sm" /> : 'Refresh'}
          </button>
        </div>
      </div>

      {/* DAG Status */}
      {dagStatus && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-lg font-semibold mb-4">DAG Status</h2>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Total Blocks</div>
              <div className="text-2xl font-bold text-blue-600">{dagStatus.total_blocks.toLocaleString()}</div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Total Rounds</div>
              <div className="text-2xl font-bold text-green-600">{dagStatus.total_rounds.toLocaleString()}</div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Current Round</div>
              <div className="text-2xl font-bold text-purple-600">{dagStatus.current_round}</div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">DAG Height</div>
              <div className="text-2xl font-bold text-orange-600">{dagStatus.dag_height.toLocaleString()}</div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Last Block</div>
              <div className="text-xs font-mono text-blue-600 break-all">{formatHash(dagStatus.last_block_hash)}</div>
            </div>
            <div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Last Round</div>
              <div className="text-xs font-mono text-green-600 break-all">{formatHash(dagStatus.last_round_hash)}</div>
            </div>
          </div>
        </div>
      )}

      {/* Tabs */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
        <div className="border-b border-gray-200 dark:border-gray-700">
          <nav className="flex space-x-8 px-6">
            {(['overview', 'blocks', 'transactions', 'submit'] as const).map((tab) => (
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
          {activeTab === 'overview' && (
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Recent Blocks</h3>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {blocks.slice(0, 6).map((block, index) => (
                    <div
                      key={block.hash}
                      className="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-600 transition"
                      onClick={() => setSelectedBlock(block.hash)}
                    >
                      <div className="flex items-center justify-between mb-2">
                        <div className="text-sm font-medium text-gray-900 dark:text-white">Block #{index + 1}</div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">Round {block.round}</div>
                      </div>
                      <div className="text-xs font-mono text-blue-600 break-all mb-2">{formatHash(block.hash)}</div>
                      <div className="grid grid-cols-2 gap-2 text-xs">
                        <div>
                          <span className="text-gray-500 dark:text-gray-400">TXs:</span> {block.transactions}
                        </div>
                        <div>
                          <span className="text-gray-500 dark:text-gray-400">Size:</span> {block.size} B
                        </div>
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400 mt-2">
                        {formatTimestamp(block.timestamp)}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {activeTab === 'blocks' && (
            <div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">All Blocks</h3>
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                  <thead>
                    <tr>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Hash</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Round</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Transactions</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Size</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Timestamp</th>
                    </tr>
                  </thead>
                  <tbody>
                    {blocks.map((block, _index) => (
                      <tr
                        key={block.hash}
                        className="hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer"
                        onClick={() => setSelectedBlock(block.hash)}
                      >
                        <td className="px-4 py-2 font-mono text-xs text-blue-600 break-all">{formatHash(block.hash)}</td>
                        <td className="px-4 py-2">{block.round}</td>
                        <td className="px-4 py-2">{block.transactions}</td>
                        <td className="px-4 py-2">{block.size} B</td>
                        <td className="px-4 py-2 text-xs">{formatTimestamp(block.timestamp)}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {activeTab === 'transactions' && (
            <div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
                Transactions {selectedBlock && `(Block: ${formatHash(selectedBlock)})`}
              </h3>
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                  <thead>
                    <tr>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">ID</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Type</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Asset</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Amount</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Status</th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Timestamp</th>
                    </tr>
                  </thead>
                  <tbody>
                    {transactions.length > 0 ? transactions.map(tx => (
                      <tr key={tx.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                        <td className="px-4 py-2 font-mono text-xs text-blue-600 break-all">{formatHash(tx.id)}</td>
                        <td className="px-4 py-2">{tx.transaction_type}</td>
                        <td className="px-4 py-2">{tx.asset}</td>
                        <td className="px-4 py-2">{tx.amount}</td>
                        <td className="px-4 py-2">{tx.status}</td>
                        <td className="px-4 py-2 text-xs">{formatTimestamp(tx.timestamp)}</td>
                      </tr>
                    )) : (
                      <tr>
                        <td colSpan={6} className="text-center text-gray-500 dark:text-gray-400 py-8">
                          No transactions found
                        </td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {activeTab === 'submit' && (
            <div className="max-w-lg">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Submit Transaction</h3>
              <form onSubmit={handleSubmitTransaction} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">To Address</label>
                  <input
                    type="text"
                    value={transactionForm.to_address}
                    onChange={(e) => setTransactionForm(prev => ({ ...prev, to_address: e.target.value }))}
                    className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                    placeholder="Enter recipient address"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Asset</label>
                  <select
                    value={transactionForm.asset}
                    onChange={(e) => setTransactionForm(prev => ({ ...prev, asset: e.target.value }))}
                    className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  >
                    <option value="EUR">EUR</option>
                    <option value="USD">USD</option>
                    <option value="GBP">GBP</option>
                    <option value="JPY">JPY</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Amount</label>
                  <input
                    type="number"
                    value={transactionForm.amount || ''}
                    onChange={(e) => setTransactionForm(prev => ({ ...prev, amount: parseFloat(e.target.value) || 0 }))}
                    className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                    placeholder="Enter amount"
                    step="0.01"
                    min="0"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Memo (optional)</label>
                  <input
                    type="text"
                    value={transactionForm.memo}
                    onChange={(e) => setTransactionForm(prev => ({ ...prev, memo: e.target.value }))}
                    className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                    placeholder="Enter memo"
                  />
                </div>
                <button
                  type="submit"
                  className="w-full btn btn-primary"
                  disabled={isLoading}
                >
                  {isLoading ? <LoadingSpinner size="sm" /> : 'Submit Transaction'}
                </button>
              </form>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default DAGExplorer; 