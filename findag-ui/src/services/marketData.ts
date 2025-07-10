import { PricePoint, TradingPair } from '../types';

// Configuration for different data sources
interface DataSourceConfig {
  name: string;
  baseUrl: string;
  apiKey?: string;
  delay: number; // Delay in milliseconds
  rateLimit: number; // Requests per minute
}

// Get API keys from localStorage
const getApiKey = (source: string): string | undefined => {
  try {
    const savedKeys = localStorage.getItem('findag_api_keys');
    if (savedKeys) {
      const keys = JSON.parse(savedKeys);
      return keys[source];
    }
  } catch (error) {
    console.error('Error loading API keys:', error);
  }
  return undefined;
};

// Available data sources
const DATA_SOURCES: Record<string, DataSourceConfig> = {
  alphaVantage: {
    name: 'Alpha Vantage',
    baseUrl: 'https://www.alphavantage.co/query',
    apiKey: getApiKey('alphaVantage'),
    delay: 12000, // 12 second delay for free tier
    rateLimit: 5, // 5 requests per minute for free tier
  },
  polygon: {
    name: 'Polygon.io',
    baseUrl: 'https://api.polygon.io/v2',
    apiKey: getApiKey('polygon'),
    delay: 2000, // 2 second delay
    rateLimit: 5,
  },
  finnhub: {
    name: 'Finnhub',
    baseUrl: 'https://finnhub.io/api/v1',
    apiKey: getApiKey('finnhub'),
    delay: 1000, // 1 second delay
    rateLimit: 60,
  },
  yahooFinance: {
    name: 'Yahoo Finance',
    baseUrl: 'https://query1.finance.yahoo.com/v8/finance',
    delay: 5000, // 5 second delay
    rateLimit: 10,
  },
};

// Market data service class
export class MarketDataService {
  private static instance: MarketDataService;
  private lastRequestTime: Record<string, number> = {};
  private requestCount: Record<string, number> = {};
  private cache: Record<string, { data: PricePoint[]; timestamp: number }> = {};

  private constructor() {}

  static getInstance(): MarketDataService {
    if (!MarketDataService.instance) {
      MarketDataService.instance = new MarketDataService();
    }
    return MarketDataService.instance;
  }

