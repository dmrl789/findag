import React, { createContext, useContext, useEffect, useState } from 'react';

type Theme = 'light' | 'dark' | 'system';

interface ThemeContextType {
  theme: Theme;
  isDark: boolean;
  setTheme: (theme: Theme) => void;
  toggleTheme: () => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
};

interface ThemeProviderProps {
  children: React.ReactNode;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [theme, setThemeState] = useState<Theme>('system');
  const [isDark, setIsDark] = useState(false);

  // Get system preference
  const getSystemTheme = (): boolean => {
    if (typeof window !== 'undefined') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    return false;
  };

  // Apply theme to document
  const applyTheme = (isDarkMode: boolean) => {
    if (typeof document !== 'undefined') {
      console.log('Applying theme:', isDarkMode ? 'dark' : 'light');
      document.documentElement.classList.toggle('dark', isDarkMode);
      document.documentElement.setAttribute('data-theme', isDarkMode ? 'dark' : 'light');
      
      // Debug: Check if class was applied
      console.log('Document classes:', document.documentElement.classList.toString());
      console.log('Document data-theme:', document.documentElement.getAttribute('data-theme'));
    }
  };

  // Set theme
  const setTheme = (newTheme: Theme) => {
    console.log('Setting theme to:', newTheme);
    setThemeState(newTheme);
    localStorage.setItem('theme', newTheme);
    
    let isDarkMode = false;
    if (newTheme === 'system') {
      isDarkMode = getSystemTheme();
      console.log('System theme detected:', isDarkMode ? 'dark' : 'light');
    } else {
      isDarkMode = newTheme === 'dark';
    }
    
    setIsDark(isDarkMode);
    applyTheme(isDarkMode);
  };

  // Toggle between light and dark
  const toggleTheme = () => {
    const newTheme = isDark ? 'light' : 'dark';
    console.log('Toggling theme from', theme, 'to', newTheme);
    setTheme(newTheme);
  };

  // Initialize theme
  useEffect(() => {
    const savedTheme = localStorage.getItem('theme') as Theme;
    const initialTheme = savedTheme || 'system';
    console.log('Initializing theme with:', initialTheme);
    setTheme(initialTheme);
  }, []);

  // Listen for system theme changes
  useEffect(() => {
    if (theme === 'system' && typeof window !== 'undefined') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      
      const handleChange = (e: MediaQueryListEvent) => {
        console.log('System theme changed to:', e.matches ? 'dark' : 'light');
        setIsDark(e.matches);
        applyTheme(e.matches);
      };

      mediaQuery.addEventListener('change', handleChange);
      return () => mediaQuery.removeEventListener('change', handleChange);
    }
  }, [theme]);

  const value: ThemeContextType = {
    theme,
    isDark,
    setTheme,
    toggleTheme,
  };

  return (
    <ThemeContext.Provider value={value}>
      {children}
    </ThemeContext.Provider>
  );
}; 