import React from 'react';
import { CheckCircle, XCircle, AlertCircle, Info } from 'lucide-react';
import { useAuthStore } from '../../store/auth';
import { useAppStore } from '../../store';

export const StatusPage: React.FC = () => {
  const { user, isAuthenticated } = useAuthStore();
  const { networkMetrics, nodeMetrics, recentBlocks, recentTransactions, validators, isLoading, errors } = useAppStore();

  const statusChecks = [
    {
      name: 'Authentication',
      status: isAuthenticated,
      details: user ? `Logged in as ${user.username} (${user.role})` : 'Not authenticated'
    },
    {
      name: 'Network Metrics',
      status: !!networkMetrics,
      details: networkMetrics ? 'Network data loaded' : 'No network data'
    },
    {
      name: 'Node Metrics',
      status: nodeMetrics.length > 0,
      details: `${nodeMetrics.length} nodes loaded`
    },
    {
      name: 'Recent Blocks',
      status: recentBlocks.length > 0,
      details: `${recentBlocks.length} blocks loaded`
    },
    {
      name: 'Recent Transactions',
      status: recentTransactions.length > 0,
      details: `${recentTransactions.length} transactions loaded`
    },
    {
      name: 'Validators',
      status: validators.length > 0,
      details: `${validators.length} validators loaded`
    },
    {
      name: 'Loading States',
      status: Object.values(isLoading).some(Boolean),
      details: `${Object.values(isLoading).filter(Boolean).length} components loading`
    },
    {
      name: 'Errors',
      status: Object.keys(errors).filter(key => errors[key as keyof typeof errors]).length === 0,
      details: `${Object.keys(errors).filter(key => errors[key as keyof typeof errors]).length} errors found`
    }
  ];

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center space-x-3 mb-6">
        <Info className="w-6 h-6 text-blue-600" />
        <h1 className="text-2xl font-bold text-gray-900">System Status</h1>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {statusChecks.map((check, index) => (
          <div key={index} className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="flex items-center justify-between mb-3">
              <h3 className="text-lg font-semibold text-gray-900">{check.name}</h3>
              {check.status ? (
                <CheckCircle className="w-5 h-5 text-green-600" />
              ) : (
                <XCircle className="w-5 h-5 text-red-600" />
              )}
            </div>
            <p className="text-sm text-gray-600">{check.details}</p>
          </div>
        ))}
      </div>

      <div className="bg-blue-50 border border-blue-200 rounded-lg p-6">
        <h3 className="text-lg font-semibold text-blue-900 mb-4">How to Access Features</h3>
        <div className="space-y-2 text-sm text-blue-800">
          <p><strong>Profile Settings:</strong> Click "Profile" in the sidebar or go to /profile</p>
          <p><strong>DAG Explorer:</strong> Click "DAG Explorer" in the sidebar or go to /dag</p>
          <p><strong>Trading Interface:</strong> Click "Trading" in the sidebar or go to /trading</p>
          <p><strong>All Other Pages:</strong> Use the sidebar navigation</p>
        </div>
      </div>

      <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-6">
        <h3 className="text-lg font-semibold text-yellow-900 mb-4">Troubleshooting</h3>
        <div className="space-y-2 text-sm text-yellow-800">
          <p>• If components aren't visible, try refreshing the page (Ctrl+F5)</p>
          <p>• Check the browser console for any JavaScript errors</p>
          <p>• Make sure you're accessing the correct URL: http://localhost:3002</p>
          <p>• Try accessing pages directly via the URL bar</p>
        </div>
      </div>
    </div>
  );
}; 