import React, { useState, useEffect } from 'react';
import { Eye, EyeOff, Lock, User, AlertCircle, Loader2 } from 'lucide-react';
import { useAuthStore } from '../../store/auth';
import { useNavigate, useLocation } from 'react-router-dom';

interface LoginFormProps {
  onLoginSuccess?: (token: string, role: string) => void;
  onLoginError?: (error: string) => void;
}

export const LoginForm: React.FC<LoginFormProps> = ({ 
  onLoginSuccess, 
  onLoginError 
}) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [rememberMe, setRememberMe] = useState(false);
  
  const navigate = useNavigate();
  const location = useLocation();
  
  const { 
    login, 
    loginLoading, 
    loginError, 
    clearLoginError,
    isAuthenticated,
    user 
  } = useAuthStore();

  // Redirect if already authenticated
  useEffect(() => {
    if (isAuthenticated && user) {
      const from = (location.state as any)?.from?.pathname || '/';
      navigate(from, { replace: true });
    }
  }, [isAuthenticated, user, navigate, location]);

  // Clear error when component mounts
  useEffect(() => {
    clearLoginError();
  }, [clearLoginError]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!username || !password) {
      return;
    }

    try {
      await login(username, password);
      
      // Call success callback if provided
      if (onLoginSuccess && user) {
        onLoginSuccess(user.token || '', user.role);
      }
      
      // Navigate to intended page or dashboard
      const from = (location.state as any)?.from?.pathname || '/';
      navigate(from, { replace: true });
      
    } catch (error: any) {
      const errorMessage = error.message || 'Login failed. Please try again.';
      
      // Call error callback if provided
      if (onLoginError) {
        onLoginError(errorMessage);
      }
    }
  };

  const handleDemoLogin = async () => {
    setUsername('admin');
    setPassword('admin123');
    
    try {
      await login('admin', 'admin123');
      navigate('/', { replace: true });
    } catch (error) {
      console.error('Demo login failed:', error);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div>
          <div className="mx-auto h-12 w-12 bg-primary-600 rounded-lg flex items-center justify-center">
            <span className="text-white font-bold text-lg">FD</span>
          </div>
          <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
            Sign in to FinDAG
          </h2>
          <p className="mt-2 text-center text-sm text-gray-600">
            Access your blockchain management dashboard
          </p>
        </div>
        
        <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
          <div className="space-y-4">
            <div>
              <label htmlFor="username" className="block text-sm font-medium text-gray-700">
                Username
              </label>
              <div className="mt-1 relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <User className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  id="username"
                  name="username"
                  type="text"
                  autoComplete="username"
                  required
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  className="appearance-none relative block w-full pl-10 pr-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-primary-500 focus:border-primary-500 focus:z-10 sm:text-sm"
                  placeholder="Enter your username"
                  disabled={loginLoading}
                />
              </div>
            </div>
            
            <div>
              <label htmlFor="password" className="block text-sm font-medium text-gray-700">
                Password
              </label>
              <div className="mt-1 relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <Lock className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  id="password"
                  name="password"
                  type={showPassword ? 'text' : 'password'}
                  autoComplete="current-password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="appearance-none relative block w-full pl-10 pr-10 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-primary-500 focus:border-primary-500 focus:z-10 sm:text-sm"
                  placeholder="Enter your password"
                  disabled={loginLoading}
                />
                <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
                  <button
                    type="button"
                    onClick={() => setShowPassword(!showPassword)}
                    className="text-gray-400 hover:text-gray-600 focus:outline-none"
                    disabled={loginLoading}
                  >
                    {showPassword ? (
                      <EyeOff className="h-5 w-5" />
                    ) : (
                      <Eye className="h-5 w-5" />
                    )}
                  </button>
                </div>
              </div>
            </div>

            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <input
                  id="remember-me"
                  name="remember-me"
                  type="checkbox"
                  checked={rememberMe}
                  onChange={(e) => setRememberMe(e.target.checked)}
                  className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  disabled={loginLoading}
                />
                <label htmlFor="remember-me" className="ml-2 block text-sm text-gray-900">
                  Remember me
                </label>
              </div>

              <div className="text-sm">
                <a href="#" className="font-medium text-primary-600 hover:text-primary-500">
                  Forgot your password?
                </a>
              </div>
            </div>
          </div>

          {loginError && (
            <div className="rounded-md bg-danger-50 p-4">
              <div className="flex">
                <div className="flex-shrink-0">
                  <AlertCircle className="h-5 w-5 text-danger-400" />
                </div>
                <div className="ml-3">
                  <h3 className="text-sm font-medium text-danger-800">
                    {loginError}
                  </h3>
                </div>
              </div>
            </div>
          )}

          <div className="space-y-3">
            <button
              type="submit"
              disabled={loginLoading || !username || !password}
              className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-lg text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loginLoading ? (
                <div className="flex items-center">
                  <Loader2 className="w-4 h-4 animate-spin mr-2" />
                  Signing in...
                </div>
              ) : (
                'Sign in'
              )}
            </button>

            <button
              type="button"
              onClick={handleDemoLogin}
              disabled={loginLoading}
              className="w-full flex justify-center py-2 px-4 border border-gray-300 text-sm font-medium rounded-lg text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Try Demo (Admin)
            </button>
          </div>

          <div className="text-center space-y-2">
            <p className="text-xs text-gray-500">
              Demo credentials: admin / admin123
            </p>
            <p className="text-xs text-gray-500">
              Don't have an account?{' '}
              <a href="#" className="font-medium text-primary-600 hover:text-primary-500">
                Contact administrator
              </a>
            </p>
          </div>
        </form>
      </div>
    </div>
  );
}; 