import React from 'react';
import { useMediaQuery } from '../../hooks/useMediaQuery';

interface ResponsiveContainerProps {
  children: React.ReactNode;
  className?: string;
  mobileClassName?: string;
  tabletClassName?: string;
  desktopClassName?: string;
  breakpoint?: 'sm' | 'md' | 'lg' | 'xl' | '2xl';
}

export const ResponsiveContainer: React.FC<ResponsiveContainerProps> = ({
  children,
  className = '',
  mobileClassName = '',
  tabletClassName = '',
  desktopClassName = '',
  breakpoint = 'lg',
}) => {
  const isMobile = useMediaQuery('(max-width: 767px)');
  const isTablet = useMediaQuery('(min-width: 768px) and (max-width: 1023px)');
  const isDesktop = useMediaQuery('(min-width: 1024px)');

  const getResponsiveClassName = () => {
    if (isMobile) return mobileClassName;
    if (isTablet) return tabletClassName;
    if (isDesktop) return desktopClassName;
    return '';
  };

  return (
    <div className={`${className} ${getResponsiveClassName()}`}>
      {children}
    </div>
  );
};

// Mobile-first responsive grid
interface ResponsiveGridProps {
  children: React.ReactNode;
  cols?: {
    mobile?: number;
    tablet?: number;
    desktop?: number;
  };
  gap?: {
    mobile?: string;
    tablet?: string;
    desktop?: string;
  };
  className?: string;
}

export const ResponsiveGrid: React.FC<ResponsiveGridProps> = ({
  children,
  cols = { mobile: 1, tablet: 2, desktop: 3 },
  gap = { mobile: '4', tablet: '6', desktop: '8' },
  className = '',
}) => {
  const gridCols = {
    mobile: `grid-cols-${cols.mobile || 1}`,
    tablet: cols.tablet ? `md:grid-cols-${cols.tablet}` : '',
    desktop: cols.desktop ? `lg:grid-cols-${cols.desktop}` : '',
  };

  const gridGap = {
    mobile: `gap-${gap.mobile || 4}`,
    tablet: gap.tablet ? `md:gap-${gap.tablet}` : '',
    desktop: gap.desktop ? `lg:gap-${gap.desktop}` : '',
  };

  const gridClasses = [
    'grid',
    gridCols.mobile,
    gridCols.tablet,
    gridCols.desktop,
    gridGap.mobile,
    gridGap.tablet,
    gridGap.desktop,
    className,
  ].filter(Boolean).join(' ');

  return <div className={gridClasses}>{children}</div>;
};

// Responsive sidebar layout
interface ResponsiveSidebarProps {
  sidebar: React.ReactNode;
  main: React.ReactNode;
  sidebarWidth?: string;
  className?: string;
}

export const ResponsiveSidebar: React.FC<ResponsiveSidebarProps> = ({
  sidebar,
  main,
  sidebarWidth = 'w-64',
  className = '',
}) => {
  return (
    <div className={`flex flex-col lg:flex-row ${className}`}>
      {/* Mobile: Full width sidebar */}
      <div className={`lg:hidden w-full border-b border-gray-200 ${sidebarWidth}`}>
        {sidebar}
      </div>
      
      {/* Desktop: Fixed width sidebar */}
      <div className={`hidden lg:block ${sidebarWidth} border-r border-gray-200`}>
        {sidebar}
      </div>
      
      {/* Main content */}
      <div className="flex-1 min-w-0">
        {main}
      </div>
    </div>
  );
};

// Responsive navigation
interface ResponsiveNavigationProps {
  items: {
    label: string;
    href: string;
    icon?: React.ReactNode;
    badge?: string | number;
  }[];
  className?: string;
}

export const ResponsiveNavigation: React.FC<ResponsiveNavigationProps> = ({
  items,
  className = '',
}) => {
  return (
    <nav className={`${className}`}>
      {/* Desktop: Horizontal navigation */}
      <div className="hidden md:flex space-x-8">
        {items.map((item) => (
          <a
            key={item.href}
            href={item.href}
            className="flex items-center space-x-2 text-gray-600 hover:text-gray-900 transition-colors"
          >
            {item.icon && <span>{item.icon}</span>}
            <span>{item.label}</span>
            {item.badge && (
              <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-primary-100 text-primary-800">
                {item.badge}
              </span>
            )}
          </a>
        ))}
      </div>

      {/* Mobile: Dropdown navigation */}
      <div className="md:hidden">
        <select className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
          {items.map((item) => (
            <option key={item.href} value={item.href}>
              {item.label} {item.badge && `(${item.badge})`}
            </option>
          ))}
        </select>
      </div>
    </nav>
  );
};

// Responsive table
interface ResponsiveTableProps {
  headers: string[];
  children: React.ReactNode;
  className?: string;
}

export const ResponsiveTable: React.FC<ResponsiveTableProps> = ({
  headers,
  children,
  className = '',
}) => {
  return (
    <div className={`overflow-x-auto ${className}`}>
      {/* Desktop: Full table */}
      <table className="hidden md:table min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            {headers.map((header, index) => (
              <th
                key={index}
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                {header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {children}
        </tbody>
      </table>

      {/* Mobile: Card-based layout */}
      <div className="md:hidden space-y-4">
        {/* This would need to be implemented with actual data */}
        <div className="bg-white rounded-lg shadow p-4">
          <p className="text-sm text-gray-500">Mobile table view - implement with actual data</p>
        </div>
      </div>
    </div>
  );
};

// Responsive modal
interface ResponsiveModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  className?: string;
}

export const ResponsiveModal: React.FC<ResponsiveModalProps> = ({
  isOpen,
  onClose,
  title,
  children,
  className = '',
}) => {
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
        <div className={`inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full ${className}`}>
          {/* Header */}
          <div className="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
            <div className="sm:flex sm:items-start">
              <div className="mt-3 text-center sm:mt-0 sm:text-left w-full">
                <h3 className="text-lg leading-6 font-medium text-gray-900">
                  {title}
                </h3>
                <div className="mt-2">
                  {children}
                </div>
              </div>
            </div>
          </div>

          {/* Footer */}
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