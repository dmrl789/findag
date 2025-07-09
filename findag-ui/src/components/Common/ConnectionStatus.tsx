import React, { useState, useEffect } from 'react';
import { Wifi, WifiOff, AlertTriangle, RefreshCw, CheckCircle } from 'lucide-react';
import { finDAGApi } from '../../services/api';

interface ConnectionStatusProps {
  showDetails?: boolean;
  className?: string;
}

export const ConnectionStatus: React.FC<ConnectionStatusProps> = ({ 
  showDetails = false, 
  className = '' 
}) => {
  const [status, setStatus] = useState<'connected' | 'disconnected' | 'connecting' | 'error' | 'failed'>('disconnected');
  const [lastError, setLastError] = useState<string | null>(null);
  const [reconnectAttempts, setReconnectAttempts] = useState<number>(0);
  const [isReconnecting, setIsReconnecting] = useState(false);

  useEffect(() => {
    const handleConnectionStatus = (event: any) => {
      const { status: newStatus, error, attempts } = event.data;
      setStatus(newStatus);
      setLastError(error || null);
      if (attempts !== undefined) {
        setReconnectAttempts(attempts);
      }
    };

    const handleConnect = () => {
      setStatus('connected');
      setLastError(null);
      setReconnectAttempts(0);
      setIsReconnecting(false);
    };

    const handleDisconnect = () => {
      setStatus('disconnected');
      setIsReconnecting(false);
    };

    const handleConnecting = () => {
      setStatus('connecting');
      setIsReconnecting(true);
    };

    // Listen for connection status events
    finDAGApi.addEventListener('connection_status', handleConnectionStatus);
    
    // Listen for socket events
    finDAGApi.addEventListener('connect', handleConnect);
    finDAGApi.addEventListener('disconnect', handleDisconnect);
    finDAGApi.addEventListener('connecting', handleConnecting);

    // Set initial status
    setStatus(finDAGApi.getConnectionStatus());

    return () => {
      finDAGApi.removeEventListener('connection_status', handleConnectionStatus);
      finDAGApi.removeEventListener('connect', handleConnect);
      finDAGApi.removeEventListener('disconnect', handleDisconnect);
      finDAGApi.removeEventListener('connecting', handleConnecting);
    };
  }, []);

  const handleReconnect = () => {
    setIsReconnecting(true);
    finDAGApi.enableReconnections();
    finDAGApi.connectWebSocket();
  };

  const getStatusColor = () => {
    switch (status) {
      case 'connected':
        return 'text-success-600 bg-success-50 border-success-200';
      case 'connecting':
        return 'text-warning-600 bg-warning-50 border-warning-200';
      case 'error':
      case 'failed':
        return 'text-danger-600 bg-danger-50 border-danger-200';
      default:
        return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  const getStatusIcon = () => {
    switch (status) {
      case 'connected':
        return <CheckCircle className="w-4 h-4" />;
      case 'connecting':
        return <RefreshCw className="w-4 h-4 animate-spin" />;
      case 'error':
      case 'failed':
        return <AlertTriangle className="w-4 h-4" />;
      default:
        return <WifiOff className="w-4 h-4" />;
    }
  };

  const getStatusText = () => {
    switch (status) {
      case 'connected':
        return 'Connected';
      case 'connecting':
        return 'Connecting...';
      case 'error':
        return 'Connection Error';
      case 'failed':
        return 'Connection Failed';
      default:
        return 'Disconnected';
    }
  };

  return (
    <div className={`flex items-center space-x-2 ${className}`}>
      <div className={`flex items-center space-x-2 px-3 py-1 rounded-full border text-sm font-medium ${getStatusColor()}`}>
        {getStatusIcon()}
        <span>{getStatusText()}</span>
      </div>

      {showDetails && (
        <div className="text-xs text-gray-500">
          {status === 'failed' && reconnectAttempts > 0 && (
            <span>Attempts: {reconnectAttempts}/10</span>
          )}
          {lastError && (
            <div className="text-danger-600 mt-1">{lastError}</div>
          )}
        </div>
      )}

      {(status === 'disconnected' || status === 'error' || status === 'failed') && (
        <button
          onClick={handleReconnect}
          disabled={isReconnecting}
          className="inline-flex items-center px-2 py-1 text-xs font-medium text-primary-600 bg-primary-50 border border-primary-200 rounded hover:bg-primary-100 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isReconnecting ? (
            <>
              <RefreshCw className="w-3 h-3 mr-1 animate-spin" />
              Reconnecting...
            </>
          ) : (
            <>
              <Wifi className="w-3 h-3 mr-1" />
              Reconnect
            </>
          )}
        </button>
      )}
    </div>
  );
};

export const ConnectionIndicator: React.FC = () => {
  const [status, setStatus] = useState<'connected' | 'disconnected' | 'connecting'>('disconnected');

  useEffect(() => {
    const handleConnectionStatus = (event: any) => {
      setStatus(event.data.status);
    };

    finDAGApi.addEventListener('connection_status', handleConnectionStatus);
    setStatus(finDAGApi.getConnectionStatus());

    return () => {
      finDAGApi.removeEventListener('connection_status', handleConnectionStatus);
    };
  }, []);

  const getStatusColor = () => {
    switch (status) {
      case 'connected':
        return 'bg-success-500';
      case 'connecting':
        return 'bg-warning-500';
      default:
        return 'bg-danger-500';
    }
  };

  return (
    <div className="flex items-center space-x-2">
      <div className={`w-2 h-2 rounded-full ${getStatusColor()}`}></div>
      <span className="text-sm text-gray-600 capitalize">{status}</span>
    </div>
  );
}; 