import React, { useState, useEffect } from 'react';
import { Users, Activity, Shield, TrendingUp, AlertTriangle, CheckCircle } from 'lucide-react';
import { useAppStore } from '../../store';
import { Validator } from '../../types';
import { formatNumber, formatTimestamp, formatLatency } from '../../utils/formatters';

export const ValidatorsPage: React.FC = () => {
  const { validators, isLoading, fetchValidators } = useAppStore();
  const [selectedValidator, setSelectedValidator] = useState<Validator | null>(null);

  useEffect(() => {
    fetchValidators();
  }, [fetchValidators]);

  const activeValidators = validators.filter(v => v.status === 'active');
  const inactiveValidators = validators.filter(v => v.status === 'inactive');
  const slashedValidators = validators.filter(v => v.status === 'slashed');

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'bg-green-100 text-green-800';
      case 'inactive': return 'bg-yellow-100 text-yellow-800';
      case 'slashed': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active': return <CheckCircle className="w-4 h-4" />;
      case 'inactive': return <AlertTriangle className="w-4 h-4" />;
      case 'slashed': return <Shield className="w-4 h-4" />;
      default: return <Activity className="w-4 h-4" />;
    }
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Validators</h1>
          <p className="text-gray-600">Validator management and status</p>
        </div>
        <div className="flex items-center space-x-3">
          <button className="btn-primary flex items-center space-x-2">
            <Users className="w-4 h-4" />
            <span>Add Validator</span>
          </button>
        </div>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-green-100 rounded-lg flex items-center justify-center">
                <CheckCircle className="w-5 h-5 text-green-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Active</p>
              <p className="text-2xl font-semibold text-gray-900">{activeValidators.length}</p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-yellow-100 rounded-lg flex items-center justify-center">
                <AlertTriangle className="w-5 h-5 text-yellow-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Inactive</p>
              <p className="text-2xl font-semibold text-gray-900">{inactiveValidators.length}</p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-red-100 rounded-lg flex items-center justify-center">
                <Shield className="w-5 h-5 text-red-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Slashed</p>
              <p className="text-2xl font-semibold text-gray-900">{slashedValidators.length}</p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center">
                <TrendingUp className="w-5 h-5 text-blue-600" />
              </div>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Total Stake</p>
              <p className="text-2xl font-semibold text-gray-900">
                {formatNumber(validators.reduce((sum, v) => sum + v.stake, 0))}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Validators Table */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Validator
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Address
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Stake
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Latency
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Last Seen
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {isLoading.validators ? (
                <tr>
                  <td colSpan={7} className="px-6 py-4 text-center text-gray-500">
                    Loading validators...
                  </td>
                </tr>
              ) : validators.length === 0 ? (
                <tr>
                  <td colSpan={7} className="px-6 py-4 text-center text-gray-500">
                    No validators found
                  </td>
                </tr>
              ) : (
                validators.map((validator) => (
                  <tr key={validator.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div className="flex-shrink-0 h-10 w-10">
                          <div className="h-10 w-10 rounded-full bg-gray-300 flex items-center justify-center">
                            <span className="text-sm font-medium text-gray-700">
                              {validator.id.slice(0, 2).toUpperCase()}
                            </span>
                          </div>
                        </div>
                        <div className="ml-4">
                          <div className="text-sm font-medium text-gray-900">
                            {validator.id}
                          </div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900 font-mono">
                        {validator.address.slice(0, 8)}...{validator.address.slice(-8)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {formatNumber(validator.stake)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(validator.status)}`}>
                        {getStatusIcon(validator.status)}
                        <span className="ml-1">{validator.status}</span>
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {formatLatency(validator.pingLatency)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {formatTimestamp(validator.lastSeen)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                      <button
                        onClick={() => setSelectedValidator(validator)}
                        className="text-blue-600 hover:text-blue-900 mr-3"
                      >
                        View Details
                      </button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Validator Details Modal */}
      {selectedValidator && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">Validator Details</h2>
              <button
                onClick={() => setSelectedValidator(null)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-500">Validator ID</label>
                  <p className="text-sm text-gray-900">{selectedValidator.id}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Status</label>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(selectedValidator.status)}`}>
                    {getStatusIcon(selectedValidator.status)}
                    <span className="ml-1">{selectedValidator.status}</span>
                  </span>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Address</label>
                  <p className="text-sm text-gray-900 font-mono break-all">{selectedValidator.address}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Public Key</label>
                  <p className="text-sm text-gray-900 font-mono break-all">{selectedValidator.publicKey}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Stake</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedValidator.stake)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Ping Latency</label>
                  <p className="text-sm text-gray-900">{formatLatency(selectedValidator.pingLatency)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Last Seen</label>
                  <p className="text-sm text-gray-900">{formatTimestamp(selectedValidator.lastSeen)}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 