import React, { useEffect, useState } from 'react';
import { BarChart3, Shield, Zap, Clock, Settings, Plus, Save, RotateCcw } from 'lucide-react';
import { MetricsCard } from './MetricsCard';
import { LoadingSpinner, LoadingOverlay } from '../Common/LoadingSpinner';
import { useAppStore } from '../../store';
import { formatNumber, formatCurrency, formatPercentage } from '../../utils/formatters';
import { DashboardWidget, DashboardLayout, useDashboardStore } from '../../store/dashboard';
import { DraggableWidget } from '../Common/DraggableWidget';
import { LayoutSelector } from './LayoutSelector';
import { PerformanceMonitor } from '../Common/PerformanceOptimizer';

export const Dashboard: React.FC = () => {
  const {
    networkMetrics,
    nodeMetrics,
    recentBlocks,
    recentTransactions,
    currentRound,
    validators,
    isLoading,
    errors,
    fetchNetworkMetrics,
    fetchNodeMetrics,
    fetchRecentBlocks,
    fetchRecentTransactions,
    fetchValidators,
    connectWebSocket,
    disconnectWebSocket,
  } = useAppStore();

  const {
    layout,
    widgets,
    isEditMode,
    toggleEditMode,
    addWidget,
    removeWidget,
    updateWidgetPosition,
    saveLayout,
    resetLayout,
    loadLayout,
  } = useDashboardStore();

  const [showWidgetSelector, setShowWidgetSelector] = useState(false);

  useEffect(() => {
    // Connect to WebSocket and fetch initial data
    connectWebSocket();

    // Load saved dashboard layout
    loadLayout();

    // Cleanup on unmount
    return () => {
      disconnectWebSocket();
    };
  }, [connectWebSocket, disconnectWebSocket, loadLayout]);

  const handleRetry = (fetchFunction: () => Promise<void>) => {
    fetchFunction();
  };

  const handleAddWidget = (widgetType: string) => {
    addWidget(widgetType);
    setShowWidgetSelector(false);
  };

  const handleSaveLayout = () => {
    saveLayout();
    toggleEditMode();
  };

  const handleResetLayout = () => {
    resetLayout();
  };

  const renderWidget = (widget: DashboardWidget) => {
    switch (widget.type) {
      case 'network-metrics':
        return (
          <LoadingOverlay isLoading={isLoading.networkMetrics} text="Loading metrics...">
            <MetricsCard
              title="Total Transactions"
              value={networkMetrics ? formatNumber(networkMetrics.totalTransactions) : '0'}
              trend={networkMetrics?.transactionGrowth ? {
                value: networkMetrics.transactionGrowth,
                isPositive: networkMetrics.transactionGrowth >= 0
              } : undefined}
              icon={BarChart3}
              color="primary"
            />
          </LoadingOverlay>
        );

      case 'active-validators':
        return (
          <LoadingOverlay isLoading={isLoading.networkMetrics} text="Loading metrics...">
            <MetricsCard
              title="Active Validators"
              value={networkMetrics ? networkMetrics.activeValidators.toString() : '0'}
              trend={networkMetrics?.validatorGrowth ? {
                value: networkMetrics.validatorGrowth,
                isPositive: networkMetrics.validatorGrowth >= 0
              } : undefined}
              icon={Shield}
              color="success"
            />
          </LoadingOverlay>
        );

      case 'hash-rate':
        return (
          <LoadingOverlay isLoading={isLoading.networkMetrics} text="Loading metrics...">
            <MetricsCard
              title="Network Hash Rate"
              value={networkMetrics ? `${formatNumber(networkMetrics.hashRate)} H/s` : '0 H/s'}
              trend={networkMetrics?.hashRateGrowth ? {
                value: networkMetrics.hashRateGrowth,
                isPositive: networkMetrics.hashRateGrowth >= 0
              } : undefined}
              icon={Zap}
              color="warning"
            />
          </LoadingOverlay>
        );

      case 'block-time':
        return (
          <LoadingOverlay isLoading={isLoading.networkMetrics} text="Loading metrics...">
            <MetricsCard
              title="Average Block Time"
              value={networkMetrics ? `${networkMetrics.averageBlockTime.toFixed(2)}s` : '0s'}
              trend={networkMetrics?.blockTimeChange ? {
                value: networkMetrics.blockTimeChange,
                isPositive: networkMetrics.blockTimeChange <= 0
              } : undefined}
              icon={Clock}
              color="gray"
            />
          </LoadingOverlay>
        );

      case 'recent-blocks':
        return (
          <div className="bg-white rounded-lg shadow">
            <div className="p-6 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">Recent Blocks</h2>
            </div>
            <LoadingOverlay isLoading={isLoading.blocks} text="Loading blocks...">
              <div className="p-6">
                {errors.blocks ? (
                  <div className="text-center py-4">
                    <p className="text-red-600 mb-2">{errors.blocks}</p>
                    <button
                      onClick={() => handleRetry(fetchRecentBlocks)}
                      className="px-4 py-2 bg-primary-600 text-white rounded hover:bg-primary-700"
                    >
                      Retry
                    </button>
                  </div>
                ) : recentBlocks.length > 0 ? (
                  <div className="space-y-3">
                    {recentBlocks.slice(0, 5).map((block) => (
                      <div key={block.hash} className="flex justify-between items-center p-3 bg-gray-50 rounded">
                        <div>
                          <p className="font-medium text-gray-900">Block #{block.number}</p>
                          <p className="text-sm text-gray-600">{block.transactionCount} transactions</p>
                        </div>
                        <div className="text-right">
                          <p className="text-sm text-gray-600">{formatNumber(block.timestamp.timestamp)}</p>
                          <p className="text-xs text-gray-500">{block.validator}</p>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-4 text-gray-500">
                    No blocks available
                  </div>
                )}
              </div>
            </LoadingOverlay>
          </div>
        );

      case 'recent-transactions':
        return (
          <div className="bg-white rounded-lg shadow">
            <div className="p-6 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">Recent Transactions</h2>
            </div>
            <LoadingOverlay isLoading={isLoading.transactions} text="Loading transactions...">
              <div className="p-6">
                {errors.transactions ? (
                  <div className="text-center py-4">
                    <p className="text-red-600 mb-2">{errors.transactions}</p>
                    <button
                      onClick={() => handleRetry(fetchRecentTransactions)}
                      className="px-4 py-2 bg-primary-600 text-white rounded hover:bg-primary-700"
                    >
                      Retry
                    </button>
                  </div>
                ) : recentTransactions.length > 0 ? (
                  <div className="space-y-3">
                    {recentTransactions.slice(0, 5).map((tx) => (
                      <div key={tx.hash} className="flex justify-between items-center p-3 bg-gray-50 rounded">
                        <div>
                          <p className="font-medium text-gray-900">{tx.hash.slice(0, 8)}...</p>
                          <p className="text-sm text-gray-600">{tx.type}</p>
                        </div>
                        <div className="text-right">
                          <p className="text-sm text-gray-600">{formatCurrency(tx.amount)}</p>
                          <p className="text-xs text-gray-500">{formatNumber(tx.timestamp.timestamp)}</p>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-4 text-gray-500">
                    No transactions available
                  </div>
                )}
              </div>
            </LoadingOverlay>
          </div>
        );

      case 'current-round':
        return currentRound ? (
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">Current Round</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <p className="text-sm text-gray-600">Round Number</p>
                <p className="text-xl font-bold text-gray-900">{currentRound.number}</p>
              </div>
              <div>
                <p className="text-sm text-gray-600">Status</p>
                <p className="text-xl font-bold text-gray-900">{currentRound.status}</p>
              </div>
              <div>
                <p className="text-sm text-gray-600">Transactions</p>
                <p className="text-xl font-bold text-gray-900">{currentRound.transactionCount}</p>
              </div>
            </div>
          </div>
        ) : null;

      case 'performance-monitor':
        return <PerformanceMonitor />;

      default:
        return <div className="bg-white rounded-lg shadow p-6">Unknown widget type: {widget.type}</div>;
    }
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
          <p className="text-gray-600">Customizable blockchain overview</p>
        </div>
        <div className="flex items-center space-x-2">
          {isEditMode ? (
            <>
              <LayoutSelector />
              <button
                onClick={handleSaveLayout}
                className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-success-600 hover:bg-success-700"
              >
                <Save className="w-4 h-4 mr-2" />
                Save Layout
              </button>
              <button
                onClick={handleResetLayout}
                className="inline-flex items-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
              >
                <RotateCcw className="w-4 h-4 mr-2" />
                Reset
              </button>
            </>
          ) : (
            <>
              <LayoutSelector />
              <button
                onClick={() => setShowWidgetSelector(true)}
                className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700"
              >
                <Plus className="w-4 h-4 mr-2" />
                Add Widget
              </button>
              <button
                onClick={toggleEditMode}
                className="inline-flex items-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
              >
                <Settings className="w-4 h-4 mr-2" />
                Customize
              </button>
            </>
          )}
        </div>
      </div>

      {/* Dashboard Grid */}
      <div className={`grid gap-6 ${layout}`}>
        {widgets.map((widget) => (
          <DraggableWidget
            key={widget.id}
            id={widget.id}
            isEditMode={isEditMode}
            onRemove={() => removeWidget(widget.id)}
            onDrop={(e, targetId) => {
              const draggedWidgetId = e.dataTransfer.getData('text/plain');
              if (draggedWidgetId !== targetId) {
                updateWidgetPosition(draggedWidgetId, targetId);
              }
            }}
            size={widget.size}
          >
            {renderWidget(widget)}
          </DraggableWidget>
        ))}
      </div>

      {/* Widget Selector Modal */}
      {showWidgetSelector && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Add Widget</h3>
            <div className="space-y-3">
              <button
                onClick={() => handleAddWidget('network-metrics')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Network Metrics</div>
                <div className="text-sm text-gray-600">Total transactions and growth</div>
              </button>
              <button
                onClick={() => handleAddWidget('active-validators')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Active Validators</div>
                <div className="text-sm text-gray-600">Validator count and status</div>
              </button>
              <button
                onClick={() => handleAddWidget('hash-rate')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Hash Rate</div>
                <div className="text-sm text-gray-600">Network hash rate and growth</div>
              </button>
              <button
                onClick={() => handleAddWidget('block-time')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Block Time</div>
                <div className="text-sm text-gray-600">Average block time</div>
              </button>
              <button
                onClick={() => handleAddWidget('recent-blocks')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Recent Blocks</div>
                <div className="text-sm text-gray-600">Latest blockchain blocks</div>
              </button>
              <button
                onClick={() => handleAddWidget('recent-transactions')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Recent Transactions</div>
                <div className="text-sm text-gray-600">Latest network transactions</div>
              </button>
              <button
                onClick={() => handleAddWidget('current-round')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Current Round</div>
                <div className="text-sm text-gray-600">Active consensus round</div>
              </button>
              <button
                onClick={() => handleAddWidget('performance-monitor')}
                className="w-full text-left p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
              >
                <div className="font-medium text-gray-900">Performance Monitor</div>
                <div className="text-sm text-gray-600">System performance metrics</div>
              </button>
            </div>
            <div className="mt-6 flex justify-end">
              <button
                onClick={() => setShowWidgetSelector(false)}
                className="px-4 py-2 text-gray-600 hover:text-gray-800"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 