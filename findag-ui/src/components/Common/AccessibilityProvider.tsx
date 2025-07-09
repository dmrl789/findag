import React, { createContext, useContext, useEffect, useState } from 'react';

interface AccessibilityState {
  // Screen reader announcements
  announcements: string[];
  addAnnouncement: (message: string) => void;
  clearAnnouncements: () => void;
  
  // Focus management
  focusableElements: Set<string>;
  registerFocusable: (id: string) => void;
  unregisterFocusable: (id: string) => void;
  focusElement: (id: string) => void;
  
  // Keyboard navigation
  isKeyboardMode: boolean;
  setKeyboardMode: (enabled: boolean) => void;
  
  // High contrast mode
  isHighContrast: boolean;
  setHighContrast: (enabled: boolean) => void;
  
  // Reduced motion
  isReducedMotion: boolean;
  setReducedMotion: (enabled: boolean) => void;
  
  // Font size
  fontSize: 'small' | 'medium' | 'large';
  setFontSize: (size: 'small' | 'medium' | 'large') => void;
  
  // Color blind support
  colorBlindMode: 'none' | 'protanopia' | 'deuteranopia' | 'tritanopia';
  setColorBlindMode: (mode: 'none' | 'protanopia' | 'deuteranopia' | 'tritanopia') => void;
}

const AccessibilityContext = createContext<AccessibilityState | undefined>(undefined);

export const useAccessibility = () => {
  const context = useContext(AccessibilityContext);
  if (!context) {
    throw new Error('useAccessibility must be used within an AccessibilityProvider');
  }
  return context;
};

interface AccessibilityProviderProps {
  children: React.ReactNode;
}

export const AccessibilityProvider: React.FC<AccessibilityProviderProps> = ({ children }) => {
  const [announcements, setAnnouncements] = useState<string[]>([]);
  const [focusableElements, setFocusableElements] = useState<Set<string>>(new Set());
  const [isKeyboardMode, setIsKeyboardMode] = useState(false);
  const [isHighContrast, setIsHighContrast] = useState(false);
  const [isReducedMotion, setIsReducedMotion] = useState(false);
  const [fontSize, setFontSize] = useState<'small' | 'medium' | 'large'>('medium');
  const [colorBlindMode, setColorBlindMode] = useState<'none' | 'protanopia' | 'deuteranopia' | 'tritanopia'>('none');

  // Screen reader announcements
  const addAnnouncement = (message: string) => {
    setAnnouncements(prev => [...prev, message]);
  };

  const clearAnnouncements = () => {
    setAnnouncements([]);
  };

  // Focus management
  const registerFocusable = (id: string) => {
    setFocusableElements(prev => new Set([...prev, id]));
  };

  const unregisterFocusable = (id: string) => {
    setFocusableElements(prev => {
      const newSet = new Set(prev);
      newSet.delete(id);
      return newSet;
    });
  };

  const focusElement = (id: string) => {
    const element = document.getElementById(id);
    if (element) {
      element.focus();
    }
  };

  // Keyboard mode detection
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Tab') {
        setIsKeyboardMode(true);
      }
    };

    const handleMouseDown = () => {
      setIsKeyboardMode(false);
    };

    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('mousedown', handleMouseDown);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('mousedown', handleMouseDown);
    };
  }, []);

  // Apply accessibility settings to document
  useEffect(() => {
    const root = document.documentElement;
    
    // Apply high contrast mode
    if (isHighContrast) {
      root.setAttribute('data-high-contrast', 'true');
    } else {
      root.removeAttribute('data-high-contrast');
    }

    // Apply reduced motion
    if (isReducedMotion) {
      root.setAttribute('data-reduced-motion', 'true');
    } else {
      root.removeAttribute('data-reduced-motion');
    }

    // Apply font size
    root.setAttribute('data-font-size', fontSize);

    // Apply color blind mode
    if (colorBlindMode !== 'none') {
      root.setAttribute('data-color-blind', colorBlindMode);
    } else {
      root.removeAttribute('data-color-blind');
    }
  }, [isHighContrast, isReducedMotion, fontSize, colorBlindMode]);

  // Screen reader live region
  useEffect(() => {
    let liveRegion = document.getElementById('accessibility-live-region');
    if (!liveRegion) {
      liveRegion = document.createElement('div');
      liveRegion.id = 'accessibility-live-region';
      liveRegion.setAttribute('aria-live', 'polite');
      liveRegion.setAttribute('aria-atomic', 'true');
      liveRegion.style.position = 'absolute';
      liveRegion.style.left = '-10000px';
      liveRegion.style.width = '1px';
      liveRegion.style.height = '1px';
      liveRegion.style.overflow = 'hidden';
      document.body.appendChild(liveRegion);
    }

    if (announcements.length > 0) {
      liveRegion.textContent = announcements[announcements.length - 1];
      // Clear after a short delay
      setTimeout(() => {
        liveRegion.textContent = '';
        clearAnnouncements();
      }, 1000);
    }
  }, [announcements]);

  const value: AccessibilityState = {
    announcements,
    addAnnouncement,
    clearAnnouncements,
    focusableElements,
    registerFocusable,
    unregisterFocusable,
    focusElement,
    isKeyboardMode,
    setKeyboardMode: setIsKeyboardMode,
    isHighContrast,
    setHighContrast: setIsHighContrast,
    isReducedMotion,
    setReducedMotion: setIsReducedMotion,
    fontSize,
    setFontSize,
    colorBlindMode,
    setColorBlindMode,
  };

  return (
    <AccessibilityContext.Provider value={value}>
      {children}
    </AccessibilityContext.Provider>
  );
};

