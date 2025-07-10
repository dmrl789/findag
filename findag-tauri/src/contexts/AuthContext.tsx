import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface User {
  id: string;
  username: string;
  email: string;
  role: 'admin' | 'user' | 'validator';
  handle?: string;
}

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (username: string, password: string) => Promise<void>;
  logout: () => void;
  register: (username: string, email: string, password: string) => Promise<void>;
  refreshToken: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const isAuthenticated = !!user;

  useEffect(() => {
    // Check for existing token on app start
    const token = localStorage.getItem('findag-auth-token');
    if (token) {
      validateToken(token);
    } else {
      setIsLoading(false);
    }
  }, []);

  const validateToken = async (_token: string) => {
    try {
      // TODO: Implement token validation with backend
      // For now, we'll simulate a successful validation
      const mockUser: User = {
        id: '1',
        username: 'admin',
        email: 'admin@findag.io',
        role: 'admin',
        handle: '@findag.admin',
      };
      setUser(mockUser);
    } catch (error) {
      console.error('Token validation failed:', error);
      localStorage.removeItem('findag-auth-token');
    } finally {
      setIsLoading(false);
    }
  };

  const login = async (username: string, _password: string) => {
    try {
      setIsLoading(true);
      
      // TODO: Implement actual login with backend
      // For now, we'll simulate a successful login
      const mockUser: User = {
        id: '1',
        username,
        email: `${username}@findag.io`,
        role: username === 'admin' ? 'admin' : 'user',
        handle: `@${username}.user`,
      };
      
      const mockToken = 'mock-jwt-token-' + Date.now();
      localStorage.setItem('findag-auth-token', mockToken);
      
      setUser(mockUser);
    } catch (error) {
      console.error('Login failed:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = () => {
    localStorage.removeItem('findag-auth-token');
    setUser(null);
  };

  const register = async (username: string, email: string, _password: string) => {
    try {
      setIsLoading(true);
      
      // TODO: Implement actual registration with backend
      // For now, we'll simulate a successful registration
      const mockUser: User = {
        id: Date.now().toString(),
        username,
        email,
        role: 'user',
        handle: `@${username}.user`,
      };
      
      const mockToken = 'mock-jwt-token-' + Date.now();
      localStorage.setItem('findag-auth-token', mockToken);
      
      setUser(mockUser);
    } catch (error) {
      console.error('Registration failed:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const refreshToken = async () => {
    try {
      const token = localStorage.getItem('findag-auth-token');
      if (token) {
        await validateToken(token);
      }
    } catch (error) {
      console.error('Token refresh failed:', error);
      logout();
    }
  };

  const value: AuthContextType = {
    user,
    isAuthenticated,
    isLoading,
    login,
    logout,
    register,
    refreshToken,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}; 