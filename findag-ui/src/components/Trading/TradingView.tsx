import React, { useState, useEffect } from 'react';
import { AdvancedTradingView } from './AdvancedTradingView';
import { PricePoint, Trade, OrderBook } from '../../types';
import { finDAGApi } from '../../services/api';
import { useNotifications, createNotification } from '../Common/NotificationSystem';

interface TradingViewProps {
  pair: string;
}

export const TradingView: React.FC<TradingViewProps> = ({ pair }) => {
  const [priceData, setPriceData] = useState<PricePoint[]>([]);
  const [trades, setTrades] = useState<Trade[]>([]);
  const [orderBook, setOrderBook] = useState<OrderBook | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [connectionStatus, setConnectionStatus] = useState<'connected' | 'disconnected' | 'connecting'>('disconnected');

  const { addNotification } = useNotifications();

  // Load initial data from backend
  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      setError(null);

      try {
        // Load price history, recent trades, and order book from backend
        const [priceHistory, recentTrades, orderBookData] = await Promise.all([
          finDAGApi.getPriceHistory(pair, '1h', 1000),
          finDAGApi.getRecentTrades(pair, 100),
          finDAGApi.getOrderBook(pair, 20)
        ]);

        setPriceData(priceHistory);
        setTrades(recentTrades);
        setOrderBook(orderBookData);

        addNotification(createNotification.success(
          'Data Loaded',
          `Successfully loaded trading data for ${pair}`,
          { category: 'trading' }
        ));
      } catch (error: any) {
        const errorMessage = error.message || 'Failed to load trading data';
        setError(errorMessage);
        
        addNotification(createNotification.error(
          'Data Load Error',
          errorMessage,
          { category: 'trading' }
        ));
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [pair, addNotification]);

  // WebSocket connection for real-time updates
  useEffect(() => {
    // Connect to WebSocket
    finDAGApi.connectWebSocket();
    
    // Subscribe to real-time data for this pair
    finDAGApi.subscribeToPair(pair);

    // Set up event listeners for real-time updates
    const handlePriceUpdate = (event: any) => {
      if (event.data && event.data.pair === pair) {
        setPriceData(prev => [...prev.slice(-999), event.data]);
        
        addNotification(createNotification.info(
          'Price Update',
          `${pair} price: ${event.data.price}`,
          { category: 'trading', priority: 'low' }
        ));
      }
    };

    const handleTradeUpdate = (event: any) => {
      if (event.data && event.data.pair === pair) {
        setTrades(prev => [event.data, ...prev.slice(0, 99)]);
        
        addNotification(createNotification.trade(
          'Trade Executed',
          `${event.data.side.toUpperCase()} ${event.data.amount} ${pair} at ${event.data.price}`,
          { category: 'trading' }
        ));
      }
    };

    const handleOrderBookUpdate = (event: any) => {
      if (event.data && event.data.pair === pair) {
        setOrderBook(event.data);
      }
    };

    const handleConnectionStatus = (event: any) => {
      setConnectionStatus(event.data.status);
      
      if (event.data.status === 'connected') {
        addNotification(createNotification.success(
          'Connected',
          'Real-time connection established',
          { category: 'system' }
        ));
      } else if (event.data.status === 'disconnected') {
        addNotification(createNotification.warning(
          'Disconnected',
          'Real-time connection lost',
          { category: 'system' }
        ));
      }
    };

    // Add event listeners
    finDAGApi.addEventListener('price', handlePriceUpdate);
    finDAGApi.addEventListener('trade', handleTradeUpdate);
    finDAGApi.addEventListener('orderbook', handleOrderBookUpdate);
    finDAGApi.addEventListener('connection_status', handleConnectionStatus);

    return () => {
      // Cleanup WebSocket connection and event listeners
      finDAGApi.removeEventListener('price', handlePriceUpdate);
      finDAGApi.removeEventListener('trade', handleTradeUpdate);
      finDAGApi.removeEventListener('orderbook', handleOrderBookUpdate);
      finDAGApi.removeEventListener('connection_status', handleConnectionStatus);
      finDAGApi.unsubscribeFromPair(pair);
    };
  }, [pair, addNotification]);

  const handleRefresh = async () => {
    setLoading(true);
    setError(null);

    try {
      // Reload data from backend
      const [priceHistory, recentTrades, orderBookData] = await Promise.all([
        finDAGApi.getPriceHistory(pair, '1h', 1000),
        finDAGApi.getRecentTrades(pair, 100),
        finDAGApi.getOrderBook(pair, 20)
      ]);

      setPriceData(priceHistory);
      setTrades(recentTrades);
      setOrderBook(orderBookData);

      addNotification(createNotification.success(
        'Data Refreshed',
        `Successfully refreshed trading data for ${pair}`,
        { category: 'trading' }
      ));
    } catch (error: any) {
      const errorMessage = error.message || 'Failed to refresh data';
      setError(errorMessage);
      
      addNotification(createNotification.error(
        'Refresh Error',
        errorMessage,
        { category: 'trading' }
      ));
    } finally {
      setLoading(false);
    }
  };

  const handleTimeFrameChange = (newTimeFrame: string) => {
    // Fetch new price data for the selected time frame
    const loadPriceData = async () => {
      try {
        const newPriceData = await finDAGApi.getPriceHistory(pair, newTimeFrame, 1000);
        setPriceData(newPriceData);
        
        addNotification(createNotification.info(
          'Timeframe Changed',
          `Chart timeframe updated to ${newTimeFrame}`,
          { category: 'trading' }
        ));
      } catch (error: any) {
        addNotification(createNotification.error(
          'Timeframe Error',
          'Failed to load data for new timeframe',
          { category: 'trading' }
        ));
      }
    };

    loadPriceData();
  };

  const handleChartTypeChange = (newChartType: string) => {
    addNotification(createNotification.info(
      'Chart Type Changed',
      `Chart type updated to ${newChartType}`,
      { category: 'trading' }
    ));
  };

  return (
    <AdvancedTradingView 
      pair={pair}
      className="w-full"
    />
  );
}; 