// Accessibility announcement component
interface AccessibilityAnnouncementProps {
  message: string;
  priority?: 'polite' | 'assertive';
}

export const AccessibilityAnnouncement: React.FC<AccessibilityAnnouncementProps> = ({ 
  message, 
  priority = 'polite' 
}) => {
  const { addAnnouncement } = useAccessibility();

  useEffect(() => {
    if (message) {
      addAnnouncement(message);
    }
  }, [message, addAnnouncement]);

  return (
    <div
      aria-live={priority}
      aria-atomic="true"
      className="sr-only"
    >
      {message}
    </div>
  );
};

// Skip link component
export const SkipLink: React.FC<{ targetId: string; children: React.ReactNode }> = ({ 
  targetId, 
  children 
}) => {
  return (
    <a
      href={`#${targetId}`}
      className="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:z-50 focus:px-4 focus:py-2 focus:bg-primary-600 focus:text-white focus:rounded focus:outline-none focus:ring-2 focus:ring-primary-500"
    >
      {children}
    </a>
  );
};

// Focus trap component
interface FocusTrapProps {
  children: React.ReactNode;
  active?: boolean;
  onEscape?: () => void;
}

export const FocusTrap: React.FC<FocusTrapProps> = ({ 
  children, 
  active = true, 
  onEscape 
}) => {
  const containerRef = React.useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!active) return;

    const container = containerRef.current;
    if (!container) return;

    const focusableElements = container.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );

    const firstElement = focusableElements[0] as HTMLElement;
    const lastElement = focusableElements[focusableElements.length - 1] as HTMLElement;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && onEscape) {
        onEscape();
        return;
      }

      if (event.key === 'Tab') {
        if (event.shiftKey) {
          if (document.activeElement === firstElement) {
            event.preventDefault();
            lastElement.focus();
          }
        } else {
          if (document.activeElement === lastElement) {
            event.preventDefault();
            firstElement.focus();
          }
        }
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    firstElement?.focus();

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [active, onEscape]);

  return (
    <div ref={containerRef} tabIndex={-1}>
      {children}
    </div>
  );
}; 