import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  Activity, 
  Blocks, 
  Users, 
  Clock, 
  BarChart3, 
  Settings, 
  Zap,
  Network,
  Database,
  Shield,
  TrendingUp
} from 'lucide-react';
import { useAppStore } from '../../store';
import { useAuthStore } from '../../store/auth';

const navigationItems = [
  {
    name: 'Dashboard',
    href: '/',
    icon: Activity,
    description: 'Network overview and metrics',
    permission: 'dashboard:read',
    roles: ['admin', 'user', 'validator'],
  },
  {
    name: 'Trading',
    href: '/trading',
    icon: TrendingUp,
    description: 'Real-time price charts and trading',
    permission: 'trading:read',
    roles: ['admin', 'user', 'validator'],
  },
  {
    name: 'DAG Explorer',
    href: '/dag',
    icon: Blocks,
    description: 'Visual DAG structure',
    permission: 'network:read',
    roles: ['admin', 'user', 'validator'],
  },
  {
    name: 'Transactions',
    href: '/transactions',
    icon: Zap,
    description: 'Transaction history and details',
    permission: 'transactions:read',
    roles: ['admin', 'user', 'validator'],
  },
  {
    name: 'Validators',
    href: '/validators',
    icon: Users,
    description: 'Validator management and status',
    permission: 'validators:read',
    roles: ['admin', 'validator'],
  },
  {
    name: 'Rounds',
    href: '/rounds',
    icon: Clock,
    description: 'Round finalization and history',
    permission: 'rounds:read',
    roles: ['admin', 'validator'],
  },
  {
    name: 'Network',
    href: '/network',
    icon: Network,
    description: 'P2P network topology',
    permission: 'network:read',
    roles: ['admin', 'user', 'validator'],
  },
  {
    name: 'Metrics',
    href: '/metrics',
    icon: BarChart3,
    description: 'Performance analytics',
    permission: 'metrics:read',
    roles: ['admin', 'user', 'validator'],
  },
  {
    name: 'Storage',
    href: '/storage',
    icon: Database,
    description: 'Blockchain storage status',
    permission: 'system:read',
    roles: ['admin'],
  },
  {
    name: 'Security',
    href: '/security',
    icon: Shield,
    description: 'Security and compliance',
    permission: 'system:read',
    roles: ['admin'],
  },
  {
    name: 'Settings',
    href: '/settings',
    icon: Settings,
    description: 'Application configuration',
    permission: 'system:read',
    roles: ['admin'],
  },
];

export const Sidebar: React.FC = () => {
  const location = useLocation();
  const { connectionStatus } = useAppStore();
  const { user, hasPermission, hasRole } = useAuthStore();

  const getStatusColor = () => {
    switch (connectionStatus) {
      case 'connected':
        return 'bg-success-500';
      case 'connecting':
        return 'bg-warning-500';
      case 'disconnected':
        return 'bg-danger-500';
      default:
        return 'bg-gray-500';
    }
  };

  const getStatusText = () => {
    switch (connectionStatus) {
      case 'connected':
        return 'Connected';
      case 'connecting':
        return 'Connecting...';
      case 'disconnected':
        return 'Disconnected';
      default:
        return 'Unknown';
    }
  };

  // Filter navigation items based on user permissions
  const filteredNavigationItems = navigationItems.filter(item => {
    // Check if user has the required permission
    if (item.permission && !hasPermission(item.permission)) {
      return false;
    }
    
    // Check if user has one of the required roles
    if (item.roles && !item.roles.some(role => hasRole(role))) {
      return false;
    }
    
    return true;
  });

  return (
    <div className="flex flex-col w-64 bg-white border-r border-gray-200 h-full">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-gray-200">
        <div className="flex items-center space-x-3">
          <div className="w-8 h-8 bg-primary-600 rounded-lg flex items-center justify-center">
            <span className="text-white font-bold text-sm">FD</span>
          </div>
          <div>
            <h1 className="text-lg font-semibold text-gray-900">FinDAG</h1>
            <p className="text-xs text-gray-500">Financial Blockchain</p>
          </div>
        </div>
      </div>

      {/* Connection Status */}
      <div className="p-4 border-b border-gray-200">
        <div className="flex items-center space-x-2">
          <div className={`w-2 h-2 rounded-full ${getStatusColor()}`} />
          <span className="text-sm font-medium text-gray-700">{getStatusText()}</span>
        </div>
      </div>

      {/* User Info */}
      {user && (
        <div className="p-4 border-b border-gray-200">
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 bg-primary-600 rounded-full flex items-center justify-center">
              <span className="text-white font-medium text-sm">
                {user.username.charAt(0).toUpperCase()}
              </span>
            </div>
            <div className="flex-1 min-w-0">
              <p className="text-sm font-medium text-gray-900 truncate">
                {user.username}
              </p>
              <p className="text-xs text-gray-500 capitalize">
                {user.role}
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Navigation */}
      <nav className="flex-1 p-4 space-y-1 overflow-y-auto">
        {filteredNavigationItems.map((item) => {
          const isActive = location.pathname === item.href || 
                          (item.href === '/trading' && location.pathname.startsWith('/trading'));
          const Icon = item.icon;
          
          return (
            <Link
              key={item.name}
              to={item.href}
              className={`group flex items-center px-3 py-2 text-sm font-medium rounded-lg transition-colors duration-200 ${
                isActive
                  ? 'bg-primary-50 text-primary-700 border border-primary-200'
                  : 'text-gray-700 hover:bg-gray-50 hover:text-gray-900'
              }`}
              title={item.description}
            >
              <Icon
                className={`mr-3 h-5 w-5 flex-shrink-0 ${
                  isActive ? 'text-primary-600' : 'text-gray-400 group-hover:text-gray-500'
                }`}
              />
              {item.name}
            </Link>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-gray-200">
        <div className="text-xs text-gray-500">
          <p>FinDAG v1.0.0</p>
          <p>High-performance blockchain</p>
          {user && (
            <p className="mt-1 text-xs text-gray-400">
              Logged in as {user.role}
            </p>
          )}
        </div>
      </div>
    </div>
  );
}; 