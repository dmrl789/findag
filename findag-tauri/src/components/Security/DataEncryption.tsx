import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useAuth } from './AuthProvider';
import LoadingSpinner from '../Common/LoadingSpinner';
import { showNotification } from '../Common/NotificationSystem';

interface EncryptionKey {
  id: string;
  name: string;
  type: 'AES-256' | 'RSA-2048' | 'ChaCha20-Poly1305';
  status: 'active' | 'inactive' | 'expired';
  createdAt: number;
  expiresAt: number;
  lastRotated: number;
  usage: string[];
}

interface EncryptionStatus {
  databaseEncrypted: boolean;
  fileSystemEncrypted: boolean;
  communicationEncrypted: boolean;
  keyRotationEnabled: boolean;
  lastKeyRotation: number;
  nextKeyRotation: number;
  encryptionAlgorithms: string[];
}

interface DataEncryptionProps {
  className?: string;
}

const DataEncryption: React.FC<DataEncryptionProps> = ({ className = '' }) => {
  const { user, hasPermission } = useAuth();
  const [keys, setKeys] = useState<EncryptionKey[]>([]);
  const [status, setStatus] = useState<EncryptionStatus | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [selectedKey, setSelectedKey] = useState<EncryptionKey | null>(null);
  const [showCreateKey, setShowCreateKey] = useState(false);
  const [newKeyData, setNewKeyData] = useState({
    name: '',
    type: 'AES-256' as const,
    expiresIn: 365,
  });

  useEffect(() => {
    fetchEncryptionData();
  }, []);

  const fetchEncryptionData = async () => {
    if (!user || !hasPermission('manage_encryption')) return;

    setIsLoading(true);
    try {
      const [keysResult, statusResult] = await Promise.all([
        invoke('get_encryption_keys'),
        invoke('get_encryption_status'),
      ]);

      if (keysResult && Array.isArray(keysResult)) {
        setKeys(keysResult as EncryptionKey[]);
      }

      if (statusResult && typeof statusResult === 'object') {
        setStatus(statusResult as EncryptionStatus);
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Failed to fetch encryption data',
        message: 'Unable to load encryption information',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateKey = async () => {
    try {
      const result = await invoke('create_encryption_key', newKeyData);
      
      if (result) {
        fetchEncryptionData();
        setShowCreateKey(false);
        setNewKeyData({ name: '', type: 'AES-256', expiresIn: 365 });
        showNotification({
          type: 'success',
          title: 'Key Created',
          message: 'Encryption key has been created successfully',
        });
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Key Creation Failed',
        message: 'Failed to create encryption key',
      });
    }
  };

  const handleRotateKey = async (keyId: string) => {
    try {
      const result = await invoke('rotate_encryption_key', { keyId });
      
      if (result) {
        fetchEncryptionData();
        showNotification({
          type: 'success',
          title: 'Key Rotated',
          message: 'Encryption key has been rotated successfully',
        });
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Key Rotation Failed',
        message: 'Failed to rotate encryption key',
      });
    }
  };

  const handleDeleteKey = async (keyId: string) => {
    try {
      const result = await invoke('delete_encryption_key', { keyId });
      
      if (result) {
        fetchEncryptionData();
        showNotification({
          type: 'success',
          title: 'Key Deleted',
          message: 'Encryption key has been deleted successfully',
        });
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Key Deletion Failed',
        message: 'Failed to delete encryption key',
      });
    }
  };

  const handleToggleEncryption = async (type: 'database' | 'filesystem' | 'communication') => {
    try {
      const result = await invoke('toggle_encryption', { type });
      
      if (result) {
        fetchEncryptionData();
        showNotification({
          type: 'success',
          title: 'Encryption Updated',
          message: `${type} encryption has been updated`,
        });
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Encryption Update Failed',
        message: `Failed to update ${type} encryption`,
      });
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'text-green-600 bg-green-100 dark:bg-green-900/20';
      case 'inactive': return 'text-gray-600 bg-gray-100 dark:bg-gray-900/20';
      case 'expired': return 'text-red-600 bg-red-100 dark:bg-red-900/20';
      default: return 'text-gray-600 bg-gray-100 dark:bg-gray-900/20';
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString();
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Data Encryption</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Manage encryption keys and security settings
          </p>
        </div>
        
        <button
          onClick={() => setShowCreateKey(true)}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          Create New Key
        </button>
      </div>

      {/* Encryption Status */}
      {status && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Database</h3>
              <div className="flex items-center space-x-2">
                <span className={`w-3 h-3 rounded-full ${status.databaseEncrypted ? 'bg-green-500' : 'bg-red-500'}`} />
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {status.databaseEncrypted ? 'Encrypted' : 'Not Encrypted'}
                </span>
              </div>
            </div>
            <button
              onClick={() => handleToggleEncryption('database')}
              className="w-full px-3 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              {status.databaseEncrypted ? 'Disable' : 'Enable'}
            </button>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">File System</h3>
              <div className="flex items-center space-x-2">
                <span className={`w-3 h-3 rounded-full ${status.fileSystemEncrypted ? 'bg-green-500' : 'bg-red-500'}`} />
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {status.fileSystemEncrypted ? 'Encrypted' : 'Not Encrypted'}
                </span>
              </div>
            </div>
            <button
              onClick={() => handleToggleEncryption('filesystem')}
              className="w-full px-3 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              {status.fileSystemEncrypted ? 'Disable' : 'Enable'}
            </button>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Communication</h3>
              <div className="flex items-center space-x-2">
                <span className={`w-3 h-3 rounded-full ${status.communicationEncrypted ? 'bg-green-500' : 'bg-red-500'}`} />
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {status.communicationEncrypted ? 'Encrypted' : 'Not Encrypted'}
                </span>
              </div>
            </div>
            <button
              onClick={() => handleToggleEncryption('communication')}
              className="w-full px-3 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              {status.communicationEncrypted ? 'Disable' : 'Enable'}
            </button>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Key Rotation</h3>
              <div className="flex items-center space-x-2">
                <span className={`w-3 h-3 rounded-full ${status.keyRotationEnabled ? 'bg-green-500' : 'bg-red-500'}`} />
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {status.keyRotationEnabled ? 'Enabled' : 'Disabled'}
                </span>
              </div>
            </div>
            <div className="text-sm text-gray-600 dark:text-gray-400">
              Next: {formatDate(status.nextKeyRotation)}
            </div>
          </div>
        </div>
      )}

      {/* Encryption Keys */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
        <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
            Encryption Keys ({keys.length})
          </h3>
        </div>
        
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead className="bg-gray-50 dark:bg-gray-700">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Key Name
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Type
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Created
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Expires
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
              {keys.map((key) => (
                <tr key={key.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                    {key.name}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {key.type}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(key.status)}`}>
                      {key.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {formatDate(key.createdAt)}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {formatDate(key.expiresAt)}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-2">
                    <button
                      onClick={() => handleRotateKey(key.id)}
                      className="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300"
                    >
                      Rotate
                    </button>
                    <button
                      onClick={() => handleDeleteKey(key.id)}
                      className="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300"
                    >
                      Delete
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Create Key Modal */}
      {showCreateKey && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white dark:bg-gray-800">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                Create New Encryption Key
              </h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Key Name
                  </label>
                  <input
                    type="text"
                    value={newKeyData.name}
                    onChange={(e) => setNewKeyData({ ...newKeyData, name: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                    placeholder="Enter key name"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Key Type
                  </label>
                  <select
                    value={newKeyData.type}
                    onChange={(e) => setNewKeyData({ ...newKeyData, type: e.target.value as any })}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                  >
                    <option value="AES-256">AES-256</option>
                    <option value="RSA-2048">RSA-2048</option>
                    <option value="ChaCha20-Poly1305">ChaCha20-Poly1305</option>
                  </select>
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Expires In (days)
                  </label>
                  <input
                    type="number"
                    value={newKeyData.expiresIn}
                    onChange={(e) => setNewKeyData({ ...newKeyData, expiresIn: parseInt(e.target.value) })}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                    min="1"
                    max="3650"
                  />
                </div>
              </div>
              
              <div className="mt-6 flex justify-end space-x-3">
                <button
                  onClick={() => setShowCreateKey(false)}
                  className="px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-400 dark:hover:bg-gray-500"
                >
                  Cancel
                </button>
                <button
                  onClick={handleCreateKey}
                  disabled={!newKeyData.name.trim()}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50"
                >
                  Create Key
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default DataEncryption; 