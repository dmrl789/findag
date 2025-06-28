import React from 'react';
import { Trade } from '../../types';
import { formatPrice, formatNumber, formatTimestamp } from '../../utils/formatters';

interface RecentTradesProps {
  trades: Trade[];
  pair: string;
}

export const RecentTrades: React.FC<RecentTradesProps> = ({ trades, pair }) => {
  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">Recent Trades</h3>
        <span className="text-sm text-gray-500">{pair}</span>
      </div>

      {/* Headers */}
      <div className="grid grid-cols-4 gap-4 text-xs font-medium text-gray-500 mb-2">
        <div>Price</div>
        <div>Amount</div>
        <div>Time</div>
        <div>Side</div>
      </div>

      {/* Trades List */}
      <div className="space-y-1 max-h-80 overflow-y-auto">
        {trades.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            <div className="animate-pulse">
              <div className="h-4 bg-gray-200 rounded w-24 mx-auto mb-2"></div>
              <div className="h-4 bg-gray-200 rounded w-32 mx-auto"></div>
            </div>
          </div>
        ) : (
          trades.map((trade) => (
            <div
              key={trade.id}
              className="grid grid-cols-4 gap-4 text-sm py-1 hover:bg-gray-50 rounded px-2"
            >
              <div className={`font-medium ${
                trade.side === 'buy' ? 'text-success-600' : 'text-danger-600'
              }`}>
                {formatPrice(trade.price)}
              </div>
              <div className="text-gray-700">
                {formatNumber(trade.amount)}
              </div>
              <div className="text-gray-500 text-xs">
                {formatTimestamp(trade.timestamp)}
              </div>
              <div>
                <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                  trade.side === 'buy' 
                    ? 'bg-success-100 text-success-800' 
                    : 'bg-danger-100 text-danger-800'
                }`}>
                  {trade.side.toUpperCase()}
                </span>
              </div>
            </div>
          ))
        )}
      </div>

      {/* Summary */}
      {trades.length > 0 && (
        <div className="mt-4 pt-4 border-t border-gray-200">
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <div className="text-gray-500">Total Volume</div>
              <div className="font-medium">
                {formatNumber(trades.reduce((sum, trade) => sum + trade.amount, 0))}
              </div>
            </div>
            <div>
              <div className="text-gray-500">Avg Price</div>
              <div className="font-medium">
                {formatPrice(
                  trades.reduce((sum, trade) => sum + trade.price * trade.amount, 0) /
                  trades.reduce((sum, trade) => sum + trade.amount, 0)
                )}
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 