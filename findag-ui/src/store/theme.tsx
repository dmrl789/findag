import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import React, { createContext, useContext, useEffect } from 'react';

export type Theme = 'light' | 'dark' | 'system';

interface ThemeState {
  theme: Theme;
  isDark: boolean;
  setTheme: (theme: Theme) => void;
  toggleTheme: () => void;
}

const getSystemTheme = (): boolean => {
  if (typeof window === 'undefined') return false;
  return window.matchMedia('(prefers-color-scheme: dark)').matches;
};

const applyTheme = (theme: Theme) => {
  if (typeof window === 'undefined') return;
  
  const root = document.documentElement;
  const isDark = theme === 'dark' || (theme === 'system' && getSystemTheme());
  
  root.setAttribute('data-theme', isDark ? 'dark' : 'light');
  
  // Update meta theme-color for mobile browsers
  const metaThemeColor = document.querySelector('meta[name="theme-color"]');
  if (metaThemeColor) {
    metaThemeColor.setAttribute('content', isDark ? '#111827' : '#ffffff');
  }
};

export const useThemeStore = create<ThemeState>()(
  persist(
    (set, get) => ({
      theme: 'system',
      isDark: getSystemTheme(),
      
      setTheme: (theme: Theme) => {
        const isDark = theme === 'dark' || (theme === 'system' && getSystemTheme());
        set({ theme, isDark });
        applyTheme(theme);
      },
      
      toggleTheme: () => {
        const { theme } = get();
        const newTheme = theme === 'light' ? 'dark' : 'light';
        get().setTheme(newTheme);
      },
    }),
    {
      name: 'findag-theme',
      onRehydrateStorage: () => (state) => {
        if (state) {
          applyTheme(state.theme);
        }
      },
    }
  )
);

// Theme Context
const ThemeContext = createContext<ThemeState | null>(null);

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return context;
};

// Theme Provider Component
interface ThemeProviderProps {
  children: React.ReactNode;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const themeStore = useThemeStore();

  useEffect(() => {
    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = (e: MediaQueryListEvent) => {
      const { theme } = themeStore;
      if (theme === 'system') {
        const isDark = e.matches;
        themeStore.setTheme('system');
      }
    };

    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, [themeStore]);

  return (
    <ThemeContext.Provider value={themeStore}>
      {children}
    </ThemeContext.Provider>
  );
};

// Initialize theme on mount
if (typeof window !== 'undefined') {
  // Apply initial theme
  const { theme } = useThemeStore.getState();
  applyTheme(theme);
} 