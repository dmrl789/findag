import React, { useState, useEffect } from 'react';
import { 
  TrendingUp, 
  TrendingDown, 
  DollarSign, 
  Users, 
  Activity,
  BarChart3,
  Globe,
  Shield,
  Zap,
  Settings,
  RefreshCw,
  Plus,
  Minus
} from 'lucide-react';
import { useAuthStore } from '../../store/auth';
import { useTheme } from '../../store/theme';

export const Dashboard: React.FC = () => {
  const { user } = useAuthStore();
  const { theme, isDark, toggleTheme } = useTheme();
  const [widgets, setWidgets] = useState([
    { id: 1, title: 'Total Volume', value: '$2.4B', change: '+12.5%', icon: DollarSign, color: 'green' },
    { id: 2, title: 'Active Users', value: '1,234', change: '+8.2%', icon: Users, color: 'blue' },
    { id: 3, title: 'Transactions', value: '45.2K', change: '+15.3%', icon: Activity, color: 'purple' },
    { id: 4, title: 'Network Nodes', value: '156', change: '+2.1%', icon: Globe, color: 'orange' },
  ]);

  const [layout, setLayout] = useState('grid');

  const handleWidgetMove = (fromIndex: number, toIndex: number) => {
    const newWidgets = [...widgets];
    const [movedWidget] = newWidgets.splice(fromIndex, 1);
    newWidgets.splice(toIndex, 0, movedWidget);
    setWidgets(newWidgets);
  };

  const recentActivity = [
    { id: 1, type: 'trade', message: 'Large EUR/USD trade executed', time: '2 min ago', amount: '$500K' },
    { id: 2, type: 'node', message: 'New validator node connected', time: '5 min ago', status: 'online' },
    { id: 3, type: 'alert', message: 'Price alert triggered for GBP/USD', time: '8 min ago', price: '1.2650' },
    { id: 4, type: 'system', message: 'System maintenance completed', time: '12 min ago', status: 'success' },
  ];

  const quickActions = [
    { name: 'New Trade', icon: Plus, color: 'blue', action: () => console.log('New trade') },
    { name: 'View Charts', icon: BarChart3, color: 'green', action: () => console.log('View charts') },
    { name: 'System Status', icon: Settings, color: 'orange', action: () => console.log('System status') },
    { name: 'Refresh Data', icon: RefreshCw, color: 'purple', action: () => console.log('Refresh data') },
  ];

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Institutional Dashboard</h1>
          <p className="text-gray-600 dark:text-gray-400">Welcome back, {user?.name || 'Institutional User'}</p>
        </div>
        <div className="flex items-center space-x-4">
          <button
            onClick={toggleTheme}
            className="p-2 rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
            title={`Current theme: ${theme} (${isDark ? 'dark' : 'light'})`}
          >
            {isDark ? <TrendingDown className="w-5 h-5" /> : <TrendingUp className="w-5 h-5" />}
          </button>
          <div className="text-sm text-gray-500 dark:text-gray-400">
            Theme: {theme} ({isDark ? 'dark' : 'light'})
          </div>
        </div>
      </div>

      {/* Dark Theme Test Section */}
      <div className="card">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Dark Theme Test</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg">
            <div className="text-sm text-gray-500 dark:text-gray-400">Light Card</div>
            <div className="text-lg font-semibold text-gray-900 dark:text-white">White Background</div>
          </div>
          <div className="p-4 bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg">
            <div className="text-sm text-gray-500 dark:text-gray-400">Gray Card</div>
            <div className="text-lg font-semibold text-gray-900 dark:text-white">Gray Background</div>
          </div>
          <div className="p-4 bg-blue-50 dark:bg-blue-900 border border-blue-200 dark:border-blue-700 rounded-lg">
            <div className="text-sm text-blue-600 dark:text-blue-400">Blue Card</div>
            <div className="text-lg font-semibold text-blue-900 dark:text-blue-100">Blue Background</div>
          </div>
          <div className="p-4 bg-green-50 dark:bg-green-900 border border-green-200 dark:border-green-700 rounded-lg">
            <div className="text-sm text-green-600 dark:text-green-400">Green Card</div>
            <div className="text-lg font-semibold text-green-900 dark:text-green-100">Green Background</div>
          </div>
        </div>
      </div>

      {/* Metrics Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {widgets.map((widget) => {
          const IconComponent = widget.icon;
          return (
            <div key={widget.id} className="card">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium text-gray-600 dark:text-gray-400">{widget.title}</p>
                  <p className="text-2xl font-bold text-gray-900 dark:text-white">{widget.value}</p>
                </div>
                <div className={`p-3 rounded-full bg-${widget.color}-100 dark:bg-${widget.color}-900`}>
                  <IconComponent className={`w-6 h-6 text-${widget.color}-600 dark:text-${widget.color}-400`} />
                </div>
              </div>
              <div className="mt-4 flex items-center">
                <span className={`text-sm font-medium text-${widget.color}-600 dark:text-${widget.color}-400`}>
                  {widget.change}
                </span>
                <span className="text-sm text-gray-500 dark:text-gray-400 ml-2">vs last month</span>
              </div>
            </div>
          );
        })}
      </div>

      {/* Layout Controls */}
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">Dashboard Layout</h2>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => setLayout('grid')}
            className={`px-3 py-1 rounded-md text-sm font-medium ${
              layout === 'grid'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300'
            }`}
          >
            Grid
          </button>
          <button
            onClick={() => setLayout('list')}
            className={`px-3 py-1 rounded-md text-sm font-medium ${
              layout === 'list'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300'
            }`}
          >
            List
          </button>
        </div>
      </div>

      {/* Recent Activity & Quick Actions */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Recent Activity */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Recent Activity</h3>
          <div className="space-y-4">
            {recentActivity.map((activity) => (
              <div key={activity.id} className="flex items-center space-x-3">
                <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                <div className="flex-1">
                  <p className="text-sm text-gray-900 dark:text-white">{activity.message}</p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">{activity.time}</p>
                </div>
                <div className="text-xs text-gray-500 dark:text-gray-400">
                  {activity.amount || activity.status || activity.price}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Quick Actions */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Quick Actions</h3>
          <div className="grid grid-cols-2 gap-4">
            {quickActions.map((action) => {
              const IconComponent = action.icon;
              return (
                <button
                  key={action.name}
                  onClick={action.action}
                  className="p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                >
                  <IconComponent className={`w-6 h-6 text-${action.color}-600 dark:text-${action.color}-400 mb-2`} />
                  <p className="text-sm font-medium text-gray-900 dark:text-white">{action.name}</p>
                </button>
              );
            })}
          </div>
        </div>
      </div>

      {/* System Status */}
      <div className="card">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">System Status</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="flex items-center space-x-3">
            <div className="w-3 h-3 bg-green-500 rounded-full"></div>
            <span className="text-sm text-gray-900 dark:text-white">Network: Online</span>
          </div>
          <div className="flex items-center space-x-3">
            <div className="w-3 h-3 bg-green-500 rounded-full"></div>
            <span className="text-sm text-gray-900 dark:text-white">Database: Connected</span>
          </div>
          <div className="flex items-center space-x-3">
            <div className="w-3 h-3 bg-green-500 rounded-full"></div>
            <span className="text-sm text-gray-900 dark:text-white">API: Operational</span>
          </div>
        </div>
      </div>
    </div>
  );
}; 