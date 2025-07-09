import React, { useState, useEffect, useCallback, useMemo } from 'react';
import { Search, X, History, Filter, Save, Loader2 } from 'lucide-react';
import { useDebounce } from '../../hooks/useDebounce';

export interface SearchResult {
  id: string;
  type: 'transaction' | 'block' | 'round' | 'validator' | 'asset' | 'handle';
  title: string;
  description: string;
  metadata: Record<string, any>;
  relevance: number;
}

export interface SearchFilter {
  type: string[];
  dateRange: { start: Date | null; end: Date | null };
  validators: string[];
  status: string[];
}

interface GlobalSearchProps {
  onSearch: (query: string, filters: SearchFilter) => Promise<SearchResult[]>;
  onResultSelect: (result: SearchResult) => void;
  placeholder?: string;
  className?: string;
}

export const GlobalSearch: React.FC<GlobalSearchProps> = ({
  onSearch,
  onResultSelect,
  placeholder = "Search transactions, blocks, rounds, validators...",
  className = ""
}) => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [showResults, setShowResults] = useState(false);
  const [searchHistory, setSearchHistory] = useState<string[]>([]);
  const [savedSearches, setSavedSearches] = useState<Array<{name: string, query: string, filters: SearchFilter}>>([]);
  const [filters, setFilters] = useState<SearchFilter>({
    type: [],
    dateRange: { start: null, end: null },
    validators: [],
    status: []
  });
  const [showFilters, setShowFilters] = useState(false);
  const [showHistory, setShowHistory] = useState(false);

  const debouncedQuery = useDebounce(query, 300);

  // Search types for filtering
  const searchTypes = [
    { id: 'transaction', label: 'Transactions', icon: '📄' },
    { id: 'block', label: 'Blocks', icon: '🧱' },
    { id: 'round', label: 'Rounds', icon: '⭕' },
    { id: 'validator', label: 'Validators', icon: '👤' },
    { id: 'asset', label: 'Assets', icon: '💰' },
    { id: 'handle', label: 'Handles', icon: '🏷️' }
  ];

  // Status options
  const statusOptions = [
    { id: 'confirmed', label: 'Confirmed' },
    { id: 'pending', label: 'Pending' },
    { id: 'failed', label: 'Failed' },
    { id: 'orphaned', label: 'Orphaned' }
  ];

  // Load search history from localStorage
  useEffect(() => {
    const history = localStorage.getItem('findag-search-history');
    if (history) {
      setSearchHistory(JSON.parse(history));
    }

    const saved = localStorage.getItem('findag-saved-searches');
    if (saved) {
      setSavedSearches(JSON.parse(saved));
    }
  }, []);

  // Perform search when query changes
  useEffect(() => {
    if (debouncedQuery.trim()) {
      performSearch(debouncedQuery, filters);
    } else {
      setResults([]);
      setShowResults(false);
    }
  }, [debouncedQuery, filters]);

  const performSearch = useCallback(async (searchQuery: string, searchFilters: SearchFilter) => {
    setIsLoading(true);
    try {
      const searchResults = await onSearch(searchQuery, searchFilters);
      setResults(searchResults);
      setShowResults(true);
      
      // Add to search history
      if (searchQuery.trim()) {
        const newHistory = [searchQuery, ...searchHistory.filter(h => h !== searchQuery)].slice(0, 10);
        setSearchHistory(newHistory);
        localStorage.setItem('findag-search-history', JSON.stringify(newHistory));
      }
    } catch (error) {
      console.error('Search error:', error);
      setResults([]);
    } finally {
      setIsLoading(false);
    }
  }, [onSearch, searchHistory]);

  const handleResultSelect = useCallback((result: SearchResult) => {
    onResultSelect(result);
    setShowResults(false);
    setQuery('');
  }, [onResultSelect]);

  const handleSaveSearch = useCallback(() => {
    const name = prompt('Enter a name for this search:');
    if (name && query.trim()) {
      const newSavedSearch = { name, query, filters };
      const updatedSearches = [...savedSearches, newSavedSearch];
      setSavedSearches(updatedSearches);
      localStorage.setItem('findag-saved-searches', JSON.stringify(updatedSearches));
    }
  }, [query, filters, savedSearches]);

  const handleLoadSearch = useCallback((savedSearch: {name: string, query: string, filters: SearchFilter}) => {
    setQuery(savedSearch.query);
    setFilters(savedSearch.filters);
    setShowHistory(false);
  }, []);

  const handleHistorySelect = useCallback((historyItem: string) => {
    setQuery(historyItem);
    setShowHistory(false);
  }, []);

  const clearSearch = useCallback(() => {
    setQuery('');
    setResults([]);
    setShowResults(false);
    setShowHistory(false);
  }, []);

  const toggleFilter = useCallback((type: string, value: string) => {
    setFilters(prev => ({
      ...prev,
      [type]: prev[type as keyof SearchFilter]?.includes(value)
        ? (prev[type as keyof SearchFilter] as string[]).filter(v => v !== value)
        : [...(prev[type as keyof SearchFilter] as string[] || []), value]
    }));
  }, []);

  const filteredResults = useMemo(() => {
    return results.sort((a, b) => b.relevance - a.relevance);
  }, [results]);

  return (
    <div className={`relative ${className}`}>
      {/* Search Input */}
      <div className="relative">
        <div className="relative flex items-center">
          <Search className="absolute left-3 h-4 w-4 text-gray-400" />
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onFocus={() => setShowHistory(true)}
            placeholder={placeholder}
            className="w-full pl-10 pr-20 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-800 dark:border-gray-600 dark:text-white"
          />
          <div className="absolute right-2 flex items-center space-x-1">
            {isLoading && (
              <Loader2 className="h-4 w-4 animate-spin text-gray-400" />
            )}
            {query && (
              <button
                onClick={clearSearch}
                className="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
              >
                <X className="h-4 w-4 text-gray-400" />
              </button>
            )}
            <button
              onClick={() => setShowFilters(!showFilters)}
              className={`p-1 rounded ${showFilters ? 'bg-blue-100 dark:bg-blue-900' : 'hover:bg-gray-200 dark:hover:bg-gray-700'}`}
              title="Advanced filters"
            >
              <Filter className="h-4 w-4 text-gray-400" />
            </button>
            <button
              onClick={() => setShowHistory(!showHistory)}
              className="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
              title="Search history"
            >
              <History className="h-4 w-4 text-gray-400" />
            </button>
          </div>
        </div>

        {/* Search History Dropdown */}
        {showHistory && !query && (
          <div className="absolute top-full left-0 right-0 mt-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg z-50">
            <div className="p-2">
              <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Recent Searches</h3>
              {searchHistory.length > 0 ? (
                <div className="space-y-1">
                  {searchHistory.map((item, index) => (
                    <button
                      key={index}
                      onClick={() => handleHistorySelect(item)}
                      className="w-full text-left px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded text-sm"
                    >
                      {item}
                    </button>
                  ))}
                </div>
              ) : (
                <p className="text-sm text-gray-500 dark:text-gray-400">No recent searches</p>
              )}
              
              {savedSearches.length > 0 && (
                <>
                  <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 mt-4">Saved Searches</h3>
                  <div className="space-y-1">
                    {savedSearches.map((saved, index) => (
                      <button
                        key={index}
                        onClick={() => handleLoadSearch(saved)}
                        className="w-full text-left px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded text-sm"
                      >
                        {saved.name}
                      </button>
                    ))}
                  </div>
                </>
              )}
            </div>
          </div>
        )}

        {/* Advanced Filters */}
        {showFilters && (
          <div className="absolute top-full left-0 right-0 mt-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg z-50 p-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* Type Filter */}
              <div>
                <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Type</h4>
                <div className="space-y-1">
                  {searchTypes.map((type) => (
                    <label key={type.id} className="flex items-center space-x-2">
                      <input
                        type="checkbox"
                        checked={filters.type.includes(type.id)}
                        onChange={() => toggleFilter('type', type.id)}
                        className="rounded"
                      />
                      <span className="text-sm">{type.icon} {type.label}</span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Status Filter */}
              <div>
                <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Status</h4>
                <div className="space-y-1">
                  {statusOptions.map((status) => (
                    <label key={status.id} className="flex items-center space-x-2">
                      <input
                        type="checkbox"
                        checked={filters.status.includes(status.id)}
                        onChange={() => toggleFilter('status', status.id)}
                        className="rounded"
                      />
                      <span className="text-sm">{status.label}</span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Date Range */}
              <div>
                <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Date Range</h4>
                <div className="grid grid-cols-2 gap-2">
                  <input
                    type="date"
                    value={filters.dateRange.start?.toISOString().split('T')[0] || ''}
                    onChange={(e) => setFilters(prev => ({
                      ...prev,
                      dateRange: { ...prev.dateRange, start: e.target.value ? new Date(e.target.value) : null }
                    }))}
                    className="text-sm border border-gray-300 dark:border-gray-600 rounded px-2 py-1 dark:bg-gray-700"
                  />
                  <input
                    type="date"
                    value={filters.dateRange.end?.toISOString().split('T')[0] || ''}
                    onChange={(e) => setFilters(prev => ({
                      ...prev,
                      dateRange: { ...prev.dateRange, end: e.target.value ? new Date(e.target.value) : null }
                    }))}
                    className="text-sm border border-gray-300 dark:border-gray-600 rounded px-2 py-1 dark:bg-gray-700"
                  />
                </div>
              </div>

              {/* Actions */}
              <div className="flex items-end space-x-2">
                <button
                  onClick={handleSaveSearch}
                  className="flex items-center space-x-1 px-3 py-1 bg-blue-500 text-white rounded text-sm hover:bg-blue-600"
                >
                  <Save className="h-3 w-3" />
                  <span>Save Search</span>
                </button>
                <button
                  onClick={() => setFilters({
                    type: [],
                    dateRange: { start: null, end: null },
                    validators: [],
                    status: []
                  })}
                  className="px-3 py-1 bg-gray-500 text-white rounded text-sm hover:bg-gray-600"
                >
                  Clear
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Search Results */}
        {showResults && filteredResults.length > 0 && (
          <div className="absolute top-full left-0 right-0 mt-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg z-50 max-h-96 overflow-y-auto">
            <div className="p-2">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {filteredResults.length} result{filteredResults.length !== 1 ? 's' : ''}
                </span>
                <button
                  onClick={handleSaveSearch}
                  className="text-sm text-blue-500 hover:text-blue-600"
                >
                  Save Search
                </button>
              </div>
              <div className="space-y-1">
                {filteredResults.map((result) => (
                  <button
                    key={result.id}
                    onClick={() => handleResultSelect(result)}
                    className="w-full text-left p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
                  >
                    <div className="flex items-center space-x-2">
                      <span className="text-lg">
                        {searchTypes.find(t => t.id === result.type)?.icon || '📄'}
                      </span>
                      <div className="flex-1">
                        <div className="font-medium text-sm">{result.title}</div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">{result.description}</div>
                      </div>
                      <div className="text-xs text-gray-400">
                        {Math.round(result.relevance * 100)}%
                      </div>
                    </div>
                  </button>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* No Results */}
        {showResults && query && !isLoading && filteredResults.length === 0 && (
          <div className="absolute top-full left-0 right-0 mt-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg z-50 p-4">
            <p className="text-sm text-gray-500 dark:text-gray-400 text-center">
              No results found for "{query}"
            </p>
          </div>
        )}
      </div>
    </div>
  );
}; 