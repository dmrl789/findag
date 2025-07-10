import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { useTheme } from '../../contexts/ThemeContext';
import { useAuth } from '../../contexts/AuthContext';

interface NavItem {
  name: string;
  path: string;
  icon: string;
  description: string;
}

const navItems: NavItem[] = [
  {
    name: 'Dashboard',
    path: '/dashboard',
    icon: '📊',
    description: 'Overview and metrics',
  },
  {
    name: 'Trading',
    path: '/trading',
    icon: '📈',
    description: 'Trading interface',
  },
  {
    name: 'DAG Explorer',
    path: '/dag',
    icon: '🔗',
    description: 'BlockDAG visualization',
  },
  {
    name: 'Wallet',
    path: '/wallet',
    icon: '💰',
    description: 'Wallet management',
  },
  {
    name: 'Network',
    path: '/network',
    icon: '🌐',
    description: 'Network status',
  },
  {
    name: 'Validators',
    path: '/validators',
    icon: '👥',
    description: 'Validator management',
  },
  {
    name: 'Settings',
    path: '/settings',
    icon: '⚙️',
    description: 'Application settings',
  },
  {
    name: 'Logs',
    path: '/logs',
    icon: '📝',
    description: 'System logs',
  },
];

const Sidebar: React.FC = () => {
  const location = useLocation();
  const { theme, toggleTheme } = useTheme();
  const { user, logout } = useAuth();

  return (
    <div className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
      {/* Header */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center space-x-3">
          <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
            <span className="text-white font-bold text-sm">FD</span>
          </div>
          <div>
            <h1 className="text-lg font-bold text-gray-900 dark:text-white">FinDAG</h1>
            <p className="text-xs text-gray-500 dark:text-gray-400">Trading Platform</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-4 space-y-2">
        {navItems.map((item) => {
          const isActive = location.pathname === item.path;
          return (
            <Link
              key={item.path}
              to={item.path}
              className={`flex items-center space-x-3 px-3 py-2 rounded-lg transition-colors duration-200 ${
                isActive
                  ? 'bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300'
                  : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
              }`}
            >
              <span className="text-lg">{item.icon}</span>
              <div className="flex-1">
                <div className="font-medium">{item.name}</div>
                <div className="text-xs text-gray-500 dark:text-gray-400">
                  {item.description}
                </div>
              </div>
            </Link>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-gray-200 dark:border-gray-700 space-y-3">
        {/* Theme Toggle */}
        <button
          onClick={toggleTheme}
          className="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200"
        >
          <span className="text-lg">
            {theme === 'dark' ? '🌙' : '☀️'}
          </span>
          <span className="font-medium">
            {theme === 'dark' ? 'Dark Mode' : 'Light Mode'}
          </span>
        </button>

        {/* User Info */}
        {user && (
          <div className="px-3 py-2 rounded-lg bg-gray-50 dark:bg-gray-700">
            <div className="flex items-center space-x-2">
              <div className="w-8 h-8 bg-blue-600 rounded-full flex items-center justify-center">
                <span className="text-white text-sm font-medium">
                  {user.username.charAt(0).toUpperCase()}
                </span>
              </div>
              <div className="flex-1 min-w-0">
                <div className="text-sm font-medium text-gray-900 dark:text-white truncate">
                  {user.username}
                </div>
                <div className="text-xs text-gray-500 dark:text-gray-400 truncate">
                  {user.handle || user.email}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Logout */}
        {user && (
          <button
            onClick={logout}
            className="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors duration-200"
          >
            <span className="text-lg">🚪</span>
            <span className="font-medium">Logout</span>
          </button>
        )}
      </div>
    </div>
  );
};

export default Sidebar; 