  // Get real-time forex data (with delay)
  async getForexData(symbol: string, interval: string = '1min'): Promise<PricePoint[]> {
    // Try Finnhub first (better for forex)
    const finnhubKey = getApiKey('finnhub');
    if (finnhubKey) {
      try {
        // Get current forex rates
        const currentResponse = await this.makeRequest(
          `https://finnhub.io/api/v1/forex/rates?base=${symbol.split('/')[0]}&token=${finnhubKey}`,
          DATA_SOURCES.finnhub
        );

        if (currentResponse && currentResponse.quote) {
          const currentPrice = currentResponse.quote[symbol.split('/')[1]] || 1.0;
          console.log(`Finnhub current price for ${symbol}:`, currentPrice);
          
          // Determine time range and resolution based on interval
          const endDate = Math.floor(Date.now() / 1000);
          let startDate: number;
          let resolution: string;
          
          switch (interval) {
            case '1m':
              startDate = endDate - (60 * 60); // 1 hour
              resolution = '1';
              break;
            case '1h':
              startDate = endDate - (24 * 60 * 60); // 1 day
              resolution = '5';
              break;
            case '3h':
              startDate = endDate - (3 * 24 * 60 * 60); // 3 days
              resolution = '15';
              break;
            case '1D':
              startDate = endDate - (7 * 24 * 60 * 60); // 7 days
              resolution = '30';
              break;
            case '1W':
              startDate = endDate - (7 * 24 * 60 * 60); // 7 days
              resolution = 'D';
              break;
            case '1M':
              startDate = endDate - (30 * 24 * 60 * 60); // 30 days
              resolution = 'D';
              break;
            case '1Y':
              startDate = endDate - (365 * 24 * 60 * 60); // 1 year
              resolution = 'W';
              break;
            case '5Y':
              startDate = endDate - (5 * 365 * 24 * 60 * 60); // 5 years
              resolution = 'M';
              break;
            default:
              startDate = endDate - (7 * 24 * 60 * 60); // 7 days
              resolution = 'D';
          }
          
          try {
            const historicalResponse = await this.makeRequest(
              `https://finnhub.io/api/v1/forex/candle?symbol=${symbol.replace('/', '')}&resolution=${resolution}&from=${startDate}&to=${endDate}&token=${finnhubKey}`,
              DATA_SOURCES.finnhub
            );

            console.log(`Finnhub historical response for ${symbol}:`, historicalResponse);

            if (historicalResponse && historicalResponse.s === 'ok' && historicalResponse.t && historicalResponse.t.length > 0) {
              // Combine historical data with current price
              const historicalData = historicalResponse.t.map((timestamp: number, index: number) => ({
                timestamp: timestamp * 1000,
                open: historicalResponse.o[index],
                high: historicalResponse.h[index],
                low: historicalResponse.l[index],
                close: historicalResponse.c[index],
                volume: historicalResponse.v[index] || 1000000,
                price: historicalResponse.c[index],
              }));

              // Add current price as the latest data point
              historicalData.push({
                timestamp: Date.now(),
                open: currentPrice,
                high: currentPrice,
                low: currentPrice,
                close: currentPrice,
                volume: 1000000,
                price: currentPrice,
              });

              console.log(`Returning ${historicalData.length} historical data points for ${symbol}`);
              return historicalData;
            } else {
              console.warn(`No historical data available for ${symbol}, creating realistic fallback`);
              return this.generateRealisticFallbackData(currentPrice, interval, symbol);
            }
          } catch (historicalError) {
            console.error(`Error fetching historical data for ${symbol}:`, historicalError);
            return this.generateRealisticFallbackData(currentPrice, interval, symbol);
          }
        }
      } catch (error) {
        console.error(`Error fetching Finnhub data for ${symbol}:`, error);
      }
    }

    // Fallback to demo data
    console.log(`Using demo data for ${symbol} (no API key or error)`);
    return this.getDemoForexData(symbol);
  }

  // Generate realistic fallback data based on interval
  private generateRealisticFallbackData(basePrice: number, interval: string, symbol: string): PricePoint[] {
    const volatility = 0.02; // 2% daily volatility
    const timestamp = Date.now();
    
    // Determine number of data points and time step based on interval
    let dataPoints: number;
    let timeStep: number;
    
    switch (interval) {
      case '1m':
        dataPoints = 60; // 60 minutes
        timeStep = 60 * 1000; // 1 minute
        break;
      case '1h':
        dataPoints = 24; // 24 hours
        timeStep = 60 * 60 * 1000; // 1 hour
        break;
      case '3h':
        dataPoints = 72; // 72 hours
        timeStep = 3 * 60 * 60 * 1000; // 3 hours
        break;
      case '1D':
        dataPoints = 7; // 7 days
        timeStep = 24 * 60 * 60 * 1000; // 1 day
        break;
      case '1W':
        dataPoints = 7; // 7 days
        timeStep = 24 * 60 * 60 * 1000; // 1 day
        break;
      case '1M':
        dataPoints = 30; // 30 days
        timeStep = 24 * 60 * 60 * 1000; // 1 day
        break;
      case '1Y':
        dataPoints = 52; // 52 weeks
        timeStep = 7 * 24 * 60 * 60 * 1000; // 1 week
        break;
      case '5Y':
        dataPoints = 60; // 60 months
        timeStep = 30 * 24 * 60 * 60 * 1000; // 1 month
        break;
      default:
        dataPoints = 100;
        timeStep = 60 * 1000; // 1 minute
    }
    
    console.log(`Generating ${dataPoints} data points for ${symbol} with ${interval} interval`);
    
    return Array.from({ length: dataPoints }, (_, i) => {
      const timeAgo = (dataPoints - i) * timeStep;
      const daysAgo = timeAgo / (24 * 60 * 60 * 1000);
      const priceChange = (Math.random() - 0.5) * volatility * Math.sqrt(daysAgo);
      const price = basePrice * (1 + priceChange);
      
      const high = price + Math.random() * price * 0.002;
      const low = price - Math.random() * price * 0.002;
      const open = price - Math.random() * price * 0.001;
      const close = price + Math.random() * price * 0.001;
      
      return {
        timestamp: timestamp - timeAgo,
        price: price,
        volume: 1000000 + Math.random() * 5000000,
        high: high,
        low: low,
        open: open,
        close: close,
      };
    });
  }

