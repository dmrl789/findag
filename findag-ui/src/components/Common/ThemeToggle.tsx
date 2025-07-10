import React, { useEffect, useState } from 'react';
import { Moon, Sun, Monitor } from 'lucide-react';
import { useTheme } from '../../store/theme';

export const ThemeToggle: React.FC = () => {
  const { theme, setTheme, isDark } = useTheme();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    console.log('ThemeToggle mounted, current theme:', theme, 'isDark:', isDark);
  }, [theme, isDark]);

  const toggleTheme = () => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    console.log('Toggling theme from', theme, 'to', newTheme);
    setTheme(newTheme);
  };

  const setSystemTheme = () => {
    console.log('Setting theme to system');
    setTheme('system');
  };

  if (!mounted) {
    return (
      <div className="w-10 h-10 bg-gray-200 rounded-lg animate-pulse"></div>
    );
  }

  return (
    <div className="flex items-center space-x-2">
      <button
        onClick={toggleTheme}
        className={`p-2 rounded-lg transition-colors ${
          theme === 'light' 
            ? 'bg-blue-100 text-blue-600 dark:bg-blue-900 dark:text-blue-400' 
            : theme === 'dark'
            ? 'bg-gray-700 text-yellow-400'
            : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300'
        }`}
        title={theme === 'light' ? 'Switch to Dark' : 'Switch to Light'}
      >
        {theme === 'light' ? <Moon className="w-4 h-4" /> : <Sun className="w-4 h-4" />}
      </button>
      
      <button
        onClick={setSystemTheme}
        className={`p-2 rounded-lg transition-colors ${
          theme === 'system'
            ? 'bg-green-100 text-green-600 dark:bg-green-900 dark:text-green-400'
            : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300'
        }`}
        title="Use System Theme"
      >
        <Monitor className="w-4 h-4" />
      </button>
      
      {/* Debug info */}
      <div className="text-xs text-gray-500 dark:text-gray-400">
        {theme} ({isDark ? 'dark' : 'light'})
      </div>
    </div>
  );
}; 