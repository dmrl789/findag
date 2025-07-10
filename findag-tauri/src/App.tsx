import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { ThemeProvider } from './contexts/ThemeContext';
import { AuthProvider } from './contexts/AuthContext';
import { NodeProvider } from './contexts/NodeContext';
import { TradingProvider } from './contexts/TradingContext';
import { WalletProvider } from './contexts/WalletContext';

// Layout Components
import Layout from './components/Layout/Layout';
import Sidebar from './components/Layout/Sidebar';
import Header from './components/Layout/Header';

// Page Components
import Dashboard from './pages/Dashboard/Dashboard';
import Trading from './pages/Trading/Trading';
import DAGExplorer from './pages/DAGExplorer/DAGExplorer';
import Wallet from './pages/Wallet/Wallet';
import Network from './pages/Network/Network';
import Validators from './pages/Validators/Validators';
import Settings from './pages/Settings/Settings';
import Logs from './pages/Logs/Logs';

// Auth Components
import Login from './pages/Auth/Login';
import Register from './pages/Auth/Register';

// Common Components
// import LoadingSpinner from './components/Common/LoadingSpinner';
import ErrorBoundary from './components/Common/ErrorBoundary';
import NotificationSystem from './components/Common/NotificationSystem';

// Styles
import './styles/globals.css';

const App: React.FC = () => {
  return (
    <ErrorBoundary>
      <ThemeProvider>
        <AuthProvider>
          <NodeProvider>
            <TradingProvider>
              <WalletProvider>
                <Router>
                  <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-200">
                    <NotificationSystem />
                    
                    <Routes>
                      {/* Auth Routes */}
                      <Route path="/login" element={<Login />} />
                      <Route path="/register" element={<Register />} />
                      
                      {/* Protected Routes */}
                      <Route path="/" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Dashboard />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/dashboard" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Dashboard />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/trading" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Trading />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/dag" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <DAGExplorer />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/wallet" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Wallet />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/network" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Network />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/validators" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Validators />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/settings" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Settings />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      <Route path="/logs" element={
                        <Layout>
                          <Sidebar />
                          <div className="flex-1 flex flex-col">
                            <Header />
                            <main className="flex-1 p-6">
                              <Logs />
                            </main>
                          </div>
                        </Layout>
                      } />
                      
                      {/* Default redirect */}
                      <Route path="*" element={<Navigate to="/" replace />} />
                    </Routes>
                  </div>
                </Router>
              </WalletProvider>
            </TradingProvider>
          </NodeProvider>
        </AuthProvider>
      </ThemeProvider>
    </ErrorBoundary>
  );
};

export default App; 