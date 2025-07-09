import React from 'react';
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight, MoreHorizontal } from 'lucide-react';
import { useAccessibility } from './AccessibilityProvider';

export interface PaginationProps {
  currentPage: number;
  totalPages: number;
  totalItems: number;
  pageSize: number;
  onPageChange: (page: number) => void;
  onPageSizeChange?: (pageSize: number) => void;
  variant?: 'default' | 'compact' | 'minimal';
  showPageSize?: boolean;
  showTotal?: boolean;
  showFirstLast?: boolean;
  maxVisiblePages?: number;
  className?: string;
  disabled?: boolean;
}

export const Pagination: React.FC<PaginationProps> = ({
  currentPage,
  totalPages,
  totalItems,
  pageSize,
  onPageChange,
  onPageSizeChange,
  variant = 'default',
  showPageSize = true,
  showTotal = true,
  showFirstLast = true,
  maxVisiblePages = 7,
  className = '',
  disabled = false,
}) => {
  const { addAnnouncement } = useAccessibility();

  const handlePageChange = (page: number) => {
    if (disabled || page < 1 || page > totalPages || page === currentPage) {
      return;
    }
    
    onPageChange(page);
    addAnnouncement(`Navigated to page ${page} of ${totalPages}`);
  };

  const handlePageSizeChange = (newPageSize: number) => {
    if (onPageSizeChange && newPageSize !== pageSize) {
      onPageSizeChange(newPageSize);
      addAnnouncement(`Page size changed to ${newPageSize} items`);
    }
  };

  const getVisiblePages = () => {
    if (totalPages <= maxVisiblePages) {
      return Array.from({ length: totalPages }, (_, i) => i + 1);
    }

    const halfVisible = Math.floor(maxVisiblePages / 2);
    let start = Math.max(1, currentPage - halfVisible);
    let end = Math.min(totalPages, start + maxVisiblePages - 1);

    if (end - start + 1 < maxVisiblePages) {
      start = Math.max(1, end - maxVisiblePages + 1);
    }

    const pages: (number | 'ellipsis')[] = [];

    if (start > 1) {
      pages.push(1);
      if (start > 2) {
        pages.push('ellipsis');
      }
    }

    for (let i = start; i <= end; i++) {
      pages.push(i);
    }

    if (end < totalPages) {
      if (end < totalPages - 1) {
        pages.push('ellipsis');
      }
      pages.push(totalPages);
    }

    return pages;
  };

  const getPageInfo = () => {
    const start = (currentPage - 1) * pageSize + 1;
    const end = Math.min(currentPage * pageSize, totalItems);
    return { start, end };
  };

  const pageInfo = getPageInfo();
  const visiblePages = getVisiblePages();

  if (variant === 'minimal') {
    return (
      <div className={`flex items-center justify-between ${className}`}>
        {showTotal && (
          <div className="text-sm text-gray-600">
            Showing {pageInfo.start}-{pageInfo.end} of {totalItems} items
          </div>
        )}
        <div className="flex items-center space-x-2">
          <button
            onClick={() => handlePageChange(currentPage - 1)}
            disabled={disabled || currentPage <= 1}
            className="p-2 text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            aria-label="Previous page"
          >
            <ChevronLeft className="w-4 h-4" />
          </button>
          <span className="text-sm text-gray-700">
            {currentPage} of {totalPages}
          </span>
          <button
            onClick={() => handlePageChange(currentPage + 1)}
            disabled={disabled || currentPage >= totalPages}
            className="p-2 text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            aria-label="Next page"
          >
            <ChevronRight className="w-4 h-4" />
          </button>
        </div>
      </div>
    );
  }

  if (variant === 'compact') {
    return (
      <div className={`flex items-center justify-between ${className}`}>
        {showTotal && (
          <div className="text-sm text-gray-600">
            {pageInfo.start}-{pageInfo.end} of {totalItems}
          </div>
        )}
        <div className="flex items-center space-x-1">
          {showFirstLast && (
            <button
              onClick={() => handlePageChange(1)}
              disabled={disabled || currentPage <= 1}
              className="p-2 text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
              aria-label="First page"
            >
              <ChevronsLeft className="w-4 h-4" />
            </button>
          )}
          <button
            onClick={() => handlePageChange(currentPage - 1)}
            disabled={disabled || currentPage <= 1}
            className="p-2 text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            aria-label="Previous page"
          >
            <ChevronLeft className="w-4 h-4" />
          </button>
          
          {visiblePages.map((page, index) => (
            <React.Fragment key={index}>
              {page === 'ellipsis' ? (
                <span className="px-2 text-gray-500">
                  <MoreHorizontal className="w-4 h-4" />
                </span>
              ) : (
                <button
                  onClick={() => handlePageChange(page)}
                  disabled={disabled}
                  className={`px-3 py-1 text-sm rounded ${
                    page === currentPage
                      ? 'bg-primary-600 text-white'
                      : 'text-gray-700 hover:bg-gray-100'
                  } disabled:opacity-50 disabled:cursor-not-allowed`}
                  aria-label={`Page ${page}`}
                  aria-current={page === currentPage ? 'page' : undefined}
                >
                  {page}
                </button>
              )}
            </React.Fragment>
          ))}
          
          <button
            onClick={() => handlePageChange(currentPage + 1)}
            disabled={disabled || currentPage >= totalPages}
            className="p-2 text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            aria-label="Next page"
          >
            <ChevronRight className="w-4 h-4" />
          </button>
          {showFirstLast && (
            <button
              onClick={() => handlePageChange(totalPages)}
              disabled={disabled || currentPage >= totalPages}
              className="p-2 text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
              aria-label="Last page"
            >
              <ChevronsRight className="w-4 h-4" />
            </button>
          )}
        </div>
      </div>
    );
  }

  // Default variant
  return (
    <div className={`flex flex-col sm:flex-row items-center justify-between space-y-4 sm:space-y-0 ${className}`}>
      <div className="flex items-center space-x-4">
        {showTotal && (
          <div className="text-sm text-gray-600">
            Showing <span className="font-medium">{pageInfo.start}</span> to{' '}
            <span className="font-medium">{pageInfo.end}</span> of{' '}
            <span className="font-medium">{totalItems}</span> results
          </div>
        )}
        
        {showPageSize && onPageSizeChange && (
          <div className="flex items-center space-x-2">
            <label htmlFor="page-size" className="text-sm text-gray-600">
              Show:
            </label>
            <select
              id="page-size"
              value={pageSize}
              onChange={(e) => handlePageSizeChange(Number(e.target.value))}
              disabled={disabled}
              className="border border-gray-300 rounded-md px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {[10, 25, 50, 100].map((size) => (
                <option key={size} value={size}>
                  {size}
                </option>
              ))}
            </select>
            <span className="text-sm text-gray-600">per page</span>
          </div>
        )}
      </div>

      <div className="flex items-center space-x-2">
        {showFirstLast && (
          <button
            onClick={() => handlePageChange(1)}
            disabled={disabled || currentPage <= 1}
            className="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            aria-label="First page"
          >
            <ChevronsLeft className="w-4 h-4" />
          </button>
        )}
        
        <button
          onClick={() => handlePageChange(currentPage - 1)}
          disabled={disabled || currentPage <= 1}
          className="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
          aria-label="Previous page"
        >
          <ChevronLeft className="w-4 h-4" />
        </button>

        <div className="flex items-center space-x-1">
          {visiblePages.map((page, index) => (
            <React.Fragment key={index}>
              {page === 'ellipsis' ? (
                <span className="px-3 py-2 text-sm text-gray-500">
                  <MoreHorizontal className="w-4 h-4" />
                </span>
              ) : (
                <button
                  onClick={() => handlePageChange(page)}
                  disabled={disabled}
                  className={`px-3 py-2 text-sm font-medium rounded-md ${
                    page === currentPage
                      ? 'bg-primary-600 text-white border border-primary-600'
                      : 'text-gray-700 bg-white border border-gray-300 hover:bg-gray-50'
                  } disabled:opacity-50 disabled:cursor-not-allowed`}
                  aria-label={`Page ${page}`}
                  aria-current={page === currentPage ? 'page' : undefined}
                >
                  {page}
                </button>
              )}
            </React.Fragment>
          ))}
        </div>

        <button
          onClick={() => handlePageChange(currentPage + 1)}
          disabled={disabled || currentPage >= totalPages}
          className="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
          aria-label="Next page"
        >
          <ChevronRight className="w-4 h-4" />
        </button>

        {showFirstLast && (
          <button
            onClick={() => handlePageChange(totalPages)}
            disabled={disabled || currentPage >= totalPages}
            className="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            aria-label="Last page"
          >
            <ChevronsRight className="w-4 h-4" />
          </button>
        )}
      </div>
    </div>
  );
};

