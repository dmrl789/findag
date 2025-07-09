import React, { useState, useEffect } from 'react';
import { 
  AlertTriangle, 
  TrendingUp, 
  TrendingDown, 
  DollarSign, 
  Percent, 
  Clock,
  Info,
  Calculator
} from 'lucide-react';
import { MarketOrder } from '../../types';

export interface AdvancedOrder {
  id: string;
  pair: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit' | 'stop' | 'stop-limit' | 'take-profit' | 'trailing-stop';
  amount: number;
  price?: number;
  stopPrice?: number;
  takeProfitPrice?: number;
  trailingStopPercent?: number;
  timeInForce: 'GTC' | 'IOC' | 'FOK';
  postOnly?: boolean;
  reduceOnly?: boolean;
  iceberg?: boolean;
  icebergAmount?: number;
  validUntil?: number;
  user: string;
  timestamp: number;
}

interface AdvancedOrderFormProps {
  pair: string;
  currentPrice: number;
  onOrderPlaced?: (order: AdvancedOrder) => void;
  onCancel?: () => void;
  className?: string;
}

export const AdvancedOrderForm: React.FC<AdvancedOrderFormProps> = ({
  pair,
  currentPrice,
  onOrderPlaced,
  onCancel,
  className = '',
}) => {
  const [orderType, setOrderType] = useState<AdvancedOrder['type']>('limit');
  const [side, setSide] = useState<'buy' | 'sell'>('buy');
  const [amount, setAmount] = useState('');
  const [price, setPrice] = useState('');
  const [stopPrice, setStopPrice] = useState('');
  const [takeProfitPrice, setTakeProfitPrice] = useState('');
  const [trailingStopPercent, setTrailingStopPercent] = useState('');
  const [timeInForce, setTimeInForce] = useState<'GTC' | 'IOC' | 'FOK'>('GTC');
  const [postOnly, setPostOnly] = useState(false);
  const [reduceOnly, setReduceOnly] = useState(false);
  const [iceberg, setIceberg] = useState(false);
  const [icebergAmount, setIcebergAmount] = useState('');
  const [validUntil, setValidUntil] = useState('');
  const [loading, setLoading] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const [baseAsset, quoteAsset] = pair.split('/');

  useEffect(() => {
    // Set default price based on current market price
    if (!price) {
      setPrice(currentPrice.toString());
    }
  }, [currentPrice, price]);

  const validateForm = () => {
    const newErrors: Record<string, string> = {};

    if (!amount || parseFloat(amount) <= 0) {
      newErrors.amount = 'Amount must be greater than 0';
    }

    if (orderType !== 'market' && (!price || parseFloat(price) <= 0)) {
      newErrors.price = 'Price must be greater than 0';
    }

    if (orderType === 'stop' || orderType === 'stop-limit') {
      if (!stopPrice || parseFloat(stopPrice) <= 0) {
        newErrors.stopPrice = 'Stop price must be greater than 0';
      }
    }

    if (orderType === 'take-profit') {
      if (!takeProfitPrice || parseFloat(takeProfitPrice) <= 0) {
        newErrors.takeProfitPrice = 'Take profit price must be greater than 0';
      }
    }

    if (orderType === 'trailing-stop') {
      if (!trailingStopPercent || parseFloat(trailingStopPercent) <= 0) {
        newErrors.trailingStopPercent = 'Trailing stop percentage must be greater than 0';
      }
    }

    if (iceberg && (!icebergAmount || parseFloat(icebergAmount) <= 0)) {
      newErrors.icebergAmount = 'Iceberg amount must be greater than 0';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!validateForm()) {
      return;
    }

    setLoading(true);

    try {
      const order: AdvancedOrder = {
        id: `order_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        pair,
        side,
        type: orderType,
        amount: parseFloat(amount),
        price: price ? parseFloat(price) : undefined,
        stopPrice: stopPrice ? parseFloat(stopPrice) : undefined,
        takeProfitPrice: takeProfitPrice ? parseFloat(takeProfitPrice) : undefined,
        trailingStopPercent: trailingStopPercent ? parseFloat(trailingStopPercent) : undefined,
        timeInForce,
        postOnly,
        reduceOnly,
        iceberg,
        icebergAmount: icebergAmount ? parseFloat(icebergAmount) : undefined,
        validUntil: validUntil ? new Date(validUntil).getTime() : undefined,
        user: 'current_user', // This would come from auth context
        timestamp: Date.now(),
      };

      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));

      onOrderPlaced?.(order);
    } catch (error) {
      console.error('Failed to place order:', error);
      setErrors({ submit: 'Failed to place order. Please try again.' });
    } finally {
      setLoading(false);
    }
  };

  const calculateTotal = () => {
    if (!amount || !price) return 0;
    return parseFloat(amount) * parseFloat(price);
  };

  const getOrderTypeDescription = () => {
    switch (orderType) {
      case 'market':
        return 'Execute immediately at current market price';
      case 'limit':
        return 'Execute only at specified price or better';
      case 'stop':
        return 'Execute when price reaches stop level';
      case 'stop-limit':
        return 'Stop order that becomes a limit order when triggered';
      case 'take-profit':
        return 'Automatically close position at profit target';
      case 'trailing-stop':
        return 'Stop order that follows price movement';
      default:
        return '';
    }
  };

  const getRiskWarning = () => {
    if (orderType === 'market') {
      return 'Market orders may execute at prices different from current market price due to slippage.';
    }
    if (orderType === 'stop' || orderType === 'stop-limit') {
      return 'Stop orders may not execute if market gaps beyond the stop price.';
    }
    return null;
  };

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 ${className}`}>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Advanced Order
        </h2>
        <div className="text-sm text-gray-500 dark:text-gray-400">
          {pair}
        </div>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        {/* Order Type Selection */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Order Type
          </label>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-2">
            {[
              { value: 'limit', label: 'Limit', icon: TrendingUp },
              { value: 'market', label: 'Market', icon: DollarSign },
              { value: 'stop', label: 'Stop', icon: AlertTriangle },
              { value: 'stop-limit', label: 'Stop Limit', icon: TrendingDown },
              { value: 'take-profit', label: 'Take Profit', icon: TrendingUp },
              { value: 'trailing-stop', label: 'Trailing Stop', icon: TrendingDown },
            ].map(({ value, label, icon: Icon }) => (
              <button
                key={value}
                type="button"
                onClick={() => setOrderType(value as AdvancedOrder['type'])}
                className={`flex items-center justify-center p-3 rounded-lg border-2 transition-colors ${
                  orderType === value
                    ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300'
                    : 'border-gray-200 dark:border-gray-600 hover:border-gray-300 dark:hover:border-gray-500'
                }`}
              >
                <Icon className="w-4 h-4 mr-2" />
                <span className="text-sm font-medium">{label}</span>
              </button>
            ))}
          </div>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {getOrderTypeDescription()}
          </p>
        </div>

        {/* Side Selection */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Side
          </label>
          <div className="grid grid-cols-2 gap-2">
            <button
              type="button"
              onClick={() => setSide('buy')}
              className={`p-3 rounded-lg border-2 transition-colors ${
                side === 'buy'
                  ? 'border-green-500 bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-300'
                  : 'border-gray-200 dark:border-gray-600 hover:border-gray-300 dark:hover:border-gray-500'
              }`}
            >
              <div className="flex items-center justify-center">
                <TrendingUp className="w-4 h-4 mr-2" />
                <span className="font-medium">Buy</span>
              </div>
            </button>
            <button
              type="button"
              onClick={() => setSide('sell')}
              className={`p-3 rounded-lg border-2 transition-colors ${
                side === 'sell'
                  ? 'border-red-500 bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-300'
                  : 'border-gray-200 dark:border-gray-600 hover:border-gray-300 dark:hover:border-gray-500'
              }`}
            >
              <div className="flex items-center justify-center">
                <TrendingDown className="w-4 h-4 mr-2" />
                <span className="font-medium">Sell</span>
              </div>
            </button>
          </div>
        </div>

        {/* Amount */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Amount ({baseAsset})
          </label>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${
              errors.amount ? 'border-red-500' : 'border-gray-300'
            }`}
            placeholder="0.00"
            step="0.000001"
            min="0"
          />
          {errors.amount && (
            <p className="mt-1 text-sm text-red-600 dark:text-red-400">{errors.amount}</p>
          )}
        </div>

        {/* Price Fields */}
        {orderType !== 'market' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Price ({quoteAsset})
            </label>
            <input
              type="number"
              value={price}
              onChange={(e) => setPrice(e.target.value)}
              className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${
                errors.price ? 'border-red-500' : 'border-gray-300'
              }`}
              placeholder="0.00"
              step="0.000001"
              min="0"
            />
            {errors.price && (
              <p className="mt-1 text-sm text-red-600 dark:text-red-400">{errors.price}</p>
            )}
          </div>
        )}

        {/* Stop Price */}
        {(orderType === 'stop' || orderType === 'stop-limit') && (
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Stop Price ({quoteAsset})
            </label>
            <input
              type="number"
              value={stopPrice}
              onChange={(e) => setStopPrice(e.target.value)}
              className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${
                errors.stopPrice ? 'border-red-500' : 'border-gray-300'
              }`}
              placeholder="0.00"
              step="0.000001"
              min="0"
            />
            {errors.stopPrice && (
              <p className="mt-1 text-sm text-red-600 dark:text-red-400">{errors.stopPrice}</p>
            )}
          </div>
        )}

        {/* Take Profit Price */}
        {orderType === 'take-profit' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Take Profit Price ({quoteAsset})
            </label>
            <input
              type="number"
              value={takeProfitPrice}
              onChange={(e) => setTakeProfitPrice(e.target.value)}
              className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${
                errors.takeProfitPrice ? 'border-red-500' : 'border-gray-300'
              }`}
              placeholder="0.00"
              step="0.000001"
              min="0"
            />
            {errors.takeProfitPrice && (
              <p className="mt-1 text-sm text-red-600 dark:text-red-400">{errors.takeProfitPrice}</p>
            )}
          </div>
        )}

        {/* Trailing Stop Percentage */}
        {orderType === 'trailing-stop' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Trailing Stop Percentage (%)
            </label>
            <input
              type="number"
              value={trailingStopPercent}
              onChange={(e) => setTrailingStopPercent(e.target.value)}
              className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${
                errors.trailingStopPercent ? 'border-red-500' : 'border-gray-300'
              }`}
              placeholder="5.0"
              step="0.1"
              min="0"
              max="100"
            />
            {errors.trailingStopPercent && (
              <p className="mt-1 text-sm text-red-600 dark:text-red-400">{errors.trailingStopPercent}</p>
            )}
          </div>
        )}

        {/* Advanced Options */}
        <div className="border-t pt-6">
          <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
            Advanced Options
          </h3>
          
          <div className="space-y-4">
            {/* Time in Force */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Time in Force
              </label>
              <select
                value={timeInForce}
                onChange={(e) => setTimeInForce(e.target.value as any)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
              >
                <option value="GTC">Good Till Cancelled (GTC)</option>
                <option value="IOC">Immediate or Cancel (IOC)</option>
                <option value="FOK">Fill or Kill (FOK)</option>
              </select>
            </div>

            {/* Order Options */}
            <div className="space-y-3">
              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={postOnly}
                  onChange={(e) => setPostOnly(e.target.checked)}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                  Post Only (Maker Order)
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={reduceOnly}
                  onChange={(e) => setReduceOnly(e.target.checked)}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                  Reduce Only
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={iceberg}
                  onChange={(e) => setIceberg(e.target.checked)}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                  Iceberg Order
                </span>
              </label>
            </div>

            {/* Iceberg Amount */}
            {iceberg && (
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Iceberg Amount ({baseAsset})
                </label>
                <input
                  type="number"
                  value={icebergAmount}
                  onChange={(e) => setIcebergAmount(e.target.value)}
                  className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${
                    errors.icebergAmount ? 'border-red-500' : 'border-gray-300'
                  }`}
                  placeholder="0.00"
                  step="0.000001"
                  min="0"
                />
                {errors.icebergAmount && (
                  <p className="mt-1 text-sm text-red-600 dark:text-red-400">{errors.icebergAmount}</p>
                )}
              </div>
            )}

            {/* Valid Until */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Valid Until (Optional)
              </label>
              <input
                type="datetime-local"
                value={validUntil}
                onChange={(e) => setValidUntil(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
        </div>

        {/* Order Summary */}
        <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
          <h4 className="text-sm font-medium text-gray-900 dark:text-white mb-3">
            Order Summary
          </h4>
          <div className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Type:</span>
              <span className="font-medium text-gray-900 dark:text-white capitalize">
                {orderType.replace('-', ' ')}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Side:</span>
              <span className={`font-medium ${side === 'buy' ? 'text-green-600' : 'text-red-600'}`}>
                {side.toUpperCase()}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Amount:</span>
              <span className="font-medium text-gray-900 dark:text-white">
                {amount || '0'} {baseAsset}
              </span>
            </div>
            {price && (
              <div className="flex justify-between">
                <span className="text-gray-600 dark:text-gray-400">Price:</span>
                <span className="font-medium text-gray-900 dark:text-white">
                  {price} {quoteAsset}
                </span>
              </div>
            )}
            {calculateTotal() > 0 && (
              <div className="flex justify-between border-t pt-2">
                <span className="text-gray-600 dark:text-gray-400">Total:</span>
                <span className="font-medium text-gray-900 dark:text-white">
                  {calculateTotal().toFixed(6)} {quoteAsset}
                </span>
              </div>
            )}
          </div>
        </div>

        {/* Risk Warning */}
        {getRiskWarning() && (
          <div className="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
            <div className="flex">
              <AlertTriangle className="w-5 h-5 text-yellow-400 mt-0.5" />
              <div className="ml-3">
                <p className="text-sm text-yellow-800 dark:text-yellow-200">
                  {getRiskWarning()}
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Error Message */}
        {errors.submit && (
          <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
            <p className="text-sm text-red-800 dark:text-red-200">{errors.submit}</p>
          </div>
        )}

        {/* Action Buttons */}
        <div className="flex space-x-3">
          <button
            type="button"
            onClick={onCancel}
            className="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={loading}
            className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors"
          >
            {loading ? 'Placing Order...' : `Place ${side.toUpperCase()} Order`}
          </button>
        </div>
      </form>
    </div>
  );
}; 