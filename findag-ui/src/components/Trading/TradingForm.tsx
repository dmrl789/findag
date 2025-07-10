import React, { useState, useEffect } from 'react';
import { ArrowUp, ArrowDown, DollarSign, Percent, AlertCircle } from 'lucide-react';
import { MarketOrder, OrderBook } from '../../types';
import { formatPrice, formatNumber } from '../../utils/formatters';
import { finDAGApi } from '../../services/api';

interface TradingFormProps {
  pair: string;
  onOrderPlaced?: () => void;
}

export const TradingForm: React.FC<TradingFormProps> = ({ pair, onOrderPlaced }) => {
  const [side, setSide] = useState<'buy' | 'sell'>('buy');
  const [orderType, setOrderType] = useState<'market' | 'limit'>('market');
  const [amount, setAmount] = useState<string>('');
  const [price, setPrice] = useState<string>('');
  const [total, setTotal] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Calculate total when amount or price changes
  useEffect(() => {
    const amountNum = parseFloat(amount) || 0;
    const priceNum = parseFloat(price) || 0;
    const calculatedTotal = amountNum * priceNum;
    setTotal(calculatedTotal.toFixed(6));
  }, [amount, price]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!amount || parseFloat(amount) <= 0) {
      alert('Please enter a valid amount');
      return;
    }

    if (orderType === 'limit' && (!price || parseFloat(price) <= 0)) {
      alert('Please enter a valid price for limit orders');
      return;
    }

    setLoading(true);
    setError(null);
    
    try {
      // Call the real API to place the order
      const order: Omit<MarketOrder, 'id' | 'timestamp' | 'status' | 'filledAmount' | 'averagePrice'> = {
        pair,
        side,
        amount: parseFloat(amount),
        price: orderType === 'limit' ? parseFloat(price) : undefined,
        type: orderType,
        user: 'current-user', // This would come from authentication
      };

      // Convert to API format
      const apiOrder = {
        symbol: order.pair,
        side: order.side,
        order_type: order.type,
        quantity: order.amount,
        price: order.price,
        client_order_id: `order-${Date.now()}`,
        currency: 'USD'
      };
      
      const response = await finDAGApi.placeOrder(apiOrder);
      const placedOrder = {
        ...order,
        id: response.order_id,
        status: response.status,
        timestamp: Date.now(),
        filledAmount: 0,
        averagePrice: 0,
      };
      console.log('Order placed successfully:', placedOrder);
      
      // Reset form
      setAmount('');
      setPrice('');
      setTotal('');
      
      // Call callback to refresh data
      onOrderPlaced?.();
      
      alert('Order placed successfully!');
    } catch (error: any) {
      const errorMessage = error.message || 'Failed to place order. Please try again.';
      setError(errorMessage);
      console.error('Failed to place order:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAmountChange = (value: string) => {
    setAmount(value);
  };

  const handlePriceChange = (value: string) => {
    setPrice(value);
  };

  const handlePercentageClick = (percentage: number) => {
    // This would calculate based on user's available balance
    const maxAmount = 1000; // Example balance
    const calculatedAmount = (maxAmount * percentage) / 100;
    setAmount(calculatedAmount.toFixed(6));
  };

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">Place Order</h3>
        <span className="text-sm text-gray-500">{pair}</span>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Buy/Sell Toggle */}
        <div className="flex bg-gray-100 rounded-lg p-1">
          <button
            type="button"
            onClick={() => setSide('buy')}
            className={`flex-1 flex items-center justify-center space-x-2 py-2 px-4 rounded-md font-medium transition-colors ${
              side === 'buy'
                ? 'bg-success-500 text-white shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            <ArrowUp className="w-4 h-4" />
            <span>Buy</span>
          </button>
          <button
            type="button"
            onClick={() => setSide('sell')}
            className={`flex-1 flex items-center justify-center space-x-2 py-2 px-4 rounded-md font-medium transition-colors ${
              side === 'sell'
                ? 'bg-danger-500 text-white shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            <ArrowDown className="w-4 h-4" />
            <span>Sell</span>
          </button>
        </div>

        {/* Order Type */}
        <div className="flex bg-gray-100 rounded-lg p-1">
          <button
            type="button"
            onClick={() => setOrderType('market')}
            className={`flex-1 py-2 px-4 text-sm font-medium rounded-md transition-colors ${
              orderType === 'market'
                ? 'bg-white text-primary-600 shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            Market
          </button>
          <button
            type="button"
            onClick={() => setOrderType('limit')}
            className={`flex-1 py-2 px-4 text-sm font-medium rounded-md transition-colors ${
              orderType === 'limit'
                ? 'bg-white text-primary-600 shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            Limit
          </button>
        </div>

        {/* Amount Input */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Amount
          </label>
          <div className="relative">
            <input
              type="number"
              value={amount}
              onChange={(e) => handleAmountChange(e.target.value)}
              placeholder="0.00"
              step="0.000001"
              min="0"
              className="input-field pr-20"
              required
            />
            <div className="absolute right-2 top-1/2 transform -translate-y-1/2 text-sm text-gray-500">
              {pair.split('/')[0]}
            </div>
          </div>
          
          {/* Percentage buttons */}
          <div className="flex space-x-2 mt-2">
            {[25, 50, 75, 100].map((percentage) => (
              <button
                key={percentage}
                type="button"
                onClick={() => handlePercentageClick(percentage)}
                className="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded transition-colors"
              >
                {percentage}%
              </button>
            ))}
          </div>
        </div>

        {/* Price Input (for limit orders) */}
        {orderType === 'limit' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Price
            </label>
            <div className="relative">
              <input
                type="number"
                value={price}
                onChange={(e) => handlePriceChange(e.target.value)}
                placeholder="0.00"
                step="0.000001"
                min="0"
                className="input-field pr-20"
                required
              />
              <div className="absolute right-2 top-1/2 transform -translate-y-1/2 text-sm text-gray-500">
                {pair.split('/')[1]}
              </div>
            </div>
          </div>
        )}

        {/* Total */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Total
          </label>
          <div className="relative">
            <input
              type="text"
              value={total}
              readOnly
              className="input-field pr-20 bg-gray-50"
            />
            <div className="absolute right-2 top-1/2 transform -translate-y-1/2 text-sm text-gray-500">
              {pair.split('/')[1]}
            </div>
          </div>
        </div>

        {/* Error Display */}
        {error && (
          <div className="rounded-md bg-danger-50 p-4">
            <div className="flex">
              <div className="flex-shrink-0">
                <AlertCircle className="h-5 w-5 text-danger-400" />
              </div>
              <div className="ml-3">
                <h3 className="text-sm font-medium text-danger-800">
                  {error}
                </h3>
              </div>
            </div>
          </div>
        )}

        {/* Submit Button */}
        <button
          type="submit"
          disabled={loading}
          className={`w-full py-3 px-4 rounded-lg font-medium transition-colors ${
            side === 'buy'
              ? 'bg-success-600 hover:bg-success-700 text-white'
              : 'bg-danger-600 hover:bg-danger-700 text-white'
          } disabled:opacity-50 disabled:cursor-not-allowed`}
        >
          {loading ? (
            <div className="flex items-center justify-center space-x-2">
              <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
              <span>Placing Order...</span>
            </div>
          ) : (
            `${side === 'buy' ? 'Buy' : 'Sell'} ${pair.split('/')[0]}`
          )}
        </button>
      </form>

      {/* Order Summary */}
      <div className="mt-6 pt-6 border-t border-gray-200">
        <h4 className="text-sm font-medium text-gray-900 mb-3">Order Summary</h4>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-gray-500">Type:</span>
            <span className="font-medium">{orderType.charAt(0).toUpperCase() + orderType.slice(1)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-500">Side:</span>
            <span className={`font-medium ${
              side === 'buy' ? 'text-success-600' : 'text-danger-600'
            }`}>
              {side.toUpperCase()}
            </span>
          </div>
          {amount && (
            <div className="flex justify-between">
              <span className="text-gray-500">Amount:</span>
              <span className="font-medium">{formatNumber(parseFloat(amount))} {pair.split('/')[0]}</span>
            </div>
          )}
          {price && orderType === 'limit' && (
            <div className="flex justify-between">
              <span className="text-gray-500">Price:</span>
              <span className="font-medium">{formatPrice(parseFloat(price))} {pair.split('/')[1]}</span>
            </div>
          )}
          {total && parseFloat(total) > 0 && (
            <div className="flex justify-between">
              <span className="text-gray-500">Total:</span>
              <span className="font-medium">{formatPrice(parseFloat(total))} {pair.split('/')[1]}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}; 