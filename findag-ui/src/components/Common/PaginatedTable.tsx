import React, { useState, useMemo } from 'react';
import { Search, Filter, Download, RefreshCw } from 'lucide-react';
import { Pagination, usePagination } from './Pagination';
import { LoadingSpinner } from './LoadingSpinner';
import { useAccessibility } from './AccessibilityProvider';

export interface Column<T> {
  key: string;
  header: string;
  width?: number | string;
  sortable?: boolean;
  render?: (item: T, index: number) => React.ReactNode;
  align?: 'left' | 'center' | 'right';
}

export interface SortConfig {
  key: string;
  direction: 'asc' | 'desc';
}

export interface PaginatedTableProps<T> {
  data: T[];
  columns: Column<T>[];
  loading?: boolean;
  error?: string | null;
  onRefresh?: () => void;
  onExport?: () => void;
  searchable?: boolean;
  filterable?: boolean;
  sortable?: boolean;
  selectable?: boolean;
  onSelectionChange?: (selectedItems: T[]) => void;
  pageSizeOptions?: number[];
  initialPageSize?: number;
  className?: string;
  emptyMessage?: string;
  getItemKey?: (item: T, index: number) => string | number;
}

export function PaginatedTable<T>({
  data,
  columns,
  loading = false,
  error = null,
  onRefresh,
  onExport,
  searchable = true,
  filterable = true,
  sortable = true,
  selectable = false,
  onSelectionChange,
  pageSizeOptions = [10, 25, 50, 100],
  initialPageSize = 25,
  className = '',
  emptyMessage = 'No data available',
  getItemKey = (_, index) => index,
}: PaginatedTableProps<T>) {
  const { addAnnouncement } = useAccessibility();
  const [searchTerm, setSearchTerm] = useState('');
  const [sortConfig, setSortConfig] = useState<SortConfig | null>(null);
  const [selectedItems, setSelectedItems] = useState<Set<string | number>>(new Set());
  const [showFilters, setShowFilters] = useState(false);

  const pagination = usePagination(data.length, initialPageSize);

  // Filter and sort data
  const processedData = useMemo(() => {
    let filtered = data;

    // Apply search filter
    if (searchTerm) {
      filtered = filtered.filter((item) => {
        return columns.some((column) => {
          const value = (item as any)[column.key];
          if (value == null) return false;
          return String(value).toLowerCase().includes(searchTerm.toLowerCase());
        });
      });
    }

    // Apply sorting
    if (sortConfig) {
      filtered = [...filtered].sort((a, b) => {
        const aValue = (a as any)[sortConfig.key];
        const bValue = (b as any)[sortConfig.key];

        if (aValue == null && bValue == null) return 0;
        if (aValue == null) return 1;
        if (bValue == null) return -1;

        let comparison = 0;
        if (typeof aValue === 'string' && typeof bValue === 'string') {
          comparison = aValue.localeCompare(bValue);
        } else {
          comparison = aValue < bValue ? -1 : aValue > bValue ? 1 : 0;
        }

        return sortConfig.direction === 'desc' ? -comparison : comparison;
      });
    }

    return filtered;
  }, [data, searchTerm, sortConfig, columns]);

  // Update pagination when filtered data changes
  React.useEffect(() => {
    pagination.goToFirstPage();
  }, [processedData.length]);

  // Get current page data
  const currentPageData = useMemo(() => {
    const start = (pagination.currentPage - 1) * pagination.pageSize;
    const end = start + pagination.pageSize;
    return processedData.slice(start, end);
  }, [processedData, pagination.currentPage, pagination.pageSize]);

  // Handle sorting
  const handleSort = (key: string) => {
    if (!sortable) return;

    setSortConfig((current) => {
      if (current?.key === key) {
        return {
          key,
          direction: current.direction === 'asc' ? 'desc' : 'asc',
        };
      }
      return { key, direction: 'asc' };
    });

    addAnnouncement(`Sorted by ${key}`);
  };

  // Handle selection
  const handleSelectAll = (checked: boolean) => {
    if (!selectable) return;

    const newSelected = new Set<string | number>();
    if (checked) {
      currentPageData.forEach((item, index) => {
        newSelected.add(getItemKey(item, index));
      });
    }

    setSelectedItems(newSelected);
    onSelectionChange?.(currentPageData.filter((_, index) => newSelected.has(getItemKey(_, index))));
  };

  const handleSelectItem = (item: T, index: number, checked: boolean) => {
    if (!selectable) return;

    const key = getItemKey(item, index);
    const newSelected = new Set(selectedItems);

    if (checked) {
      newSelected.add(key);
    } else {
      newSelected.delete(key);
    }

    setSelectedItems(newSelected);
    const selectedData = data.filter((_, i) => newSelected.has(getItemKey(_, i)));
    onSelectionChange?.(selectedData);
  };

  const getSortIcon = (key: string) => {
    if (!sortConfig || sortConfig.key !== key) {
      return null;
    }
    return sortConfig.direction === 'asc' ? '↑' : '↓';
  };

  const getColumnAlignment = (align?: 'left' | 'center' | 'right') => {
    switch (align) {
      case 'center':
        return 'text-center';
      case 'right':
        return 'text-right';
      default:
        return 'text-left';
    }
  };

  if (error) {
    return (
      <div className={`bg-white rounded-lg shadow-sm border border-gray-200 p-6 ${className}`}>
        <div className="text-center">
          <div className="text-red-600 mb-2">Error loading data</div>
          <div className="text-sm text-gray-600 mb-4">{error}</div>
          {onRefresh && (
            <button
              onClick={onRefresh}
              className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700"
            >
              Try Again
            </button>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-white rounded-lg shadow-sm border border-gray-200 ${className}`}>
      {/* Header */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-4 sm:space-y-0">
          <div className="flex items-center space-x-4">
            {searchable && (
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                <input
                  type="text"
                  placeholder="Search..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                />
              </div>
            )}
            
            {filterable && (
              <button
                onClick={() => setShowFilters(!showFilters)}
                className={`px-3 py-2 border rounded-lg flex items-center space-x-2 ${
                  showFilters
                    ? 'border-primary-500 text-primary-600 bg-primary-50'
                    : 'border-gray-300 text-gray-600 hover:bg-gray-50'
                }`}
              >
                <Filter className="w-4 h-4" />
                <span>Filters</span>
              </button>
            )}
          </div>

          <div className="flex items-center space-x-2">
            {onRefresh && (
              <button
                onClick={onRefresh}
                disabled={loading}
                className="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg disabled:opacity-50"
                title="Refresh data"
              >
                <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
              </button>
            )}
            
            {onExport && (
              <button
                onClick={onExport}
                className="px-3 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 flex items-center space-x-2"
              >
                <Download className="w-4 h-4" />
                <span>Export</span>
              </button>
            )}
          </div>
        </div>

        {/* Filters Panel */}
        {showFilters && filterable && (
          <div className="mt-4 p-4 bg-gray-50 rounded-lg">
            <div className="text-sm text-gray-600 mb-2">Filters (coming soon)</div>
            <div className="text-xs text-gray-500">Advanced filtering options will be available here</div>
          </div>
        )}
      </div>

      {/* Table */}
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-50">
            <tr>
              {selectable && (
                <th className="px-6 py-3 text-left">
                  <input
                    type="checkbox"
                    checked={currentPageData.length > 0 && currentPageData.every((_, index) => 
                      selectedItems.has(getItemKey(_, index))
                    )}
                    onChange={(e) => handleSelectAll(e.target.checked)}
                    className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                  />
                </th>
              )}
              {columns.map((column) => (
                <th
                  key={column.key}
                  className={`px-6 py-3 text-xs font-medium text-gray-500 uppercase tracking-wider ${
                    getColumnAlignment(column.align)
                  } ${column.sortable && sortable ? 'cursor-pointer hover:bg-gray-100' : ''}`}
                  style={{ width: column.width }}
                  onClick={() => column.sortable && handleSort(column.key)}
                >
                  <div className="flex items-center space-x-1">
                    <span>{column.header}</span>
                    {column.sortable && sortable && (
                      <span className="text-gray-400">{getSortIcon(column.key)}</span>
                    )}
                  </div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {loading ? (
              <tr>
                <td
                  colSpan={columns.length + (selectable ? 1 : 0)}
                  className="px-6 py-12 text-center"
                >
                  <LoadingSpinner size="lg" text="Loading data..." />
                </td>
              </tr>
            ) : currentPageData.length === 0 ? (
              <tr>
                <td
                  colSpan={columns.length + (selectable ? 1 : 0)}
                  className="px-6 py-12 text-center text-gray-500"
                >
                  {searchTerm ? 'No results found for your search' : emptyMessage}
                </td>
              </tr>
            ) : (
              currentPageData.map((item, index) => (
                <tr
                  key={getItemKey(item, index)}
                  className="hover:bg-gray-50"
                >
                  {selectable && (
                    <td className="px-6 py-4">
                      <input
                        type="checkbox"
                        checked={selectedItems.has(getItemKey(item, index))}
                        onChange={(e) => handleSelectItem(item, index, e.target.checked)}
                        className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                      />
                    </td>
                  )}
                  {columns.map((column) => (
                    <td
                      key={column.key}
                      className={`px-6 py-4 whitespace-nowrap text-sm text-gray-900 ${getColumnAlignment(column.align)}`}
                    >
                      {column.render
                        ? column.render(item, index)
                        : (item as any)[column.key] ?? '-'}
                    </td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      <div className="px-6 py-4 border-t border-gray-200">
        <Pagination
          currentPage={pagination.currentPage}
          totalPages={pagination.totalPages}
          totalItems={processedData.length}
          pageSize={pagination.pageSize}
          onPageChange={pagination.goToPage}
          onPageSizeChange={pagination.changePageSize}
          showPageSize={true}
          showTotal={true}
          showFirstLast={true}
          variant="default"
        />
      </div>
    </div>
  );
}

// Hook for table state management
export const usePaginatedTable = <T,>(
  data: T[],
  initialPageSize: number = 25
) => {
  const [searchTerm, setSearchTerm] = useState('');
  const [sortConfig, setSortConfig] = useState<SortConfig | null>(null);
  const [selectedItems, setSelectedItems] = useState<T[]>([]);

  const pagination = usePagination(data.length, initialPageSize);

  const filteredData = useMemo(() => {
    let filtered = data;

    if (searchTerm) {
      filtered = filtered.filter((item) => {
        return Object.values(item as any).some((value) =>
          String(value).toLowerCase().includes(searchTerm.toLowerCase())
        );
      });
    }

    if (sortConfig) {
      filtered = [...filtered].sort((a, b) => {
        const aValue = (a as any)[sortConfig.key];
        const bValue = (b as any)[sortConfig.key];

        if (aValue == null && bValue == null) return 0;
        if (aValue == null) return 1;
        if (bValue == null) return -1;

        let comparison = 0;
        if (typeof aValue === 'string' && typeof bValue === 'string') {
          comparison = aValue.localeCompare(bValue);
        } else {
          comparison = aValue < bValue ? -1 : aValue > bValue ? 1 : 0;
        }

        return sortConfig.direction === 'desc' ? -comparison : comparison;
      });
    }

    return filtered;
  }, [data, searchTerm, sortConfig]);

  const currentPageData = useMemo(() => {
    const start = (pagination.currentPage - 1) * pagination.pageSize;
    const end = start + pagination.pageSize;
    return filteredData.slice(start, end);
  }, [filteredData, pagination.currentPage, pagination.pageSize]);

  return {
    searchTerm,
    setSearchTerm,
    sortConfig,
    setSortConfig,
    selectedItems,
    setSelectedItems,
    pagination,
    filteredData,
    currentPageData,
  };
}; 