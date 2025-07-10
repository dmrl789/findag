import React, { useState, useEffect } from 'react';
import { 
  Shield, 
  AlertTriangle, 
  CheckCircle, 
  FileText, 
  Database, 
  Activity,
  TrendingUp,
  Clock,
  Users,
  Globe
} from 'lucide-react';

interface ComplianceMessage {
  id: string;
  type: 'pacs.002' | 'pacs.004' | 'pacs.008' | 'pacs.009' | 'pain.001' | 'pain.002';
  status: 'valid' | 'invalid' | 'pending' | 'rejected';
  timestamp: string;
  sender: string;
  receiver: string;
  amount: number;
  currency: string;
  description: string;
  errors?: string[];
  warnings?: string[];
}

interface ComplianceStats {
  totalMessages: number;
  validMessages: number;
  invalidMessages: number;
  pendingMessages: number;
  rejectionRate: number;
  averageProcessingTime: number;
  last24Hours: number;
}

export const ISO20022Compliance: React.FC = () => {
  const [messages, setMessages] = useState<ComplianceMessage[]>([]);
  const [stats, setStats] = useState<ComplianceStats>({
    totalMessages: 0,
    validMessages: 0,
    invalidMessages: 0,
    pendingMessages: 0,
    rejectionRate: 0,
    averageProcessingTime: 0,
    last24Hours: 0
  });
  const [selectedMessage, setSelectedMessage] = useState<ComplianceMessage | null>(null);
  const [filter, setFilter] = useState<'all' | 'valid' | 'invalid' | 'pending'>('all');

  // Mock data generation
  useEffect(() => {
    const mockMessages: ComplianceMessage[] = [
      {
        id: 'MSG-2024-001',
        type: 'pacs.008',
        status: 'valid',
        timestamp: '2024-01-15T10:30:00Z',
        sender: 'BANK001',
        receiver: 'BANK002',
        amount: 50000.00,
        currency: 'EUR',
        description: 'Settlement payment for EUR/USD trade'
      },
      {
        id: 'MSG-2024-002',
        type: 'pacs.002',
        status: 'invalid',
        timestamp: '2024-01-15T11:15:00Z',
        sender: 'BANK003',
        receiver: 'BANK001',
        amount: 25000.00,
        currency: 'USD',
        description: 'Payment confirmation',
        errors: ['Invalid BIC code', 'Missing mandatory field: OrgnlTxId']
      },
      {
        id: 'MSG-2024-003',
        type: 'pain.001',
        status: 'pending',
        timestamp: '2024-01-15T12:00:00Z',
        sender: 'BANK002',
        receiver: 'BANK004',
        amount: 75000.00,
        currency: 'GBP',
        description: 'Bulk payment initiation'
      },
      {
        id: 'MSG-2024-004',
        type: 'pacs.009',
        status: 'valid',
        timestamp: '2024-01-15T13:45:00Z',
        sender: 'BANK001',
        receiver: 'BANK005',
        amount: 100000.00,
        currency: 'EUR',
        description: 'High-value payment'
      },
      {
        id: 'MSG-2024-005',
        type: 'pacs.004',
        status: 'rejected',
        timestamp: '2024-01-15T14:20:00Z',
        sender: 'BANK006',
        receiver: 'BANK001',
        amount: 15000.00,
        currency: 'USD',
        description: 'Payment return',
        errors: ['Insufficient funds', 'Account blocked']
      }
    ];

    setMessages(mockMessages);
    
    // Calculate stats
    const total = mockMessages.length;
    const valid = mockMessages.filter(m => m.status === 'valid').length;
    const invalid = mockMessages.filter(m => m.status === 'invalid').length;
    const pending = mockMessages.filter(m => m.status === 'pending').length;
    const rejected = mockMessages.filter(m => m.status === 'rejected').length;

    setStats({
      totalMessages: total,
      validMessages: valid,
      invalidMessages: invalid,
      pendingMessages: pending,
      rejectionRate: ((invalid + rejected) / total) * 100,
      averageProcessingTime: 2.3,
      last24Hours: 156
    });
  }, []);

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'valid':
        return <CheckCircle className="w-5 h-5 text-green-500" />;
      case 'invalid':
        return <AlertTriangle className="w-5 h-5 text-red-500" />;
      case 'pending':
        return <Clock className="w-5 h-5 text-yellow-500" />;
      case 'rejected':
        return <AlertTriangle className="w-5 h-5 text-red-600" />;
      default:
        return <Activity className="w-5 h-5 text-gray-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'valid':
        return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
      case 'invalid':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
      case 'pending':
        return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
      case 'rejected':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200';
    }
  };

  const filteredMessages = messages.filter(message => {
    if (filter === 'all') return true;
    return message.status === filter;
  });

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <Shield className="w-8 h-8 text-blue-600" />
          <div>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
              ISO20022 Compliance Monitor
            </h2>
            <p className="text-gray-600 dark:text-gray-400">
              Real-time monitoring of financial message compliance
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-2">
          <span className="text-sm text-gray-500 dark:text-gray-400">Last updated:</span>
          <span className="text-sm font-medium text-gray-900 dark:text-white">
            {new Date().toLocaleTimeString()}
          </span>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Total Messages</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{stats.totalMessages}</p>
            </div>
            <Database className="w-8 h-8 text-blue-500" />
          </div>
        </div>

        <div className="card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Valid Messages</p>
              <p className="text-2xl font-bold text-green-600">{stats.validMessages}</p>
            </div>
            <CheckCircle className="w-8 h-8 text-green-500" />
          </div>
        </div>

        <div className="card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Rejection Rate</p>
              <p className="text-2xl font-bold text-red-600">{stats.rejectionRate.toFixed(1)}%</p>
            </div>
            <AlertTriangle className="w-8 h-8 text-red-500" />
          </div>
        </div>

        <div className="card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Avg Processing</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{stats.averageProcessingTime}s</p>
            </div>
            <TrendingUp className="w-8 h-8 text-blue-500" />
          </div>
        </div>
      </div>

      {/* Filters */}
      <div className="flex items-center space-x-4">
        <span className="text-sm font-medium text-gray-700 dark:text-gray-300">Filter:</span>
        <div className="flex space-x-2">
          {(['all', 'valid', 'invalid', 'pending'] as const).map((filterType) => (
            <button
              key={filterType}
              onClick={() => setFilter(filterType)}
              className={`px-3 py-1 rounded-full text-sm font-medium transition-colors ${
                filter === filterType
                  ? 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200'
                  : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
              }`}
            >
              {filterType.charAt(0).toUpperCase() + filterType.slice(1)}
            </button>
          ))}
        </div>
      </div>

      {/* Messages Table */}
      <div className="card">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead className="bg-gray-50 dark:bg-gray-800">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Message ID
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Type
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Sender
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Receiver
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Amount
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Time
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
              {filteredMessages.map((message) => (
                <tr key={message.id} className="hover:bg-gray-50 dark:hover:bg-gray-800">
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="flex items-center">
                      {getStatusIcon(message.status)}
                      <span className={`ml-2 px-2 py-1 text-xs font-medium rounded-full ${getStatusColor(message.status)}`}>
                        {message.status}
                      </span>
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                    {message.id}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    {message.type}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {message.sender}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {message.receiver}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {message.amount.toLocaleString()} {message.currency}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    {new Date(message.timestamp).toLocaleString()}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <button
                      onClick={() => setSelectedMessage(message)}
                      className="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300"
                    >
                      View Details
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Message Details Modal */}
      {selectedMessage && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                Message Details - {selectedMessage.id}
              </h3>
              <button
                onClick={() => setSelectedMessage(null)}
                className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              >
                ×
              </button>
            </div>

            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="text-sm font-medium text-gray-500 dark:text-gray-400">Message Type</label>
                  <p className="text-sm text-gray-900 dark:text-white">{selectedMessage.type}</p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-500 dark:text-gray-400">Status</label>
                  <div className="flex items-center">
                    {getStatusIcon(selectedMessage.status)}
                    <span className={`ml-2 px-2 py-1 text-xs font-medium rounded-full ${getStatusColor(selectedMessage.status)}`}>
                      {selectedMessage.status}
                    </span>
                  </div>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-500 dark:text-gray-400">Sender</label>
                  <p className="text-sm text-gray-900 dark:text-white">{selectedMessage.sender}</p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-500 dark:text-gray-400">Receiver</label>
                  <p className="text-sm text-gray-900 dark:text-white">{selectedMessage.receiver}</p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-500 dark:text-gray-400">Amount</label>
                  <p className="text-sm text-gray-900 dark:text-white">
                    {selectedMessage.amount.toLocaleString()} {selectedMessage.currency}
                  </p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-500 dark:text-gray-400">Timestamp</label>
                  <p className="text-sm text-gray-900 dark:text-white">
                    {new Date(selectedMessage.timestamp).toLocaleString()}
                  </p>
                </div>
              </div>

              <div>
                <label className="text-sm font-medium text-gray-500 dark:text-gray-400">Description</label>
                <p className="text-sm text-gray-900 dark:text-white">{selectedMessage.description}</p>
              </div>

              {selectedMessage.errors && selectedMessage.errors.length > 0 && (
                <div>
                  <label className="text-sm font-medium text-red-500">Errors</label>
                  <ul className="mt-1 space-y-1">
                    {selectedMessage.errors.map((error, index) => (
                      <li key={index} className="text-sm text-red-600 dark:text-red-400">
                        • {error}
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {selectedMessage.warnings && selectedMessage.warnings.length > 0 && (
                <div>
                  <label className="text-sm font-medium text-yellow-500">Warnings</label>
                  <ul className="mt-1 space-y-1">
                    {selectedMessage.warnings.map((warning, index) => (
                      <li key={index} className="text-sm text-yellow-600 dark:text-yellow-400">
                        • {warning}
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </div>

            <div className="mt-6 flex justify-end space-x-3">
              <button
                onClick={() => setSelectedMessage(null)}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 border border-gray-300 rounded-md hover:bg-gray-200 dark:bg-gray-700 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-600"
              >
                Close
              </button>
              <button className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700">
                Export XML
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 