import React, { useState } from 'react';
import { Grid, Square, Columns, Rows, ChevronDown } from 'lucide-react';
import { LayoutType, LAYOUT_PRESETS, useDashboardStore } from '../../store/dashboard';

interface LayoutSelectorProps {
  className?: string;
}

export const LayoutSelector: React.FC<LayoutSelectorProps> = ({ className = '' }) => {
  const [isOpen, setIsOpen] = useState(false);
  const { layout, setLayout } = useDashboardStore();

  const currentLayout = Object.entries(LAYOUT_PRESETS).find(
    ([key]) => key === layout
  ) || ['default', LAYOUT_PRESETS.default];

  const getLayoutIcon = (layoutKey: string) => {
    switch (layoutKey) {
      case 'default':
        return <Grid className="w-4 h-4" />;
      case 'compact':
        return <Square className="w-4 h-4" />;
      case 'wide':
        return <Columns className="w-4 h-4" />;
      case 'single':
        return <Rows className="w-4 h-4" />;
      default:
        return <Grid className="w-4 h-4" />;
    }
  };

  const handleLayoutChange = (layoutKey: string) => {
    setLayout(layoutKey as LayoutType);
    setIsOpen(false);
  };

  return (
    <div className={`relative ${className}`}>
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="inline-flex items-center justify-between w-full px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 transition-colors duration-200"
        aria-label="Select layout"
        aria-expanded={isOpen}
        aria-haspopup="listbox"
      >
        <div className="flex items-center">
          {getLayoutIcon(currentLayout[0])}
          <span className="ml-2">{currentLayout[1].name}</span>
        </div>
        <ChevronDown className={`w-4 h-4 ml-2 transition-transform duration-200 ${isOpen ? 'rotate-180' : ''}`} />
      </button>

      {isOpen && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 z-10"
            onClick={() => setIsOpen(false)}
          />
          
          {/* Dropdown */}
          <div className="absolute right-0 mt-1 w-56 bg-white border border-gray-200 rounded-lg shadow-lg z-20">
            <div className="py-1">
              <div className="px-3 py-2 text-xs font-medium text-gray-500 uppercase tracking-wider">
                Layout Presets
              </div>
              {Object.entries(LAYOUT_PRESETS).map(([key, preset]) => (
                <button
                  key={key}
                  onClick={() => handleLayoutChange(key)}
                  className={`w-full flex items-center px-3 py-2 text-sm text-left hover:bg-gray-100 focus:outline-none focus:bg-gray-100 transition-colors duration-150 ${
                    layout === key ? 'bg-primary-50 text-primary-700' : 'text-gray-700'
                  }`}
                >
                  {getLayoutIcon(key)}
                  <div className="ml-3 flex-1">
                    <div className="font-medium">{preset.name}</div>
                    <div className="text-xs text-gray-500">{preset.description}</div>
                  </div>
                  {layout === key && (
                    <div className="ml-auto w-2 h-2 bg-primary-600 rounded-full" />
                  )}
                </button>
              ))}
            </div>
          </div>
        </>
      )}
    </div>
  );
};

// Layout preview component
interface LayoutPreviewProps {
  layout: LayoutType;
  className?: string;
}

export const LayoutPreview: React.FC<LayoutPreviewProps> = ({ layout, className = '' }) => {
  const getPreviewGrid = () => {
    switch (layout) {
      case 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4':
        return (
          <div className="grid grid-cols-4 gap-1">
            <div className="w-3 h-3 bg-primary-400 rounded"></div>
            <div className="w-3 h-3 bg-primary-400 rounded"></div>
            <div className="w-3 h-3 bg-primary-400 rounded"></div>
            <div className="w-3 h-3 bg-primary-400 rounded"></div>
            <div className="w-3 h-3 bg-primary-300 rounded"></div>
            <div className="w-3 h-3 bg-primary-300 rounded"></div>
          </div>
        );
      case 'grid-cols-1 lg:grid-cols-2':
        return (
          <div className="grid grid-cols-2 gap-1">
            <div className="w-4 h-3 bg-primary-400 rounded"></div>
            <div className="w-4 h-3 bg-primary-400 rounded"></div>
            <div className="w-4 h-3 bg-primary-300 rounded"></div>
            <div className="w-4 h-3 bg-primary-300 rounded"></div>
          </div>
        );
      case 'grid-cols-1 md:grid-cols-3':
        return (
          <div className="grid grid-cols-3 gap-1">
            <div className="w-3 h-3 bg-primary-400 rounded"></div>
            <div className="w-3 h-3 bg-primary-400 rounded"></div>
            <div className="w-3 h-3 bg-primary-400 rounded"></div>
            <div className="w-3 h-3 bg-primary-300 rounded"></div>
            <div className="w-3 h-3 bg-primary-300 rounded"></div>
            <div className="w-3 h-3 bg-primary-300 rounded"></div>
          </div>
        );
      case 'grid-cols-1':
        return (
          <div className="space-y-1">
            <div className="w-6 h-2 bg-primary-400 rounded"></div>
            <div className="w-6 h-2 bg-primary-300 rounded"></div>
            <div className="w-6 h-2 bg-primary-300 rounded"></div>
          </div>
        );
      default:
        return null;
    }
  };

  return (
    <div className={`flex items-center justify-center p-2 ${className}`}>
      {getPreviewGrid()}
    </div>
  );
}; 