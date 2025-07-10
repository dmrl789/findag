import React, { useState, useEffect } from 'react';
import { AdvancedChart } from './AdvancedChart';
import { PricePoint } from '../../types';
import { demoPriceHistory } from '../../data/demo';
import { marketDataService } from '../../services/marketData';
import { Settings, RefreshCw } from 'lucide-react';
import { DataSourceConfig } from '../Common/DataSourceConfig';

export const ChartPage: React.FC = () => {
  const [timeFrame, setTimeFrame] = useState<'1m' | '1h' | '3h' | '1D' | '1W' | '1M' | '1Y' | '5Y'>('1W');
  const [chartType, setChartType] = useState<'line' | 'candlestick' | 'area' | 'volume' | 'technical'>('line');
  const [data, setData] = useState<PricePoint[]>([]);
  const [loading, setLoading] = useState(true);
  const [realData, setRealData] = useState<PricePoint[]>([]);
  const [isLoadingRealData, setIsLoadingRealData] = useState(false);
  const [showDataSourceConfig, setShowDataSourceConfig] = useState(false);
  const [dataSourceStatus, setDataSourceStatus] = useState<string>('demo');
  const [selectedPair, setSelectedPair] = useState<string>('EUR/USD');

  // Fetch real market data
  const fetchRealData = async () => {
    setIsLoadingRealData(true);
    try {
      let data;
      let dataType: 'forex' | 'stock' | 'commodity';
      
      if (selectedPair.includes('/')) {
        // Forex pair
        dataType = 'forex';
        data = await marketDataService.getForexData(selectedPair, timeFrame);
      } else if (selectedPair === 'AAPL' || selectedPair === 'MSFT') {
        // Stock
        dataType = 'stock';
        data = await marketDataService.getStockData(selectedPair, timeFrame);
      } else {
        // Commodity
        dataType = 'commodity';
        data = await marketDataService.getCommodityData(selectedPair);
      }
      
      console.log(`Fetched ${dataType} data for ${selectedPair}:`, data.length, 'data points');
      setRealData(data);
      setDataSourceStatus('real');
    } catch (error) {
      console.error('Error fetching real data:', error);
      setDataSourceStatus('demo');
    } finally {
      setIsLoadingRealData(false);
    }
  };

  // Force refresh data
  const forceRefreshData = async () => {
    setIsLoadingRealData(true);
    try {
      let data;
      let dataType: 'forex' | 'stock' | 'commodity';
      
      if (selectedPair.includes('/')) {
        dataType = 'forex';
        data = await marketDataService.forceRefreshData(selectedPair, 'forex');
      } else if (selectedPair === 'AAPL' || selectedPair === 'MSFT') {
        dataType = 'stock';
        data = await marketDataService.forceRefreshData(selectedPair, 'stock');
      } else {
        dataType = 'commodity';
        data = await marketDataService.forceRefreshData(selectedPair, 'commodity');
      }
      
      console.log(`Force refreshed ${dataType} data for ${selectedPair}:`, data.length, 'data points');
      setRealData(data);
      setDataSourceStatus('real');
    } catch (error) {
      console.error('Error force refreshing data:', error);
      setDataSourceStatus('demo');
    } finally {
      setIsLoadingRealData(false);
    }
  };

  // Check if API keys are configured
  useEffect(() => {
    const hasApiKey = marketDataService.isApiKeyConfigured('alphaVantage') ||
                     marketDataService.isApiKeyConfigured('polygon') ||
                     marketDataService.isApiKeyConfigured('finnhub');
    
    if (hasApiKey) {
      fetchRealData();
    }
  }, [selectedPair]);

  // Auto-refresh data every 15 seconds if using real data, every 5 seconds for demo data
  useEffect(() => {
    if (dataSourceStatus === 'real') {
      const interval = setInterval(fetchRealData, 15000);
      return () => clearInterval(interval);
    } else {
      // For demo data, refresh more frequently to show price movements
      const interval = setInterval(fetchRealData, 5000);
      return () => clearInterval(interval);
    }
  }, [dataSourceStatus]);

  useEffect(() => {
    // Use real data if available, otherwise use demo data
    setLoading(true);
    setTimeout(() => {
      if (realData.length > 0) {
        setData(realData);
      } else {
        setData(demoPriceHistory);
      }
      setLoading(false);
    }, 1000);
  }, [timeFrame, realData]);

  const handleTimeFrameChange = (newTimeFrame: string) => {
    setTimeFrame(newTimeFrame as '1m' | '1h' | '3h' | '1D' | '1W' | '1M' | '1Y' | '5Y');
  };

  const handleChartTypeChange = (newChartType: string) => {
    setChartType(newChartType as any);
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 mb-2">Institutional Financial Markets Chart</h1>
              <p className="text-gray-600">
                Professional charting platform for institutional financial instruments (Forex, Stocks, Commodities) with advanced drawing tools for technical analysis. 
                No fees, institutional-grade infrastructure. Use the drawing toolbar to add trend lines, support/resistance levels, Fibonacci retracements, and more.
              </p>
            </div>
            <div className="flex items-center space-x-3">
              <div className="flex items-center space-x-2">
                <div className={`w-2 h-2 rounded-full ${
                  dataSourceStatus === 'real' ? 'bg-green-500' : 'bg-yellow-500'
                }`}></div>
                <span className="text-sm text-gray-600">
                  {dataSourceStatus === 'real' ? 'Real-time Data' : 'Demo Data'}
                </span>
              </div>
              {isLoadingRealData && (
                <div className="flex items-center space-x-2">
                  <RefreshCw className="w-4 h-4 animate-spin text-blue-600" />
                  <span className="text-sm text-blue-600">Updating...</span>
                </div>
              )}
              <button 
                onClick={() => setShowDataSourceConfig(!showDataSourceConfig)}
                className="btn-secondary flex items-center space-x-2"
              >
                <Settings className="w-4 h-4" />
                <span>Data Sources</span>
              </button>
              <button 
                onClick={fetchRealData}
                disabled={isLoadingRealData}
                className="btn-secondary flex items-center space-x-2"
              >
                <RefreshCw className={`w-4 h-4 ${isLoadingRealData ? 'animate-spin' : ''}`} />
                <span>Refresh</span>
              </button>
              <button 
                onClick={forceRefreshData}
                disabled={isLoadingRealData}
                className="btn-secondary flex items-center space-x-2"
                title="Force refresh (bypass cache)"
              >
                <RefreshCw className={`w-4 h-4 ${isLoadingRealData ? 'animate-spin' : ''}`} />
                <span>Force Refresh</span>
              </button>
            </div>
          </div>
          
          {/* Instrument and Time Frame Selectors */}
          <div className="flex items-center space-x-4 mb-4">
            <div className="flex items-center space-x-2">
              <label className="text-sm font-medium text-gray-700">Instrument:</label>
              <select
                value={selectedPair}
                onChange={(e) => setSelectedPair(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="EUR/USD">EUR/USD</option>
                <option value="GBP/USD">GBP/USD</option>
                <option value="USD/JPY">USD/JPY</option>
                <option value="AAPL">AAPL</option>
                <option value="MSFT">MSFT</option>
                <option value="XAU/USD">XAU/USD (Gold)</option>
                <option value="WTI/USD">WTI/USD (Oil)</option>
              </select>
            </div>
            
            <div className="flex items-center space-x-2">
              <label className="text-sm font-medium text-gray-700">Time Frame:</label>
              <select
                value={timeFrame}
                onChange={(e) => {
                  setTimeFrame(e.target.value as '1m' | '1h' | '3h' | '1D' | '1W' | '1M' | '1Y' | '5Y');
                  fetchRealData(); // Refresh data with new time frame
                }}
                className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="1m">1 Minute</option>
                <option value="1h">1 Hour</option>
                <option value="3h">3 Hours</option>
                <option value="1D">1 Day</option>
                <option value="1W">1 Week</option>
                <option value="1M">1 Month</option>
                <option value="1Y">1 Year</option>
                <option value="5Y">5 Years</option>
              </select>
            </div>
          </div>
        </div>

        {/* Data Source Configuration */}
        {showDataSourceConfig && (
          <div className="mb-6">
            <DataSourceConfig onConfigChange={fetchRealData} />
          </div>
        )}

        {/* Chart Container */}
        <div className="bg-white rounded-lg shadow-lg overflow-hidden">
          <AdvancedChart
            pair={selectedPair}
            data={data}
            timeFrame={timeFrame}
            chartType={chartType}
            onTimeFrameChange={handleTimeFrameChange}
            onChartTypeChange={handleChartTypeChange}
            loading={loading}
            showVolume={true}
            showMA={true}
            showBB={false}
            showRSI={false}
          />
        </div>

        {/* Instructions */}
        <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
          <h3 className="text-lg font-semibold text-blue-900 mb-2">How to Use Drawing Tools</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-blue-800">
            <div>
              <h4 className="font-medium mb-1">Basic Tools:</h4>
              <ul className="space-y-1">
                <li>• <strong>Trend Line:</strong> Click two points to draw a trend line</li>
                <li>• <strong>Horizontal Line:</strong> Click once to add a support/resistance level</li>
                <li>• <strong>Vertical Line:</strong> Click once to mark a time point</li>
                <li>• <strong>Rectangle:</strong> Click and drag to create a rectangle</li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium mb-1">Advanced Tools:</h4>
              <ul className="space-y-1">
                <li>• <strong>Ellipse:</strong> Click and drag to create an ellipse</li>
                <li>• <strong>Arrow:</strong> Click two points to draw an arrow</li>
                <li>• <strong>Fibonacci:</strong> Click two points for Fibonacci retracements</li>
                <li>• <strong>Text:</strong> Click to add text annotations</li>
              </ul>
            </div>
          </div>
          <div className="mt-3 text-sm text-blue-700">
            <p><strong>Tip:</strong> Use the Undo/Redo buttons to manage your drawings, and the Eraser to clear all drawings.</p>
          </div>
        </div>

        {/* Features */}
        <div className="mt-6 grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="bg-white rounded-lg p-4 shadow">
            <h3 className="font-semibold text-gray-900 mb-2">Institutional Drawing Tools</h3>
            <p className="text-gray-600 text-sm">
              Complete set of professional technical analysis drawing tools including trend lines, 
              Fibonacci retracements, support/resistance levels, and geometric shapes.
            </p>
          </div>
          <div className="bg-white rounded-lg p-4 shadow">
            <h3 className="font-semibold text-gray-900 mb-2">Institutional Markets</h3>
            <p className="text-gray-600 text-sm">
              Real-time institutional data for Forex pairs (EUR/USD, GBP/USD), Stocks (AAPL, MSFT), 
              Commodities (Gold, Oil), and Bonds with multiple timeframes. No fees.
            </p>
          </div>
          <div className="bg-white rounded-lg p-4 shadow">
            <h3 className="font-semibold text-gray-900 mb-2">Professional History</h3>
            <p className="text-gray-600 text-sm">
              Undo/Redo functionality to manage your drawings, and the ability to save
              and load chart annotations for institutional compliance.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}; 