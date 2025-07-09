import React, { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle, RefreshCw, Home } from 'lucide-react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
  errorInfo?: ErrorInfo;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('ErrorBoundary caught an error:', error, errorInfo);
    this.setState({ error, errorInfo });
    
    // Log error to external service (e.g., Sentry)
    // logErrorToService(error, errorInfo);
  }

  private handleRetry = () => {
    this.setState({ hasError: false, error: undefined, errorInfo: undefined });
  };

  private handleGoHome = () => {
    window.location.href = '/';
  };

  public render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
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
                    onClick={this.handleRetry}
                    className="flex-1 flex items-center justify-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                  >
                    <RefreshCw className="w-4 h-4 mr-2" />
                    Try Again
                  </button>
                  <button
                    onClick={this.handleGoHome}
                    className="flex-1 flex items-center justify-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                  >
                    <Home className="w-4 h-4 mr-2" />
                    Go Home
                  </button>
                </div>
              </div>
            </div>

            <div className="text-center">
              <p className="text-xs text-gray-500">
                If this problem persists, please contact support with the error details above.
              </p>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
} 