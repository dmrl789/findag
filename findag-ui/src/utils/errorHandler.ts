import { ApiError } from '../services/api';

// Error types
export enum ErrorType {
  NETWORK = 'network',
  AUTHENTICATION = 'authentication',
  AUTHORIZATION = 'authorization',
  VALIDATION = 'validation',
  SERVER = 'server',
  CLIENT = 'client',
  UNKNOWN = 'unknown'
}

export interface ErrorInfo {
  type: ErrorType;
  message: string;
  code?: string;
  status?: number;
  details?: any;
  timestamp: number;
  userFriendly: string;
  shouldRetry: boolean;
  retryCount?: number;
}

// Error message mappings
const ERROR_MESSAGES = {
  network: {
    timeout: 'Request timed out. Please check your connection and try again.',
    offline: 'You are offline. Please check your internet connection.',
    connection: 'Unable to connect to the server. Please try again later.',
    default: 'A network error occurred. Please try again.'
  },
  authentication: {
    invalid_credentials: 'Invalid username or password.',
    token_expired: 'Your session has expired. Please log in again.',
    unauthorized: 'You are not authorized to perform this action.',
    default: 'Authentication failed. Please log in again.'
  },
  authorization: {
    forbidden: 'You do not have permission to access this resource.',
    role_required: 'This action requires specific permissions.',
    default: 'Access denied. Please contact your administrator.'
  },
  validation: {
    invalid_input: 'Please check your input and try again.',
    required_field: 'This field is required.',
    invalid_format: 'Please enter a valid format.',
    default: 'Please correct the errors and try again.'
  },
  server: {
    internal_error: 'An internal server error occurred. Please try again later.',
    service_unavailable: 'The service is temporarily unavailable.',
    maintenance: 'The system is under maintenance. Please try again later.',
    default: 'A server error occurred. Please try again later.'
  },
  client: {
    bad_request: 'Invalid request. Please check your input.',
    not_found: 'The requested resource was not found.',
    conflict: 'This resource already exists.',
    default: 'An error occurred with your request.'
  },
  unknown: {
    default: 'An unexpected error occurred. Please try again.'
  }
};

// Error logging
class ErrorLogger {
  private logs: ErrorInfo[] = [];
  private maxLogs = 100;

  log(error: ErrorInfo): void {
    this.logs.push(error);
    
    // Keep only the last maxLogs entries
    if (this.logs.length > this.maxLogs) {
      this.logs = this.logs.slice(-this.maxLogs);
    }

    // Log to console in development
    if ((import.meta as any).env?.DEV) {
      console.error('Error logged:', error);
    }

    // In production, you could send to an error tracking service
    // this.sendToErrorService(error);
  }

  getLogs(): ErrorInfo[] {
    return [...this.logs];
  }

  clearLogs(): void {
    this.logs = [];
  }

  // Send to external error tracking service (e.g., Sentry)
  private sendToErrorService(error: ErrorInfo): void {
    // Implementation for external error tracking
    // Example: Sentry.captureException(error);
  }
}

export const errorLogger = new ErrorLogger();

// Main error handler
export class ErrorHandler {
  private static instance: ErrorHandler;
  private retryDelays = [1000, 2000, 5000, 10000]; // Exponential backoff

  static getInstance(): ErrorHandler {
    if (!ErrorHandler.instance) {
      ErrorHandler.instance = new ErrorHandler();
    }
    return ErrorHandler.instance;
  }

  // Parse API error and convert to user-friendly format
  parseApiError(error: ApiError): ErrorInfo {
    const status = error.status || 0;
    const code = error.code || 'unknown';
    const message = error.message || 'An error occurred';

    let type: ErrorType;
    let userFriendly: string;
    let shouldRetry = false;

    // Determine error type based on status code
    if (status >= 500) {
      type = ErrorType.SERVER;
      userFriendly = this.getErrorMessage('server', code);
      shouldRetry = true;
    } else if (status === 401) {
      type = ErrorType.AUTHENTICATION;
      userFriendly = this.getErrorMessage('authentication', code);
    } else if (status === 403) {
      type = ErrorType.AUTHORIZATION;
      userFriendly = this.getErrorMessage('authorization', code);
    } else if (status === 400) {
      type = ErrorType.VALIDATION;
      userFriendly = this.getErrorMessage('validation', code);
    } else if (status === 0 || status === 408 || status === 504) {
      type = ErrorType.NETWORK;
      userFriendly = this.getErrorMessage('network', code);
      shouldRetry = true;
    } else {
      type = ErrorType.CLIENT;
      userFriendly = this.getErrorMessage('client', code);
    }

    const errorInfo: ErrorInfo = {
      type,
      message,
      code,
      status,
      details: error.details,
      timestamp: Date.now(),
      userFriendly,
      shouldRetry
    };

    // Log the error
    errorLogger.log(errorInfo);

    return errorInfo;
  }

