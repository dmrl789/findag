import React, { useState, useEffect } from 'react';
import { Settings, Key, Clock, AlertCircle, CheckCircle, XCircle } from 'lucide-react';
import { marketDataService } from '../../services/marketData';

interface DataSourceConfigProps {
  onConfigChange?: () => void;
}

export const DataSourceConfig: React.FC<DataSourceConfigProps> = ({ onConfigChange }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [apiKeys, setApiKeys] = useState<Record<string, string>>({});
  const [testResults, setTestResults] = useState<Record<string, boolean>>({});
  const [isTesting, setIsTesting] = useState<Record<string, boolean>>({});

  const dataSources = marketDataService.getAvailableSources();

  useEffect(() => {
    // Load saved API keys from localStorage
    const savedKeys = localStorage.getItem('findag_api_keys');
    if (savedKeys) {
      setApiKeys(JSON.parse(savedKeys));
    }
  }, []);

  const handleApiKeyChange = (source: string, key: string) => {
    const newKeys = { ...apiKeys, [source]: key };
    setApiKeys(newKeys);
    localStorage.setItem('findag_api_keys', JSON.stringify(newKeys));
    
    // Update environment variables for the service
    if (key) {
      // Note: In Vite, we can't dynamically set environment variables
      // The API keys will be loaded from localStorage instead
      console.log(`API key for ${source} updated`);
    }
    
    onConfigChange?.();
  };

  const testApiKey = async (source: string) => {
    setIsTesting(prev => ({ ...prev, [source]: true }));
    
    try {
      const config = marketDataService.getDataSourceConfig(source);
      if (!config) return;

      let testUrl: string;
      let isValid: boolean;

      // Test each API with their specific endpoints
      switch (source) {
        case 'alphaVantage':
          testUrl = `${config.baseUrl}?function=TIME_SERIES_INTRADAY&symbol=AAPL&interval=1min&apikey=${apiKeys[source]}`;
          const alphaResponse = await fetch(testUrl);
          const alphaData = await alphaResponse.json();
          isValid = !alphaData['Error Message'] && alphaData['Meta Data'];
          break;

        case 'finnhub':
          testUrl = `https://finnhub.io/api/v1/quote?symbol=AAPL&token=${apiKeys[source]}`;
          const finnhubResponse = await fetch(testUrl);
          const finnhubData = await finnhubResponse.json();
          isValid = finnhubData.c && finnhubData.c > 0; // Check if we got a valid price
          break;

        case 'polygon':
          testUrl = `https://api.polygon.io/v2/aggs/ticker/AAPL/range/1/day/2023-01-01/2023-01-01?adjusted=true&sort=asc&limit=1&apiKey=${apiKeys[source]}`;
          const polygonResponse = await fetch(testUrl);
          const polygonData = await polygonResponse.json();
          isValid = polygonData.results && polygonData.results.length > 0;
          break;

        case 'yahooFinance':
          // Yahoo Finance doesn't require API key for basic usage
          testUrl = 'https://query1.finance.yahoo.com/v8/finance/chart/AAPL?range=1d&interval=1m';
          const yahooResponse = await fetch(testUrl);
          const yahooData = await yahooResponse.json();
          isValid = yahooData.chart && yahooData.chart.result;
          break;

        default:
          isValid = false;
      }

      console.log(`API test for ${source}:`, { isValid, data: source === 'finnhub' ? 'price data' : 'time series data' });
      setTestResults(prev => ({ ...prev, [source]: isValid }));
    } catch (error) {
      console.error(`API test failed for ${source}:`, error);
      setTestResults(prev => ({ ...prev, [source]: false }));
    } finally {
      setIsTesting(prev => ({ ...prev, [source]: false }));
    }
  };

  const getDataSourceInfo = (source: string) => {
    const config = marketDataService.getDataSourceConfig(source);
    if (!config) return null;

    return {
      name: config.name,
      delay: config.delay,
      rateLimit: config.rateLimit,
      description: getDataSourceDescription(source),
    };
  };

  const getDataSourceDescription = (source: string): string => {
    switch (source) {
      case 'alphaVantage':
        return 'Free tier: 5 requests/minute, 12-second delay. Paid tier: 500+ requests/minute, real-time data.';
      case 'polygon':
        return 'Free tier: 5 requests/minute, 2-second delay. Paid tier: Real-time data with higher limits.';
      case 'finnhub':
        return 'Free tier: 60 requests/minute, 1-second delay. Paid tier: Real-time data with higher limits.';
      case 'yahooFinance':
        return 'Free tier: 10 requests/minute, 5-second delay. No API key required for basic usage.';
      default:
        return 'Real-time financial data with configurable delays.';
    }
  };

  const getStatusIcon = (source: string) => {
    if (isTesting[source]) {
      return <Clock className="w-4 h-4 text-yellow-500" />;
    }
    
    if (testResults[source] === true) {
      return <CheckCircle className="w-4 h-4 text-green-500" />;
    }
    
    if (testResults[source] === false) {
      return <XCircle className="w-4 h-4 text-red-500" />;
    }
    
    return <AlertCircle className="w-4 h-4 text-gray-400" />;
  };

  const getStatusText = (source: string) => {
    if (isTesting[source]) return 'Testing...';
    if (testResults[source] === true) return 'Valid';
    if (testResults[source] === false) return 'Invalid';
    return 'Not tested';
  };

  return (
    <div className="bg-white rounded-lg shadow-lg">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-gray-200">
        <div>
          <h2 className="text-xl font-semibold text-gray-900">Data Source Configuration</h2>
          <p className="text-sm text-gray-600">Configure API keys for real-time financial data</p>
        </div>
        <button
          onClick={() => setIsOpen(!isOpen)}
          className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          <Settings className="w-4 h-4" />
          <span>{isOpen ? 'Hide' : 'Configure'}</span>
        </button>
      </div>

      {/* Configuration Panel */}
      {isOpen && (
        <div className="p-6 space-y-6">
          {dataSources.map((source) => {
            const info = getDataSourceInfo(source);
            if (!info) return null;

            return (
              <div key={source} className="border border-gray-200 rounded-lg p-4">
                <div className="flex items-center justify-between mb-4">
                  <div>
                    <h3 className="text-lg font-medium text-gray-900">{info.name}</h3>
                    <p className="text-sm text-gray-600">{info.description}</p>
                  </div>
                  <div className="flex items-center space-x-2">
                    {getStatusIcon(source)}
                    <span className="text-sm text-gray-600">{getStatusText(source)}</span>
                  </div>
                </div>

                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      API Key
                    </label>
                    <div className="flex space-x-2">
                      <input
                        type="password"
                        value={apiKeys[source] || ''}
                        onChange={(e) => handleApiKeyChange(source, e.target.value)}
                        placeholder="Enter your API key"
                        className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      />
                      <button
                        onClick={() => testApiKey(source)}
                        disabled={!apiKeys[source] || isTesting[source]}
                        className="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:bg-gray-300 disabled:cursor-not-allowed"
                      >
                        {isTesting[source] ? 'Testing...' : 'Test'}
                      </button>
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4 text-sm">
                    <div>
                      <span className="text-gray-600">Delay:</span>
                      <span className="ml-2 font-medium">{info.delay / 1000}s</span>
                    </div>
                    <div>
                      <span className="text-gray-600">Rate Limit:</span>
                      <span className="ml-2 font-medium">{info.rateLimit}/min</span>
                    </div>
                  </div>
                </div>
              </div>
            );
          })}

          {/* Instructions */}
          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
            <h4 className="font-medium text-blue-900 mb-2">How to Get API Keys</h4>
            <div className="space-y-2 text-sm text-blue-800">
              <p><strong>Alpha Vantage:</strong> Free API key at <a href="https://www.alphavantage.co/support/#api-key" target="_blank" rel="noopener noreferrer" className="underline">alphavantage.co</a></p>
              <p><strong>Polygon.io:</strong> Free API key at <a href="https://polygon.io/" target="_blank" rel="noopener noreferrer" className="underline">polygon.io</a></p>
              <p><strong>Finnhub:</strong> Free API key at <a href="https://finnhub.io/" target="_blank" rel="noopener noreferrer" className="underline">finnhub.io</a></p>
              <p><strong>Yahoo Finance:</strong> No API key required for basic usage</p>
            </div>
          </div>

          {/* Data Quality Notice */}
          <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
            <h4 className="font-medium text-yellow-900 mb-2">Data Quality Notice</h4>
            <p className="text-sm text-yellow-800">
              Free tier APIs have delays and rate limits. For institutional use, consider paid tiers for real-time data with higher limits.
            </p>
          </div>
        </div>
      )}
    </div>
  );
}; 