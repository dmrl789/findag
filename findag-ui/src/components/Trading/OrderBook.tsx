import React from 'react';
import { OrderBook as OrderBookType, OrderBookEntry } from '../../types';
import { formatPrice, formatNumber } from '../../utils/formatters';

interface OrderBookProps {
  data: OrderBookType | null;
  pair: string;
}

export const OrderBook: React.FC<OrderBookProps> = ({ data, pair }) => {
  if (!data) {
    return (
      <div className="card">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Order Book</h3>
        <div className="text-center text-gray-500 py-8">
          <div className="animate-pulse">
            <div className="h-4 bg-gray-200 rounded w-24 mx-auto mb-2"></div>
            <div className="h-4 bg-gray-200 rounded w-32 mx-auto"></div>
          </div>
        </div>
      </div>
    );
  }

  const maxTotal = Math.max(
    ...data.bids.map(bid => bid.total),
    ...data.asks.map(ask => ask.total)
  );

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">Order Book</h3>
        <span className="text-sm text-gray-500">{pair}</span>
      </div>

      {/* Headers */}
      <div className="grid grid-cols-3 gap-4 text-xs font-medium text-gray-500 mb-2">
        <div>Price</div>
        <div>Amount</div>
        <div>Total</div>
      </div>

      {/* Asks (Sell Orders) */}
      <div className="space-y-1 mb-4">
        {data.asks.slice(0, 10).map((ask, index) => (
          <div
            key={`ask-${index}`}
            className="grid grid-cols-3 gap-4 text-sm relative group cursor-pointer hover:bg-gray-50 rounded px-2 py-1"
          >
            <div className="text-danger-600 font-medium">
              {formatPrice(ask.price)}
            </div>
            <div className="text-gray-700">
              {formatNumber(ask.amount)}
            </div>
            <div className="text-gray-700">
              {formatNumber(ask.total)}
            </div>
            {/* Volume bar */}
            <div
              className="absolute right-0 top-0 bottom-0 bg-danger-100 opacity-20"
              style={{
                width: `${(ask.total / maxTotal) * 100}%`,
                zIndex: -1,
              }}
            />
          </div>
        ))}
      </div>

      {/* Spread */}
      {data.asks.length > 0 && data.bids.length > 0 && (
        <div className="border-t border-gray-200 py-2 mb-4">
          <div className="text-center">
            <div className="text-sm text-gray-500">Spread</div>
            <div className="font-medium text-gray-900">
              {formatPrice(data.asks[0].price - data.bids[0].price)}
            </div>
            <div className="text-xs text-gray-500">
              ({((data.asks[0].price - data.bids[0].price) / data.asks[0].price * 100).toFixed(2)}%)
            </div>
          </div>
        </div>
      )}

      {/* Bids (Buy Orders) */}
      <div className="space-y-1">
        {data.bids.slice(0, 10).map((bid, index) => (
          <div
            key={`bid-${index}`}
            className="grid grid-cols-3 gap-4 text-sm relative group cursor-pointer hover:bg-gray-50 rounded px-2 py-1"
          >
            <div className="text-success-600 font-medium">
              {formatPrice(bid.price)}
            </div>
            <div className="text-gray-700">
              {formatNumber(bid.amount)}
            </div>
            <div className="text-gray-700">
              {formatNumber(bid.total)}
            </div>
            {/* Volume bar */}
            <div
              className="absolute right-0 top-0 bottom-0 bg-success-100 opacity-20"
              style={{
                width: `${(bid.total / maxTotal) * 100}%`,
                zIndex: -1,
              }}
            />
          </div>
        ))}
      </div>

      {/* Last Update */}
      <div className="mt-4 pt-4 border-t border-gray-200">
        <div className="text-xs text-gray-500 text-center">
          Last update: {new Date().toLocaleTimeString()}
        </div>
      </div>
    </div>
  );
}; 