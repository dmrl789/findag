import React, { useState, useEffect, useCallback } from 'react';
import { Settings, Globe, Palette, Monitor, Bell, Shield, User, Save, RotateCcw } from 'lucide-react';
import { useTheme } from '../../store/theme';

export interface UserPreferences {
  // Display Settings
  theme: 'light' | 'dark' | 'system';
  fontSize: 'small' | 'medium' | 'large';
  density: 'compact' | 'comfortable' | 'spacious';
  animations: boolean;
  reducedMotion: boolean;
  
  // Language & Localization
  language: string;
  timezone: string;
  dateFormat: 'MM/DD/YYYY' | 'DD/MM/YYYY' | 'YYYY-MM-DD';
  timeFormat: '12h' | '24h';
  currency: string;
  numberFormat: 'US' | 'EU' | 'IN';
  
  // Trading Preferences
  defaultOrderType: 'market' | 'limit' | 'stop' | 'stop-limit';
  defaultQuantity: number;
  confirmOrders: boolean;
  showPnL: boolean;
  riskLevel: 'conservative' | 'moderate' | 'aggressive';
  
  // Notifications
  emailNotifications: boolean;
  pushNotifications: boolean;
  tradeAlerts: boolean;
  priceAlerts: boolean;
  systemAlerts: boolean;
  
  // Privacy & Security
  dataSharing: boolean;
  analytics: boolean;
  sessionTimeout: number;
  twoFactorAuth: boolean;
  
  // Performance
  autoRefresh: boolean;
  refreshInterval: number;
  cacheEnabled: boolean;
  virtualScrolling: boolean;
}

interface UserPreferencesProps {
  preferences: UserPreferences;
  onPreferencesChange: (preferences: UserPreferences) => void;
  onReset: () => void;
  className?: string;
}

