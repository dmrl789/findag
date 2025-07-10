import React from 'react';
import { useAuth } from './AuthProvider';
import LoadingSpinner from '../Common/LoadingSpinner';

interface RoleBasedRouteProps {
  children: React.ReactNode;
  requiredRole?: string;
  requiredPermissions?: string[];
  fallback?: React.ReactNode;
  showLoading?: boolean;
}

const RoleBasedRoute: React.FC<RoleBasedRouteProps> = ({
  children,
  requiredRole,
  requiredPermissions = [],
  fallback,
  showLoading = true,
}) => {
  const { user, isAuthenticated, isLoading, hasRole, hasPermission } = useAuth();

  // Show loading spinner while checking authentication
  if (isLoading && showLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  // If not authenticated, show fallback or redirect
  if (!isAuthenticated) {
    return fallback ? (
      <>{fallback}</>
    ) : (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            Authentication Required
          </h2>
          <p className="text-gray-600 dark:text-gray-400">
            Please log in to access this page.
          </p>
        </div>
      </div>
    );
  }

  // Check role requirement
  if (requiredRole && !hasRole(requiredRole)) {
    return fallback ? (
      <>{fallback}</>
    ) : (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            Access Denied
          </h2>
          <p className="text-gray-600 dark:text-gray-400">
            You don't have the required role ({requiredRole}) to access this page.
          </p>
        </div>
      </div>
    );
  }

  // Check permission requirements
  if (requiredPermissions.length > 0) {
    const missingPermissions = requiredPermissions.filter(permission => !hasPermission(permission));
    
    if (missingPermissions.length > 0) {
      return fallback ? (
        <>{fallback}</>
      ) : (
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
              Insufficient Permissions
            </h2>
            <p className="text-gray-600 dark:text-gray-400">
              You don't have the required permissions to access this page.
            </p>
            <p className="text-sm text-gray-500 dark:text-gray-500 mt-2">
              Missing: {missingPermissions.join(', ')}
            </p>
          </div>
        </div>
      );
    }
  }

  // All checks passed, render children
  return <>{children}</>;
};

export default RoleBasedRoute; 