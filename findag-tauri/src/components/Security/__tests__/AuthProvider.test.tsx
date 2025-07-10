import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { AuthProvider, useAuth } from '../AuthProvider';

// Mock Tauri invoke
jest.mock('@tauri-apps/api/tauri', () => ({
  invoke: jest.fn(),
}));

// Mock notification system
jest.mock('../../Common/NotificationSystem', () => ({
  showNotification: jest.fn(),
}));

// Test component to access auth context
const TestComponent: React.FC = () => {
  const { user, isAuthenticated, isLoading, login, logout } = useAuth();
  
  return (
    <div>
      <div data-testid="user-info">
        {isAuthenticated ? `Logged in as ${user?.username}` : 'Not authenticated'}
      </div>
      <div data-testid="loading-status">
        {isLoading ? 'Loading' : 'Not loading'}
      </div>
      <button data-testid="login-btn" onClick={() => login('test@example.com', 'password')}>
        Login
      </button>
      <button data-testid="logout-btn" onClick={() => logout()}>
        Logout
      </button>
    </div>
  );
};

const renderWithAuth = (component: React.ReactElement) => {
  return render(
    <AuthProvider>
      {component}
    </AuthProvider>
  );
};

describe('AuthProvider', () => {
  const mockInvoke = require('@tauri-apps/api/tauri').invoke;

  beforeEach(() => {
    jest.clearAllMocks();
    localStorage.clear();
  });

  it('renders children without crashing', () => {
    renderWithAuth(<TestComponent />);
    expect(screen.getByTestId('user-info')).toBeInTheDocument();
  });

  it('shows loading state initially', () => {
    renderWithAuth(<TestComponent />);
    expect(screen.getByTestId('loading-status')).toHaveTextContent('Loading');
  });

  it('handles successful login', async () => {
    const mockUser = {
      id: '1',
      email: 'test@example.com',
      username: 'testuser',
      role: 'user' as const,
      permissions: ['read'],
      lastLogin: Date.now(),
      isActive: true,
    };

    mockInvoke.mockResolvedValueOnce({
      token: 'mock-token',
      user: mockUser,
    });

    renderWithAuth(<TestComponent />);

    const loginButton = screen.getByTestId('login-btn');
    fireEvent.click(loginButton);

    await waitFor(() => {
      expect(screen.getByTestId('user-info')).toHaveTextContent('Logged in as testuser');
    });

    expect(mockInvoke).toHaveBeenCalledWith('authenticate_user', {
      email: 'test@example.com',
      password: 'password',
    });
  });

  it('handles login failure', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('Invalid credentials'));

    renderWithAuth(<TestComponent />);

    const loginButton = screen.getByTestId('login-btn');
    fireEvent.click(loginButton);

    await waitFor(() => {
      expect(screen.getByTestId('user-info')).toHaveTextContent('Not authenticated');
    });
  });

  it('handles logout', async () => {
    // First login
    const mockUser = {
      id: '1',
      email: 'test@example.com',
      username: 'testuser',
      role: 'user' as const,
      permissions: ['read'],
      lastLogin: Date.now(),
      isActive: true,
    };

    mockInvoke
      .mockResolvedValueOnce({
        token: 'mock-token',
        user: mockUser,
      })
      .mockResolvedValueOnce(undefined); // logout response

    renderWithAuth(<TestComponent />);

    // Login first
    const loginButton = screen.getByTestId('login-btn');
    fireEvent.click(loginButton);

    await waitFor(() => {
      expect(screen.getByTestId('user-info')).toHaveTextContent('Logged in as testuser');
    });

    // Then logout
    const logoutButton = screen.getByTestId('logout-btn');
    fireEvent.click(logoutButton);

    await waitFor(() => {
      expect(screen.getByTestId('user-info')).toHaveTextContent('Not authenticated');
    });
  });

  it('checks for existing session on mount', async () => {
    const mockUser = {
      id: '1',
      email: 'test@example.com',
      username: 'testuser',
      role: 'user' as const,
      permissions: ['read'],
      lastLogin: Date.now(),
      isActive: true,
    };

    localStorage.setItem('findag_auth_token', 'existing-token');
    mockInvoke.mockResolvedValueOnce(mockUser);

    renderWithAuth(<TestComponent />);

    await waitFor(() => {
      expect(screen.getByTestId('user-info')).toHaveTextContent('Logged in as testuser');
    });

    expect(mockInvoke).toHaveBeenCalledWith('validate_token', {
      token: 'existing-token',
    });
  });

  it('clears invalid token from localStorage', async () => {
    localStorage.setItem('findag_auth_token', 'invalid-token');
    mockInvoke.mockResolvedValueOnce(null); // Invalid token

    renderWithAuth(<TestComponent />);

    await waitFor(() => {
      expect(localStorage.getItem('findag_auth_token')).toBeNull();
    });
  });

  it('throws error when useAuth is used outside provider', () => {
    // Suppress console.error for this test
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});

    expect(() => {
      render(<TestComponent />);
    }).toThrow('useAuth must be used within an AuthProvider');

    consoleSpy.mockRestore();
  });
}); 