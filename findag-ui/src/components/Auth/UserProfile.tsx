import React, { useState } from 'react';
import { 
  User, 
  LogOut, 
  Settings, 
  Shield, 
  Clock, 
  Key,
  ChevronDown,
  ChevronUp
} from 'lucide-react';
import { useAuthStore } from '../../store/auth';
import { formatTimestamp } from '../../utils/formatters';

interface UserProfileProps {
  className?: string;
}

export const UserProfile: React.FC<UserProfileProps> = ({ className = '' }) => {
  const [isOpen, setIsOpen] = useState(false);
  const { user, logout, logoutLoading, sessionTimeout, lastActivity } = useAuthStore();

  if (!user) {
    return null;
  }

  const handleLogout = async () => {
    try {
      await logout();
    } catch (error) {
      console.error('Logout failed:', error);
    }
  };

  const getRoleColor = (role: string) => {
    switch (role) {
      case 'admin':
        return 'text-danger-600 bg-danger-50 border-danger-200';
      case 'validator':
        return 'text-warning-600 bg-warning-50 border-warning-200';
      default:
        return 'text-primary-600 bg-primary-50 border-primary-200';
    }
  };

  const getRoleIcon = (role: string) => {
    switch (role) {
      case 'admin':
        return <Shield className="w-4 h-4" />;
      case 'validator':
        return <Key className="w-4 h-4" />;
      default:
        return <User className="w-4 h-4" />;
    }
  };

  const formatSessionTime = (timestamp: number) => {
    const now = Date.now();
    const diff = now - timestamp;
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(minutes / 60);
    
    if (hours > 0) {
      return `${hours}h ${minutes % 60}m ago`;
    }
    return `${minutes}m ago`;
  };

  const getSessionStatus = () => {
    if (!sessionTimeout) return 'Unknown';
    
    const now = Date.now();
    const timeLeft = sessionTimeout - now;
    
    if (timeLeft <= 0) {
      return 'Expired';
    }
    
    const minutes = Math.floor(timeLeft / 60000);
    const hours = Math.floor(minutes / 60);
    
    if (hours > 0) {
      return `${hours}h ${minutes % 60}m remaining`;
    }
    return `${minutes}m remaining`;
  };

  return (
    <div className={`relative ${className}`}>
      {/* User Button */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center space-x-3 p-2 rounded-lg hover:bg-gray-100 transition-colors"
      >
        <div className="w-8 h-8 bg-primary-600 rounded-full flex items-center justify-center">
          <span className="text-white font-medium text-sm">
            {user.username.charAt(0).toUpperCase()}
          </span>
        </div>
        <div className="text-left">
          <p className="text-sm font-medium text-gray-900">{user.username}</p>
          <p className="text-xs text-gray-500 capitalize">{user.role}</p>
        </div>
        {isOpen ? (
          <ChevronUp className="w-4 h-4 text-gray-400" />
        ) : (
          <ChevronDown className="w-4 h-4 text-gray-400" />
        )}
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <div className="absolute right-0 mt-2 w-80 bg-white rounded-lg shadow-lg border border-gray-200 z-50">
          <div className="p-4">
            {/* User Info */}
            <div className="flex items-center space-x-3 mb-4">
              <div className="w-12 h-12 bg-primary-600 rounded-full flex items-center justify-center">
                <span className="text-white font-medium text-lg">
                  {user.username.charAt(0).toUpperCase()}
                </span>
              </div>
              <div className="flex-1">
                <h3 className="text-sm font-medium text-gray-900">{user.username}</h3>
                <p className="text-xs text-gray-500">{user.email}</p>
                <div className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border ${getRoleColor(user.role)} mt-1`}>
                  {getRoleIcon(user.role)}
                  <span className="ml-1 capitalize">{user.role}</span>
                </div>
              </div>
            </div>

            {/* Session Info */}
            <div className="space-y-3 mb-4">
              <div className="flex items-center justify-between text-sm">
                <span className="text-gray-500">Last Activity:</span>
                <span className="text-gray-900">{formatSessionTime(lastActivity)}</span>
              </div>
              <div className="flex items-center justify-between text-sm">
                <span className="text-gray-500">Session Status:</span>
                <span className="text-gray-900">{getSessionStatus()}</span>
              </div>
              <div className="flex items-center justify-between text-sm">
                <span className="text-gray-500">Last Login:</span>
                <span className="text-gray-900">{formatTimestamp(user.lastLogin)}</span>
              </div>
            </div>

            {/* Permissions */}
            <div className="mb-4">
              <h4 className="text-xs font-medium text-gray-700 mb-2">Permissions</h4>
              <div className="flex flex-wrap gap-1">
                {user.permissions.slice(0, 6).map((permission) => (
                  <span
                    key={permission}
                    className="inline-flex items-center px-2 py-1 rounded text-xs bg-gray-100 text-gray-700"
                  >
                    {permission}
                  </span>
                ))}
                {user.permissions.length > 6 && (
                  <span className="inline-flex items-center px-2 py-1 rounded text-xs bg-gray-100 text-gray-700">
                    +{user.permissions.length - 6} more
                  </span>
                )}
              </div>
            </div>

            {/* Actions */}
            <div className="space-y-2">
              <button
                onClick={() => {/* TODO: Navigate to settings */}}
                className="w-full flex items-center px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded-md transition-colors"
              >
                <Settings className="w-4 h-4 mr-2" />
                Settings
              </button>
              <button
                onClick={handleLogout}
                disabled={logoutLoading}
                className="w-full flex items-center px-3 py-2 text-sm text-danger-600 hover:bg-danger-50 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <LogOut className="w-4 h-4 mr-2" />
                {logoutLoading ? 'Signing out...' : 'Sign out'}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Backdrop */}
      {isOpen && (
        <div
          className="fixed inset-0 z-40"
          onClick={() => setIsOpen(false)}
        />
      )}
    </div>
  );
}; 