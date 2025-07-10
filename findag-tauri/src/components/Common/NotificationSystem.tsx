import React, { useState, useEffect } from 'react';

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  duration?: number;
}

interface NotificationSystemProps {
  children?: React.ReactNode;
}

const NotificationSystem: React.FC<NotificationSystemProps> = ({ children }) => {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  useEffect(() => {
    // Global notification handler
    const handleNotification = (event: CustomEvent<Notification>) => {
      const notification = {
        ...event.detail,
        id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
        duration: event.detail.duration || 5000,
      };
      
      setNotifications(prev => [...prev, notification]);
      
      // Auto-remove notification
      setTimeout(() => {
        setNotifications(prev => prev.filter(n => n.id !== notification.id));
      }, notification.duration);
    };

    window.addEventListener('show-notification', handleNotification as EventListener);
    
    return () => {
      window.removeEventListener('show-notification', handleNotification as EventListener);
    };
  }, []);

  const removeNotification = (id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id));
  };

  const getNotificationStyles = (type: Notification['type']) => {
    const baseStyles = 'p-4 rounded-lg shadow-lg border-l-4 max-w-sm';
    const typeStyles = {
      success: 'bg-green-50 border-green-400 text-green-800 dark:bg-green-900 dark:border-green-500 dark:text-green-200',
      error: 'bg-red-50 border-red-400 text-red-800 dark:bg-red-900 dark:border-red-500 dark:text-red-200',
      warning: 'bg-yellow-50 border-yellow-400 text-yellow-800 dark:bg-yellow-900 dark:border-yellow-500 dark:text-yellow-200',
      info: 'bg-blue-50 border-blue-400 text-blue-800 dark:bg-blue-900 dark:border-blue-500 dark:text-blue-200',
    };
    
    return `${baseStyles} ${typeStyles[type]}`;
  };

  const getIcon = (type: Notification['type']) => {
    const icons = {
      success: '✅',
      error: '❌',
      warning: '⚠️',
      info: 'ℹ️',
    };
    return icons[type];
  };

  return (
    <>
      {children}
      
      {/* Notification Container */}
      <div className="fixed top-4 right-4 z-50 space-y-2">
        {notifications.map((notification) => (
          <div
            key={notification.id}
            className={`${getNotificationStyles(notification.type)} transform transition-all duration-300 ease-in-out`}
          >
            <div className="flex items-start space-x-3">
              <span className="text-lg">{getIcon(notification.type)}</span>
              <div className="flex-1">
                <h4 className="font-medium">{notification.title}</h4>
                <p className="text-sm mt-1">{notification.message}</p>
              </div>
              <button
                onClick={() => removeNotification(notification.id)}
                className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors duration-200"
              >
                ✕
              </button>
            </div>
          </div>
        ))}
      </div>
    </>
  );
};

// Utility function to show notifications
export const showNotification = (notification: Omit<Notification, 'id'>) => {
  const event = new CustomEvent('show-notification', { detail: notification });
  window.dispatchEvent(event);
};

export default NotificationSystem; 