import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Sidebar } from './components/Layout/Sidebar';
import { Dashboard } from './components/Dashboard/Dashboard';
import { AdvancedTradingView } from './components/Trading/AdvancedTradingView';
import { DAGPage } from './components/DAG/DAGPage';
import { TransactionsPage } from './components/Transactions/TransactionsPage';
import { ValidatorsPage } from './components/Validators/ValidatorsPage';
import { MetricsPage } from './components/Metrics/MetricsPage';
import { NetworkPage } from './components/Network/NetworkPage';
import { RoundsPage } from './components/Rounds/RoundsPage';
import { UserProfile } from './components/Auth/UserProfile';
import { StatusPage } from './components/Common/StatusPage';
import { ChartPage } from './components/Charts/ChartPage';
import { ISO20022Compliance } from './components/Compliance/ISO20022Compliance';
import { ComplianceDashboard } from './components/Compliance/ComplianceDashboard';
import { ThemeProvider } from './store/theme';
import { useAuthStore } from './store/auth';

function App() {
  const { checkAuth } = useAuthStore();

  React.useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  return (
    <ThemeProvider>
      <Router>
        <div className="flex h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-200">
          <Sidebar />
          <main className="flex-1 overflow-auto">
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/trading" element={<AdvancedTradingView pair="EUR/USD" />} />
              <Route path="/dag" element={<DAGPage />} />
              <Route path="/transactions" element={<TransactionsPage />} />
              <Route path="/validators" element={<ValidatorsPage />} />
              <Route path="/metrics" element={<MetricsPage />} />
              <Route path="/network" element={<NetworkPage />} />
              <Route path="/rounds" element={<RoundsPage />} />
              <Route path="/profile" element={<UserProfile />} />
              <Route path="/status" element={<StatusPage />} />
              <Route path="/charts" element={<ChartPage />} />
              <Route path="/compliance" element={<ISO20022Compliance />} />
              <Route path="/compliance-dashboard" element={<ComplianceDashboard />} />
            </Routes>
          </main>
        </div>
      </Router>
    </ThemeProvider>
  );
}

export default App; 