  // Get real-time stock data
  async getStockData(symbol: string, interval: string = '1min'): Promise<PricePoint[]> {
    // Try Finnhub first (better for stocks)
    const finnhubKey = getApiKey('finnhub');
    if (finnhubKey) {
      try {
        // Get current stock quote
        const currentResponse = await this.makeRequest(
          `https://finnhub.io/api/v1/quote?symbol=${symbol}&token=${finnhubKey}`,
          DATA_SOURCES.finnhub
        );

        if (currentResponse && currentResponse.c) {
          const currentPrice = currentResponse.c;
          
          // Determine time range and resolution based on interval
          const endDate = Math.floor(Date.now() / 1000);
          let startDate: number;
          let resolution: string;
          
          switch (interval) {
            case '1m':
              startDate = endDate - (60 * 60); // 1 hour
              resolution = '1';
              break;
            case '1h':
              startDate = endDate - (24 * 60 * 60); // 1 day
              resolution = '5';
              break;
            case '3h':
              startDate = endDate - (3 * 24 * 60 * 60); // 3 days
              resolution = '15';
              break;
            case '1D':
              startDate = endDate - (7 * 24 * 60 * 60); // 7 days
              resolution = '30';
              break;
            case '1W':
              startDate = endDate - (7 * 24 * 60 * 60); // 7 days
              resolution = 'D';
              break;
            case '1M':
              startDate = endDate - (30 * 24 * 60 * 60); // 30 days
              resolution = 'D';
              break;
            case '1Y':
              startDate = endDate - (365 * 24 * 60 * 60); // 1 year
              resolution = 'W';
              break;
            case '5Y':
              startDate = endDate - (5 * 365 * 24 * 60 * 60); // 5 years
              resolution = 'M';
              break;
            default:
              startDate = endDate - (7 * 24 * 60 * 60); // 7 days
              resolution = 'D';
          }
          
                      const historicalResponse = await this.makeRequest(
              `https://finnhub.io/api/v1/stock/candle?symbol=${symbol}&resolution=${resolution}&from=${startDate}&to=${endDate}&token=${finnhubKey}`,
              DATA_SOURCES.finnhub
            );

          if (historicalResponse && historicalResponse.s === 'ok' && historicalResponse.t) {
            // Combine historical data with current price
            const historicalData = historicalResponse.t.map((timestamp: number, index: number) => ({
              timestamp: timestamp * 1000,
              open: historicalResponse.o[index],
              high: historicalResponse.h[index],
              low: historicalResponse.l[index],
              close: historicalResponse.c[index],
              volume: historicalResponse.v[index] || 1000000,
              price: historicalResponse.c[index],
            }));

            // Add current price as the latest data point
            historicalData.push({
              timestamp: Date.now(),
              open: currentResponse.o || currentPrice,
              high: currentResponse.h || currentPrice,
              low: currentResponse.l || currentPrice,
              close: currentPrice,
              volume: currentResponse.v || 1000000,
              price: currentPrice,
            });

            return historicalData;
          } else {
            console.warn(`No historical data available for ${symbol}, creating realistic fallback`);
            return this.generateRealisticFallbackData(currentPrice, interval, symbol);
          }
        }
      } catch (error) {
        console.error('Finnhub stock data failed, trying Alpha Vantage:', error);
      }
    }

    // Fallback to Alpha Vantage
    const alphaVantageKey = getApiKey('alphaVantage');
    if (alphaVantageKey) {
      try {
        const response = await this.makeRequest(
          `${DATA_SOURCES.alphaVantage.baseUrl}?function=TIME_SERIES_INTRADAY&symbol=${symbol}&interval=${interval}&apikey=${alphaVantageKey}`,
          DATA_SOURCES.alphaVantage
        );

        if (response['Error Message']) {
          throw new Error(response['Error Message']);
        }

        const timeSeries = response[`Time Series (${interval})`];
        if (!timeSeries) {
          throw new Error('No data available');
        }

        return Object.entries(timeSeries).map(([timestamp, data]: [string, any]) => ({
          timestamp: new Date(timestamp).getTime(),
          open: parseFloat(data['1. open']),
          high: parseFloat(data['2. high']),
          low: parseFloat(data['3. low']),
          close: parseFloat(data['4. close']),
          volume: parseInt(data['5. volume']),
          price: parseFloat(data['4. close']),
        })).reverse();
      } catch (error) {
        console.error('Alpha Vantage stock data failed:', error);
      }
    }

    console.warn('No API keys configured, using demo data');
    return this.getDemoStockData(symbol);
  }

