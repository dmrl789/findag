import React, { useState, useEffect } from 'react';
import { 
  Clock, 
  CheckCircle, 
  XCircle, 
  AlertTriangle, 
  Eye,
  RefreshCw,
  Trash2
} from 'lucide-react';
import { useTradingStore } from '../../store/trading';
import { useAuthStore } from '../../store/auth';
import { MarketOrder } from '../../types';
import { formatPrice, formatNumber, formatTimestamp } from '../../utils/formatters';
import { LoadingSpinner } from '../Common/LoadingSpinner';

export const UserOrders: React.FC = () => {
  const { user } = useAuthStore();
  const {
    userOrders,
    isLoading,
    errors,
    fetchUserOrders,
    cancelOrder,
    setError,
  } = useTradingStore();
  
  const [selectedOrder, setSelectedOrder] = useState<MarketOrder | null>(null);
  const [cancellingOrder, setCancellingOrder] = useState<string | null>(null);

  useEffect(() => {
    if (user?.username) {
      fetchUserOrders(user.username);
    }
  }, [user?.username, fetchUserOrders]);

  const getStatusColor = (status: MarketOrder['status']) => {
    switch (status) {
      case 'pending': return 'bg-yellow-100 text-yellow-800';
      case 'filled': return 'bg-green-100 text-green-800';
      case 'cancelled': return 'bg-red-100 text-red-800';
      case 'rejected': return 'bg-gray-100 text-gray-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: MarketOrder['status']) => {
    switch (status) {
      case 'pending': return <Clock className="w-4 h-4" />;
      case 'filled': return <CheckCircle className="w-4 h-4" />;
      case 'cancelled': return <XCircle className="w-4 h-4" />;
      case 'rejected': return <AlertTriangle className="w-4 h-4" />;
      default: return <Clock className="w-4 h-4" />;
    }
  };

  const handleCancelOrder = async (orderId: string) => {
    if (!confirm('Are you sure you want to cancel this order?')) {
      return;
    }

    setCancellingOrder(orderId);
    setError('userOrders');
    
    try {
      await cancelOrder(orderId);
    } catch (error) {
      // Error is already handled in the store
    } finally {
      setCancellingOrder(null);
    }
  };

  const handleRefresh = () => {
    if (user?.username) {
      fetchUserOrders(user.username);
    }
  };

  const pendingOrders = userOrders.filter(order => order.status === 'pending');
  const completedOrders = userOrders.filter(order => order.status !== 'pending');

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">My Orders</h3>
        <button
          onClick={handleRefresh}
          disabled={isLoading.userOrders}
          className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
        >
          {isLoading.userOrders ? (
            <>
              <RefreshCw className="w-4 h-4 mr-2 animate-spin" />
              Refreshing...
            </>
          ) : (
            <>
              <RefreshCw className="w-4 h-4 mr-2" />
              Refresh
            </>
          )}
        </button>
      </div>

      {errors.userOrders && (
        <div className="mb-4 rounded-md bg-danger-50 p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <AlertTriangle className="h-5 w-5 text-danger-400" />
            </div>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-danger-800">
                {errors.userOrders}
              </h3>
            </div>
          </div>
        </div>
      )}

      {isLoading.userOrders ? (
        <div className="flex items-center justify-center py-8">
          <LoadingSpinner size="lg" text="Loading orders..." />
        </div>
      ) : userOrders.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          <Clock className="w-12 h-12 mx-auto mb-4 text-gray-300" />
          <p>No orders found</p>
          <p className="text-sm">Your trading orders will appear here</p>
        </div>
      ) : (
        <div className="space-y-6">
          {/* Pending Orders */}
          {pendingOrders.length > 0 && (
            <div>
              <h4 className="text-md font-medium text-gray-900 mb-3">Pending Orders</h4>
              <div className="space-y-2">
                {pendingOrders.map((order) => (
                  <div
                    key={order.id}
                    className="flex items-center justify-between p-3 bg-yellow-50 border border-yellow-200 rounded-lg"
                  >
                    <div className="flex-1">
                      <div className="flex items-center space-x-3">
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(order.status)}`}>
                          {getStatusIcon(order.status)}
                          <span className="ml-1">{order.status}</span>
                        </span>
                        <span className="text-sm font-medium text-gray-900">
                          {order.side.toUpperCase()} {order.pair}
                        </span>
                      </div>
                      <div className="mt-1 text-sm text-gray-600">
                        {formatNumber(order.amount)} @ {order.price ? formatPrice(order.price) : 'Market'}
                      </div>
                      <div className="text-xs text-gray-500">
                        {formatTimestamp(order.timestamp)}
                      </div>
                    </div>
                    <div className="flex items-center space-x-2">
                      <button
                        onClick={() => setSelectedOrder(order)}
                        className="text-blue-600 hover:text-blue-900"
                        title="View Details"
                      >
                        <Eye className="w-4 h-4" />
                      </button>
                      <button
                        onClick={() => handleCancelOrder(order.id)}
                        disabled={cancellingOrder === order.id}
                        className="text-red-600 hover:text-red-900 disabled:opacity-50"
                        title="Cancel Order"
                      >
                        {cancellingOrder === order.id ? (
                          <RefreshCw className="w-4 h-4 animate-spin" />
                        ) : (
                          <Trash2 className="w-4 h-4" />
                        )}
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Completed Orders */}
          {completedOrders.length > 0 && (
            <div>
              <h4 className="text-md font-medium text-gray-900 mb-3">Completed Orders</h4>
              <div className="space-y-2">
                {completedOrders.slice(0, 10).map((order) => (
                  <div
                    key={order.id}
                    className="flex items-center justify-between p-3 bg-gray-50 border border-gray-200 rounded-lg"
                  >
                    <div className="flex-1">
                      <div className="flex items-center space-x-3">
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(order.status)}`}>
                          {getStatusIcon(order.status)}
                          <span className="ml-1">{order.status}</span>
                        </span>
                        <span className="text-sm font-medium text-gray-900">
                          {order.side.toUpperCase()} {order.pair}
                        </span>
                      </div>
                      <div className="mt-1 text-sm text-gray-600">
                        {formatNumber(order.filledAmount || order.amount)} @ {order.averagePrice ? formatPrice(order.averagePrice) : (order.price ? formatPrice(order.price) : 'Market')}
                      </div>
                      <div className="text-xs text-gray-500">
                        {formatTimestamp(order.timestamp)}
                      </div>
                    </div>
                    <button
                      onClick={() => setSelectedOrder(order)}
                      className="text-blue-600 hover:text-blue-900"
                      title="View Details"
                    >
                      <Eye className="w-4 h-4" />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Order Details Modal */}
      {selectedOrder && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4 max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">Order Details</h2>
              <button
                onClick={() => setSelectedOrder(null)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-500">Order ID</label>
                  <p className="text-sm text-gray-900 font-mono">{selectedOrder.id}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Status</label>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(selectedOrder.status)}`}>
                    {getStatusIcon(selectedOrder.status)}
                    <span className="ml-1">{selectedOrder.status}</span>
                  </span>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Pair</label>
                  <p className="text-sm text-gray-900">{selectedOrder.pair}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Side</label>
                  <p className={`text-sm font-medium ${
                    selectedOrder.side === 'buy' ? 'text-success-600' : 'text-danger-600'
                  }`}>
                    {selectedOrder.side.toUpperCase()}
                  </p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Type</label>
                  <p className="text-sm text-gray-900">{selectedOrder.type}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Amount</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedOrder.amount)}</p>
                </div>
                {selectedOrder.price && (
                  <div>
                    <label className="block text-sm font-medium text-gray-500">Price</label>
                    <p className="text-sm text-gray-900">{formatPrice(selectedOrder.price)}</p>
                  </div>
                )}
                <div>
                  <label className="block text-sm font-medium text-gray-500">Filled Amount</label>
                  <p className="text-sm text-gray-900">{formatNumber(selectedOrder.filledAmount)}</p>
                </div>
                {selectedOrder.averagePrice && (
                  <div>
                    <label className="block text-sm font-medium text-gray-500">Average Price</label>
                    <p className="text-sm text-gray-900">{formatPrice(selectedOrder.averagePrice)}</p>
                  </div>
                )}
                <div>
                  <label className="block text-sm font-medium text-gray-500">Timestamp</label>
                  <p className="text-sm text-gray-900">{formatTimestamp(selectedOrder.timestamp)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">User</label>
                  <p className="text-sm text-gray-900">{selectedOrder.user}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 