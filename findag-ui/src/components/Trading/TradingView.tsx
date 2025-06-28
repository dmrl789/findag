import React, { useState, useEffect } from 'react';
import { 
  TrendingUp, 
  TrendingDown, 
  Clock, 
  DollarSign,
  BarChart3,
  Activity
} from 'lucide-react';
import { PriceChart } from './PriceChart';
import { OrderBook } from './OrderBook';
import { RecentTrades } from './RecentTrades';
import { TradingForm } from './TradingForm';
import { useAppStore } from '../../store';
import { 
  TradingPair, 
  PricePoint, 
  Trade, 
  OrderBook as OrderBookType,
  MarketOrder 
} from '../../types';
import { formatPrice, formatNumber, formatTimestamp } from '../../utils/formatters';

interface TradingViewProps {
  pair: string;
}

export const TradingView: React.FC<TradingViewProps> = ({ pair }) => {
  const [timeFrame, setTimeFrame] = useState<'1m' | '5m' | '15m' | '1h' | '4h' | '1d' | '1w'>('1h');
  const [chartType, setChartType] = useState<'line' | 'candlestick' | 'volume'>('line');
  const [priceHistory, setPriceHistory] = useState<PricePoint[]>([]);
  const [recentTrades, setRecentTrades] = useState<Trade[]>([]);
  const [orderBook, setOrderBook] = useState<OrderBookType | null>(null);
  const [tradingPair, setTradingPair] = useState<TradingPair | null>(null);
  const [loading, setLoading] = useState(true);

  const { finDAGApi } = useAppStore();

  // Load initial data
  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        const [pairData, priceData, tradesData, orderBookData] = await Promise.all([
          finDAGApi.getTradingPair(pair),
          finDAGApi.getPriceHistory(pair, timeFrame),
          finDAGApi.getRecentTrades(pair),
          finDAGApi.getOrderBook(pair),
        ]);

        setTradingPair(pairData);
        setPriceHistory(priceData);
        setRecentTrades(tradesData);
        setOrderBook(orderBookData);
      } catch (error) {
        console.error('Failed to load trading data:', error);
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [pair, timeFrame, finDAGApi]);

  // Subscribe to real-time updates
  useEffect(() => {
    finDAGApi.subscribeToPair(pair);

    const handlePriceUpdate = (event: any) => {
      if (event.type === 'price' && event.data) {
        setPriceHistory(prev => [...prev.slice(-999), event.data]);
      }
    };

    const handleTradeUpdate = (event: any) => {
      if (event.type === 'trade' && event.data) {
        setRecentTrades(prev => [event.data, ...prev.slice(0, 99)]);
      }
    };

    const handleOrderBookUpdate = (event: any) => {
      if (event.type === 'orderbook' && event.data) {
        setOrderBook(event.data);
      }
    };

    finDAGApi.addEventListener('price', handlePriceUpdate);
    finDAGApi.addEventListener('trade', handleTradeUpdate);
    finDAGApi.addEventListener('orderbook', handleOrderBookUpdate);

    return () => {
      finDAGApi.unsubscribeFromPair(pair);
      finDAGApi.removeEventListener('price', handlePriceUpdate);
      finDAGApi.removeEventListener('trade', handleTradeUpdate);
      finDAGApi.removeEventListener('orderbook', handleOrderBookUpdate);
    };
  }, [pair, finDAGApi]);

  const handleTimeFrameChange = (newTimeFrame: string) => {
    setTimeFrame(newTimeFrame as any);
  };

  const handleChartTypeChange = (newChartType: string) => {
    setChartType(newChartType as any);
  };

  if (loading) {
    return (
      <div className="p-6">
        <div className="animate-pulse space-y-6">
          <div className="h-8 bg-gray-200 rounded w-32"></div>
          <div className="h-96 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }

  if (!tradingPair) {
    return (
      <div className="p-6">
        <div className="text-center text-gray-500">
          <BarChart3 className="w-12 h-12 mx-auto mb-4 text-gray-300" />
          <p>Trading pair not found</p>
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">{pair}</h1>
          <p className="text-gray-600">Real-time trading and price data</p>
        </div>
        <div className="flex items-center space-x-4">
          <div className="text-right">
            <div className="text-lg font-semibold text-gray-900">
              {formatPrice(tradingPair.price)}
            </div>
            <div className={`flex items-center space-x-1 text-sm ${
              tradingPair.priceChange24h >= 0 ? 'text-success-600' : 'text-danger-600'
            }`}>
              {tradingPair.priceChange24h >= 0 ? (
                <TrendingUp className="w-4 h-4" />
              ) : (
                <TrendingDown className="w-4 h-4" />
              )}
              <span>
                {tradingPair.priceChange24h >= 0 ? '+' : ''}
                {formatPrice(tradingPair.priceChange24h)}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Price Chart */}
      <PriceChart
        pair={pair}
        data={priceHistory}
        timeFrame={timeFrame}
        chartType={chartType}
        onTimeFrameChange={handleTimeFrameChange}
        onChartTypeChange={handleChartTypeChange}
        loading={loading}
      />

      {/* Trading Interface */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Order Book */}
        <div className="lg:col-span-1">
          <OrderBook data={orderBook} pair={pair} />
        </div>

        {/* Recent Trades */}
        <div className="lg:col-span-1">
          <RecentTrades trades={recentTrades} pair={pair} />
        </div>

        {/* Trading Form */}
        <div className="lg:col-span-1">
          <TradingForm pair={pair} />
        </div>
      </div>

      {/* Market Statistics */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="card">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-primary-100 rounded-lg">
              <DollarSign className="w-5 h-5 text-primary-600" />
            </div>
            <div>
              <p className="text-sm text-gray-500">24h Volume</p>
              <p className="font-semibold text-gray-900">
                {formatNumber(tradingPair.volume24h)}
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-success-100 rounded-lg">
              <TrendingUp className="w-5 h-5 text-success-600" />
            </div>
            <div>
              <p className="text-sm text-gray-500">24h High</p>
              <p className="font-semibold text-gray-900">
                {formatPrice(tradingPair.price + tradingPair.priceChange24h)}
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-danger-100 rounded-lg">
              <TrendingDown className="w-5 h-5 text-danger-600" />
            </div>
            <div>
              <p className="text-sm text-gray-500">24h Low</p>
              <p className="font-semibold text-gray-900">
                {formatPrice(tradingPair.price - Math.abs(tradingPair.priceChange24h))}
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-warning-100 rounded-lg">
              <Activity className="w-5 h-5 text-warning-600" />
            </div>
            <div>
              <p className="text-sm text-gray-500">Last Trade</p>
              <p className="font-semibold text-gray-900">
                {tradingPair.lastTrade ? formatTimestamp(tradingPair.lastTrade.timestamp) : 'N/A'}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}; 