  // Get real-time commodity data
  async getCommodityData(symbol: string): Promise<PricePoint[]> {
    const config = DATA_SOURCES.alphaVantage;
    const apiKey = getApiKey('alphaVantage');
    
    if (!apiKey) {
      console.warn('Alpha Vantage API key not configured, using demo data');
      return this.getDemoCommodityData(symbol);
    }

    try {
      // For commodities, we might need to use different endpoints
      const response = await this.makeRequest(
        `${config.baseUrl}?function=DIGITAL_CURRENCY_INTRADAY&symbol=${symbol}&market=USD&apikey=${apiKey}`,
        config
      );

      if (response['Error Message']) {
        throw new Error(response['Error Message']);
      }

      const timeSeries = response['Time Series (Digital Currency Intraday)'];
      if (!timeSeries) {
        throw new Error('No data available');
      }

      return Object.entries(timeSeries).map(([timestamp, data]: [string, any]) => ({
        timestamp: new Date(timestamp).getTime(),
        open: parseFloat(data['1a. open (USD)']),
        high: parseFloat(data['2a. high (USD)']),
        low: parseFloat(data['3a. low (USD)']),
        close: parseFloat(data['4a. close (USD)']),
        volume: parseFloat(data['5. volume']),
        price: parseFloat(data['4a. close (USD)']),
      })).reverse();
    } catch (error) {
      console.error('Error fetching commodity data:', error);
      return this.getDemoCommodityData(symbol);
    }
  }

  // Make rate-limited request
  private async makeRequest(url: string, config: DataSourceConfig): Promise<any> {
    const now = Date.now();
    const lastRequest = this.lastRequestTime[config.name] || 0;
    const timeSinceLastRequest = now - lastRequest;

    // Check rate limiting
    if (timeSinceLastRequest < (60000 / config.rateLimit)) {
      const waitTime = (60000 / config.rateLimit) - timeSinceLastRequest;
      console.log(`Rate limiting: waiting ${waitTime}ms for ${config.name}`);
      await new Promise(resolve => setTimeout(resolve, waitTime));
    }

    // Add delay for data freshness
    console.log(`Making request to ${config.name}:`, url);
    await new Promise(resolve => setTimeout(resolve, config.delay));

    this.lastRequestTime[config.name] = Date.now();
    this.requestCount[config.name] = (this.requestCount[config.name] || 0) + 1;

    try {
      const response = await fetch(url);
      console.log(`Response status for ${config.name}:`, response.status);
      
      if (!response.ok) {
        const errorText = await response.text();
        console.error(`HTTP error for ${config.name}:`, response.status, errorText);
        throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
      }

      const data = await response.json();
      console.log(`Response data for ${config.name}:`, data);
      return data;
    } catch (error) {
      console.error(`Request failed for ${config.name}:`, error);
      throw error;
    }
  }

