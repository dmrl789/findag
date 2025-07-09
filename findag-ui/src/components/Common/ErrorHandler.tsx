import React, { useState, useEffect } from 'react';
import { AlertTriangle, RefreshCw, X, Info, CheckCircle } from 'lucide-react';

export interface ErrorInfo {
  id: string;
  type: 'error' | 'warning' | 'info' | 'success';
  title: string;
  message: string;
  details?: string;
  retryAction?: () => Promise<void>;
  dismissible?: boolean;
  autoDismiss?: number; // milliseconds
  timestamp: number;
}

interface ErrorHandlerProps {
  error: ErrorInfo | null;
  onDismiss: (id: string) => void;
  onRetry?: (id: string) => void;
}

export const ErrorHandler: React.FC<ErrorHandlerProps> = ({ 
  error, 
  onDismiss, 
  onRetry 
}) => {
  const [isRetrying, setIsRetrying] = useState(false);

  useEffect(() => {
    if (error?.autoDismiss) {
      const timer = setTimeout(() => {
        onDismiss(error.id);
      }, error.autoDismiss);
      
      return () => clearTimeout(timer);
    }
  }, [error, onDismiss]);

  if (!error) return null;

  const handleRetry = async () => {
    if (!error.retryAction) return;
    
    setIsRetrying(true);
    try {
      await error.retryAction();
      onDismiss(error.id);
    } catch (retryError) {
      console.error('Retry failed:', retryError);
    } finally {
      setIsRetrying(false);
    }
  };

  const getIcon = () => {
    switch (error.type) {
      case 'error':
        return <AlertTriangle className="w-5 h-5 text-danger-400" />;
      case 'warning':
        return <AlertTriangle className="w-5 h-5 text-warning-400" />;
      case 'info':
        return <Info className="w-5 h-5 text-primary-400" />;
      case 'success':
        return <CheckCircle className="w-5 h-5 text-success-400" />;
      default:
        return <AlertTriangle className="w-5 h-5 text-gray-400" />;
    }
  };

  const getBackgroundColor = () => {
    switch (error.type) {
      case 'error':
        return 'bg-danger-50 border-danger-200';
      case 'warning':
        return 'bg-warning-50 border-warning-200';
      case 'info':
        return 'bg-primary-50 border-primary-200';
      case 'success':
        return 'bg-success-50 border-success-200';
      default:
        return 'bg-gray-50 border-gray-200';
    }
  };

  const getTextColor = () => {
    switch (error.type) {
      case 'error':
        return 'text-danger-800';
      case 'warning':
        return 'text-warning-800';
      case 'info':
        return 'text-primary-800';
      case 'success':
        return 'text-success-800';
      default:
        return 'text-gray-800';
    }
  };

  return (
    <div className={`fixed top-4 right-4 z-50 max-w-md w-full`}>
      <div className={`rounded-lg border p-4 shadow-lg ${getBackgroundColor()}`}>
        <div className="flex items-start">
          <div className="flex-shrink-0">
            {getIcon()}
          </div>
          <div className="ml-3 flex-1">
            <h3 className={`text-sm font-medium ${getTextColor()}`}>
              {error.title}
            </h3>
            <div className="mt-1">
              <p className={`text-sm ${getTextColor()}`}>
                {error.message}
              </p>
              {error.details && (
                <details className="mt-2">
                  <summary className="text-xs text-gray-500 cursor-pointer hover:text-gray-700">
                    Show details
                  </summary>
                  <pre className="mt-1 text-xs text-gray-600 whitespace-pre-wrap bg-white bg-opacity-50 p-2 rounded">
                    {error.details}
                  </pre>
                </details>
              )}
            </div>
            <div className="mt-3 flex space-x-2">
              {error.retryAction && (
                <button
                  onClick={handleRetry}
                  disabled={isRetrying}
                  className="inline-flex items-center px-2 py-1 text-xs font-medium text-gray-700 bg-white border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isRetrying ? (
                    <>
                      <RefreshCw className="w-3 h-3 mr-1 animate-spin" />
                      Retrying...
                    </>
                  ) : (
                    <>
                      <RefreshCw className="w-3 h-3 mr-1" />
                      Retry
                    </>
                  )}
                </button>
              )}
              {error.dismissible && (
                <button
                  onClick={() => onDismiss(error.id)}
                  className="inline-flex items-center px-2 py-1 text-xs font-medium text-gray-700 bg-white border border-gray-300 rounded hover:bg-gray-50"
                >
                  Dismiss
                </button>
              )}
            </div>
          </div>
          {error.dismissible && (
            <div className="ml-4 flex-shrink-0">
              <button
                onClick={() => onDismiss(error.id)}
                className="inline-flex text-gray-400 hover:text-gray-600"
              >
                <X className="w-4 h-4" />
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

// Global error handler hook
export const useErrorHandler = () => {
  const [errors, setErrors] = useState<ErrorInfo[]>([]);

  const addError = (error: Omit<ErrorInfo, 'id' | 'timestamp'>) => {
    const newError: ErrorInfo = {
      ...error,
      id: `error-${Date.now()}-${Math.random()}`,
      timestamp: Date.now(),
    };
    setErrors(prev => [...prev, newError]);
  };

  const dismissError = (id: string) => {
    setErrors(prev => prev.filter(error => error.id !== id));
  };

  const clearErrors = () => {
    setErrors([]);
  };

  const addApiError = (error: any, context: string, retryAction?: () => Promise<void>) => {
    const message = error?.message || error?.response?.data?.message || 'An unexpected error occurred';
    const details = error?.response?.data?.details || error?.stack;
    
    addError({
      type: 'error',
      title: `${context} Failed`,
      message,
      details,
      retryAction,
      dismissible: true,
    });
  };

  const addNetworkError = (retryAction?: () => Promise<void>) => {
    addError({
      type: 'error',
      title: 'Network Error',
      message: 'Unable to connect to the server. Please check your internet connection.',
      retryAction,
      dismissible: true,
    });
  };

  const addValidationError = (field: string, message: string) => {
    addError({
      type: 'warning',
      title: 'Validation Error',
      message: `${field}: ${message}`,
      dismissible: true,
      autoDismiss: 5000,
    });
  };

  const addSuccessMessage = (title: string, message: string) => {
    addError({
      type: 'success',
      title,
      message,
      dismissible: true,
      autoDismiss: 3000,
    });
  };

  const addInfoMessage = (title: string, message: string) => {
    addError({
      type: 'info',
      title,
      message,
      dismissible: true,
      autoDismiss: 5000,
    });
  };

  return {
    errors,
    addError,
    dismissError,
    clearErrors,
    addApiError,
    addNetworkError,
    addValidationError,
    addSuccessMessage,
    addInfoMessage,
  };
};

// Global error boundary component
export const GlobalErrorBoundary: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { addError } = useErrorHandler();

  const handleError = (error: Error, errorInfo: React.ErrorInfo) => {
    console.error('Global error caught:', error, errorInfo);
    
    addError({
      type: 'error',
      title: 'Application Error',
      message: 'An unexpected error occurred. Please refresh the page or contact support if the problem persists.',
      details: `${error.message}\n\nStack trace:\n${error.stack}\n\nComponent stack:\n${errorInfo.componentStack}`,
      dismissible: true,
    });
  };

  return (
    <ErrorBoundary onError={handleError}>
      {children}
    </ErrorBoundary>
  );
};

// Enhanced ErrorBoundary component
interface ErrorBoundaryProps {
  children: React.ReactNode;
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void;
  fallback?: React.ComponentType<{ error: Error; resetError: () => void }>;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

class ErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    this.props.onError?.(error, errorInfo);
  }

  resetError = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        const FallbackComponent = this.props.fallback;
        return <FallbackComponent error={this.state.error!} resetError={this.resetError} />;
      }

      return (
        <div className="min-h-screen bg-gray-50 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
          <div className="max-w-md w-full space-y-8">
            <div className="text-center">
              <div className="mx-auto h-12 w-12 bg-danger-100 rounded-lg flex items-center justify-center">
                <AlertTriangle className="h-6 w-6 text-danger-600" />
              </div>
              <h2 className="mt-6 text-3xl font-extrabold text-gray-900">
                Something went wrong
              </h2>
              <p className="mt-2 text-sm text-gray-600">
                An unexpected error occurred. Please try again or contact support if the problem persists.
              </p>
            </div>

            <div className="bg-white shadow rounded-lg p-6">
              <div className="space-y-4">
                {this.state.error && (
                  <div>
                    <h3 className="text-sm font-medium text-gray-900 mb-2">
                      Error Details
                    </h3>
                    <div className="bg-gray-50 rounded p-3">
                      <p className="text-sm text-gray-700 font-mono">
                        {this.state.error.message}
                      </p>
                      {this.state.error.stack && (
                        <details className="mt-2">
                          <summary className="text-xs text-gray-500 cursor-pointer">
                            Show stack trace
                          </summary>
                          <pre className="text-xs text-gray-600 mt-2 whitespace-pre-wrap">
                            {this.state.error.stack}
                          </pre>
                        </details>
                      )}
                    </div>
                  </div>
                )}

                <div className="flex space-x-3">
                  <button
                    onClick={this.resetError}
                    className="flex-1 flex items-center justify-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                  >
                    <RefreshCw className="w-4 h-4 mr-2" />
                    Try Again
                  </button>
                  <button
                    onClick={() => window.location.reload()}
                    className="flex-1 flex items-center justify-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                  >
                    Refresh Page
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
} 