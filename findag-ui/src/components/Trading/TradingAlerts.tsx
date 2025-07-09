import React, { useState, useEffect } from 'react';
import { 
  Bell, 
  Plus, 
  Edit, 
  Trash2, 
  AlertTriangle, 
  TrendingUp, 
  TrendingDown,
  DollarSign,
  Percent,
  BarChart3,
  Settings,
  CheckCircle,
  XCircle,
  Clock,
  Mail,
  Smartphone
} from 'lucide-react';
import { useNotifications, createNotification } from '../Common/NotificationSystem';

export interface TradingAlert {
  id: string;
  pair: string;
  type: 'price' | 'percentage' | 'volume' | 'technical';
  condition: 'above' | 'below' | 'crosses';
  value: number;
  triggered: boolean;
  triggeredAt?: number;
  active: boolean;
  notificationType: 'email' | 'push' | 'both';
  description?: string;
  createdAt: number;
  lastChecked: number;
}

interface TradingAlertsProps {
  className?: string;
}

export const TradingAlerts: React.FC<TradingAlertsProps> = ({ className = '' }) => {
  const [alerts, setAlerts] = useState<TradingAlert[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [editingAlert, setEditingAlert] = useState<TradingAlert | null>(null);
  const [formData, setFormData] = useState({
    pair: 'BTC/USD',
    type: 'price' as TradingAlert['type'],
    condition: 'above' as TradingAlert['condition'],
    value: '',
    description: '',
    notificationType: 'push' as TradingAlert['notificationType'],
  });

  const { addNotification } = useNotifications();

  // Mock data - in real app this would come from API
  useEffect(() => {
    const mockAlerts: TradingAlert[] = [
      {
        id: '1',
        pair: 'BTC/USD',
        type: 'price',
        condition: 'above',
        value: 55000,
        triggered: false,
        active: true,
        notificationType: 'push',
        description: 'BTC price alert above $55,000',
        createdAt: Date.now() - 86400000,
        lastChecked: Date.now() - 300000,
      },
      {
        id: '2',
        pair: 'ETH/USD',
        type: 'percentage',
        condition: 'below',
        value: -5,
        triggered: true,
        triggeredAt: Date.now() - 1800000,
        active: true,
        notificationType: 'both',
        description: 'ETH price drop alert',
        createdAt: Date.now() - 172800000,
        lastChecked: Date.now() - 1800000,
      },
      {
        id: '3',
        pair: 'BTC/USD',
        type: 'volume',
        condition: 'above',
        value: 1000000,
        triggered: false,
        active: false,
        notificationType: 'email',
        description: 'High volume alert',
        createdAt: Date.now() - 259200000,
        lastChecked: Date.now() - 600000,
      },
    ];
    setAlerts(mockAlerts);
  }, []);

  const resetForm = () => {
    setFormData({
      pair: 'BTC/USD',
      type: 'price',
      condition: 'above',
      value: '',
      description: '',
      notificationType: 'push',
    });
    setEditingAlert(null);
  };

  const handleCreateAlert = () => {
    if (!formData.value || isNaN(Number(formData.value))) {
      addNotification(createNotification.error(
        'Invalid Value',
        'Please enter a valid numeric value for the alert',
        { category: 'trading' }
      ));
      return;
    }

    const newAlert: TradingAlert = {
      id: `alert_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      pair: formData.pair,
      type: formData.type,
      condition: formData.condition,
      value: Number(formData.value),
      triggered: false,
      active: true,
      notificationType: formData.notificationType,
      description: formData.description,
      createdAt: Date.now(),
      lastChecked: Date.now(),
    };

    setAlerts(prev => [newAlert, ...prev]);
    setShowForm(false);
    resetForm();

    addNotification(createNotification.success(
      'Alert Created',
      `Price alert for ${formData.pair} has been created successfully`,
      {
        category: 'trading',
        actions: [
          {
            label: 'View Alert',
            action: () => {
              // Scroll to the new alert or highlight it
            },
            variant: 'primary'
          }
        ]
      }
    ));
  };

  const handleEditAlert = () => {
    if (!editingAlert) return;

    if (!formData.value || isNaN(Number(formData.value))) {
      addNotification(createNotification.error(
        'Invalid Value',
        'Please enter a valid numeric value for the alert',
        { category: 'trading' }
      ));
      return;
    }

    const updatedAlert: TradingAlert = {
      ...editingAlert,
      pair: formData.pair,
      type: formData.type,
      condition: formData.condition,
      value: Number(formData.value),
      notificationType: formData.notificationType,
      description: formData.description,
    };

    setAlerts(prev => prev.map(alert => 
      alert.id === editingAlert.id ? updatedAlert : alert
    ));
    setShowForm(false);
    resetForm();

    addNotification(createNotification.success(
      'Alert Updated',
      `Alert for ${formData.pair} has been updated successfully`,
      { category: 'trading' }
    ));
  };

  const handleDeleteAlert = (alertId: string) => {
    const alert = alerts.find(a => a.id === alertId);
    setAlerts(prev => prev.filter(alert => alert.id !== alertId));

    addNotification(createNotification.info(
      'Alert Deleted',
      `Alert for ${alert?.pair} has been deleted`,
      { category: 'trading' }
    ));
  };

  const handleToggleAlert = (alertId: string) => {
    const alert = alerts.find(a => a.id === alertId);
    if (!alert) return;

    const updatedAlert = { ...alert, active: !alert.active };
    setAlerts(prev => prev.map(a => a.id === alertId ? updatedAlert : a));

    addNotification(createNotification.info(
      `Alert ${updatedAlert.active ? 'Activated' : 'Deactivated'}`,
      `Alert for ${alert.pair} has been ${updatedAlert.active ? 'activated' : 'deactivated'}`,
      { category: 'trading' }
    ));
  };

  const openEditForm = (alert: TradingAlert) => {
    setEditingAlert(alert);
    setFormData({
      pair: alert.pair,
      type: alert.type,
      condition: alert.condition,
      value: alert.value.toString(),
      description: alert.description || '',
      notificationType: alert.notificationType,
    });
    setShowForm(true);
  };

  const getAlertIcon = (alert: TradingAlert) => {
    switch (alert.type) {
      case 'price':
        return <DollarSign className="w-4 h-4" />;
      case 'percentage':
        return <Percent className="w-4 h-4" />;
      case 'volume':
        return <BarChart3 className="w-4 h-4" />;
      case 'technical':
        return <TrendingUp className="w-4 h-4" />;
      default:
        return <Bell className="w-4 h-4" />;
    }
  };

  const getConditionText = (alert: TradingAlert) => {
    switch (alert.condition) {
      case 'above':
        return 'Above';
      case 'below':
        return 'Below';
      case 'crosses':
        return 'Crosses';
      default:
        return alert.condition;
    }
  };

  const getValueDisplay = (alert: TradingAlert) => {
    switch (alert.type) {
      case 'price':
        return `$${alert.value.toLocaleString()}`;
      case 'percentage':
        return `${alert.value > 0 ? '+' : ''}${alert.value}%`;
      case 'volume':
        return `${(alert.value / 1000000).toFixed(1)}M`;
      case 'technical':
        return alert.value.toString();
      default:
        return alert.value.toString();
    }
  };

  const getStatusColor = (alert: TradingAlert) => {
    if (alert.triggered) return 'text-green-600 dark:text-green-400';
    if (!alert.active) return 'text-gray-500 dark:text-gray-400';
    return 'text-blue-600 dark:text-blue-400';
  };

  const getNotificationIcon = (type: TradingAlert['notificationType']) => {
    switch (type) {
      case 'email':
        return <Mail className="w-4 h-4" />;
      case 'push':
        return <Smartphone className="w-4 h-4" />;
      case 'both':
        return (
          <div className="flex space-x-1">
            <Mail className="w-3 h-3" />
            <Smartphone className="w-3 h-3" />
          </div>
        );
      default:
        return <Bell className="w-4 h-4" />;
    }
  };

  const activeAlerts = alerts.filter(alert => alert.active);
  const triggeredAlerts = alerts.filter(alert => alert.triggered);
  const inactiveAlerts = alerts.filter(alert => !alert.active);

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg ${className}`}>
      {/* Header */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
              Trading Alerts
            </h1>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Manage price alerts and notifications
            </p>
          </div>
          <button
            onClick={() => setShowForm(true)}
            className="flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
          >
            <Plus className="w-4 h-4 mr-2" />
            Create Alert
          </button>
        </div>

        {/* Statistics */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mt-6">
          <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
            <div className="flex items-center">
              <Bell className="w-5 h-5 text-blue-600 dark:text-blue-400" />
              <div className="ml-3">
                <p className="text-sm font-medium text-blue-600 dark:text-blue-400">Total Alerts</p>
                <p className="text-2xl font-bold text-blue-900 dark:text-blue-100">{alerts.length}</p>
              </div>
            </div>
          </div>
          <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
            <div className="flex items-center">
              <CheckCircle className="w-5 h-5 text-green-600 dark:text-green-400" />
              <div className="ml-3">
                <p className="text-sm font-medium text-green-600 dark:text-green-400">Active</p>
                <p className="text-2xl font-bold text-green-900 dark:text-green-100">{activeAlerts.length}</p>
              </div>
            </div>
          </div>
          <div className="bg-orange-50 dark:bg-orange-900/20 p-4 rounded-lg">
            <div className="flex items-center">
              <AlertTriangle className="w-5 h-5 text-orange-600 dark:text-orange-400" />
              <div className="ml-3">
                <p className="text-sm font-medium text-orange-600 dark:text-orange-400">Triggered</p>
                <p className="text-2xl font-bold text-orange-900 dark:text-orange-100">{triggeredAlerts.length}</p>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <div className="flex items-center">
              <XCircle className="w-5 h-5 text-gray-600 dark:text-gray-400" />
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-600 dark:text-gray-400">Inactive</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-gray-100">{inactiveAlerts.length}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Alerts List */}
      <div className="p-6">
        {alerts.length === 0 ? (
          <div className="text-center py-12">
            <Bell className="w-12 h-12 mx-auto text-gray-400 mb-4" />
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
              No alerts created
            </h3>
            <p className="text-gray-500 dark:text-gray-400 mb-6">
              Create your first price alert to get notified about market movements
            </p>
            <button
              onClick={() => setShowForm(true)}
              className="inline-flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium"
            >
              <Plus className="w-4 h-4 mr-2" />
              Create Alert
            </button>
          </div>
        ) : (
          <div className="space-y-4">
            {alerts.map((alert) => (
              <div
                key={alert.id}
                className={`p-4 border rounded-lg transition-colors ${
                  alert.triggered
                    ? 'border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20'
                    : alert.active
                    ? 'border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-900/20'
                    : 'border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800'
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <div className={`p-2 rounded-lg ${
                      alert.triggered
                        ? 'bg-green-100 dark:bg-green-900/40'
                        : alert.active
                        ? 'bg-blue-100 dark:bg-blue-900/40'
                        : 'bg-gray-100 dark:bg-gray-700'
                    }`}>
                      {getAlertIcon(alert)}
                    </div>
                    <div>
                      <div className="flex items-center space-x-2">
                        <h3 className="font-medium text-gray-900 dark:text-white">
                          {alert.pair}
                        </h3>
                        <span className={`text-sm ${getStatusColor(alert)}`}>
                          {alert.triggered ? 'Triggered' : alert.active ? 'Active' : 'Inactive'}
                        </span>
                      </div>
                      <p className="text-sm text-gray-600 dark:text-gray-400">
                        {getConditionText(alert)} {getValueDisplay(alert)}
                        {alert.description && ` - ${alert.description}`}
                      </p>
                      <div className="flex items-center space-x-4 mt-1 text-xs text-gray-500 dark:text-gray-400">
                        <span>Created {new Date(alert.createdAt).toLocaleDateString()}</span>
                        {alert.triggeredAt && (
                          <span>Triggered {new Date(alert.triggeredAt).toLocaleDateString()}</span>
                        )}
                        <div className="flex items-center space-x-1">
                          {getNotificationIcon(alert.notificationType)}
                        </div>
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <button
                      onClick={() => handleToggleAlert(alert.id)}
                      className={`p-2 rounded-lg transition-colors ${
                        alert.active
                          ? 'text-green-600 hover:bg-green-100 dark:hover:bg-green-900/40'
                          : 'text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'
                      }`}
                    >
                      {alert.active ? <CheckCircle className="w-4 h-4" /> : <XCircle className="w-4 h-4" />}
                    </button>
                    <button
                      onClick={() => openEditForm(alert)}
                      className="p-2 text-gray-400 hover:text-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                    >
                      <Edit className="w-4 h-4" />
                    </button>
                    <button
                      onClick={() => handleDeleteAlert(alert.id)}
                      className="p-2 text-gray-400 hover:text-red-600 hover:bg-red-100 dark:hover:bg-red-900/40 rounded-lg transition-colors"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Create/Edit Alert Modal */}
      {showForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
            <div className="p-6">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
                  {editingAlert ? 'Edit Alert' : 'Create Alert'}
                </h2>
                <button
                  onClick={() => {
                    setShowForm(false);
                    resetForm();
                  }}
                  className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                >
                  <XCircle className="w-5 h-5" />
                </button>
              </div>

              <form onSubmit={(e) => {
                e.preventDefault();
                editingAlert ? handleEditAlert() : handleCreateAlert();
              }}>
                <div className="space-y-4">
                  {/* Trading Pair */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Trading Pair
                    </label>
                    <select
                      value={formData.pair}
                      onChange={(e) => setFormData(prev => ({ ...prev, pair: e.target.value }))}
                      className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                    >
                      <option value="BTC/USD">BTC/USD</option>
                      <option value="ETH/USD">ETH/USD</option>
                      <option value="ADA/USD">ADA/USD</option>
                      <option value="DOT/USD">DOT/USD</option>
                    </select>
                  </div>

                  {/* Alert Type */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Alert Type
                    </label>
                    <select
                      value={formData.type}
                      onChange={(e) => setFormData(prev => ({ ...prev, type: e.target.value as TradingAlert['type'] }))}
                      className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                    >
                      <option value="price">Price</option>
                      <option value="percentage">Percentage Change</option>
                      <option value="volume">Volume</option>
                      <option value="technical">Technical Indicator</option>
                    </select>
                  </div>

                  {/* Condition */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Condition
                    </label>
                    <select
                      value={formData.condition}
                      onChange={(e) => setFormData(prev => ({ ...prev, condition: e.target.value as TradingAlert['condition'] }))}
                      className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                    >
                      <option value="above">Above</option>
                      <option value="below">Below</option>
                      <option value="crosses">Crosses</option>
                    </select>
                  </div>

                  {/* Value */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Value
                    </label>
                    <input
                      type="number"
                      step="any"
                      value={formData.value}
                      onChange={(e) => setFormData(prev => ({ ...prev, value: e.target.value }))}
                      placeholder={formData.type === 'price' ? '50000' : formData.type === 'percentage' ? '5' : '1000000'}
                      className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                    />
                  </div>

                  {/* Description */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Description (Optional)
                    </label>
                    <input
                      type="text"
                      value={formData.description}
                      onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
                      placeholder="Enter alert description"
                      className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                    />
                  </div>

                  {/* Notification Type */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Notification Type
                    </label>
                    <select
                      value={formData.notificationType}
                      onChange={(e) => setFormData(prev => ({ ...prev, notificationType: e.target.value as TradingAlert['notificationType'] }))}
                      className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                    >
                      <option value="push">Push Notification</option>
                      <option value="email">Email</option>
                      <option value="both">Both</option>
                    </select>
                  </div>
                </div>

                <div className="flex space-x-3 mt-6">
                  <button
                    type="button"
                    onClick={() => {
                      setShowForm(false);
                      resetForm();
                    }}
                    className="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                  >
                    Cancel
                  </button>
                  <button
                    type="submit"
                    className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
                  >
                    {editingAlert ? 'Update Alert' : 'Create Alert'}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 