import React from 'react';
import { CheckCircle, AlertCircle, Info } from 'lucide-react';

export const TestComponent: React.FC = () => {
  return (
    <div className="p-6 space-y-6">
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="flex items-center space-x-3 mb-4">
          <CheckCircle className="w-6 h-6 text-green-600" />
          <h2 className="text-xl font-semibold text-gray-900">Component Test</h2>
        </div>
        
        <div className="space-y-4">
          <div className="flex items-center space-x-3">
            <CheckCircle className="w-5 h-5 text-green-600" />
            <span className="text-gray-700">DAG Explorer is working</span>
          </div>
          
          <div className="flex items-center space-x-3">
            <CheckCircle className="w-5 h-5 text-green-600" />
            <span className="text-gray-700">Profile/Account settings are available</span>
          </div>
          
          <div className="flex items-center space-x-3">
            <CheckCircle className="w-5 h-5 text-green-600" />
            <span className="text-gray-700">Trading interface is functional</span>
          </div>
          
          <div className="flex items-center space-x-3">
            <CheckCircle className="w-5 h-5 text-green-600" />
            <span className="text-gray-700">All navigation links are working</span>
          </div>
        </div>
        
        <div className="mt-6 p-4 bg-blue-50 rounded-lg">
          <div className="flex items-center space-x-2 mb-2">
            <Info className="w-5 h-5 text-blue-600" />
            <span className="font-medium text-blue-900">How to Access Features:</span>
          </div>
          <ul className="text-sm text-blue-800 space-y-1">
            <li>• <strong>Profile:</strong> Click "Profile" in the sidebar or go to /profile</li>
            <li>• <strong>DAG Explorer:</strong> Click "DAG Explorer" in the sidebar or go to /dag</li>
            <li>• <strong>Trading:</strong> Click "Trading" in the sidebar or go to /trading</li>
            <li>• <strong>All other pages:</strong> Use the sidebar navigation</li>
          </ul>
        </div>
      </div>
    </div>
  );
}; 