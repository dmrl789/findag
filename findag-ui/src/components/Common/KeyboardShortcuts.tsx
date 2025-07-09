import React, { useEffect, useCallback } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { useThemeStore } from '../../store/theme';

interface KeyboardShortcut {
  key: string;
  description: string;
  action: () => void;
  modifier?: 'ctrl' | 'alt' | 'shift' | 'meta';
  category?: string;
}

interface KeyboardShortcutsProps {
  children: React.ReactNode;
}

export const KeyboardShortcuts: React.FC<KeyboardShortcutsProps> = ({ children }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const { toggleTheme } = useThemeStore();

  // Define keyboard shortcuts
  const shortcuts: KeyboardShortcut[] = [
    // Navigation shortcuts
    {
      key: 'g',
      description: 'Go to Dashboard',
      action: () => navigate('/'),
      modifier: 'g',
      category: 'Navigation',
    },
    {
      key: 't',
      description: 'Go to Trading',
      action: () => navigate('/trading'),
      modifier: 'g',
      category: 'Navigation',
    },
    {
      key: 'd',
      description: 'Go to DAG Explorer',
      action: () => navigate('/dag'),
      modifier: 'g',
      category: 'Navigation',
    },
    {
      key: 'x',
      description: 'Go to Transactions',
      action: () => navigate('/transactions'),
      modifier: 'g',
      category: 'Navigation',
    },
    {
      key: 'v',
      description: 'Go to Validators',
      action: () => navigate('/validators'),
      modifier: 'g',
      category: 'Navigation',
    },
    {
      key: 'r',
      description: 'Go to Rounds',
      action: () => navigate('/rounds'),
      modifier: 'g',
      category: 'Navigation',
    },
    {
      key: 'n',
      description: 'Go to Network',
      action: () => navigate('/network'),
      modifier: 'g',
      category: 'Navigation',
    },
    {
      key: 'm',
      description: 'Go to Metrics',
      action: () => navigate('/metrics'),
      modifier: 'g',
      category: 'Navigation',
    },

    // Application shortcuts
    {
      key: '?',
      description: 'Show keyboard shortcuts',
      action: () => {
        // This would typically show a modal with all shortcuts
        console.log('Show keyboard shortcuts help');
      },
      category: 'Application',
    },
    {
      key: 'd',
      description: 'Toggle dark mode',
      action: toggleTheme,
      modifier: 'ctrl',
      category: 'Application',
    },
    {
      key: 'r',
      description: 'Refresh page',
      action: () => window.location.reload(),
      modifier: 'ctrl',
      category: 'Application',
    },
    {
      key: 'f',
      description: 'Focus search',
      action: () => {
        const searchInput = document.querySelector('input[type="search"], input[placeholder*="search" i]') as HTMLInputElement;
        if (searchInput) {
          searchInput.focus();
        }
      },
      modifier: 'ctrl',
      category: 'Application',
    },

    // Trading shortcuts (when on trading page)
    {
      key: 'b',
      description: 'Buy order form',
      action: () => {
        if (location.pathname === '/trading') {
          // Focus buy form
          const buyButton = document.querySelector('[data-action="buy"]') as HTMLElement;
          if (buyButton) buyButton.click();
        }
      },
      modifier: 'alt',
      category: 'Trading',
    },
    {
      key: 's',
      description: 'Sell order form',
      action: () => {
        if (location.pathname === '/trading') {
          // Focus sell form
          const sellButton = document.querySelector('[data-action="sell"]') as HTMLElement;
          if (sellButton) sellButton.click();
        }
      },
      modifier: 'alt',
      category: 'Trading',
    },

    // Utility shortcuts
    {
      key: 'Escape',
      description: 'Close modal/dialog',
      action: () => {
        // Close any open modals or dropdowns
        const modals = document.querySelectorAll('[data-modal="true"]');
        modals.forEach(modal => {
          const closeButton = modal.querySelector('[data-action="close"]') as HTMLElement;
          if (closeButton) closeButton.click();
        });
      },
      category: 'Utility',
    },
  ];

  // Handle keyboard events
  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    // Don't trigger shortcuts when typing in input fields
    if (
      event.target instanceof HTMLInputElement ||
      event.target instanceof HTMLTextAreaElement ||
      event.target instanceof HTMLSelectElement ||
      (event.target as HTMLElement).contentEditable === 'true'
    ) {
      return;
    }

    const key = event.key.toLowerCase();
    const modifiers = {
      ctrl: event.ctrlKey,
      alt: event.altKey,
      shift: event.shiftKey,
      meta: event.metaKey,
    };

    // Find matching shortcut
    const shortcut = shortcuts.find(s => {
      const keyMatch = s.key.toLowerCase() === key;
      const modifierMatch = !s.modifier || modifiers[s.modifier];
      const otherModifiers = Object.entries(modifiers)
        .filter(([mod]) => mod !== s.modifier)
        .every(([, pressed]) => !pressed);
      
      return keyMatch && modifierMatch && otherModifiers;
    });

    if (shortcut) {
      event.preventDefault();
      shortcut.action();
    }
  }, [shortcuts, location.pathname, toggleTheme]);

  // Register keyboard event listener
  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [handleKeyDown]);

  return <>{children}</>;
};

