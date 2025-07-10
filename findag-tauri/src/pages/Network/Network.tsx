import React, { useState, useEffect, useCallback } from 'react';
import { networkAPI, Peer, NetworkStats } from '../../services/api';
import { showNotification } from '../../components/Common/NotificationSystem';
import LoadingSpinner from '../../components/Common/LoadingSpinner';

interface AddPeerForm {
  address: string;
  port: number;
}

const Network: React.FC = () => {
  const [peers, setPeers] = useState<Peer[]>([]);
  const [networkStats, setNetworkStats] = useState<NetworkStats | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);
  // const [selectedPeer, setSelectedPeer] = useState<Peer | null>(null);
  const [showAddPeer, setShowAddPeer] = useState(false);
  const [addPeerForm, setAddPeerForm] = useState<AddPeerForm>({
    address: '',
    port: 8080,
  });
  const [error, setError] = useState<string | null>(null);

  const fetchNetworkData = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [status, peerList] = await Promise.all([
        networkAPI.getStatus(),
        networkAPI.getPeerList(),
      ]);
      setNetworkStats(status);
      setPeers(peerList);
    } catch (err) {
      setError('Failed to fetch network data');
    } finally {
      setIsLoading(false);
      setIsRefreshing(false);
    }
  }, []);

  useEffect(() => {
    fetchNetworkData();
    const interval = setInterval(() => {
      fetchNetworkData();
    }, 15000); // Refresh every 15 seconds
    return () => clearInterval(interval);
  }, [fetchNetworkData]);

  const handleRefresh = () => {
    setIsRefreshing(true);
    fetchNetworkData();
  };

  const handleAddPeer = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!addPeerForm.address.trim()) {
      showNotification({
        type: 'error',
        title: 'Invalid Address',
        message: 'Please enter a valid peer address',
      });
      return;
    }

    setIsLoading(true);
    try {
      await networkAPI.addPeer(addPeerForm.address, addPeerForm.port);
      
      setAddPeerForm({
        address: '',
        port: 8080,
      });
      setShowAddPeer(false);
      
      showNotification({
        type: 'success',
        title: 'Peer Added',
        message: `Peer ${addPeerForm.address}:${addPeerForm.port} has been added`,
      });
      
      fetchNetworkData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Add Failed',
        message: 'Failed to add peer',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleRemovePeer = async (peerId: string) => {
    setIsLoading(true);
    try {
      await networkAPI.removePeer(peerId);
      
      showNotification({
        type: 'success',
        title: 'Peer Removed',
        message: 'Peer has been removed from the network',
      });
      
      fetchNetworkData();
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Remove Failed',
        message: 'Failed to remove peer',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'connected':
        return 'text-green-600';
      case 'connecting':
        return 'text-yellow-600';
      case 'disconnected':
        return 'text-red-600';
      default:
        return 'text-gray-600';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status.toLowerCase()) {
      case 'connected':
        return '🟢';
      case 'connecting':
        return '🟡';
      case 'disconnected':
        return '🔴';
      default:
        return '⚪';
    }
  };

  const formatLatency = (latency: number) => {
    if (latency === 0) return 'N/A';
    return `${latency}ms`;
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  if (isLoading && !networkStats) {
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
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Network</h1>
          <p className="text-gray-600 dark:text-gray-400">Network status and peer management</p>
        </div>
        
        <div className="flex space-x-2">
          <button
            onClick={() => setShowAddPeer(true)}
            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition"
          >
            Add Peer
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

      {/* Network Overview */}
      {networkStats && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Connected Peers</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">{networkStats.connected_peers}</p>
              </div>
              <div className="text-3xl">🌐</div>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Average Latency</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">{networkStats.average_latency}ms</p>
              </div>
              <div className="text-3xl">⚡</div>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Bandwidth</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">
                  {(networkStats.total_bandwidth / (1024 * 1024)).toFixed(1)} MB/s
                </p>
              </div>
              <div className="text-3xl">📊</div>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">Blocks Exchanged</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">
                  {networkStats.blocks_received + networkStats.blocks_sent}
                </p>
              </div>
              <div className="text-3xl">📦</div>
            </div>
          </div>
        </div>
      )}

      {/* Network Statistics */}
      {networkStats && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-lg font-semibold mb-4">Network Statistics</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Total Peers:</span>
              <span className="ml-2 font-medium">{networkStats.total_peers}</span>
            </div>
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Uptime:</span>
              <span className="ml-2 font-medium">{Math.floor(networkStats.uptime / 3600)}h {Math.floor((networkStats.uptime % 3600) / 60)}m</span>
            </div>
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Blocks Received:</span>
              <span className="ml-2 font-medium">{networkStats.blocks_received}</span>
            </div>
            <div>
              <span className="text-sm text-gray-500 dark:text-gray-400">Blocks Sent:</span>
              <span className="ml-2 font-medium">{networkStats.blocks_sent}</span>
            </div>
          </div>
        </div>
      )}

      {/* Peer List */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h2 className="text-lg font-semibold mb-4">Connected Peers</h2>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead>
              <tr>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Status</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Address</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Port</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Latency</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Version</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Last Seen</th>
                <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Actions</th>
              </tr>
            </thead>
            <tbody>
              {peers.length > 0 ? peers.map(peer => (
                <tr key={peer.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                  <td className="px-4 py-2">
                    <div className="flex items-center space-x-2">
                      <span>{getStatusIcon(peer.status)}</span>
                      <span className={`text-sm font-medium ${getStatusColor(peer.status)}`}>
                        {peer.status}
                      </span>
                    </div>
                  </td>
                  <td className="px-4 py-2 font-mono text-sm">{peer.address}</td>
                  <td className="px-4 py-2">{peer.port}</td>
                  <td className="px-4 py-2">{formatLatency(peer.latency)}</td>
                  <td className="px-4 py-2 text-sm">{peer.version}</td>
                  <td className="px-4 py-2 text-sm">{formatTimestamp(peer.last_seen)}</td>
                  <td className="px-4 py-2">
                    <button
                      onClick={() => handleRemovePeer(peer.id)}
                      className="text-red-600 hover:text-red-800 text-sm"
                    >
                      Remove
                    </button>
                  </td>
                </tr>
              )) : (
                <tr>
                  <td colSpan={7} className="text-center text-gray-500 dark:text-gray-400 py-8">
                    No peers connected
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Add Peer Modal */}
      {showAddPeer && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Add Peer</h3>
            <form onSubmit={handleAddPeer} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Address</label>
                <input
                  type="text"
                  value={addPeerForm.address}
                  onChange={(e) => setAddPeerForm(prev => ({ ...prev, address: e.target.value }))}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  placeholder="Enter peer address (e.g., 192.168.1.100)"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Port</label>
                <input
                  type="number"
                  value={addPeerForm.port}
                  onChange={(e) => setAddPeerForm(prev => ({ ...prev, port: parseInt(e.target.value) || 8080 }))}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white"
                  placeholder="8080"
                  min="1"
                  max="65535"
                  required
                />
              </div>
              <div className="flex space-x-3">
                <button
                  type="submit"
                  className="btn btn-primary flex-1"
                  disabled={isLoading}
                >
                  {isLoading ? <LoadingSpinner size="sm" /> : 'Add Peer'}
                </button>
                <button
                  type="button"
                  onClick={() => setShowAddPeer(false)}
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

export default Network; 