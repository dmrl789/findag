import React, { useState } from 'react';
import { Sun, Moon, Monitor, ChevronDown } from 'lucide-react';
import { useThemeStore, Theme } from '../../store/theme';

interface ThemeToggleProps {
  className?: string;
  showLabel?: boolean;
  variant?: 'button' | 'dropdown';
}

export const ThemeToggle: React.FC<ThemeToggleProps> = ({ 
  className = '', 
  showLabel = false,
  variant = 'button'
}) => {
  const { theme, isDark, setTheme, toggleTheme } = useThemeStore();
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);

  const themeOptions: { value: Theme; label: string; icon: React.ReactNode }[] = [
    { value: 'light', label: 'Light', icon: <Sun className="w-4 h-4" /> },
    { value: 'dark', label: 'Dark', icon: <Moon className="w-4 h-4" /> },
    { value: 'system', label: 'System', icon: <Monitor className="w-4 h-4" /> },
  ];

  const currentTheme = themeOptions.find(option => option.value === theme);

  const handleThemeChange = (newTheme: Theme) => {
    setTheme(newTheme);
    setIsDropdownOpen(false);
  };

  if (variant === 'button') {
    return (
      <button
        onClick={toggleTheme}
        className={`inline-flex items-center justify-center p-2 rounded-lg bg-gray-100 hover:bg-gray-200 text-gray-700 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 ${className}`}
        aria-label={`Switch to ${isDark ? 'light' : 'dark'} mode`}
        title={`Switch to ${isDark ? 'light' : 'dark'} mode`}
      >
        {isDark ? (
          <Sun className="w-5 h-5" />
        ) : (
          <Moon className="w-5 h-5" />
        )}
        {showLabel && (
          <span className="ml-2 text-sm font-medium">
            {isDark ? 'Light' : 'Dark'}
          </span>
        )}
      </button>
    );
  }

  return (
    <div className={`relative ${className}`}>
      <button
        onClick={() => setIsDropdownOpen(!isDropdownOpen)}
        className="inline-flex items-center justify-between w-full px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 transition-colors duration-200"
        aria-label="Select theme"
        aria-expanded={isDropdownOpen}
        aria-haspopup="listbox"
      >
        <div className="flex items-center">
          {currentTheme?.icon}
          <span className="ml-2">{currentTheme?.label}</span>
        </div>
        <ChevronDown className={`w-4 h-4 ml-2 transition-transform duration-200 ${isDropdownOpen ? 'rotate-180' : ''}`} />
      </button>

      {isDropdownOpen && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 z-10"
            onClick={() => setIsDropdownOpen(false)}
          />
          
          {/* Dropdown */}
          <div className="absolute right-0 mt-1 w-48 bg-white border border-gray-200 rounded-lg shadow-lg z-20">
            <ul
              role="listbox"
              className="py-1"
              aria-label="Theme options"
            >
              {themeOptions.map((option) => (
                <li key={option.value} role="option" aria-selected={theme === option.value}>
                  <button
                    onClick={() => handleThemeChange(option.value)}
                    className={`w-full flex items-center px-4 py-2 text-sm text-left hover:bg-gray-100 focus:outline-none focus:bg-gray-100 transition-colors duration-150 ${
                      theme === option.value ? 'bg-primary-50 text-primary-700' : 'text-gray-700'
                    }`}
                  >
                    {option.icon}
                    <span className="ml-3">{option.label}</span>
                    {theme === option.value && (
                      <div className="ml-auto w-2 h-2 bg-primary-600 rounded-full" />
                    )}
                  </button>
                </li>
              ))}
            </ul>
          </div>
        </>
      )}
    </div>
  );
};

// Compact theme toggle for header
export const CompactThemeToggle: React.FC<{ className?: string }> = ({ className = '' }) => {
  const { isDark, toggleTheme } = useThemeStore();

  return (
    <button
      onClick={toggleTheme}
      className={`p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 ${className}`}
      aria-label={`Switch to ${isDark ? 'light' : 'dark'} mode`}
      title={`Switch to ${isDark ? 'light' : 'dark'} mode`}
    >
      {isDark ? (
        <Sun className="w-5 h-5 text-gray-300" />
      ) : (
        <Moon className="w-5 h-5 text-gray-600" />
      )}
    </button>
  );
}; 