// Keyboard shortcuts help modal
interface KeyboardShortcutsHelpProps {
  isOpen: boolean;
  onClose: () => void;
}

export const KeyboardShortcutsHelp: React.FC<KeyboardShortcutsHelpProps> = ({ isOpen, onClose }) => {
  const shortcuts = [
    {
      category: 'Navigation',
      shortcuts: [
        { key: 'g + g', description: 'Go to Dashboard' },
        { key: 'g + t', description: 'Go to Trading' },
        { key: 'g + d', description: 'Go to DAG Explorer' },
        { key: 'g + x', description: 'Go to Transactions' },
        { key: 'g + v', description: 'Go to Validators' },
        { key: 'g + r', description: 'Go to Rounds' },
        { key: 'g + n', description: 'Go to Network' },
        { key: 'g + m', description: 'Go to Metrics' },
      ],
    },
    {
      category: 'Application',
      shortcuts: [
        { key: '?', description: 'Show keyboard shortcuts' },
        { key: 'Ctrl + d', description: 'Toggle dark mode' },
        { key: 'Ctrl + r', description: 'Refresh page' },
        { key: 'Ctrl + f', description: 'Focus search' },
      ],
    },
    {
      category: 'Trading',
      shortcuts: [
        { key: 'Alt + b', description: 'Buy order form' },
        { key: 'Alt + s', description: 'Sell order form' },
      ],
    },
    {
      category: 'Utility',
      shortcuts: [
        { key: 'Escape', description: 'Close modal/dialog' },
      ],
    },
  ];

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        {/* Backdrop */}
        <div
          className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
          onClick={onClose}
        />

        {/* Modal */}
        <div className="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
          <div className="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
            <div className="sm:flex sm:items-start">
              <div className="mt-3 text-center sm:mt-0 sm:text-left w-full">
                <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                  Keyboard Shortcuts
                </h3>
                <div className="space-y-4">
                  {shortcuts.map((category) => (
                    <div key={category.category}>
                      <h4 className="text-sm font-medium text-gray-700 mb-2">
                        {category.category}
                      </h4>
                      <div className="space-y-2">
                        {category.shortcuts.map((shortcut) => (
                          <div key={shortcut.key} className="flex justify-between items-center">
                            <span className="text-sm text-gray-600">{shortcut.description}</span>
                            <kbd className="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-300 rounded">
                              {shortcut.key}
                            </kbd>
                          </div>
                        ))}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
            <button
              type="button"
              className="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-primary-600 text-base font-medium text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 sm:ml-3 sm:w-auto sm:text-sm"
              onClick={onClose}
            >
              Close
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}; 