import React, { useState, useEffect, useCallback } from 'react';
import { walletAPI, WalletInfo, Balance, Transaction } from '../../services/api';
import { showNotification } from '../../components/Common/NotificationSystem';
import LoadingSpinner from '../../components/Common/LoadingSpinner';

interface SendForm {
  toAddress: string;
  asset: string;
  amount: number;
  memo: string;
}

const Wallet: React.FC = () => {
  const [walletInfo, setWalletInfo] = useState<WalletInfo | null>(null);
  const [balances, setBalances] = useState<Balance[]>([]);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<'overview' | 'send' | 'receive' | 'history'>('overview');
  const [sendForm, setSendForm] = useState<SendForm>({
    toAddress: '',
    asset: 'EUR',
    amount: 0,
    memo: '',
  });
  const [showCreateWallet, setShowCreateWallet] = useState(false);
  const [showImportWallet, setShowImportWallet] = useState(false);
  const [privateKey, setPrivateKey] = useState('');
  const [error, setError] = useState<string | null>(null);

  const fetchWalletData = useCallback(async (wallet: WalletInfo) => {
    setIsLoading(true);
    setError(null);
    try {
      const [bals, txs] = await Promise.all([
        walletAPI.getBalance(wallet.address),
        walletAPI.getTransactionHistory(wallet.address),
      ]);
      setBalances(bals);
      setTransactions(txs);
    } catch (err) {
      setError('Failed to fetch wallet data');
    } finally {
      setIsLoading(false);
    }
  }, []);

  // On mount, try to load wallet from localStorage (if any)
  useEffect(() => {
    const stored = localStorage.getItem('findag_wallet_info');
    if (stored) {
      const parsed: WalletInfo = JSON.parse(stored);
      setWalletInfo(parsed);
      fetchWalletData(parsed);
    } else {
      setIsLoading(false);
    }
  }, [fetchWalletData]);

  // Save wallet info to localStorage
  useEffect(() => {
    if (walletInfo) {
      localStorage.setItem('findag_wallet_info', JSON.stringify(walletInfo));
    }
  }, [walletInfo]);

  const handleCreateWallet = async () => {
    setIsLoading(true);
    try {
      const wallet = await walletAPI.createWallet();
      setWalletInfo(wallet);
      fetchWalletData(wallet);
      setShowCreateWallet(false);
      showNotification({
        type: 'success',
        title: 'Wallet Created',
        message: 'New wallet has been created successfully',
      });
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Creation Failed',
        message: 'Failed to create wallet',
      });
      setIsLoading(false);
    }
  };

  const handleImportWallet = async () => {
    if (!privateKey.trim()) {
      showNotification({
        type: 'error',
        title: 'Invalid Key',
        message: 'Please enter a valid private key',
      });
      return;
    }
    setIsLoading(true);
    try {
      const wallet = await walletAPI.importWallet(privateKey);
      setWalletInfo(wallet);
      fetchWalletData(wallet);
      setShowImportWallet(false);
      setPrivateKey('');
      showNotification({
        type: 'success',
        title: 'Wallet Imported',
        message: 'Wallet has been imported successfully',
      });
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Import Failed',
        message: 'Failed to import wallet',
      });
      setIsLoading(false);
    }
  };

  const handleSendTransaction = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!sendForm.toAddress.trim()) {
      showNotification({
        type: 'error',
        title: 'Invalid Address',
        message: 'Please enter a valid recipient address',
      });
      return;
    }
    if (!sendForm.amount || sendForm.amount <= 0) {
      showNotification({
        type: 'error',
        title: 'Invalid Amount',
        message: 'Please enter a valid amount',
      });
      return;
    }
    const selectedBalance = balances.find(b => b.asset === sendForm.asset);
    if (!selectedBalance || selectedBalance.available < sendForm.amount) {
      showNotification({
        type: 'error',
        title: 'Insufficient Balance',
        message: 'Insufficient balance for this transaction',
      });
      return;
    }
    setIsLoading(true);
    try {
      await walletAPI.sendTransaction(sendForm.toAddress, sendForm.asset, sendForm.amount, sendForm.memo);
      setSendForm({ toAddress: '', asset: 'EUR', amount: 0, memo: '' });
      showNotification({
        type: 'success',
        title: 'Transaction Sent',
        message: `Sent ${sendForm.amount} ${sendForm.asset}`,
      });
      if (walletInfo) fetchWalletData(walletInfo);
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Send Failed',
        message: 'Failed to send transaction',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    showNotification({
      type: 'success',
      title: 'Copied',
      message: 'Address copied to clipboard',
    });
  };

  const totalBalance = balances.reduce((sum, balance) => sum + balance.amount, 0);
  const availableAssets = balances.map(b => b.asset);

  if (isLoading) {
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

  if (!walletInfo) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Wallet</h1>
            <p className="text-gray-600 dark:text-gray-400">Wallet management</p>
          </div>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-8">
          <div className="text-center">
            <div className="text-6xl mb-4">💰</div>
            <h2 className="text-xl font-bold text-gray-900 dark:text-white mb-2">No Wallet Found</h2>
            <p className="text-gray-600 dark:text-gray-400 mb-6">
              You need to create or import a wallet to start using FinDAG.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <button onClick={() => setShowCreateWallet(true)} className="btn btn-primary">Create New Wallet</button>
              <button onClick={() => setShowImportWallet(true)} className="btn btn-secondary">Import Wallet</button>
            </div>
          </div>
        </div>
        {/* Create Wallet Modal */}
        {showCreateWallet && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Create New Wallet</h3>
              <p className="text-gray-600 dark:text-gray-400 mb-6">This will create a new wallet with a new address and private key.</p>
              <div className="flex space-x-3">
                <button onClick={handleCreateWallet} className="btn btn-primary flex-1">Create Wallet</button>
                <button onClick={() => setShowCreateWallet(false)} className="btn btn-secondary flex-1">Cancel</button>
              </div>
            </div>
          </div>
        )}
        {/* Import Wallet Modal */}
        {showImportWallet && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Import Wallet</h3>
              <input
                type="text"
                className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded mb-4 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                placeholder="Enter private key"
                value={privateKey}
                onChange={e => setPrivateKey(e.target.value)}
              />
              <div className="flex space-x-3">
                <button onClick={handleImportWallet} className="btn btn-primary flex-1">Import</button>
                <button onClick={() => setShowImportWallet(false)} className="btn btn-secondary flex-1">Cancel</button>
              </div>
            </div>
          </div>
        )}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Wallet</h1>
          <p className="text-gray-600 dark:text-gray-400">Wallet management</p>
        </div>
        <div className="flex space-x-2">
          <button onClick={() => setShowCreateWallet(true)} className="btn btn-primary">New Wallet</button>
          <button onClick={() => setShowImportWallet(true)} className="btn btn-secondary">Import</button>
        </div>
      </div>
      {/* Wallet Info */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div className="flex flex-col md:flex-row md:items-center md:justify-between">
          <div>
            <div className="text-lg font-semibold text-gray-900 dark:text-white">Address</div>
            <div className="font-mono text-sm text-gray-700 dark:text-gray-300 break-all">{walletInfo.address}</div>
            <button onClick={() => copyToClipboard(walletInfo.address)} className="text-xs text-blue-600 hover:underline mt-1">Copy</button>
          </div>
          <div className="mt-4 md:mt-0">
            <div className="text-lg font-semibold text-gray-900 dark:text-white">Total Balance</div>
            <div className="text-2xl font-bold text-green-600">{totalBalance.toLocaleString()} EUR</div>
          </div>
        </div>
      </div>
      {/* Tabs */}
      <div className="flex space-x-4 border-b border-gray-200 dark:border-gray-700">
        <button onClick={() => setActiveTab('overview')} className={`py-2 px-4 font-medium ${activeTab === 'overview' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-600 dark:text-gray-400'}`}>Overview</button>
        <button onClick={() => setActiveTab('send')} className={`py-2 px-4 font-medium ${activeTab === 'send' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-600 dark:text-gray-400'}`}>Send</button>
        <button onClick={() => setActiveTab('history')} className={`py-2 px-4 font-medium ${activeTab === 'history' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-600 dark:text-gray-400'}`}>History</button>
      </div>
      {/* Tab Content */}
      {activeTab === 'overview' && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-lg font-semibold mb-4">Balances</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {balances.map((b, i) => (
              <div key={i} className="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <div className="font-medium text-gray-900 dark:text-white">{b.asset}</div>
                <div className="text-2xl font-bold text-green-600">{b.amount.toLocaleString()}</div>
                <div className="text-sm text-gray-500 dark:text-gray-400">Available: {b.available.toLocaleString()}</div>
                <div className="text-sm text-gray-500 dark:text-gray-400">Locked: {b.locked.toLocaleString()}</div>
              </div>
            ))}
          </div>
        </div>
      )}
      {activeTab === 'send' && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 max-w-lg">
          <h2 className="text-lg font-semibold mb-4">Send Funds</h2>
          <form onSubmit={handleSendTransaction} className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">To Address</label>
              <input type="text" className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white" value={sendForm.toAddress} onChange={e => setSendForm({ ...sendForm, toAddress: e.target.value })} required />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Asset</label>
              <select className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white" value={sendForm.asset} onChange={e => setSendForm({ ...sendForm, asset: e.target.value })}>
                {availableAssets.map(asset => (
                  <option key={asset} value={asset}>{asset}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Amount</label>
              <input type="number" className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white" value={sendForm.amount} onChange={e => setSendForm({ ...sendForm, amount: Number(e.target.value) })} min={0} step={0.01} required />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Memo (optional)</label>
              <input type="text" className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white" value={sendForm.memo} onChange={e => setSendForm({ ...sendForm, memo: e.target.value })} />
            </div>
            <button type="submit" className="btn btn-primary w-full">Send</button>
          </form>
        </div>
      )}
      {activeTab === 'history' && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-lg font-semibold mb-4">Transaction History</h2>
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
                    <td className="px-4 py-2 font-mono text-xs text-blue-600 break-all">{tx.id}</td>
                    <td className="px-4 py-2">{tx.transaction_type}</td>
                    <td className="px-4 py-2">{tx.asset}</td>
                    <td className="px-4 py-2">{tx.amount}</td>
                    <td className="px-4 py-2">{tx.status}</td>
                    <td className="px-4 py-2">{new Date(tx.timestamp * 1000).toLocaleString()}</td>
                  </tr>
                )) : (
                  <tr>
                    <td colSpan={6} className="text-center text-gray-500 dark:text-gray-400 py-8">No transactions found</td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};

export default Wallet; 