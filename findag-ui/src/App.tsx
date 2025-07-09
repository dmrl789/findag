import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider } from './store/theme.tsx';
import { AccessibilityProvider } from './components/Common/AccessibilityProvider';
import { TimezoneProvider } from './components/Common/TimezoneProvider';
import { PerformanceProvider } from './components/Common/PerformanceOptimizer';
import { NotificationProvider, NotificationCenter } from './components/Common/NotificationSystem';
import { Sidebar } from './components/Layout/Sidebar';
import { Dashboard } from './components/Dashboard/Dashboard';
import { TradingView } from './components/Trading/TradingView';
import { DAGVisualizer } from './components/DAG/DAGVisualizer';
import { TransactionsPage } from './components/Transactions/TransactionsPage';
import { ValidatorsPage } from './components/Validators/ValidatorsPage';
import { RoundsPage } from './components/Rounds/RoundsPage';
import { NetworkPage } from './components/Network/NetworkPage';
import { MetricsPage } from './components/Metrics/MetricsPage';
import { ThemeToggle } from './components/Common/ThemeToggle';
import { ConnectionStatus } from './components/Common/ConnectionStatus';
import { AccessibilityQuickToggle } from './components/Common/AccessibilitySettings';
import { SkipLink } from './components/Common/AccessibilityProvider';
import { ErrorBoundary } from './components/Common/ErrorBoundary';
import { LoadingProvider } from './components/Common/LoadingManager';
import { KeyboardShortcuts } from './components/Common/KeyboardShortcuts';
import './index.css';

function App() {
  return (
    <ErrorBoundary>
      <ThemeProvider>
        <AccessibilityProvider>
          <TimezoneProvider>
            <PerformanceProvider>
              <NotificationProvider>
                <LoadingProvider>
                  <KeyboardShortcuts>
                    <Router>
                      <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-200">
                        {/* Skip Links for Accessibility */}
                        <SkipLink targetId="main-content">Skip to main content</SkipLink>
                        <SkipLink targetId="main-navigation">Skip to navigation</SkipLink>

                        {/* Header */}
                        <header className="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
                          <div className="flex items-center justify-between px-4 py-3">
                            <div className="flex items-center space-x-4">
                              <h1 className="text-xl font-bold text-gray-900 dark:text-white">
                                FinDAG
                              </h1>
                              <ConnectionStatus showDetails={false} />
                            </div>
                            
                            <div className="flex items-center space-x-3">
                              <NotificationCenter />
                              <AccessibilityQuickToggle />
                              <ThemeToggle />
                            </div>
                          </div>
                        </header>

                        <div className="flex">
                          {/* Sidebar */}
                          <Sidebar />
                          
                          {/* Main Content */}
                          <main id="main-content" className="flex-1 p-6">
                            <Routes>
                              <Route path="/" element={<Dashboard />} />
                              <Route path="/trading" element={<TradingView pair="BTC/USD" />} />
                              <Route path="/dag" element={<DAGVisualizer data={{ nodes: [], edges: [] }} />} />
                              <Route path="/transactions" element={<TransactionsPage />} />
                              <Route path="/validators" element={<ValidatorsPage />} />
                              <Route path="/rounds" element={<RoundsPage />} />
                              <Route path="/network" element={<NetworkPage />} />
                              <Route path="/metrics" element={<MetricsPage />} />
                            </Routes>
                          </main>
                        </div>
                      </div>
                    </Router>
                  </KeyboardShortcuts>
                </LoadingProvider>
              </NotificationProvider>
            </PerformanceProvider>
          </TimezoneProvider>
        </AccessibilityProvider>
      </ThemeProvider>
    </ErrorBoundary>
  );
}

export default App; 