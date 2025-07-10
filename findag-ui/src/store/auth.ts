import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { finDAGApi } from '../services/api';

interface User {
  id: string;
  username: string;
  email: string;
  role: 'admin' | 'user' | 'validator';
  permissions: string[];
  lastLogin: number;
  isActive: boolean;
  token?: string;
}

interface AuthState {
  // Auth state
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
  
  // Loading states
  loading: boolean;
  loginLoading: boolean;
  logoutLoading: boolean;
  
  // Error states
  error: string | null;
  loginError: string | null;
  
  // Session management
  sessionTimeout: number | null;
  lastActivity: number;
  
  // Actions
  login: (username: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  refreshToken: () => Promise<void>;
  checkAuth: () => Promise<void>;
  clearError: () => void;
  clearLoginError: () => void;
  updateLastActivity: () => void;
  hasPermission: (permission: string) => boolean;
  hasRole: (role: string) => boolean;
}

const SESSION_TIMEOUT = 24 * 60 * 60 * 1000; // 24 hours
const ACTIVITY_TIMEOUT = 30 * 60 * 1000; // 30 minutes

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      // Initial state
      isAuthenticated: false,
      user: null,
      token: null,
      loading: false,
      loginLoading: false,
      logoutLoading: false,
      error: null,
      loginError: null,
      sessionTimeout: null,
      lastActivity: Date.now(),

      // Login action
      login: async (username: string, password: string) => {
        set({ loginLoading: true, loginError: null });
        
        try {
          const { token, role } = await finDAGApi.login(username, password);
          
          // Create user object with role-based permissions
          const user: User = {
            id: `user-${Date.now()}`,
            username,
            email: `${username}@findag.com`, // Mock email
            role: role as 'admin' | 'user' | 'validator',
            permissions: getPermissionsForRole(role),
            lastLogin: Date.now(),
            isActive: true,
          };

          // Set session timeout
          const sessionTimeout = Date.now() + SESSION_TIMEOUT;

          set({
            isAuthenticated: true,
            user,
            token,
            loginLoading: false,
            sessionTimeout,
            lastActivity: Date.now(),
          });

          // Start session monitoring
          startSessionMonitoring();
          
        } catch (error: any) {
          const errorMessage = error.message || 'Login failed. Please check your credentials.';
          set({ 
            loginLoading: false, 
            loginError: errorMessage,
            isAuthenticated: false,
            user: null,
            token: null,
          });
          throw error;
        }
      },

      // Logout action
      logout: async () => {
        set({ logoutLoading: true });
        
        try {
          // Call logout API if available
          await finDAGApi.logout();
        } catch (error) {
          console.warn('Logout API call failed:', error);
        } finally {
          // Clear local state regardless of API call success
          set({
            isAuthenticated: false,
            user: null,
            token: null,
            logoutLoading: false,
            sessionTimeout: null,
            lastActivity: Date.now(),
          });
          
          // Stop session monitoring
          stopSessionMonitoring();
          
          // Clear any stored data
          localStorage.removeItem('auth_token');
          sessionStorage.clear();
        }
      },

      // Refresh token action
      refreshToken: async () => {
        const { token } = get();
        if (!token) return;

        try {
          // Call refresh token API if available
          // For now, we'll just update the session timeout
          const sessionTimeout = Date.now() + SESSION_TIMEOUT;
          set({ sessionTimeout, lastActivity: Date.now() });
        } catch (error) {
          console.error('Token refresh failed:', error);
          // If refresh fails, logout the user
          get().logout();
        }
      },

