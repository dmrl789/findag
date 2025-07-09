import React, { useState, useEffect } from 'react';
import { 
  X, 
  Copy, 
  ExternalLink, 
  Clock, 
  User, 
  Hash, 
  DollarSign,
  TrendingUp,
  TrendingDown,
  CheckCircle,
  AlertCircle,
  Info,
  Download,
  Share2
} from 'lucide-react';
import { useAccessibility } from '../Common/AccessibilityProvider';
import { DAGNode } from './EnhancedDAGVisualizer';

export interface Transaction {
  id: string;
  hash: string;
  from: string;
  to: string;
  amount: number;
  currency: string;
  timestamp: number;
  status: 'pending' | 'confirmed' | 'failed' | 'orphaned';
  fee: number;
  gasUsed?: number;
  gasPrice?: number;
  nonce: number;
  blockHash?: string;
  blockNumber?: number;
  confirmations?: number;
  metadata?: Record<string, any>;
}

export interface TransactionDetailsModalProps {
  isOpen: boolean;
  onClose: () => void;
  node: DAGNode | null;
  transactions?: Transaction[];
  loading?: boolean;
  onTransactionClick?: (transaction: Transaction) => void;
  onExportTransactions?: () => void;
}

export const TransactionDetailsModal: React.FC<TransactionDetailsModalProps> = ({
  isOpen,
  onClose,
  node,
  transactions = [],
  loading = false,
  onTransactionClick,
  onExportTransactions,
}) => {
  const { addAnnouncement } = useAccessibility();
  const [selectedTransaction, setSelectedTransaction] = useState<Transaction | null>(null);
  const [showRawData, setShowRawData] = useState(false);
  const [copiedField, setCopiedField] = useState<string | null>(null);

  // Reset state when modal closes
  useEffect(() => {
    if (!isOpen) {
      setSelectedTransaction(null);
      setShowRawData(false);
      setCopiedField(null);
    }
  }, [isOpen]);

  const copyToClipboard = async (text: string, field: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedField(field);
      addAnnouncement(`${field} copied to clipboard`);
      setTimeout(() => setCopiedField(null), 2000);
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
    }
  };

  const formatAmount = (amount: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency === 'USD' ? 'USD' : 'USD',
      minimumFractionDigits: 2,
      maximumFractionDigits: 8,
    }).format(amount);
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'confirmed':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'pending':
        return <Clock className="w-4 h-4 text-yellow-500" />;
      case 'failed':
        return <AlertCircle className="w-4 h-4 text-red-500" />;
      case 'orphaned':
        return <Info className="w-4 h-4 text-gray-500" />;
      default:
        return <Info className="w-4 h-4 text-gray-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'confirmed':
        return 'text-green-600 bg-green-50 border-green-200';
      case 'pending':
        return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'failed':
        return 'text-red-600 bg-red-50 border-red-200';
      case 'orphaned':
        return 'text-gray-600 bg-gray-50 border-gray-200';
      default:
        return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  const getTransactionType = (transaction: Transaction) => {
    if (transaction.amount > 0) {
      return { type: 'Incoming', icon: <TrendingUp className="w-4 h-4 text-green-500" /> };
    } else {
      return { type: 'Outgoing', icon: <TrendingDown className="w-4 h-4 text-red-500" /> };
    }
  };

  if (!isOpen || !node) return null;

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        {/* Backdrop */}
        <div
          className="fixed inset-0 bg-black bg-opacity-50 transition-opacity"
          onClick={onClose}
        />

        {/* Modal */}
        <div className="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-4xl sm:w-full">
          {/* Header */}
          <div className="bg-gray-50 px-6 py-4 border-b border-gray-200">
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-3">
                <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
                  <Hash className="w-4 h-4 text-white" />
                </div>
                <div>
                  <h3 className="text-lg font-medium text-gray-900">
                    {node.label} Details
                  </h3>
                  <p className="text-sm text-gray-500">
                    {transactions.length} transactions • Level {node.level}
                  </p>
                </div>
              </div>
              
              <div className="flex items-center space-x-2">
                {onExportTransactions && (
                  <button
                    onClick={onExportTransactions}
                    className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
                    title="Export Transactions"
                  >
                    <Download className="w-4 h-4" />
                  </button>
                )}
                
                <button
                  onClick={onClose}
                  className="text-gray-400 hover:text-gray-600 transition-colors"
                  aria-label="Close modal"
                >
                  <X className="w-6 h-6" />
                </button>
              </div>
            </div>
          </div>

          {/* Content */}
          <div className="px-6 py-4">
            {/* Node Information */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
              <div className="space-y-4">
                <div>
                  <h4 className="text-sm font-medium text-gray-900 mb-2">Node Information</h4>
                  <div className="bg-gray-50 rounded-lg p-4 space-y-3">
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">Label:</span>
                      <span className="text-sm font-medium text-gray-900">{node.label}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">Validator:</span>
                      <span className="text-sm font-medium text-gray-900">{node.validator}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">Level:</span>
                      <span className="text-sm font-medium text-gray-900">{node.level}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">Status:</span>
                      <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border ${getStatusColor(node.status || 'pending')}`}>
                        {getStatusIcon(node.status || 'pending')}
                        <span className="ml-1 capitalize">{node.status || 'pending'}</span>
                      </span>
                    </div>
                  </div>
                </div>

                {node.hash && (
                  <div>
                    <h4 className="text-sm font-medium text-gray-900 mb-2">Hash</h4>
                    <div className="bg-gray-50 rounded-lg p-4">
                      <div className="flex items-center justify-between">
                        <code className="text-sm text-gray-700 font-mono break-all">
                          {node.hash}
                        </code>
                        <button
                          onClick={() => copyToClipboard(node.hash!, 'Hash')}
                          className="ml-2 p-1 text-gray-500 hover:text-gray-700 hover:bg-gray-200 rounded"
                          title="Copy hash"
                        >
                          <Copy className={`w-4 h-4 ${copiedField === 'Hash' ? 'text-green-500' : ''}`} />
                        </button>
                      </div>
                    </div>
                  </div>
                )}
              </div>

              <div className="space-y-4">
                <div>
                  <h4 className="text-sm font-medium text-gray-900 mb-2">Transaction Summary</h4>
                  <div className="bg-gray-50 rounded-lg p-4 space-y-3">
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">Total Transactions:</span>
                      <span className="text-sm font-medium text-gray-900">{node.transactionCount}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-600">Timestamp:</span>
                      <span className="text-sm font-medium text-gray-900">
                        {formatTimestamp(node.timestamp)}
                      </span>
                    </div>
                    {transactions.length > 0 && (
                      <>
                        <div className="flex justify-between">
                          <span className="text-sm text-gray-600">Total Volume:</span>
                          <span className="text-sm font-medium text-gray-900">
                            {formatAmount(
                              transactions.reduce((sum, tx) => sum + Math.abs(tx.amount), 0),
                              'USD'
                            )}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-sm text-gray-600">Average Fee:</span>
                          <span className="text-sm font-medium text-gray-900">
                            {formatAmount(
                              transactions.reduce((sum, tx) => sum + tx.fee, 0) / transactions.length,
                              'USD'
                            )}
                          </span>
                        </div>
                      </>
                    )}
                  </div>
                </div>

                {node.metadata && Object.keys(node.metadata).length > 0 && (
                  <div>
                    <h4 className="text-sm font-medium text-gray-900 mb-2">Metadata</h4>
                    <div className="bg-gray-50 rounded-lg p-4">
                      <div className="space-y-2">
                        {Object.entries(node.metadata).map(([key, value]) => (
                          <div key={key} className="flex justify-between">
                            <span className="text-sm text-gray-600 capitalize">{key}:</span>
                            <span className="text-sm font-medium text-gray-900">
                              {typeof value === 'object' ? JSON.stringify(value) : String(value)}
                            </span>
                          </div>
                        ))}
                      </div>
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* Transactions List */}
            <div>
              <div className="flex items-center justify-between mb-4">
                <h4 className="text-sm font-medium text-gray-900">Transactions</h4>
                <div className="flex items-center space-x-2">
                  <button
                    onClick={() => setShowRawData(!showRawData)}
                    className={`px-3 py-1 text-xs rounded-lg border ${
                      showRawData
                        ? 'border-primary-500 text-primary-600 bg-primary-50'
                        : 'border-gray-300 text-gray-600 hover:bg-gray-50'
                    }`}
                  >
                    {showRawData ? 'Hide Raw Data' : 'Show Raw Data'}
                  </button>
                </div>
              </div>

              {loading ? (
                <div className="text-center py-8">
                  <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600 mx-auto"></div>
                  <p className="mt-2 text-sm text-gray-600">Loading transactions...</p>
                </div>
              ) : transactions.length === 0 ? (
                <div className="text-center py-8">
                  <Info className="w-8 h-8 text-gray-400 mx-auto mb-2" />
                  <p className="text-sm text-gray-600">No transactions found for this node</p>
                </div>
              ) : (
                <div className="space-y-3 max-h-96 overflow-y-auto">
                  {transactions.map((transaction) => {
                    const transactionType = getTransactionType(transaction);
                    return (
                      <div
                        key={transaction.id}
                        className={`border rounded-lg p-4 cursor-pointer transition-colors ${
                          selectedTransaction?.id === transaction.id
                            ? 'border-primary-500 bg-primary-50'
                            : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
                        }`}
                        onClick={() => {
                          setSelectedTransaction(transaction);
                          onTransactionClick?.(transaction);
                        }}
                      >
                        <div className="flex items-center justify-between">
                          <div className="flex items-center space-x-3">
                            {transactionType.icon}
                            <div>
                              <div className="flex items-center space-x-2">
                                <span className="text-sm font-medium text-gray-900">
                                  {transactionType.type}
                                </span>
                                <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border ${getStatusColor(transaction.status)}`}>
                                  {getStatusIcon(transaction.status)}
                                  <span className="ml-1 capitalize">{transaction.status}</span>
                                </span>
                              </div>
                              <div className="text-sm text-gray-600 mt-1">
                                {formatAmount(transaction.amount, transaction.currency)}
                              </div>
                            </div>
                          </div>
                          
                          <div className="text-right">
                            <div className="text-sm text-gray-600">
                              {formatTimestamp(transaction.timestamp)}
                            </div>
                            <div className="text-xs text-gray-500 mt-1">
                              Fee: {formatAmount(transaction.fee, transaction.currency)}
                            </div>
                          </div>
                        </div>

                        {showRawData && (
                          <div className="mt-3 pt-3 border-t border-gray-200">
                            <div className="grid grid-cols-2 gap-4 text-xs">
                              <div>
                                <span className="text-gray-500">From:</span>
                                <div className="font-mono text-gray-700 break-all">{transaction.from}</div>
                              </div>
                              <div>
                                <span className="text-gray-500">To:</span>
                                <div className="font-mono text-gray-700 break-all">{transaction.to}</div>
                              </div>
                              <div>
                                <span className="text-gray-500">Hash:</span>
                                <div className="font-mono text-gray-700 break-all">{transaction.hash}</div>
                              </div>
                              <div>
                                <span className="text-gray-500">Nonce:</span>
                                <div className="font-mono text-gray-700">{transaction.nonce}</div>
                              </div>
                            </div>
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          </div>

          {/* Footer */}
          <div className="bg-gray-50 px-6 py-3 border-t border-gray-200">
            <div className="flex justify-between items-center">
              <div className="text-sm text-gray-600">
                {transactions.length} transaction(s) • Last updated: {formatTimestamp(Date.now())}
              </div>
              <button
                onClick={onClose}
                className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}; 