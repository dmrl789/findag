import React, { useState, useEffect, useCallback } from 'react';
import { validatorAPI, Validator, ValidatorStats } from '../../services/api';
import { showNotification } from '../../components/Common/NotificationSystem';
import LoadingSpinner from '../../components/Common/LoadingSpinner';

interface AddValidatorForm {
  address: string;
  publicKey: string;
  metadata: string;
  adminToken: string;
}

const Validators: React.FC = () => {
  const [validators, setValidators] = useState<Validator[]>([]);
  const [validatorStats, setValidatorStats] = useState<ValidatorStats | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);
  // const [selectedValidator, setSelectedValidator] = useState<Validator | null>(null);
  const [showAddValidator, setShowAddValidator] = useState(false);
  const [addValidatorForm, setAddValidatorForm] = useState<AddValidatorForm>({
    address: '',
    publicKey: '',
    metadata: '',
    adminToken: '',
  });
  const [error, setError] = useState<string | null>(null);

  const fetchValidatorData = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [validatorList, stats] = await Promise.all([
        validatorAPI.getValidatorList(),
        validatorAPI.getValidatorStats(),
      ]);
      setValidators(validatorList);
      setValidatorStats(stats);
    } catch (err) {
      setError('Failed to fetch validator data');
    } finally {
      setIsLoading(false);
      setIsRefreshing(false);
    }
  }, []);

  useEffect(() => {
    fetchValidatorData();
    const interval = setInterval(() => {
      fetchValidatorData();
    }, 30000); // Refresh every 30 seconds
    return () => clearInterval(interval);
  }, [fetchValidatorData]);

  const handleRefresh = () => {
    setIsRefreshing(true);
    fetchValidatorData();
  };

  const handleAddValidator = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!addValidatorForm.address.trim() || !addValidatorForm.publicKey.trim()) {
      showNotification({
        type: 'error',
        title: 'Invalid Input',
        message: 'Please enter both address and public key',
      });
      return;
    }

    setIsLoading(true);
    try {
      await validatorAPI.addValidator(
        addValidatorForm.address,
        addValidatorForm.publicKey,
        addValidatorForm.metadata,
        addValidatorForm.adminToken
      );
      
      setAddValidatorForm({
        address: '',
        publicKey: '',
        metadata: '',
        adminToken: '',
      });
      setShowAddValidator(false);
      
      showNotification({
        type: 'success',
        title: 'Validator Added',
        message: `Validator ${addValidatorForm.address} has been added`,
      });
      
      fetchValidatorData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Add Failed',
        message: 'Failed to add validator',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleRemoveValidator = async (address: string) => {
    setIsLoading(true);
    try {
      await validatorAPI.removeValidator(address, addValidatorForm.adminToken);
      
      showNotification({
        type: 'success',
        title: 'Validator Removed',
        message: 'Validator has been removed from the network',
      });
      
      fetchValidatorData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Remove Failed',
        message: 'Failed to remove validator',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleSlashValidator = async (address: string) => {
    if (!window.confirm(`Are you sure you want to slash validator ${address}?`)) {
      return;
    }

    setIsLoading(true);
    try {
      await validatorAPI.slashValidator(address, addValidatorForm.adminToken);
      
      showNotification({
        type: 'success',
        title: 'Validator Slashed',
        message: 'Validator has been slashed',
      });
      
      fetchValidatorData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Slash Failed',
        message: 'Failed to slash validator',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'active':
        return 'text-green-600';
      case 'inactive':
        return 'text-red-600';
      case 'slashed':
        return 'text-orange-600';
      default:
        return 'text-gray-600';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status.toLowerCase()) {
      case 'active':
        return '🟢';
      case 'inactive':
        return '🔴';
      case 'slashed':
        return '🟠';
      default:
        return '⚪';
    }
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const formatStake = (stake: number) => {
    return (stake / 1000000).toFixed(2) + ' FDG';
  };

  if (isLoading && !validatorStats) {
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
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Validators</h1>
          <p className="text-gray-600 dark:text-gray-400">Validator management and monitoring</p>
        </div>
        
        <div className="flex space-x-2">
          <button
            onClick={() => setShowAddValidator(true)}
            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition"
          >
            Add Validator
          </button>
          <button
            onClick={handleRefresh}
            disabled={isRefreshing}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition disabled:opacity-50"
          >
            {isRefreshing ? <LoadingSpinner size="sm" /> : '🔄'}
            <span className="ml-2">Refresh</span>
          </button>
        </div>
      </div>

      {/* Validator Overview */}
      {validatorStats && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Total Validators</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">{validatorStats.total_validators}</p>
              </div>
              <div className="text-3xl">👥</div>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Active Validators</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">{validatorStats.active_validators}</p>
              </div>
              <div className="text-3xl">✅</div>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Total Stake</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">
                  {formatStake(validatorStats.total_stake)}
                </p>
              </div>
              <div className="text-3xl">💰</div>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Average Performance</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">
                  {validatorStats.average_performance.toFixed(1)}%
                </p>
              </div>
              <div className="text-3xl">📊</div>
            </div>
          </div>
        </div>
      )}

      {/* Validator Statistics */}
      {validatorStats && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-lg font-semibold mb-4">Validator Statistics</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Inactive Validators:</span>
              <span className="ml-2 font-medium">{validatorStats.inactive_validators}</span>
            </div>
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Slashed Validators:</span>
              <span className="ml-2 font-medium">{validatorStats.slashed_validators}</span>
            </div>
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Blocks Produced:</span>
              <span className="ml-2 font-medium">{validatorStats.blocks_produced}</span>
            </div>
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Votes Cast:</span>
              <span className="ml-2 font-medium">{validatorStats.votes_cast}</span>
            </div>
          </div>
        </div>
      )}

      {/* Validator List */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h2 className="text-lg font-semibold mb-4">Validator Network</h2>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead>
              <tr>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Status</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Address</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Stake</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Performance</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Blocks</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Last Active</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Actions</th>
              </tr>
            </thead>
            <tbody>
              {validators.length > 0 ? validators.map(validator => (
                <tr key={validator.address} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                  <td className="px-4 py-2">
                    <div className="flex items-center space-x-2">
                      <span>{getStatusIcon(validator.status)}</span>
                      <span className={`text-sm font-medium ${getStatusColor(validator.status)}`}>
                        {validator.status}
                      </span>
                    </div>
                  </td>
                  <td className="px-4 py-2 font-mono text-sm">{validator.address}</td>
                  <td className="px-4 py-2">{formatStake(validator.stake)}</td>
                  <td className="px-4 py-2">{validator.performance.toFixed(1)}%</td>
                  <td className="px-4 py-2">{validator.blocks_produced}</td>
                  <td className="px-4 py-2 text-sm">{formatTimestamp(validator.last_active)}</td>
                  <td className="px-4 py-2">
                    <div className="flex space-x-2">
                      <button
                        onClick={() => handleRemoveValidator(validator.address)}
                        className="text-red-600 hover:text-red-800 text-sm"
                        disabled={validator.status === 'active'}
                      >
                        Remove
                      </button>
                      <button
                        onClick={() => handleSlashValidator(validator.address)}
                        className="text-orange-600 hover:text-orange-800 text-sm"
                        disabled={validator.status === 'slashed'}
                      >
                        Slash
                      </button>
                    </div>
                  </td>
                </tr>
              )) : (
                <tr>
                  <td colSpan={7} className="text-center text-gray-500 dark:text-gray-400 py-8">
                    No validators found
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Add Validator Modal */}
      {showAddValidator && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Add Validator</h3>
            <form onSubmit={handleAddValidator} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Address</label>
                <input
                  type="text"
                  value={addValidatorForm.address}
                  onChange={(e) => setAddValidatorForm(prev => ({ ...prev, address: e.target.value }))}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  placeholder="Enter validator address"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Public Key</label>
                <input
                  type="text"
                  value={addValidatorForm.publicKey}
                  onChange={(e) => setAddValidatorForm(prev => ({ ...prev, publicKey: e.target.value }))}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  placeholder="Enter public key"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Metadata</label>
                <input
                  type="text"
                  value={addValidatorForm.metadata}
                  onChange={(e) => setAddValidatorForm(prev => ({ ...prev, metadata: e.target.value }))}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  placeholder="Enter metadata (optional)"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Admin Token</label>
                <input
                  type="password"
                  value={addValidatorForm.adminToken}
                  onChange={(e) => setAddValidatorForm(prev => ({ ...prev, adminToken: e.target.value }))}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  placeholder="Enter admin token"
                  required
                />
              </div>
              <div className="flex space-x-3">
                <button
                  type="submit"
                  className="btn btn-primary flex-1"
                  disabled={isLoading}
                >
                  {isLoading ? <LoadingSpinner size="sm" /> : 'Add Validator'}
                </button>
                <button
                  type="button"
                  onClick={() => setShowAddValidator(false)}
                  className="btn btn-secondary flex-1"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default Validators; 