      // Check authentication status
      checkAuth: async () => {
        set({ loading: true });
        
        try {
          const token = localStorage.getItem('auth_token');
          if (!token) {
            // Initialize with demo user for development
            const demoUser: User = {
              id: 'demo-user-001',
              username: 'demo_user',
              email: 'demo@findag.com',
              role: 'admin',
              permissions: getPermissionsForRole('admin'),
              lastLogin: Date.now(),
              isActive: true,
            };

            set({ 
              isAuthenticated: true,
              user: demoUser,
              token: 'demo-token',
              loading: false,
              sessionTimeout: Date.now() + SESSION_TIMEOUT,
              lastActivity: Date.now(),
            });
            return;
          }

          // Validate token with backend
          const isValid = await validateToken(token);
          if (!isValid) {
            set({ 
              isAuthenticated: false, 
              user: null, 
              token: null, 
              loading: false 
            });
            localStorage.removeItem('auth_token');
            return;
          }

          // Check session timeout
          const { sessionTimeout, lastActivity } = get();
          if (sessionTimeout && Date.now() > sessionTimeout) {
            set({ 
              isAuthenticated: false, 
              user: null, 
              token: null, 
              loading: false 
            });
            localStorage.removeItem('auth_token');
            return;
          }

          // Check activity timeout
          if (Date.now() - lastActivity > ACTIVITY_TIMEOUT) {
            set({ 
              isAuthenticated: false, 
              user: null, 
              token: null, 
              loading: false 
            });
            localStorage.removeItem('auth_token');
            return;
          }

          // Token is valid, restore session
          set({
            isAuthenticated: true,
            token,
            loading: false,
            lastActivity: Date.now(),
          });

          // Start session monitoring
          startSessionMonitoring();
          
        } catch (error) {
          console.error('Auth check failed:', error);
          set({ 
            isAuthenticated: false, 
            user: null, 
            token: null, 
            loading: false 
          });
          localStorage.removeItem('auth_token');
        }
      },

      // Clear error
      clearError: () => set({ error: null }),
      clearLoginError: () => set({ loginError: null }),

      // Update last activity
      updateLastActivity: () => set({ lastActivity: Date.now() }),

      // Permission checking
      hasPermission: (permission: string) => {
        const { user } = get();
        return user?.permissions.includes(permission) || false;
      },

      hasRole: (role: string) => {
        const { user } = get();
        return user?.role === role || false;
      },
    }),
    {
      name: 'findag-auth',
      partialize: (state) => ({
        token: state.token,
        user: state.user,
        sessionTimeout: state.sessionTimeout,
        lastActivity: state.lastActivity,
      }),
    }
  )
);

// Helper functions
function getPermissionsForRole(role: string): string[] {
  const permissions = {
    admin: [
      'dashboard:read',
      'trading:read',
      'trading:write',
      'transactions:read',
      'transactions:write',
      'validators:read',
      'validators:write',
      'network:read',
      'network:write',
      'rounds:read',
      'rounds:write',
      'metrics:read',
      'metrics:write',
      'users:read',
      'users:write',
      'system:read',
      'system:write',
    ],
    validator: [
      'dashboard:read',
      'trading:read',
      'trading:write',
      'transactions:read',
      'validators:read',
      'network:read',
      'rounds:read',
      'metrics:read',
      'system:read',
    ],
    user: [
      'dashboard:read',
      'trading:read',
      'trading:write',
      'transactions:read',
      'network:read',
      'metrics:read',
    ],
  };

  return permissions[role as keyof typeof permissions] || permissions.user;
}

async function validateToken(token: string): Promise<boolean> {
  try {
    // For now, we'll assume the token is valid if it exists
    // In a real implementation, this would call the backend to validate the token
    return !!token;
  } catch (error) {
    console.error('Token validation failed:', error);
    return false;
  }
}

// Session monitoring
let sessionCheckInterval: number | null = null;

function startSessionMonitoring() {
  if (sessionCheckInterval) {
    clearInterval(sessionCheckInterval);
  }

  sessionCheckInterval = setInterval(() => {
    const { sessionTimeout, lastActivity, isAuthenticated } = useAuthStore.getState();
    
    if (!isAuthenticated) {
      stopSessionMonitoring();
      return;
    }

    // Check session timeout
    if (sessionTimeout && Date.now() > sessionTimeout) {
      console.log('Session expired');
      useAuthStore.getState().logout();
      return;
    }

    // Check activity timeout
    if (Date.now() - lastActivity > ACTIVITY_TIMEOUT) {
      console.log('Activity timeout');
      useAuthStore.getState().logout();
      return;
    }
  }, 60000); // Check every minute
}

function stopSessionMonitoring() {
  if (sessionCheckInterval) {
    clearInterval(sessionCheckInterval);
    sessionCheckInterval = null;
  }
}

// Activity tracking
if (typeof window !== 'undefined') {
  const events = ['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart', 'click'];
  
  events.forEach(event => {
    document.addEventListener(event, () => {
      const { isAuthenticated } = useAuthStore.getState();
      if (isAuthenticated) {
        useAuthStore.getState().updateLastActivity();
      }
    }, true);
  });
} 