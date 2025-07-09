import React, { useState } from 'react';
import { 
  Settings, 
  Eye, 
  Type, 
  Palette, 
  Volume2, 
  X,
  Check,
  Monitor,
  Smartphone
} from 'lucide-react';
import { useAccessibility } from './AccessibilityProvider';

interface AccessibilitySettingsProps {
  isOpen: boolean;
  onClose: () => void;
}

export const AccessibilitySettings: React.FC<AccessibilitySettingsProps> = ({
  isOpen,
  onClose,
}) => {
  const {
    isHighContrast,
    setHighContrast,
    isReducedMotion,
    setReducedMotion,
    fontSize,
    setFontSize,
    colorBlindMode,
    setColorBlindMode,
  } = useAccessibility();

  const [activeTab, setActiveTab] = useState<'visual' | 'motion' | 'text' | 'color'>('visual');

  if (!isOpen) return null;

  const tabs = [
    { id: 'visual', label: 'Visual', icon: Eye },
    { id: 'motion', label: 'Motion', icon: Volume2 },
    { id: 'text', label: 'Text', icon: Type },
    { id: 'color', label: 'Color', icon: Palette },
  ] as const;

  const fontSizes = [
    { value: 'small', label: 'Small', description: 'Default size' },
    { value: 'medium', label: 'Medium', description: '20% larger' },
    { value: 'large', label: 'Large', description: '40% larger' },
  ] as const;

  const colorBlindModes = [
    { value: 'none', label: 'None', description: 'Standard colors' },
    { value: 'protanopia', label: 'Protanopia', description: 'Red-green color blindness' },
    { value: 'deuteranopia', label: 'Deuteranopia', description: 'Green-red color blindness' },
    { value: 'tritanopia', label: 'Tritanopia', description: 'Blue-yellow color blindness' },
  ] as const;

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        {/* Backdrop */}
        <div
          className="fixed inset-0 bg-black bg-opacity-50 transition-opacity"
          onClick={onClose}
        />

        {/* Modal */}
        <div className="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-2xl sm:w-full">
          {/* Header */}
          <div className="bg-gray-50 px-6 py-4 border-b border-gray-200">
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <Settings className="w-6 h-6 text-gray-600 mr-3" />
                <h3 className="text-lg font-medium text-gray-900">
                  Accessibility Settings
                </h3>
              </div>
              <button
                onClick={onClose}
                className="text-gray-400 hover:text-gray-600 transition-colors"
                aria-label="Close accessibility settings"
              >
                <X className="w-6 h-6" />
              </button>
            </div>
          </div>

          {/* Content */}
          <div className="px-6 py-4">
            {/* Tabs */}
            <div className="border-b border-gray-200 mb-6">
              <nav className="-mb-px flex space-x-8">
                {tabs.map((tab) => {
                  const Icon = tab.icon;
                  const isActive = activeTab === tab.id;
                  return (
                    <button
                      key={tab.id}
                      onClick={() => setActiveTab(tab.id)}
                      className={`flex items-center py-2 px-1 border-b-2 font-medium text-sm transition-colors ${
                        isActive
                          ? 'border-primary-500 text-primary-600'
                          : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                      }`}
                    >
                      <Icon className="w-4 h-4 mr-2" />
                      {tab.label}
                    </button>
                  );
                })}
              </nav>
            </div>

            {/* Tab Content */}
            <div className="space-y-6">
              {activeTab === 'visual' && (
                <div>
                  <h4 className="text-md font-medium text-gray-900 mb-4">Visual Preferences</h4>
                  
                  {/* High Contrast Mode */}
                  <div className="flex items-center justify-between py-3 border-b border-gray-200">
                    <div>
                      <h5 className="text-sm font-medium text-gray-900">High Contrast Mode</h5>
                      <p className="text-sm text-gray-500">Increase contrast for better visibility</p>
                    </div>
                    <button
                      onClick={() => setHighContrast(!isHighContrast)}
                      className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                        isHighContrast ? 'bg-primary-600' : 'bg-gray-200'
                      }`}
                      role="switch"
                      aria-checked={isHighContrast}
                      aria-label="Toggle high contrast mode"
                    >
                      <span
                        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                          isHighContrast ? 'translate-x-6' : 'translate-x-1'
                        }`}
                      />
                    </button>
                  </div>

                  {/* Font Size */}
                  <div className="py-3">
                    <h5 className="text-sm font-medium text-gray-900 mb-3">Font Size</h5>
                    <div className="space-y-2">
                      {fontSizes.map((size) => (
                        <label
                          key={size.value}
                          className="flex items-center p-3 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors"
                        >
                          <input
                            type="radio"
                            name="fontSize"
                            value={size.value}
                            checked={fontSize === size.value}
                            onChange={(e) => setFontSize(e.target.value as any)}
                            className="sr-only"
                          />
                          <div className={`w-4 h-4 border-2 rounded-full mr-3 flex items-center justify-center ${
                            fontSize === size.value
                              ? 'border-primary-500 bg-primary-500'
                              : 'border-gray-300'
                          }`}>
                            {fontSize === size.value && (
                              <Check className="w-2 h-2 text-white" />
                            )}
                          </div>
                          <div>
                            <div className="text-sm font-medium text-gray-900">{size.label}</div>
                            <div className="text-xs text-gray-500">{size.description}</div>
                          </div>
                        </label>
                      ))}
                    </div>
                  </div>
                </div>
              )}

              {activeTab === 'motion' && (
                <div>
                  <h4 className="text-md font-medium text-gray-900 mb-4">Motion Preferences</h4>
                  
                  {/* Reduced Motion */}
                  <div className="flex items-center justify-between py-3 border-b border-gray-200">
                    <div>
                      <h5 className="text-sm font-medium text-gray-900">Reduced Motion</h5>
                      <p className="text-sm text-gray-500">Minimize animations and transitions</p>
                    </div>
                    <button
                      onClick={() => setReducedMotion(!isReducedMotion)}
                      className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                        isReducedMotion ? 'bg-primary-600' : 'bg-gray-200'
                      }`}
                      role="switch"
                      aria-checked={isReducedMotion}
                      aria-label="Toggle reduced motion"
                    >
                      <span
                        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                          isReducedMotion ? 'translate-x-6' : 'translate-x-1'
                        }`}
                      />
                    </button>
                  </div>

                  <div className="mt-4 p-4 bg-blue-50 rounded-lg">
                    <div className="flex">
                      <Monitor className="w-5 h-5 text-blue-600 mr-2 mt-0.5" />
                      <div>
                        <h6 className="text-sm font-medium text-blue-900">Motion Sensitivity</h6>
                        <p className="text-sm text-blue-700 mt-1">
                          These settings help users who are sensitive to motion or have vestibular disorders.
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              )}

              {activeTab === 'text' && (
                <div>
                  <h4 className="text-md font-medium text-gray-900 mb-4">Text Preferences</h4>
                  
                  <div className="space-y-4">
                    {/* Font Size Preview */}
                    <div>
                      <h5 className="text-sm font-medium text-gray-900 mb-3">Font Size Preview</h5>
                      <div className="p-4 border border-gray-200 rounded-lg bg-gray-50">
                        <p className="text-sm text-gray-600 mb-2">Sample text with current font size:</p>
                        <p className="text-gray-900">
                          This is how text will appear with your selected font size. 
                          The FinDAG interface will use this size throughout the application.
                        </p>
                      </div>
                    </div>

                    {/* Reading Preferences */}
                    <div className="p-4 bg-yellow-50 rounded-lg">
                      <div className="flex">
                        <Type className="w-5 h-5 text-yellow-600 mr-2 mt-0.5" />
                        <div>
                          <h6 className="text-sm font-medium text-yellow-900">Reading Assistance</h6>
                          <p className="text-sm text-yellow-700 mt-1">
                            Larger text sizes can help users with visual impairments or reading difficulties.
                          </p>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              )}

              {activeTab === 'color' && (
                <div>
                  <h4 className="text-md font-medium text-gray-900 mb-4">Color Preferences</h4>
                  
                  {/* Color Blind Support */}
                  <div className="mb-4">
                    <h5 className="text-sm font-medium text-gray-900 mb-3">Color Blind Support</h5>
                    <div className="space-y-2">
                      {colorBlindModes.map((mode) => (
                        <label
                          key={mode.value}
                          className="flex items-center p-3 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors"
                        >
                          <input
                            type="radio"
                            name="colorBlindMode"
                            value={mode.value}
                            checked={colorBlindMode === mode.value}
                            onChange={(e) => setColorBlindMode(e.target.value as any)}
                            className="sr-only"
                          />
                          <div className={`w-4 h-4 border-2 rounded-full mr-3 flex items-center justify-center ${
                            colorBlindMode === mode.value
                              ? 'border-primary-500 bg-primary-500'
                              : 'border-gray-300'
                          }`}>
                            {colorBlindMode === mode.value && (
                              <Check className="w-2 h-2 text-white" />
                            )}
                          </div>
                          <div>
                            <div className="text-sm font-medium text-gray-900">{mode.label}</div>
                            <div className="text-xs text-gray-500">{mode.description}</div>
                          </div>
                        </label>
                      ))}
                    </div>
                  </div>

                  {/* Color Preview */}
                  <div className="p-4 bg-gray-50 rounded-lg">
                    <h6 className="text-sm font-medium text-gray-900 mb-3">Color Preview</h6>
                    <div className="grid grid-cols-4 gap-2">
                      <div className="w-8 h-8 bg-primary-500 rounded" title="Primary"></div>
                      <div className="w-8 h-8 bg-success-500 rounded" title="Success"></div>
                      <div className="w-8 h-8 bg-warning-500 rounded" title="Warning"></div>
                      <div className="w-8 h-8 bg-danger-500 rounded" title="Danger"></div>
                    </div>
                    <p className="text-xs text-gray-500 mt-2">
                      These colors will be adjusted based on your color blind mode selection.
                    </p>
                  </div>
                </div>
              )}
            </div>
          </div>

          {/* Footer */}
          <div className="bg-gray-50 px-6 py-3 border-t border-gray-200">
            <div className="flex justify-between items-center">
              <p className="text-sm text-gray-500">
                Settings are saved automatically
              </p>
              <button
                onClick={onClose}
                className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
              >
                Done
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

// Accessibility quick toggle component
export const AccessibilityQuickToggle: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const { isHighContrast, isReducedMotion } = useAccessibility();

  const hasActiveSettings = isHighContrast || isReducedMotion;

  return (
    <>
      <button
        onClick={() => setIsOpen(true)}
        className={`p-2 rounded-lg transition-colors ${
          hasActiveSettings
            ? 'bg-primary-100 text-primary-600 hover:bg-primary-200'
            : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
        }`}
        aria-label="Open accessibility settings"
        title="Accessibility Settings"
      >
        <Settings className="w-5 h-5" />
        {hasActiveSettings && (
          <div className="absolute -top-1 -right-1 w-3 h-3 bg-primary-500 rounded-full"></div>
        )}
      </button>

      <AccessibilitySettings isOpen={isOpen} onClose={() => setIsOpen(false)} />
    </>
  );
}; 