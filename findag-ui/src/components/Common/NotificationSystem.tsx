import React, { useState, useEffect, useRef, useCallback } from 'react';
import { 
  Bell, 
  BellOff, 
  X, 
  CheckCircle, 
  AlertCircle, 
  Info, 
  AlertTriangle,
  Settings,
  Mail,
  Smartphone,
  Volume2,
  VolumeX,
  Clock,
  Trash2,
  Filter,
  Search
} from 'lucide-react';

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info' | 'trade' | 'price' | 'order' | 'system';
  title: string;
  message: string;
  timestamp: number;
  read: boolean;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  category: 'trading' | 'price' | 'order' | 'system' | 'security';
  actions?: NotificationAction[];
  data?: Record<string, any>;
  expiresAt?: number;
  userId?: string;
}

export interface NotificationAction {
  label: string;
  action: () => void;
  variant?: 'primary' | 'secondary' | 'danger';
}

export interface NotificationPreferences {
  enabled: boolean;
  types: {
    success: boolean;
    error: boolean;
    warning: boolean;
    info: boolean;
    trade: boolean;
    price: boolean;
    order: boolean;
    system: boolean;
  };
  channels: {
    toast: boolean;
    push: boolean;
    email: boolean;
  };
  priority: {
    low: boolean;
    medium: boolean;
    high: boolean;
    urgent: boolean;
  };
  categories: {
    trading: boolean;
    price: boolean;
    order: boolean;
    system: boolean;
    security: boolean;
  };
  quietHours: {
    enabled: boolean;
    start: string; // HH:mm format
    end: string; // HH:mm format
    timezone: string;
  };
  sound: boolean;
  vibration: boolean;
}

interface NotificationSystemProps {
  children: React.ReactNode;
}

interface NotificationContextType {
  notifications: Notification[];
  unreadCount: number;
  preferences: NotificationPreferences;
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => void;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  removeNotification: (id: string) => void;
  clearAll: () => void;
  updatePreferences: (preferences: Partial<NotificationPreferences>) => void;
  isQuietHours: () => boolean;
  getFilteredNotifications: (filters?: NotificationFilters) => Notification[];
}

interface NotificationFilters {
  type?: Notification['type'];
  category?: Notification['category'];
  priority?: Notification['priority'];
  read?: boolean;
  search?: string;
  dateRange?: {
    start: Date;
    end: Date;
  };
}

const NotificationContext = React.createContext<NotificationContextType | null>(null);

export const useNotifications = () => {
  const context = React.useContext(NotificationContext);
  if (!context) {
    throw new Error('useNotifications must be used within NotificationProvider');
  }
  return context;
};

