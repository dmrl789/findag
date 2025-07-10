import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  Home, 
  TrendingUp, 
  Network, 
  Users, 
  BarChart3, 
  Activity,
  Settings,
  User,
  FileText,
  Shield,
  Target,
  Globe,
  Zap
} from 'lucide-react';
import { useAuthStore } from '../../store/auth';
import { ThemeToggle } from '../Common/ThemeToggle';

interface NavItem {
  name: string;
  path: string;
  icon: React.ReactNode;
  badge?: string;
  permission?: string;
}

export const Sidebar: React.FC = () => {
  const location = useLocation();
  const { user } = useAuthStore();

  const navigationItems: NavItem[] = [
    { name: 'Dashboard', path: '/', icon: <Home className="w-5 h-5" /> },
    { name: 'Trading', path: '/trading', icon: <TrendingUp className="w-5 h-5" /> },
    { name: 'DAG Explorer', path: '/dag', icon: <Network className="w-5 h-5" /> },
    { name: 'Transactions', path: '/transactions', icon: <Activity className="w-5 h-5" /> },
    { name: 'Validators', path: '/validators', icon: <Users className="w-5 h-5" /> },
    { name: 'Metrics', path: '/metrics', icon: <BarChart3 className="w-5 h-5" /> },
    { name: 'Network', path: '/network', icon: <Globe className="w-5 h-5" /> },
    { name: 'Rounds', path: '/rounds', icon: <Zap className="w-5 h-5" /> },
    { name: 'Charts', path: '/charts', icon: <BarChart3 className="w-5 h-5" /> },
    { name: 'Compliance', path: '/compliance', icon: <Shield className="w-5 h-5" /> },
    { name: 'Compliance Dashboard', path: '/compliance-dashboard', icon: <Target className="w-5 h-5" /> },
    { name: 'Profile', path: '/profile', icon: <User className="w-5 h-5" /> },
    { name: 'Status', path: '/status', icon: <Settings className="w-5 h-5" /> },
  ];

  return (
    <div className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center space-x-3">
          <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
            <span className="text-white font-bold text-sm">F</span>
          </div>
          <div>
            <h1 className="text-lg font-bold text-gray-900 dark:text-white">FinDAG</h1>
            <p className="text-xs text-gray-500 dark:text-gray-400">Institutional Platform</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-4 space-y-2">
        {navigationItems.map((item) => {
          const isActive = location.pathname === item.path;
          return (
            <Link
              key={item.name}
              to={item.path}
              className={`flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                isActive
                  ? 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-200'
                  : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
              }`}
            >
              {item.icon}
              <span>{item.name}</span>
              {item.badge && (
                <span className="ml-auto bg-red-100 text-red-800 text-xs px-2 py-1 rounded-full">
                  {item.badge}
                </span>
              )}
            </Link>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 bg-green-500 rounded-full"></div>
            <span className="text-xs text-gray-500 dark:text-gray-400">System Online</span>
          </div>
          <ThemeToggle />
        </div>
      </div>
    </div>
  );
}; 