// Hook for pagination state management
export const usePagination = (
  totalItems: number,
  initialPageSize: number = 25
) => {
  const [currentPage, setCurrentPage] = React.useState(1);
  const [pageSize, setPageSize] = React.useState(initialPageSize);

  const totalPages = Math.ceil(totalItems / pageSize);

  const goToPage = (page: number) => {
    const validPage = Math.max(1, Math.min(page, totalPages));
    setCurrentPage(validPage);
  };

  const goToNextPage = () => {
    if (currentPage < totalPages) {
      setCurrentPage(currentPage + 1);
    }
  };

  const goToPreviousPage = () => {
    if (currentPage > 1) {
      setCurrentPage(currentPage - 1);
    }
  };

  const goToFirstPage = () => {
    setCurrentPage(1);
  };

  const goToLastPage = () => {
    setCurrentPage(totalPages);
  };

  const changePageSize = (newPageSize: number) => {
    setPageSize(newPageSize);
    setCurrentPage(1); // Reset to first page when changing page size
  };

  const getPageInfo = () => {
    const start = (currentPage - 1) * pageSize + 1;
    const end = Math.min(currentPage * pageSize, totalItems);
    return { start, end };
  };

  // Reset to first page when total items change
  React.useEffect(() => {
    setCurrentPage(1);
  }, [totalItems]);

  return {
    currentPage,
    pageSize,
    totalPages,
    totalItems,
    goToPage,
    goToNextPage,
    goToPreviousPage,
    goToFirstPage,
    goToLastPage,
    changePageSize,
    getPageInfo,
  };
}; 