  // Demo data fallbacks
  private getDemoForexData(symbol: string): PricePoint[] {
    const basePrice = symbol === 'EUR/USD' ? 1.0850 : 
                     symbol === 'GBP/USD' ? 1.2650 : 
                     symbol === 'USD/JPY' ? 148.25 : 1.0850;
    
    return this.generateRealisticPriceMovement(basePrice, 100, 0.02);
  }

  private getDemoStockData(symbol: string): PricePoint[] {
    const basePrice = symbol === 'AAPL' ? 185.50 : 
                     symbol === 'MSFT' ? 415.80 : 185.50;
    
    return this.generateRealisticPriceMovement(basePrice, 100, 0.03);
  }

  private getDemoCommodityData(symbol: string): PricePoint[] {
    const basePrice = symbol === 'XAU/USD' ? 2045.50 : 
                     symbol === 'WTI/USD' ? 78.25 : 2045.50;
    
    return this.generateRealisticPriceMovement(basePrice, 100, 0.04);
  }

  // Get available data sources
  getAvailableSources(): string[] {
    return Object.keys(DATA_SOURCES);
  }

  // Get data source configuration
  getDataSourceConfig(sourceName: string): DataSourceConfig | null {
    return DATA_SOURCES[sourceName] || null;
  }

  // Check if API key is configured
  isApiKeyConfigured(sourceName: string): boolean {
    const apiKey = getApiKey(sourceName);
    return !!apiKey;
  }

  // Get API key for a source
  getApiKey(sourceName: string): string | undefined {
    return getApiKey(sourceName);
  }

  // Force refresh data (bypass cache)
  async forceRefreshData(symbol: string, type: 'forex' | 'stock' | 'commodity'): Promise<PricePoint[]> {
    // Clear any cached data for this symbol
    console.log(`Force refreshing ${type} data for ${symbol}`);
    
    // Clear the last request time to force a fresh API call
    const sourceKey = type === 'forex' || type === 'stock' ? 'finnhub' : 'alphaVantage';
    delete this.lastRequestTime[sourceKey];
    
    switch (type) {
      case 'forex':
        return this.getForexData(symbol);
      case 'stock':
        return this.getStockData(symbol);
      case 'commodity':
        return this.getCommodityData(symbol);
      default:
        return this.getForexData(symbol);
    }
  }

  // Get data source info for debugging
  getDataSourceInfo(): { source: string; hasApiKey: boolean; lastRequest: number }[] {
    return Object.keys(DATA_SOURCES).map(source => ({
      source,
      hasApiKey: !!getApiKey(source),
      lastRequest: this.lastRequestTime[source] || 0,
    }));
  }

  // Generate more realistic price movements for better updating
  private generateRealisticPriceMovement(basePrice: number, timeSteps: number, volatility: number = 0.02): PricePoint[] {
    const timestamp = Date.now();
    const dataPoints: PricePoint[] = [];
    
    for (let i = 0; i < timeSteps; i++) {
      const timeAgo = (timeSteps - i) * 60000; // 1 minute intervals
      const daysAgo = timeAgo / (24 * 60 * 60 * 1000);
      
      // Use a more realistic price movement model
      const trend = Math.sin(daysAgo * 0.5) * 0.001; // Long-term trend
      const noise = (Math.random() - 0.5) * volatility * Math.sqrt(daysAgo);
      const momentum = Math.sin(i * 0.1) * 0.0005; // Short-term momentum
      
      const price = basePrice * (1 + trend + noise + momentum);
      
      const high = price + Math.random() * price * 0.002;
      const low = price - Math.random() * price * 0.002;
      const open = price - Math.random() * price * 0.001;
      const close = price + Math.random() * price * 0.001;
      
      dataPoints.push({
        timestamp: timestamp - timeAgo,
        price: price,
        volume: 1000000 + Math.random() * 5000000,
        high: high,
        low: low,
        open: open,
        close: close,
      });
    }
    
    return dataPoints;
  }
}

// Export singleton instance
export const marketDataService = MarketDataService.getInstance(); 