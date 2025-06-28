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

const navigationItems = [
  {
    name: 'Dashboard',
    href: '/',
    icon: Activity,
    description: 'Network overview and metrics',
  },
  {
    name: 'Trading',
    href: '/trading',
    icon: TrendingUp,
    description: 'Real-time price charts and trading',
  },
  {
    name: 'DAG Explorer',
    href: '/dag',
    icon: Blocks,
    description: 'Visual DAG structure',
  },
  {
    name: 'Transactions',
    href: '/transactions',
    icon: Zap,
    description: 'Transaction history and details',
  },
  {
    name: 'Validators',
    href: '/validators',
    icon: Users,
    description: 'Validator management and status',
  },
  {
    name: 'Rounds',
    href: '/rounds',
    icon: Clock,
    description: 'Round finalization and history',
  },
  {
    name: 'Network',
    href: '/network',
    icon: Network,
    description: 'P2P network topology',
  },
  {
    name: 'Metrics',
    href: '/metrics',
    icon: BarChart3,
    description: 'Performance analytics',
  },
  {
    name: 'Storage',
    href: '/storage',
    icon: Database,
    description: 'Blockchain storage status',
  },
  {
    name: 'Security',
    href: '/security',
    icon: Shield,
    description: 'Security and compliance',
  },
  {
    name: 'Settings',
    href: '/settings',
    icon: Settings,
    description: 'Application configuration',
  },
];

export const Sidebar: React.FC = () => {
  const location = useLocation();
  const { connectionStatus } = useAppStore();

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

      {/* Navigation */}
      <nav className="flex-1 p-4 space-y-1 overflow-y-auto">
        {navigationItems.map((item) => {
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
        </div>
      </div>
    </div>
  );
}; 