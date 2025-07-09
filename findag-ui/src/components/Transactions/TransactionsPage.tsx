import React, { useState, useEffect } from 'react';
import { Search, Filter, Download, Eye, ExternalLink } from 'lucide-react';
import { useAppStore } from '../../store';
import { Transaction } from '../../types';
import { formatNumber, formatTimestamp, formatAddress } from '../../utils/formatters';
import { PaginatedTable } from '../Common/PaginatedTable';
import { exportTransactions } from '../../utils/export';

export const TransactionsPage: React.FC = () => {
  const { recentTransactions, isLoading, fetchRecentTransactions } = useAppStore();
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [typeFilter, setTypeFilter] = useState<string>('all');
  const [selectedTransaction, setSelectedTransaction] = useState<Transaction | null>(null);

  const columns = [
    {
      key: 'id',
      header: 'Transaction ID',
      render: (tx: Transaction) => (
        <div className="text-sm font-medium text-gray-900">
          {formatAddress(tx.id, 8, 8)}
        </div>
      ),
    },
    {
      key: 'from',
      header: 'From Address',
      render: (tx: Transaction) => (
        <div className="text-sm text-gray-900">
          {formatAddress(tx.from, 6, 4)}
        </div>
      ),
    },
    {
      key: 'to',
      header: 'To Address',
      render: (tx: Transaction) => (
        <div className="text-sm text-gray-900">
          {formatAddress(tx.to, 6, 4)}
        </div>
      ),
    },
    {
      key: 'amount',
      header: 'Amount',
      align: 'right' as const,
      render: (tx: Transaction) => (
        <div>
          <div className="text-sm text-gray-900">
            {formatNumber(tx.amount)} {tx.asset}
          </div>
          {tx.price && (
            <div className="text-xs text-gray-500">
              @ ${formatNumber(tx.price)}
            </div>
          )}
        </div>
      ),
    },
    {
      key: 'type',
      header: 'Type',
      render: (tx: Transaction) => {
        const getTypeColor = (type: string) => {
          switch (type) {
            case 'transfer': return 'bg-purple-100 text-purple-800';
            case 'buy': return 'bg-green-100 text-green-800';
            case 'sell': return 'bg-red-100 text-red-800';
            case 'mint': return 'bg-blue-100 text-blue-800';
            case 'burn': return 'bg-orange-100 text-orange-800';
            default: return 'bg-gray-100 text-gray-800';
          }
        };

        return (
          <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getTypeColor(tx.type)}`}>
            {tx.type}
          </span>
        );
      },
    },
    {
      key: 'status',
      header: 'Status',
      render: (tx: Transaction) => {
        const getStatusColor = (status: string) => {
          switch (status) {
            case 'finalized': return 'bg-green-100 text-green-800';
            case 'confirmed': return 'bg-blue-100 text-blue-800';
            case 'pending': return 'bg-yellow-100 text-yellow-800';
            case 'failed': return 'bg-red-100 text-red-800';
            default: return 'bg-gray-100 text-gray-800';
          }
        };

        return (
          <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(tx.status)}`}>
            {tx.status}
          </span>
        );
      },
    },
    {
      key: 'timestamp',
      header: 'Timestamp',
      render: (tx: Transaction) => (
        <div className="text-sm text-gray-500">
          {formatTimestamp(tx.timestamp.timestamp)}
        </div>
      ),
    },
  ];

  useEffect(() => {
    fetchRecentTransactions();
  }, [fetchRecentTransactions]);

  const filteredTransactions = recentTransactions.filter(tx => {
    const matchesSearch = 
      tx.id.toLowerCase().includes(searchTerm.toLowerCase()) ||
      tx.from.toLowerCase().includes(searchTerm.toLowerCase()) ||
      tx.to.toLowerCase().includes(searchTerm.toLowerCase());
    
    const matchesStatus = statusFilter === 'all' || tx.status === statusFilter;
    const matchesType = typeFilter === 'all' || tx.type === typeFilter;
    
    return matchesSearch && matchesStatus && matchesType;
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'finalized': return 'bg-green-100 text-green-800';
      case 'confirmed': return 'bg-blue-100 text-blue-800';
      case 'pending': return 'bg-yellow-100 text-yellow-800';
      case 'failed': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'transfer': return 'bg-purple-100 text-purple-800';
      case 'buy': return 'bg-green-100 text-green-800';
      case 'sell': return 'bg-red-100 text-red-800';
      case 'mint': return 'bg-blue-100 text-blue-800';
      case 'burn': return 'bg-orange-100 text-orange-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Transactions</h1>
          <p className="text-gray-600">Transaction history and details</p>
        </div>
        <div className="flex items-center space-x-3">
          <button className="btn-secondary flex items-center space-x-2">
            <Download className="w-4 h-4" />
            <span>Export</span>
          </button>
        </div>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
            <input
              type="text"
              placeholder="Search transactions..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          {/* Status Filter */}
          <select
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value)}
            className="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="all">All Status</option>
            <option value="pending">Pending</option>
            <option value="confirmed">Confirmed</option>
            <option value="finalized">Finalized</option>
            <option value="failed">Failed</option>
          </select>

          {/* Type Filter */}
          <select
            value={typeFilter}
            onChange={(e) => setTypeFilter(e.target.value)}
            className="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="all">All Types</option>
            <option value="transfer">Transfer</option>
            <option value="buy">Buy</option>
            <option value="sell">Sell</option>
            <option value="mint">Mint</option>
            <option value="burn">Burn</option>
          </select>

          {/* Results Count */}
          <div className="flex items-center justify-end text-sm text-gray-500">
            {filteredTransactions.length} transactions
          </div>
        </div>
      </div>

      {/* Paginated Table */}
      <PaginatedTable
        data={filteredTransactions}
        columns={columns}
        loading={isLoading.transactions}
        error={null}
        onRefresh={fetchRecentTransactions}
        onExport={() => exportTransactions(filteredTransactions, {
          format: 'csv',
          filename: 'transactions_export.csv',
        })}
        searchable={false} // We handle search manually
        filterable={false} // We handle filtering manually
        sortable={true}
        selectable={true}
        pageSizeOptions={[10, 25, 50, 100]}
        initialPageSize={25}
        emptyMessage="No transactions found matching your criteria"
        getItemKey={(tx) => tx.id}
      />

      {/* Transaction Details Modal */}
      {selectedTransaction && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">Transaction Details</h2>
              <button
                onClick={() => setSelectedTransaction(null)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-500">Transaction ID</label>
                  <p className="text-sm text-gray-900 font-mono">{selectedTransaction.id}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Status</label>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(selectedTransaction.status)}`}>
                    {selectedTransaction.status}
                  </span>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">From</label>
                  <p className="text-sm text-gray-900 font-mono">{selectedTransaction.from}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">To</label>
                  <p className="text-sm text-gray-900 font-mono">{selectedTransaction.to}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Amount</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedTransaction.amount)} {selectedTransaction.asset}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Type</label>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getTypeColor(selectedTransaction.type)}`}>
                    {selectedTransaction.type}
                  </span>
                </div>
                {selectedTransaction.price && (
                  <div>
                    <label className="block text-sm font-medium text-gray-500">Price</label>
                    <p className="text-sm text-gray-900">${formatNumber(selectedTransaction.price)}</p>
                  </div>
                )}
                {selectedTransaction.fee && (
                  <div>
                    <label className="block text-sm font-medium text-gray-500">Fee</label>
                    <p className="text-sm text-gray-900">{formatNumber(selectedTransaction.fee)}</p>
                  </div>
                )}
                <div>
                  <label className="block text-sm font-medium text-gray-500">Timestamp</label>
                  <p className="text-sm text-gray-900">{new Date(selectedTransaction.timestamp.timestamp).toLocaleString()}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Node ID</label>
                  <p className="text-sm text-gray-900">{selectedTransaction.timestamp.nodeId}</p>
                </div>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-500">Signature</label>
                <p className="text-sm text-gray-900 font-mono break-all">{selectedTransaction.signature}</p>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 