import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useAuth } from '../Security/AuthProvider';
import LoadingSpinner from '../Common/LoadingSpinner';
import { showNotification } from '../Common/NotificationSystem';

interface ComplianceRequirement {
  id: string;
  name: string;
  description: string;
  framework: 'GDPR' | 'SOX' | 'PCI-DSS';
  status: 'compliant' | 'non-compliant' | 'pending' | 'not-applicable';
  lastChecked: number;
  nextReview: number;
  riskLevel: 'low' | 'medium' | 'high' | 'critical';
  evidence: string[];
  notes: string;
}

interface ComplianceReport {
  framework: string;
  overallStatus: 'compliant' | 'non-compliant' | 'partial';
  requirements: ComplianceRequirement[];
  lastAudit: number;
  nextAudit: number;
  complianceScore: number;
}

interface ComplianceDashboardProps {
  className?: string;
}

const ComplianceDashboard: React.FC<ComplianceDashboardProps> = ({ className = '' }) => {
  const { user, hasPermission } = useAuth();
  const [reports, setReports] = useState<ComplianceReport[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [selectedFramework, setSelectedFramework] = useState<string>('all');
  const [selectedRequirement, setSelectedRequirement] = useState<ComplianceRequirement | null>(null);

  useEffect(() => {
    fetchComplianceData();
  }, []);

  const fetchComplianceData = async () => {
    if (!user || !hasPermission('view_compliance')) return;

    setIsLoading(true);
    try {
      const result = await invoke('get_compliance_reports');
      if (result && Array.isArray(result)) {
        setReports(result as ComplianceReport[]);
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Failed to fetch compliance data',
        message: 'Unable to load compliance reports',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleUpdateRequirement = async (requirementId: string, updates: Partial<ComplianceRequirement>) => {
    try {
      const result = await invoke('update_compliance_requirement', {
        requirementId,
        updates,
      });
      
      if (result) {
        fetchComplianceData();
        showNotification({
          type: 'success',
          title: 'Requirement Updated',
          message: 'Compliance requirement has been updated',
        });
      }
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Update Failed',
        message: 'Failed to update compliance requirement',
      });
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'compliant': return 'text-green-600 bg-green-100 dark:bg-green-900/20';
      case 'non-compliant': return 'text-red-600 bg-red-100 dark:bg-red-900/20';
      case 'pending': return 'text-yellow-600 bg-yellow-100 dark:bg-yellow-900/20';
      case 'not-applicable': return 'text-gray-600 bg-gray-100 dark:bg-gray-900/20';
      default: return 'text-gray-600 bg-gray-100 dark:bg-gray-900/20';
    }
  };

  const getRiskColor = (risk: string) => {
    switch (risk) {
      case 'critical': return 'text-red-600';
      case 'high': return 'text-orange-600';
      case 'medium': return 'text-yellow-600';
      case 'low': return 'text-green-600';
      default: return 'text-gray-600';
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString();
  };

  const getOverallStatusColor = (status: string) => {
    switch (status) {
      case 'compliant': return 'text-green-600';
      case 'non-compliant': return 'text-red-600';
      case 'partial': return 'text-yellow-600';
      default: return 'text-gray-600';
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  const filteredReports = selectedFramework === 'all' 
    ? reports 
    : reports.filter(report => report.framework === selectedFramework);

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Compliance Dashboard</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Monitor and manage regulatory compliance requirements
          </p>
        </div>
        
        <div className="flex items-center space-x-4">
          <select
            value={selectedFramework}
            onChange={(e) => setSelectedFramework(e.target.value)}
            className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
          >
            <option value="all">All Frameworks</option>
            <option value="GDPR">GDPR</option>
            <option value="SOX">SOX</option>
            <option value="PCI-DSS">PCI-DSS</option>
          </select>
          
          <button
            onClick={fetchComplianceData}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            Refresh
          </button>
        </div>
      </div>

      {/* Compliance Overview */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {filteredReports.map((report) => (
          <div key={report.framework} className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                {report.framework}
              </h3>
              <span className={`px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(report.overallStatus)}`}>
                {report.overallStatus}
              </span>
            </div>
            
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Compliance Score</span>
                <span className={`text-lg font-bold ${getOverallStatusColor(report.overallStatus)}`}>
                  {report.complianceScore}%
                </span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Requirements</span>
                <span className="text-sm text-gray-900 dark:text-white">
                  {report.requirements.length}
                </span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Last Audit</span>
                <span className="text-sm text-gray-900 dark:text-white">
                  {formatDate(report.lastAudit)}
                </span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Next Audit</span>
                <span className="text-sm text-gray-900 dark:text-white">
                  {formatDate(report.nextAudit)}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Requirements List */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
        <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
            Compliance Requirements
          </h3>
        </div>
        
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead className="bg-gray-50 dark:bg-gray-700">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Requirement
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Framework
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Risk Level
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Last Checked
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
              {filteredReports.flatMap(report => 
                report.requirements.map((requirement) => (
                  <tr key={requirement.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                    <td className="px-6 py-4">
                      <div>
                        <div className="text-sm font-medium text-gray-900 dark:text-white">
                          {requirement.name}
                        </div>
                        <div className="text-sm text-gray-500 dark:text-gray-400">
                          {requirement.description}
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                      {requirement.framework}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(requirement.status)}`}>
                        {requirement.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`text-sm font-medium ${getRiskColor(requirement.riskLevel)}`}>
                        {requirement.riskLevel}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                      {formatDate(requirement.lastChecked)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                      <button
                        onClick={() => setSelectedRequirement(requirement)}
                        className="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300"
                      >
                        View Details
                      </button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Requirement Details Modal */}
      {selectedRequirement && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white dark:bg-gray-800">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                Requirement Details
              </h3>
              <div className="space-y-3">
                <div>
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Name</label>
                  <p className="text-sm text-gray-900 dark:text-white">{selectedRequirement.name}</p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Description</label>
                  <p className="text-sm text-gray-900 dark:text-white">{selectedRequirement.description}</p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Framework</label>
                  <p className="text-sm text-gray-900 dark:text-white">{selectedRequirement.framework}</p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Status</label>
                  <select
                    value={selectedRequirement.status}
                    onChange={(e) => handleUpdateRequirement(selectedRequirement.id, { status: e.target.value as any })}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                  >
                    <option value="compliant">Compliant</option>
                    <option value="non-compliant">Non-Compliant</option>
                    <option value="pending">Pending</option>
                    <option value="not-applicable">Not Applicable</option>
                  </select>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Risk Level</label>
                  <p className="text-sm text-gray-900 dark:text-white">{selectedRequirement.riskLevel}</p>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Evidence</label>
                  <div className="text-sm text-gray-900 dark:text-white">
                    {selectedRequirement.evidence.length > 0 ? (
                      <ul className="list-disc list-inside">
                        {selectedRequirement.evidence.map((item, index) => (
                          <li key={index}>{item}</li>
                        ))}
                      </ul>
                    ) : (
                      <p className="text-gray-500">No evidence provided</p>
                    )}
                  </div>
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Notes</label>
                  <textarea
                    value={selectedRequirement.notes}
                    onChange={(e) => handleUpdateRequirement(selectedRequirement.id, { notes: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                    rows={3}
                  />
                </div>
              </div>
              <div className="mt-6 flex justify-end">
                <button
                  onClick={() => setSelectedRequirement(null)}
                  className="px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-400 dark:hover:bg-gray-500"
                >
                  Close
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default ComplianceDashboard; 