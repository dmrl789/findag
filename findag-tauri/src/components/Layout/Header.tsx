import React from 'react';
import { useNode } from '../../contexts/NodeContext';
import { useAuth } from '../../contexts/AuthContext';

const Header: React.FC = () => {
  const { status, startNode, stopNode } = useNode();
  const { user } = useAuth();

  const getStatusColor = () => {
    if (status.isRunning) return 'bg-green-500';
    if (status.isConnected) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  const getStatusText = () => {
    if (status.isRunning) return 'Running';
    if (status.isConnected) return 'Connected';
    return 'Stopped';
  };

  return (
    <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4">
      <div className="flex items-center justify-between">
        {/* Left side - Status and controls */}
        <div className="flex items-center space-x-6">
          {/* Node Status */}
          <div className="flex items-center space-x-3">
            <div className={`w-3 h-3 rounded-full ${getStatusColor()}`}></div>
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
              Node: {getStatusText()}
            </span>
          </div>

          {/* Node Controls */}
          <div className="flex items-center space-x-2">
            <button
              onClick={startNode}
              disabled={status.isRunning}
              className="px-3 py-1 text-xs font-medium bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
            >
              Start
            </button>
            <button
              onClick={stopNode}
              disabled={!status.isRunning}
              className="px-3 py-1 text-xs font-medium bg-red-600 text-white rounded-md hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
            >
              Stop
            </button>
          </div>

          {/* Metrics */}
          {status.isRunning && (
            <div className="flex items-center space-x-4 text-sm text-gray-600 dark:text-gray-400">
              <div className="flex items-center space-x-1">
                <span>TPS:</span>
                <span className="font-mono font-medium">{status.tps}</span>
              </div>
              <div className="flex items-center space-x-1">
                <span>Peers:</span>
                <span className="font-mono font-medium">{status.peers}</span>
              </div>
              <div className="flex items-center space-x-1">
                <span>Blocks/s:</span>
                <span className="font-mono font-medium">{status.blocksPerSecond}</span>
              </div>
            </div>
          )}
        </div>

        {/* Right side - User and actions */}
        <div className="flex items-center space-x-4">
          {/* Notifications */}
          <button className="relative p-2 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors duration-200">
            <span className="text-lg">🔔</span>
            <span className="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full"></span>
          </button>

          {/* Help */}
          <button className="p-2 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors duration-200">
            <span className="text-lg">❓</span>
          </button>

          {/* User Menu */}
          {user && (
            <div className="flex items-center space-x-2">
              <div className="w-8 h-8 bg-blue-600 rounded-full flex items-center justify-center">
                <span className="text-white text-sm font-medium">
                  {user.username.charAt(0).toUpperCase()}
                </span>
              </div>
              <div className="hidden md:block">
                <div className="text-sm font-medium text-gray-900 dark:text-white">
                  {user.username}
                </div>
                <div className="text-xs text-gray-500 dark:text-gray-400">
                  {user.role}
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </header>
  );
};

export default Header; 