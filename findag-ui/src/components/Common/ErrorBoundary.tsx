import React, { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle, RefreshCw, Home, ArrowLeft } from 'lucide-react';
import { errorHandler, ErrorType, ErrorInfo as ErrorInfoType } from '../../utils/errorHandler';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
  resetOnPropsChange?: boolean;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
  errorType: ErrorType;
  userFriendlyMessage: string;
  retryCount: number;
}

export class ErrorBoundary extends Component<Props, State> {
  private maxRetries = 3;

  constructor(props: Props) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
      errorType: ErrorType.UNKNOWN,
      userFriendlyMessage: 'An unexpected error occurred.',
      retryCount: 0
    };
  }

  static getDerivedStateFromError(error: Error): Partial<State> {
    // Parse the error to determine type and user-friendly message
    const parsedError = errorHandler.parseNetworkError(error);
    
    return {
      hasError: true,
      error,
      errorType: parsedError.type,
      userFriendlyMessage: parsedError.userFriendly
    };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    // Log the error
    console.error('Error caught by boundary:', error, errorInfo);

    // Parse error for better handling
    const parsedError = errorHandler.parseNetworkError(error);
    
    this.setState({
      errorInfo,
      errorType: parsedError.type,
      userFriendlyMessage: parsedError.userFriendly
    });

    // Call onError callback if provided
    if (this.props.onError) {
      this.props.onError(error, errorInfo);
    }

    // Log to error tracking service in production
    if ((import.meta as any).env?.PROD) {
      // Example: Sentry.captureException(error, { extra: errorInfo });
    }
  }

  componentDidUpdate(prevProps: Props) {
    // Reset error state when props change (if enabled)
    if (this.props.resetOnPropsChange && prevProps !== this.props) {
      this.resetError();
    }
  }

  resetError = () => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
      errorType: ErrorType.UNKNOWN,
      userFriendlyMessage: 'An unexpected error occurred.',
      retryCount: 0
    });
  };

  retry = () => {
    const { retryCount } = this.state;
    
    if (retryCount < this.maxRetries) {
      this.setState({ retryCount: retryCount + 1 });
      this.resetError();
    } else {
      // Show permanent error after max retries
      this.setState({
        userFriendlyMessage: 'Unable to recover from this error. Please refresh the page or contact support.'
      });
    }
  };

  goHome = () => {
    window.location.href = '/';
  };

  goBack = () => {
    window.history.back();
  };

  getErrorIcon = () => {
    switch (this.state.errorType) {
      case ErrorType.NETWORK:
        return <RefreshCw className="w-8 h-8 text-orange-500" />;
      case ErrorType.AUTHENTICATION:
        return <AlertTriangle className="w-8 h-8 text-red-500" />;
      case ErrorType.AUTHORIZATION:
        return <AlertTriangle className="w-8 h-8 text-red-500" />;
      default:
        return <AlertTriangle className="w-8 h-8 text-red-500" />;
    }
  };

  getErrorColor = () => {
    switch (this.state.errorType) {
      case ErrorType.NETWORK:
        return 'bg-orange-50 border-orange-200 text-orange-800';
      case ErrorType.AUTHENTICATION:
        return 'bg-red-50 border-red-200 text-red-800';
      case ErrorType.AUTHORIZATION:
        return 'bg-red-50 border-red-200 text-red-800';
      default:
        return 'bg-red-50 border-red-200 text-red-800';
    }
  };

  render() {
    if (this.state.hasError) {
      // Custom fallback UI
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <div className="min-h-screen flex items-center justify-center bg-gray-50">
          <div className="max-w-md w-full mx-auto">
            <div className={`rounded-lg border p-6 ${this.getErrorColor()}`}>
              <div className="flex items-center justify-center mb-4">
                {this.getErrorIcon()}
              </div>
              
              <div className="text-center">
                <h2 className="text-lg font-semibold mb-2">
                  {this.state.errorType === ErrorType.NETWORK ? 'Connection Error' : 'Something went wrong'}
                </h2>
                
                <p className="text-sm mb-6">
                  {this.state.userFriendlyMessage}
                </p>

                {this.state.retryCount < this.maxRetries && (
                  <div className="text-xs text-gray-600 mb-4">
                    Retry attempt {this.state.retryCount + 1} of {this.maxRetries}
                  </div>
                )}

                <div className="flex flex-col space-y-2">
                  {this.state.retryCount < this.maxRetries && (
                    <button
                      onClick={this.retry}
                      className="btn-primary flex items-center justify-center space-x-2"
                    >
                      <RefreshCw className="w-4 h-4" />
                      <span>Try Again</span>
                    </button>
                  )}

                  <button
                    onClick={this.goBack}
                    className="btn-secondary flex items-center justify-center space-x-2"
                  >
                    <ArrowLeft className="w-4 h-4" />
                    <span>Go Back</span>
                  </button>

                  <button
                    onClick={this.goHome}
                    className="btn-secondary flex items-center justify-center space-x-2"
                  >
                    <Home className="w-4 h-4" />
                    <span>Go Home</span>
                  </button>
                </div>

                {(import.meta as any).env?.DEV && this.state.error && (
                  <details className="mt-4 text-left">
                    <summary className="cursor-pointer text-xs font-medium">
                      Show Error Details (Development)
                    </summary>
                    <pre className="mt-2 text-xs bg-gray-100 p-2 rounded overflow-auto">
                      {this.state.error.toString()}
                      {this.state.errorInfo && `\n\n${this.state.errorInfo.componentStack}`}
                    </pre>
                  </details>
                )}
              </div>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}

// Higher-order component for error boundary
export const withErrorBoundary = <P extends object>(
  Component: React.ComponentType<P>,
  fallback?: ReactNode
) => {
  return class WithErrorBoundary extends React.Component<P> {
    render() {
      return (
        <ErrorBoundary fallback={fallback}>
          <Component {...this.props} />
        </ErrorBoundary>
      );
    }
  };
};

// Hook for error handling in functional components
export const useErrorHandler = () => {
  const handleError = (error: Error, context?: string) => {
    const parsedError = errorHandler.parseNetworkError(error);
    
    console.error(`Error in ${context || 'component'}:`, error);
    
    // You could dispatch to a global error state here
    // Example: dispatch({ type: 'SET_ERROR', payload: parsedError });
    
    return parsedError;
  };

  const handleAsyncError = async <T,>(
    asyncFn: () => Promise<T>,
    context?: string
  ): Promise<T | null> => {
    try {
      return await asyncFn();
    } catch (error) {
      handleError(error as Error, context);
      return null;
    }
  };

  return { handleError, handleAsyncError };
}; 