  // Handle network errors
  parseNetworkError(error: any): ErrorInfo {
    let type: ErrorType;
    let userFriendly: string;
    let shouldRetry = true;

    if (error.name === 'TypeError' && error.message.includes('fetch')) {
      type = ErrorType.NETWORK;
      userFriendly = ERROR_MESSAGES.network.offline;
    } else if (error.name === 'AbortError') {
      type = ErrorType.NETWORK;
      userFriendly = ERROR_MESSAGES.network.timeout;
    } else {
      type = ErrorType.UNKNOWN;
      userFriendly = ERROR_MESSAGES.unknown.default;
      shouldRetry = false;
    }

    const errorInfo: ErrorInfo = {
      type,
      message: error.message || 'Network error',
      timestamp: Date.now(),
      userFriendly,
      shouldRetry
    };

    errorLogger.log(errorInfo);
    return errorInfo;
  }

  // Get user-friendly error message
  private getErrorMessage(category: keyof typeof ERROR_MESSAGES, code: string): string {
    const categoryMessages = ERROR_MESSAGES[category];
    return categoryMessages[code as keyof typeof categoryMessages] || categoryMessages.default;
  }

  // Retry logic for failed requests
  async retryRequest<T>(
    requestFn: () => Promise<T>,
    maxRetries: number = 3,
    onRetry?: (attempt: number, error: ErrorInfo) => void
  ): Promise<T> {
    let lastError: ErrorInfo;

    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        return await requestFn();
      } catch (error) {
        const errorInfo = this.parseApiError(error as ApiError);
        lastError = errorInfo;

        if (attempt === maxRetries || !errorInfo.shouldRetry) {
          throw errorInfo;
        }

        // Call retry callback
        if (onRetry) {
          onRetry(attempt + 1, errorInfo);
        }

        // Wait before retrying
        const delay = this.retryDelays[Math.min(attempt, this.retryDelays.length - 1)];
        await new Promise(resolve => setTimeout(resolve, delay));
      }
    }

    throw lastError!;
  }

  // Handle form validation errors
  parseValidationErrors(errors: Record<string, string[]>): ErrorInfo {
    const messages = Object.values(errors).flat();
    const message = messages.join(', ');

    return {
      type: ErrorType.VALIDATION,
      message,
      timestamp: Date.now(),
      userFriendly: ERROR_MESSAGES.validation.default,
      shouldRetry: false
    };
  }

  // Check if error is retryable
  isRetryableError(error: ErrorInfo): boolean {
    return error.shouldRetry && error.type !== ErrorType.AUTHENTICATION;
  }

  // Get error severity for UI display
  getErrorSeverity(error: ErrorInfo): 'error' | 'warning' | 'info' {
    switch (error.type) {
      case ErrorType.AUTHENTICATION:
      case ErrorType.AUTHORIZATION:
        return 'error';
      case ErrorType.VALIDATION:
        return 'warning';
      case ErrorType.NETWORK:
        return 'warning';
      default:
        return 'error';
    }
  }

  // Format error for display
  formatErrorForDisplay(error: ErrorInfo): string {
    return error.userFriendly;
  }

  // Clear error logs
  clearErrorLogs(): void {
    errorLogger.clearLogs();
  }

  // Get error statistics
  getErrorStats(): Record<ErrorType, number> {
    const logs = errorLogger.getLogs();
    const stats: Record<ErrorType, number> = {
      [ErrorType.NETWORK]: 0,
      [ErrorType.AUTHENTICATION]: 0,
      [ErrorType.AUTHORIZATION]: 0,
      [ErrorType.VALIDATION]: 0,
      [ErrorType.SERVER]: 0,
      [ErrorType.CLIENT]: 0,
      [ErrorType.UNKNOWN]: 0
    };

    logs.forEach(log => {
      stats[log.type]++;
    });

    return stats;
  }
}

// Export singleton instance
export const errorHandler = ErrorHandler.getInstance();

// Utility functions
export const isNetworkError = (error: any): boolean => {
  return error?.type === ErrorType.NETWORK || 
         error?.name === 'TypeError' || 
         error?.name === 'AbortError';
};

export const isAuthError = (error: any): boolean => {
  return error?.type === ErrorType.AUTHENTICATION || 
         error?.status === 401;
};

export const isValidationError = (error: any): boolean => {
  return error?.type === ErrorType.VALIDATION || 
         error?.status === 400;
};

export const shouldRetryRequest = (error: any): boolean => {
  return errorHandler.isRetryableError(error);
}; 