export const UserPreferences: React.FC<UserPreferencesProps> = ({
  preferences,
  onPreferencesChange,
  onReset,
  className = ""
}) => {
  const [activeTab, setActiveTab] = useState<'display' | 'trading' | 'notifications' | 'privacy' | 'performance'>('display');
  const [localPreferences, setLocalPreferences] = useState<UserPreferences>(preferences);
  const [hasChanges, setHasChanges] = useState(false);
  const { setTheme } = useTheme();

  useEffect(() => {
    setLocalPreferences(preferences);
    setHasChanges(false);
  }, [preferences]);

  useEffect(() => {
    const hasUnsavedChanges = JSON.stringify(localPreferences) !== JSON.stringify(preferences);
    setHasChanges(hasUnsavedChanges);
  }, [localPreferences, preferences]);

  const updatePreference = useCallback((key: keyof UserPreferences, value: any) => {
    setLocalPreferences(prev => ({
      ...prev,
      [key]: value
    }));

    // Apply theme changes immediately
    if (key === 'theme') {
      setTheme(value);
    }
  }, [setTheme]);

  const handleSave = useCallback(() => {
    onPreferencesChange(localPreferences);
    setHasChanges(false);
  }, [localPreferences, onPreferencesChange]);

  const handleReset = useCallback(() => {
    setLocalPreferences(preferences);
    setHasChanges(false);
  }, [preferences]);

  const handleResetAll = useCallback(() => {
    onReset();
  }, [onReset]);

  // Available options
  const languages = [
    { code: 'en', name: 'English', flag: '🇺🇸' },
    { code: 'es', name: 'Español', flag: '🇪🇸' },
    { code: 'fr', name: 'Français', flag: '🇫🇷' },
    { code: 'de', name: 'Deutsch', flag: '🇩🇪' },
    { code: 'zh', name: '中文', flag: '🇨🇳' },
    { code: 'ja', name: '日本語', flag: '🇯🇵' },
    { code: 'ko', name: '한국어', flag: '🇰🇷' },
    { code: 'ar', name: 'العربية', flag: '🇸🇦' }
  ];

  const currencies = [
    { code: 'USD', name: 'US Dollar', symbol: '$' },
    { code: 'EUR', name: 'Euro', symbol: '€' },
    { code: 'GBP', name: 'British Pound', symbol: '£' },
    { code: 'JPY', name: 'Japanese Yen', symbol: '¥' },
    { code: 'CNY', name: 'Chinese Yuan', symbol: '¥' },
    { code: 'INR', name: 'Indian Rupee', symbol: '₹' },
    { code: 'BTC', name: 'Bitcoin', symbol: '₿' },
    { code: 'ETH', name: 'Ethereum', symbol: 'Ξ' }
  ];

  const timezones = [
    { code: 'UTC', name: 'UTC' },
    { code: 'America/New_York', name: 'Eastern Time' },
    { code: 'America/Chicago', name: 'Central Time' },
    { code: 'America/Denver', name: 'Mountain Time' },
    { code: 'America/Los_Angeles', name: 'Pacific Time' },
    { code: 'Europe/London', name: 'London' },
    { code: 'Europe/Paris', name: 'Paris' },
    { code: 'Asia/Tokyo', name: 'Tokyo' },
    { code: 'Asia/Shanghai', name: 'Shanghai' },
    { code: 'Asia/Dubai', name: 'Dubai' }
  ];

  const tabs = [
    { id: 'display', name: 'Display', icon: Monitor },
    { id: 'trading', name: 'Trading', icon: User },
    { id: 'notifications', name: 'Notifications', icon: Bell },
    { id: 'privacy', name: 'Privacy & Security', icon: Shield },
    { id: 'performance', name: 'Performance', icon: Settings }
  ];

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg ${className}`}>
      {/* Header */}
      <div className="border-b border-gray-200 dark:border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <Settings className="h-6 w-6 text-gray-600 dark:text-gray-400" />
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">User Preferences</h2>
          </div>
          <div className="flex items-center space-x-2">
            {hasChanges && (
              <span className="text-sm text-orange-600 dark:text-orange-400">
                Unsaved changes
              </span>
            )}
            <button
              onClick={handleResetAll}
              className="text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              Reset All
            </button>
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 dark:border-gray-700">
        <nav className="flex space-x-8 px-6">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2 ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              <tab.icon className="h-4 w-4" />
              <span>{tab.name}</span>
            </button>
          ))}
        </nav>
      </div>

      <div className="p-6">
        {/* Display Settings Tab */}
        {activeTab === 'display' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Display Settings</h3>
              
              {/* Theme */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Theme
                </label>
                <div className="grid grid-cols-3 gap-3">
                  {[
                    { id: 'light', name: 'Light', icon: '☀️' },
                    { id: 'dark', name: 'Dark', icon: '🌙' },
                    { id: 'system', name: 'System', icon: '🖥️' }
                  ].map((theme) => (
                    <button
                      key={theme.id}
                      onClick={() => updatePreference('theme', theme.id)}
                      className={`p-3 border rounded-lg text-center ${
                        localPreferences.theme === theme.id
                          ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                          : 'border-gray-300 dark:border-gray-600 hover:border-gray-400'
                      }`}
                    >
                      <div className="text-2xl mb-1">{theme.icon}</div>
                      <div className="text-sm font-medium">{theme.name}</div>
                    </button>
                  ))}
                </div>
              </div>

              {/* Font Size */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Font Size
                </label>
                <select
                  value={localPreferences.fontSize}
                  onChange={(e) => updatePreference('fontSize', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="small">Small</option>
                  <option value="medium">Medium</option>
                  <option value="large">Large</option>
                </select>
              </div>

              {/* Interface Density */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Interface Density
                </label>
                <select
                  value={localPreferences.density}
                  onChange={(e) => updatePreference('density', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="compact">Compact</option>
                  <option value="comfortable">Comfortable</option>
                  <option value="spacious">Spacious</option>
                </select>
              </div>

              {/* Animations */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Enable Animations
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Smooth transitions and animations
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.animations}
                    onChange={(e) => updatePreference('animations', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Reduced Motion */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Reduced Motion
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Reduce animations for accessibility
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.reducedMotion}
                    onChange={(e) => updatePreference('reducedMotion', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Language & Localization Tab */}
        {activeTab === 'display' && (
          <div className="space-y-6 mt-8">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Language & Localization</h3>
              
              {/* Language */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Language
                </label>
                <select
                  value={localPreferences.language}
                  onChange={(e) => updatePreference('language', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  {languages.map((lang) => (
                    <option key={lang.code} value={lang.code}>
                      {lang.flag} {lang.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* Timezone */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Timezone
                </label>
                <select
                  value={localPreferences.timezone}
                  onChange={(e) => updatePreference('timezone', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  {timezones.map((tz) => (
                    <option key={tz.code} value={tz.code}>
                      {tz.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* Date Format */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Date Format
                </label>
                <select
                  value={localPreferences.dateFormat}
                  onChange={(e) => updatePreference('dateFormat', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="MM/DD/YYYY">MM/DD/YYYY</option>
                  <option value="DD/MM/YYYY">DD/MM/YYYY</option>
                  <option value="YYYY-MM-DD">YYYY-MM-DD</option>
                </select>
              </div>

              {/* Time Format */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Time Format
                </label>
                <select
                  value={localPreferences.timeFormat}
                  onChange={(e) => updatePreference('timeFormat', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="12h">12-hour</option>
                  <option value="24h">24-hour</option>
                </select>
              </div>

              {/* Currency */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Currency
                </label>
                <select
                  value={localPreferences.currency}
                  onChange={(e) => updatePreference('currency', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  {currencies.map((currency) => (
                    <option key={currency.code} value={currency.code}>
                      {currency.symbol} {currency.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* Number Format */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Number Format
                </label>
                <select
                  value={localPreferences.numberFormat}
                  onChange={(e) => updatePreference('numberFormat', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="US">US (1,234.56)</option>
                  <option value="EU">European (1.234,56)</option>
                  <option value="IN">Indian (1,23,456.78)</option>
                </select>
              </div>
            </div>
          </div>
        )}

        {/* Trading Preferences Tab */}
        {activeTab === 'trading' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Trading Preferences</h3>
              
              {/* Default Order Type */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Default Order Type
                </label>
                <select
                  value={localPreferences.defaultOrderType}
                  onChange={(e) => updatePreference('defaultOrderType', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="market">Market</option>
                  <option value="limit">Limit</option>
                  <option value="stop">Stop</option>
                  <option value="stop-limit">Stop Limit</option>
                </select>
              </div>

              {/* Default Quantity */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Default Quantity
                </label>
                <input
                  type="number"
                  value={localPreferences.defaultQuantity}
                  onChange={(e) => updatePreference('defaultQuantity', parseFloat(e.target.value))}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                  min="0"
                  step="0.01"
                />
              </div>

              {/* Confirm Orders */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Confirm Orders
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Show confirmation dialog before placing orders
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.confirmOrders}
                    onChange={(e) => updatePreference('confirmOrders', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Show P&L */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Show P&L
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Display profit and loss information
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.showPnL}
                    onChange={(e) => updatePreference('showPnL', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Risk Level */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Risk Level
                </label>
                <select
                  value={localPreferences.riskLevel}
                  onChange={(e) => updatePreference('riskLevel', e.target.value)}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="conservative">Conservative</option>
                  <option value="moderate">Moderate</option>
                  <option value="aggressive">Aggressive</option>
                </select>
              </div>
            </div>
          </div>
        )}

        {/* Notifications Tab */}
        {activeTab === 'notifications' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Notification Settings</h3>
              
              {/* Email Notifications */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Email Notifications
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Receive notifications via email
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.emailNotifications}
                    onChange={(e) => updatePreference('emailNotifications', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Push Notifications */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Push Notifications
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Receive browser push notifications
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.pushNotifications}
                    onChange={(e) => updatePreference('pushNotifications', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Trade Alerts */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Trade Alerts
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Notify when orders are executed
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.tradeAlerts}
                    onChange={(e) => updatePreference('tradeAlerts', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Price Alerts */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Price Alerts
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Notify when price targets are reached
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.priceAlerts}
                    onChange={(e) => updatePreference('priceAlerts', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* System Alerts */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      System Alerts
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Receive system and maintenance notifications
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.systemAlerts}
                    onChange={(e) => updatePreference('systemAlerts', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Privacy & Security Tab */}
        {activeTab === 'privacy' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Privacy & Security</h3>
              
              {/* Data Sharing */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Data Sharing
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Share anonymous usage data to improve the platform
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.dataSharing}
                    onChange={(e) => updatePreference('dataSharing', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Analytics */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Analytics
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Allow analytics tracking
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.analytics}
                    onChange={(e) => updatePreference('analytics', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Session Timeout */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Session Timeout (minutes)
                </label>
                <input
                  type="number"
                  value={localPreferences.sessionTimeout}
                  onChange={(e) => updatePreference('sessionTimeout', parseInt(e.target.value))}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                  min="5"
                  max="1440"
                />
              </div>

              {/* Two-Factor Authentication */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Two-Factor Authentication
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Enable 2FA for enhanced security
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.twoFactorAuth}
                    onChange={(e) => updatePreference('twoFactorAuth', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Performance Tab */}
        {activeTab === 'performance' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Performance Settings</h3>
              
              {/* Auto Refresh */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Auto Refresh
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Automatically refresh data
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.autoRefresh}
                    onChange={(e) => updatePreference('autoRefresh', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Refresh Interval */}
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Refresh Interval (seconds)
                </label>
                <input
                  type="number"
                  value={localPreferences.refreshInterval}
                  onChange={(e) => updatePreference('refreshInterval', parseInt(e.target.value))}
                  className="w-full border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                  min="5"
                  max="300"
                />
              </div>

              {/* Cache Enabled */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Enable Caching
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Cache data for better performance
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.cacheEnabled}
                    onChange={(e) => updatePreference('cacheEnabled', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>

              {/* Virtual Scrolling */}
              <div className="mb-6">
                <div className="flex items-center justify-between">
                  <div>
                    <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Virtual Scrolling
                    </label>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Use virtual scrolling for large datasets
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={localPreferences.virtualScrolling}
                    onChange={(e) => updatePreference('virtualScrolling', e.target.checked)}
                    className="rounded"
                  />
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Action Buttons */}
        <div className="flex items-center justify-between pt-6 border-t border-gray-200 dark:border-gray-700">
          <button
            onClick={handleReset}
            className="flex items-center space-x-2 px-4 py-2 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
          >
            <RotateCcw className="h-4 w-4" />
            <span>Reset</span>
          </button>
          
          <div className="flex items-center space-x-3">
            <button
              onClick={handleSave}
              disabled={!hasChanges}
              className="flex items-center space-x-2 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <Save className="h-4 w-4" />
              <span>Save Changes</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}; 