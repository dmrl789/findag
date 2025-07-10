import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { showNotification } from '../Common/NotificationSystem';

interface User {
  id: string;
  email: string;
  username: string;
  role: 'admin' | 'user' | 'validator';
  permissions: string[];
  lastLogin: number;
  isActive: boolean;
}

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  register: (email: string, username: string, password: string) => Promise<boolean>;
  refreshToken: () => Promise<boolean>;
  hasPermission: (permission: string) => boolean;
  hasRole: (role: string) => boolean;
  updateUser: (userData: Partial<User>) => Promise<void>;
  changePassword: (currentPassword: string, newPassword: string) => Promise<boolean>;
  resetPassword: (email: string) => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: React.ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [token, setToken] = useState<string | null>(null);

  // Check for existing session on mount
  useEffect(() => {
    checkExistingSession();
  }, []);

  // Set up token refresh interval
  useEffect(() => {
    if (token) {
      const interval = setInterval(() => {
        refreshToken();
      }, 14 * 60 * 1000); // Refresh every 14 minutes (tokens expire in 15 minutes)

      return () => clearInterval(interval);
    }
  }, [token]);

  const checkExistingSession = async () => {
    try {
      const storedToken = localStorage.getItem('findag_auth_token');
      if (storedToken) {
        setToken(storedToken);
        const userData = await invoke('validate_token', { token: storedToken });
        if (userData) {
          setUser(userData as User);
        } else {
          // Token is invalid, clear it
          localStorage.removeItem('findag_auth_token');
          setToken(null);
        }
      }
    } catch (error) {
      console.error('Failed to check existing session:', error);
      localStorage.removeItem('findag_auth_token');
    } finally {
      setIsLoading(false);
    }
  };

  const login = async (email: string, password: string): Promise<boolean> => {
    try {
      setIsLoading(true);
      const result = await invoke('authenticate_user', { email, password });
      
      if (result && typeof result === 'object' && 'token' in result && 'user' in result) {
        const authResult = result as { token: string; user: User };
        setToken(authResult.token);
        setUser(authResult.user);
        localStorage.setItem('findag_auth_token', authResult.token);
        
        showNotification({
          type: 'success',
          title: 'Login Successful',
          message: `Welcome back, ${authResult.user.username}!`,
        });
        
        return true;
      }
      
      return false;
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Login Failed',
        message: 'Invalid email or password',
      });
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async (): Promise<void> => {
    try {
      if (token) {
        await invoke('logout_user', { token });
      }
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      setUser(null);
      setToken(null);
      localStorage.removeItem('findag_auth_token');
      
      showNotification({
        type: 'success',
        title: 'Logged Out',
        message: 'You have been successfully logged out',
      });
    }
  };

  const register = async (email: string, username: string, password: string): Promise<boolean> => {
    try {
      setIsLoading(true);
      const result = await invoke('register_user', { email, username, password });
      
      if (result) {
        showNotification({
          type: 'success',
          title: 'Registration Successful',
          message: 'Account created successfully. Please log in.',
        });
        return true;
      }
      
      return false;
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Registration Failed',
        message: 'Failed to create account. Please try again.',
      });
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const refreshToken = async (): Promise<boolean> => {
    try {
      if (!token) return false;
      
      const result = await invoke('refresh_token', { token });
      
      if (result && typeof result === 'object' && 'token' in result) {
        const authResult = result as { token: string; user: User };
        setToken(authResult.token);
        setUser(authResult.user);
        localStorage.setItem('findag_auth_token', authResult.token);
        return true;
      }
      
      return false;
    } catch (error) {
      console.error('Token refresh failed:', error);
      // Token is invalid, logout user
      await logout();
      return false;
    }
  };

  const hasPermission = (permission: string): boolean => {
    if (!user) return false;
    return user.permissions.includes(permission);
  };

  const hasRole = (role: string): boolean => {
    if (!user) return false;
    return user.role === role;
  };

  const updateUser = async (userData: Partial<User>): Promise<void> => {
    try {
      if (!token) throw new Error('No authentication token');
      
      const result = await invoke('update_user_profile', { 
        token, 
        userData 
      });
      
      if (result) {
        setUser(prev => prev ? { ...prev, ...userData } : null);
        showNotification({
          type: 'success',
          title: 'Profile Updated',
          message: 'Your profile has been updated successfully',
        });
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Update Failed',
        message: 'Failed to update profile',
      });
    }
  };

  const changePassword = async (currentPassword: string, newPassword: string): Promise<boolean> => {
    try {
      if (!token) return false;
      
      const result = await invoke('change_password', { 
        token, 
        currentPassword, 
        newPassword 
      });
      
      if (result) {
        showNotification({
          type: 'success',
          title: 'Password Changed',
          message: 'Your password has been changed successfully',
        });
        return true;
      }
      
      return false;
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Password Change Failed',
        message: 'Failed to change password. Please check your current password.',
      });
      return false;
    }
  };

  const resetPassword = async (email: string): Promise<boolean> => {
    try {
      const result = await invoke('reset_password', { email });
      
      if (result) {
        showNotification({
          type: 'success',
          title: 'Password Reset',
          message: 'Password reset instructions have been sent to your email',
        });
        return true;
      }
      
      return false;
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Password Reset Failed',
        message: 'Failed to send password reset email',
      });
      return false;
    }
  };

  const value: AuthContextType = {
    user,
    isAuthenticated: !!user,
    isLoading,
    login,
    logout,
    register,
    refreshToken,
    hasPermission,
    hasRole,
    updateUser,
    changePassword,
    resetPassword,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}; 