export const NotificationProvider: React.FC<NotificationSystemProps> = ({ children }) => {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [preferences, setPreferences] = useState<NotificationPreferences>({
    enabled: true,
    types: {
      success: true,
      error: true,
      warning: true,
      info: true,
      trade: true,
      price: true,
      order: true,
      system: true,
    },
    channels: {
      toast: true,
      push: true,
      email: false,
    },
    priority: {
      low: true,
      medium: true,
      high: true,
      urgent: true,
    },
    categories: {
      trading: true,
      price: true,
      order: true,
      system: true,
      security: true,
    },
    quietHours: {
      enabled: false,
      start: '22:00',
      end: '08:00',
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
    },
    sound: true,
    vibration: true,
  });

  const audioRef = useRef<HTMLAudioElement | null>(null);
  const notificationTimeoutRefs = useRef<Map<string, number>>(new Map());

  // Load preferences from localStorage
  useEffect(() => {
    const savedPreferences = localStorage.getItem('notificationPreferences');
    if (savedPreferences) {
      try {
        setPreferences(JSON.parse(savedPreferences));
      } catch (error) {
        console.error('Failed to load notification preferences:', error);
      }
    }

    const savedNotifications = localStorage.getItem('notifications');
    if (savedNotifications) {
      try {
        const parsed = JSON.parse(savedNotifications);
        setNotifications(parsed.filter((n: Notification) => !n.expiresAt || n.expiresAt > Date.now()));
      } catch (error) {
        console.error('Failed to load notifications:', error);
      }
    }
  }, []);

  // Save preferences to localStorage
  useEffect(() => {
    localStorage.setItem('notificationPreferences', JSON.stringify(preferences));
  }, [preferences]);

  // Save notifications to localStorage
  useEffect(() => {
    localStorage.setItem('notifications', JSON.stringify(notifications));
  }, [notifications]);

  // Initialize audio for notification sounds
  useEffect(() => {
    if (typeof window !== 'undefined') {
      audioRef.current = new Audio('/notification-sound.mp3'); // You can add a sound file
      audioRef.current.volume = 0.5;
    }
  }, []);

  const isQuietHours = useCallback(() => {
    if (!preferences.quietHours.enabled) return false;

    const now = new Date();
    const currentTime = now.toLocaleTimeString('en-US', { 
      hour12: false, 
      timeZone: preferences.quietHours.timezone 
    });
    
    const start = preferences.quietHours.start;
    const end = preferences.quietHours.end;
    
    if (start <= end) {
      return currentTime >= start && currentTime <= end;
    } else {
      // Handles overnight quiet hours (e.g., 22:00 to 08:00)
      return currentTime >= start || currentTime <= end;
    }
  }, [preferences.quietHours]);

  const shouldShowNotification = useCallback((notification: Notification) => {
    if (!preferences.enabled) return false;
    if (isQuietHours() && notification.priority !== 'urgent') return false;
    if (!preferences.types[notification.type]) return false;
    if (!preferences.priority[notification.priority]) return false;
    if (!preferences.categories[notification.category]) return false;
    return true;
  }, [preferences, isQuietHours]);

  const showToastNotification = useCallback((notification: Notification) => {
    // This function will be called by the toast component
    // For now, we'll just add it to the notifications list
    // The actual toast display is handled by the NotificationToast component
  }, []);

  const addNotification = useCallback((notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => {
    const newNotification: Notification = {
      ...notification,
      id: `notification_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      timestamp: Date.now(),
      read: false,
    };

    if (shouldShowNotification(newNotification)) {
      setNotifications(prev => [newNotification, ...prev.slice(0, 999)]); // Keep max 1000 notifications

      // Show toast notification
      if (preferences.channels.toast) {
        showToastNotification(newNotification);
      }

      // Play sound
      if (preferences.sound && audioRef.current) {
        audioRef.current.play().catch(() => {
          // Ignore audio play errors
        });
      }

      // Vibrate (if supported)
      if (preferences.vibration && 'vibrate' in navigator) {
        navigator.vibrate(200);
      }

      // Set expiration timeout
      if (newNotification.expiresAt) {
        const timeout = setTimeout(() => {
          removeNotification(newNotification.id);
        }, newNotification.expiresAt - Date.now());
        notificationTimeoutRefs.current.set(newNotification.id, timeout);
      }
    }
  }, [preferences, shouldShowNotification, showToastNotification]);

  const markAsRead = useCallback((id: string) => {
    setNotifications(prev => 
      prev.map(n => n.id === id ? { ...n, read: true } : n)
    );
  }, []);

  const markAllAsRead = useCallback(() => {
    setNotifications(prev => prev.map(n => ({ ...n, read: true })));
  }, []);

  const removeNotification = useCallback((id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id));
    
    // Clear timeout if exists
    const timeout = notificationTimeoutRefs.current.get(id);
    if (timeout) {
      clearTimeout(timeout);
      notificationTimeoutRefs.current.delete(id);
    }
  }, []);

  const clearAll = useCallback(() => {
    setNotifications([]);
    
    // Clear all timeouts
    notificationTimeoutRefs.current.forEach(timeout => clearTimeout(timeout));
    notificationTimeoutRefs.current.clear();
  }, []);

  const updatePreferences = useCallback((newPreferences: Partial<NotificationPreferences>) => {
    setPreferences(prev => ({ ...prev, ...newPreferences }));
  }, []);

  const getFilteredNotifications = useCallback((filters?: NotificationFilters) => {
    let filtered = notifications;

    if (filters?.type) {
      filtered = filtered.filter(n => n.type === filters.type);
    }

    if (filters?.category) {
      filtered = filtered.filter(n => n.category === filters.category);
    }

    if (filters?.priority) {
      filtered = filtered.filter(n => n.priority === filters.priority);
    }

    if (filters?.read !== undefined) {
      filtered = filtered.filter(n => n.read === filters.read);
    }

    if (filters?.search) {
      const search = filters.search.toLowerCase();
      filtered = filtered.filter(n => 
        n.title.toLowerCase().includes(search) || 
        n.message.toLowerCase().includes(search)
      );
    }

    if (filters?.dateRange) {
      filtered = filtered.filter(n => 
        n.timestamp >= filters.dateRange!.start.getTime() && 
        n.timestamp <= filters.dateRange!.end.getTime()
      );
    }

    return filtered;
  }, [notifications]);

  const unreadCount = notifications.filter(n => !n.read).length;

  const value: NotificationContextType = {
    notifications,
    unreadCount,
    preferences,
    addNotification,
    markAsRead,
    markAllAsRead,
    removeNotification,
    clearAll,
    updatePreferences,
    isQuietHours,
    getFilteredNotifications,
  };

  return (
    <NotificationContext.Provider value={value}>
      {children}
      <NotificationToast />
      <NotificationCenter />
    </NotificationContext.Provider>
  );
};

// Toast Notification Component
const NotificationToast: React.FC = () => {
  const [toasts, setToasts] = useState<Notification[]>([]);
  const { addNotification } = useNotifications();

  useEffect(() => {
    const handleNewNotification = (notification: Notification) => {
      setToasts(prev => [notification, ...prev.slice(0, 4)]); // Max 5 toasts
      
      // Auto-remove toast after 5 seconds
      setTimeout(() => {
        setToasts(prev => prev.filter(t => t.id !== notification.id));
      }, 5000);
    };

    // This would be connected to the notification system
    // For now, we'll simulate it
    return () => {
      // Cleanup
    };
  }, [addNotification]);

  const removeToast = (id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  };

  const getToastIcon = (type: Notification['type']) => {
    switch (type) {
      case 'success':
        return <CheckCircle className="w-5 h-5 text-green-500" />;
      case 'error':
        return <AlertCircle className="w-5 h-5 text-red-500" />;
      case 'warning':
        return <AlertTriangle className="w-5 h-5 text-yellow-500" />;
      case 'info':
        return <Info className="w-5 h-5 text-blue-500" />;
      default:
        return <Bell className="w-5 h-5 text-gray-500" />;
    }
  };

  const getToastColor = (type: Notification['type']) => {
    switch (type) {
      case 'success':
        return 'border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20';
      case 'error':
        return 'border-red-200 bg-red-50 dark:border-red-800 dark:bg-red-900/20';
      case 'warning':
        return 'border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-900/20';
      case 'info':
        return 'border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-900/20';
      default:
        return 'border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900/20';
    }
  };

  return (
    <div className="fixed top-4 right-4 z-50 space-y-2">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`flex items-start p-4 border rounded-lg shadow-lg max-w-sm transition-all duration-300 ${getToastColor(toast.type)}`}
        >
          <div className="flex-shrink-0 mr-3">
            {getToastIcon(toast.type)}
          </div>
          <div className="flex-1 min-w-0">
            <h4 className="text-sm font-medium text-gray-900 dark:text-white">
              {toast.title}
            </h4>
            <p className="text-sm text-gray-600 dark:text-gray-300 mt-1">
              {toast.message}
            </p>
            {toast.actions && toast.actions.length > 0 && (
              <div className="flex space-x-2 mt-2">
                {toast.actions.map((action, index) => (
                  <button
                    key={index}
                    onClick={() => {
                      action.action();
                      removeToast(toast.id);
                    }}
                    className={`text-xs px-2 py-1 rounded ${
                      action.variant === 'primary'
                        ? 'bg-blue-600 text-white hover:bg-blue-700'
                        : action.variant === 'danger'
                        ? 'bg-red-600 text-white hover:bg-red-700'
                        : 'bg-gray-200 text-gray-800 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200'
                    }`}
                  >
                    {action.label}
                  </button>
                ))}
              </div>
            )}
          </div>
          <button
            onClick={() => removeToast(toast.id)}
            className="flex-shrink-0 ml-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      ))}
    </div>
  );
};

// Notification Center Component
export const NotificationCenter: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [activeTab, setActiveTab] = useState<'all' | 'unread' | 'trading' | 'system'>('all');
  const [searchTerm, setSearchTerm] = useState('');
  const { 
    notifications, 
    unreadCount, 
    markAsRead, 
    markAllAsRead, 
    removeNotification, 
    clearAll,
    getFilteredNotifications 
  } = useNotifications();

  const filteredNotifications = getFilteredNotifications({
    search: searchTerm,
    read: activeTab === 'unread' ? false : undefined,
    category: activeTab === 'trading' ? 'trading' : activeTab === 'system' ? 'system' : undefined,
  });

  const handleNotificationClick = (notification: Notification) => {
    if (!notification.read) {
      markAsRead(notification.id);
    }
    // Handle notification action
  };

  const getNotificationIcon = (type: Notification['type']) => {
    switch (type) {
      case 'success':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'error':
        return <AlertCircle className="w-4 h-4 text-red-500" />;
      case 'warning':
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
      case 'info':
        return <Info className="w-4 h-4 text-blue-500" />;
      case 'trade':
        return <Bell className="w-4 h-4 text-purple-500" />;
      case 'price':
        return <Bell className="w-4 h-4 text-orange-500" />;
      case 'order':
        return <Bell className="w-4 h-4 text-indigo-500" />;
      default:
        return <Bell className="w-4 h-4 text-gray-500" />;
    }
  };

  const formatTimestamp = (timestamp: number) => {
    const now = Date.now();
    const diff = now - timestamp;
    
    if (diff < 60000) return 'Just now';
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    return new Date(timestamp).toLocaleDateString();
  };

  return (
    <>
      {/* Notification Bell */}
      <button
        onClick={() => setIsOpen(true)}
        className="relative p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
      >
        <Bell className="w-5 h-5" />
        {unreadCount > 0 && (
          <span className="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-5 w-5 flex items-center justify-center">
            {unreadCount > 99 ? '99+' : unreadCount}
          </span>
        )}
      </button>

      {/* Notification Center Modal */}
      {isOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-start justify-end z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-96 max-h-screen overflow-hidden">
            {/* Header */}
            <div className="p-4 border-b border-gray-200 dark:border-gray-700">
              <div className="flex items-center justify-between">
                <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
                  Notifications
                </h2>
                <div className="flex items-center space-x-2">
                  <button
                    onClick={markAllAsRead}
                    className="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400"
                  >
                    Mark all read
                  </button>
                  <button
                    onClick={() => setIsOpen(false)}
                    className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  >
                    <X className="w-5 h-5" />
                  </button>
                </div>
              </div>

              {/* Search */}
              <div className="mt-3 relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                <input
                  type="text"
                  placeholder="Search notifications..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                />
              </div>

              {/* Tabs */}
              <div className="mt-3 flex space-x-1">
                {[
                  { key: 'all', label: 'All' },
                  { key: 'unread', label: 'Unread' },
                  { key: 'trading', label: 'Trading' },
                  { key: 'system', label: 'System' },
                ].map((tab) => (
                  <button
                    key={tab.key}
                    onClick={() => setActiveTab(tab.key as any)}
                    className={`px-3 py-1 text-sm rounded-md transition-colors ${
                      activeTab === tab.key
                        ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/20 dark:text-blue-400'
                        : 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200'
                    }`}
                  >
                    {tab.label}
                  </button>
                ))}
              </div>
            </div>

            {/* Notifications List */}
            <div className="overflow-y-auto max-h-96">
              {filteredNotifications.length === 0 ? (
                <div className="p-8 text-center text-gray-500 dark:text-gray-400">
                  <Bell className="w-12 h-12 mx-auto mb-4 text-gray-300" />
                  <p>No notifications</p>
                </div>
              ) : (
                <div className="divide-y divide-gray-200 dark:divide-gray-700">
                  {filteredNotifications.map((notification) => (
                    <div
                      key={notification.id}
                      onClick={() => handleNotificationClick(notification)}
                      className={`p-4 cursor-pointer transition-colors ${
                        notification.read
                          ? 'bg-white dark:bg-gray-800'
                          : 'bg-blue-50 dark:bg-blue-900/20'
                      } hover:bg-gray-50 dark:hover:bg-gray-700`}
                    >
                      <div className="flex items-start space-x-3">
                        <div className="flex-shrink-0">
                          {getNotificationIcon(notification.type)}
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center justify-between">
                            <h4 className={`text-sm font-medium ${
                              notification.read
                                ? 'text-gray-900 dark:text-white'
                                : 'text-gray-900 dark:text-white font-semibold'
                            }`}>
                              {notification.title}
                            </h4>
                            <div className="flex items-center space-x-2">
                              <span className="text-xs text-gray-500 dark:text-gray-400">
                                {formatTimestamp(notification.timestamp)}
                              </span>
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  removeNotification(notification.id);
                                }}
                                className="text-gray-400 hover:text-red-500"
                              >
                                <Trash2 className="w-3 h-3" />
                              </button>
                            </div>
                          </div>
                          <p className="text-sm text-gray-600 dark:text-gray-300 mt-1">
                            {notification.message}
                          </p>
                          {notification.actions && notification.actions.length > 0 && (
                            <div className="flex space-x-2 mt-2">
                              {notification.actions.map((action, index) => (
                                <button
                                  key={index}
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    action.action();
                                  }}
                                  className={`text-xs px-2 py-1 rounded ${
                                    action.variant === 'primary'
                                      ? 'bg-blue-600 text-white hover:bg-blue-700'
                                      : action.variant === 'danger'
                                      ? 'bg-red-600 text-white hover:bg-red-700'
                                      : 'bg-gray-200 text-gray-800 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200'
                                  }`}
                                >
                                  {action.label}
                                </button>
                              ))}
                            </div>
                          )}
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* Footer */}
            <div className="p-4 border-t border-gray-200 dark:border-gray-700">
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  {filteredNotifications.length} notification{filteredNotifications.length !== 1 ? 's' : ''}
                </span>
                <button
                  onClick={clearAll}
                  className="text-sm text-red-600 hover:text-red-700 dark:text-red-400"
                >
                  Clear all
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

// Notification Preferences Component
export const NotificationPreferences: React.FC = () => {
  const { preferences, updatePreferences } = useNotifications();
  const [isOpen, setIsOpen] = useState(false);

  const handleToggle = (key: keyof NotificationPreferences['types'] | keyof NotificationPreferences['channels'] | keyof NotificationPreferences['priority'] | keyof NotificationPreferences['categories']) => {
    if (key in preferences.types) {
      updatePreferences({
        types: { ...preferences.types, [key]: !preferences.types[key as keyof NotificationPreferences['types']] }
      });
    } else if (key in preferences.channels) {
      updatePreferences({
        channels: { ...preferences.channels, [key]: !preferences.channels[key as keyof NotificationPreferences['channels']] }
      });
    } else if (key in preferences.priority) {
      updatePreferences({
        priority: { ...preferences.priority, [key]: !preferences.priority[key as keyof NotificationPreferences['priority']] }
      });
    } else if (key in preferences.categories) {
      updatePreferences({
        categories: { ...preferences.categories, [key]: !preferences.categories[key as keyof NotificationPreferences['categories']] }
      });
    }
  };

  return (
    <>
      <button
        onClick={() => setIsOpen(true)}
        className="flex items-center space-x-2 px-3 py-2 text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200"
      >
        <Settings className="w-4 h-4" />
        <span>Notification Settings</span>
      </button>

      {isOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md mx-4 max-h-[90vh] overflow-y-auto">
            <div className="p-6">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
                  Notification Preferences
                </h2>
                <button
                  onClick={() => setIsOpen(false)}
                  className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>

              {/* General Settings */}
              <div className="mb-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-3">
                  General
                </h3>
                <div className="space-y-3">
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      checked={preferences.enabled}
                      onChange={(e) => updatePreferences({ enabled: e.target.checked })}
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="ml-2 text-gray-700 dark:text-gray-300">Enable notifications</span>
                  </label>
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      checked={preferences.sound}
                      onChange={(e) => updatePreferences({ sound: e.target.checked })}
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="ml-2 text-gray-700 dark:text-gray-300">Play sound</span>
                  </label>
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      checked={preferences.vibration}
                      onChange={(e) => updatePreferences({ vibration: e.target.checked })}
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="ml-2 text-gray-700 dark:text-gray-300">Vibrate</span>
                  </label>
                </div>
              </div>

              {/* Notification Types */}
              <div className="mb-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-3">
                  Notification Types
                </h3>
                <div className="space-y-2">
                  {Object.entries(preferences.types).map(([key, value]) => (
                    <label key={key} className="flex items-center">
                      <input
                        type="checkbox"
                        checked={value}
                        onChange={() => handleToggle(key as keyof NotificationPreferences['types'])}
                        className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      />
                      <span className="ml-2 text-gray-700 dark:text-gray-300 capitalize">
                        {key}
                      </span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Channels */}
              <div className="mb-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-3">
                  Notification Channels
                </h3>
                <div className="space-y-2">
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      checked={preferences.channels.toast}
                      onChange={() => handleToggle('toast')}
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="ml-2 text-gray-700 dark:text-gray-300">Toast notifications</span>
                  </label>
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      checked={preferences.channels.push}
                      onChange={() => handleToggle('push')}
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="ml-2 text-gray-700 dark:text-gray-300">Push notifications</span>
                  </label>
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      checked={preferences.channels.email}
                      onChange={() => handleToggle('email')}
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="ml-2 text-gray-700 dark:text-gray-300">Email notifications</span>
                  </label>
                </div>
              </div>

              {/* Priority Levels */}
              <div className="mb-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-3">
                  Priority Levels
                </h3>
                <div className="space-y-2">
                  {Object.entries(preferences.priority).map(([key, value]) => (
                    <label key={key} className="flex items-center">
                      <input
                        type="checkbox"
                        checked={value}
                        onChange={() => handleToggle(key as keyof NotificationPreferences['priority'])}
                        className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      />
                      <span className="ml-2 text-gray-700 dark:text-gray-300 capitalize">
                        {key}
                      </span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Categories */}
              <div className="mb-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-3">
                  Categories
                </h3>
                <div className="space-y-2">
                  {Object.entries(preferences.categories).map(([key, value]) => (
                    <label key={key} className="flex items-center">
                      <input
                        type="checkbox"
                        checked={value}
                        onChange={() => handleToggle(key as keyof NotificationPreferences['categories'])}
                        className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      />
                      <span className="ml-2 text-gray-700 dark:text-gray-300 capitalize">
                        {key}
                      </span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Quiet Hours */}
              <div className="mb-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-3">
                  Quiet Hours
                </h3>
                <div className="space-y-3">
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      checked={preferences.quietHours.enabled}
                      onChange={(e) => updatePreferences({
                        quietHours: { ...preferences.quietHours, enabled: e.target.checked }
                      })}
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="ml-2 text-gray-700 dark:text-gray-300">Enable quiet hours</span>
                  </label>
                  {preferences.quietHours.enabled && (
                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="block text-sm text-gray-700 dark:text-gray-300 mb-1">
                          Start Time
                        </label>
                        <input
                          type="time"
                          value={preferences.quietHours.start}
                          onChange={(e) => updatePreferences({
                            quietHours: { ...preferences.quietHours, start: e.target.value }
                          })}
                          className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                        />
                      </div>
                      <div>
                        <label className="block text-sm text-gray-700 dark:text-gray-300 mb-1">
                          End Time
                        </label>
                        <input
                          type="time"
                          value={preferences.quietHours.end}
                          onChange={(e) => updatePreferences({
                            quietHours: { ...preferences.quietHours, end: e.target.value }
                          })}
                          className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                        />
                      </div>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

// Utility functions for creating notifications
export const createNotification = {
  success: (title: string, message: string, options?: Partial<Notification>) => ({
    type: 'success' as const,
    title,
    message,
    priority: 'medium' as const,
    category: 'system' as const,
    ...options,
  }),
  error: (title: string, message: string, options?: Partial<Notification>) => ({
    type: 'error' as const,
    title,
    message,
    priority: 'high' as const,
    category: 'system' as const,
    ...options,
  }),
  warning: (title: string, message: string, options?: Partial<Notification>) => ({
    type: 'warning' as const,
    title,
    message,
    priority: 'medium' as const,
    category: 'system' as const,
    ...options,
  }),
  info: (title: string, message: string, options?: Partial<Notification>) => ({
    type: 'info' as const,
    title,
    message,
    priority: 'low' as const,
    category: 'system' as const,
    ...options,
  }),
  trade: (title: string, message: string, options?: Partial<Notification>) => ({
    type: 'trade' as const,
    title,
    message,
    priority: 'medium' as const,
    category: 'trading' as const,
    ...options,
  }),
  price: (title: string, message: string, options?: Partial<Notification>) => ({
    type: 'price' as const,
    title,
    message,
    priority: 'medium' as const,
    category: 'price' as const,
    ...options,
  }),
  order: (title: string, message: string, options?: Partial<Notification>) => ({
    type: 'order' as const,
    title,
    message,
    priority: 'high' as const,
    category: 'order' as const,
    ...options,
  }),
}; 