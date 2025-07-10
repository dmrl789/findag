import React, { useState, useEffect } from 'react';
import { 
  Shield, 
  AlertTriangle, 
  CheckCircle, 
  FileText, 
  Database, 
  Activity,
  TrendingUp,
  Clock,
  Users,
  Globe,
  BarChart3,
  Target,
  Zap,
  Eye
} from 'lucide-react';

interface ComplianceMetric {
  name: string;
  value: number;
  target: number;
  unit: string;
  status: 'good' | 'warning' | 'critical';
  trend: 'up' | 'down' | 'stable';
}

interface AuditEvent {
  id: string;
  timestamp: string;
  user: string;
  action: string;
  resource: string;
  status: 'success' | 'failure' | 'warning';
  details: string;
}

interface RiskAssessment {
  category: string;
  riskLevel: 'low' | 'medium' | 'high' | 'critical';
  probability: number;
  impact: number;
  mitigation: string;
}

export const ComplianceDashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<ComplianceMetric[]>([]);
  const [auditEvents, setAuditEvents] = useState<AuditEvent[]>([]);
  const [riskAssessments, setRiskAssessments] = useState<RiskAssessment[]>([]);
  const [selectedTimeframe, setSelectedTimeframe] = useState<'24h' | '7d' | '30d'>('24h');

  // Mock data generation
  useEffect(() => {
    const mockMetrics: ComplianceMetric[] = [
      {
        name: 'Message Validation Rate',
        value: 98.5,
        target: 99.0,
        unit: '%',
        status: 'good',
        trend: 'up'
      },
      {
        name: 'Processing Time',
        value: 2.3,
        target: 3.0,
        unit: 's',
        status: 'good',
        trend: 'down'
      },
      {
        name: 'Error Rate',
        value: 1.2,
        target: 1.0,
        unit: '%',
        status: 'warning',
        trend: 'up'
      },
      {
        name: 'System Uptime',
        value: 99.95,
        target: 99.9,
        unit: '%',
        status: 'good',
        trend: 'stable'
      },
      {
        name: 'Compliance Score',
        value: 94.8,
        target: 95.0,
        unit: '%',
        status: 'warning',
        trend: 'up'
      },
      {
        name: 'Security Incidents',
        value: 0,
        target: 0,
        unit: '',
        status: 'good',
        trend: 'stable'
      }
    ];

    const mockAuditEvents: AuditEvent[] = [
      {
        id: 'AUDIT-001',
        timestamp: '2024-01-15T14:30:00Z',
        user: 'admin@findag.com',
        action: 'Configuration Change',
        resource: 'ISO20022 Schema',
        status: 'success',
        details: 'Updated validation rules for pacs.008 messages'
      },
      {
        id: 'AUDIT-002',
        timestamp: '2024-01-15T13:45:00Z',
        user: 'compliance@findag.com',
        action: 'Risk Assessment',
        resource: 'Payment Processing',
        status: 'success',
        details: 'Completed monthly risk assessment for high-value transactions'
      },
      {
        id: 'AUDIT-003',
        timestamp: '2024-01-15T12:20:00Z',
        user: 'system@findag.com',
        action: 'Security Alert',
        resource: 'Network Access',
        status: 'warning',
        details: 'Multiple failed login attempts detected from unknown IP'
      },
      {
        id: 'AUDIT-004',
        timestamp: '2024-01-15T11:15:00Z',
        user: 'auditor@findag.com',
        action: 'Compliance Review',
        resource: 'Transaction Logs',
        status: 'success',
        details: 'Completed quarterly compliance audit for regulatory reporting'
      }
    ];

    const mockRiskAssessments: RiskAssessment[] = [
      {
        category: 'Data Security',
        riskLevel: 'low',
        probability: 0.1,
        impact: 0.3,
        mitigation: 'Encrypted data transmission, regular security audits'
      },
      {
        category: 'System Availability',
        riskLevel: 'medium',
        probability: 0.2,
        impact: 0.7,
        mitigation: 'Redundant infrastructure, automated failover systems'
      },
      {
        category: 'Regulatory Compliance',
        riskLevel: 'low',
        probability: 0.05,
        impact: 0.8,
        mitigation: 'Automated compliance monitoring, regular audits'
      },
      {
        category: 'Operational Risk',
        riskLevel: 'medium',
        probability: 0.3,
        impact: 0.5,
        mitigation: 'Process automation, staff training programs'
      }
    ];

    setMetrics(mockMetrics);
    setAuditEvents(mockAuditEvents);
    setRiskAssessments(mockRiskAssessments);
  }, []);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'good':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900';
      case 'warning':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900';
      case 'critical':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900';
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900';
    }
  };

  const getRiskColor = (level: string) => {
    switch (level) {
      case 'low':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900';
      case 'medium':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900';
      case 'high':
        return 'text-orange-600 bg-orange-100 dark:text-orange-400 dark:bg-orange-900';
      case 'critical':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900';
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900';
    }
  };

  const getTrendIcon = (trend: string) => {
    switch (trend) {
      case 'up':
        return <TrendingUp className="w-4 h-4 text-green-500" />;
      case 'down':
        return <TrendingUp className="w-4 h-4 text-red-500 transform rotate-180" />;
      default:
        return <Activity className="w-4 h-4 text-gray-500" />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <Shield className="w-8 h-8 text-blue-600" />
          <div>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
              Compliance Dashboard
            </h2>
            <p className="text-gray-600 dark:text-gray-400">
              Real-time monitoring of regulatory compliance and risk management
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-4">
          <select
            value={selectedTimeframe}
            onChange={(e) => setSelectedTimeframe(e.target.value as any)}
            className="px-3 py-2 border border-gray-300 rounded-md bg-white dark:bg-gray-800 dark:border-gray-600 text-sm"
          >
            <option value="24h">Last 24 Hours</option>
            <option value="7d">Last 7 Days</option>
            <option value="30d">Last 30 Days</option>
          </select>
        </div>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {metrics.map((metric) => (
          <div key={metric.name} className="card">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600 dark:text-gray-400">{metric.name}</p>
                <div className="flex items-center space-x-2 mt-1">
                  <p className="text-2xl font-bold text-gray-900 dark:text-white">
                    {metric.value}{metric.unit}
                  </p>
                  {getTrendIcon(metric.trend)}
                </div>
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  Target: {metric.target}{metric.unit}
                </p>
              </div>
              <div className={`px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(metric.status)}`}>
                {metric.status}
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Risk Assessment */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Risk Assessment</h3>
            <Target className="w-5 h-5 text-blue-500" />
          </div>
          <div className="space-y-3">
            {riskAssessments.map((risk) => (
              <div key={risk.category} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <div>
                  <p className="text-sm font-medium text-gray-900 dark:text-white">{risk.category}</p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    Probability: {(risk.probability * 100).toFixed(1)}% | Impact: {(risk.impact * 100).toFixed(1)}%
                  </p>
                </div>
                <span className={`px-2 py-1 rounded-full text-xs font-medium ${getRiskColor(risk.riskLevel)}`}>
                  {risk.riskLevel}
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Audit Trail */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Recent Audit Events</h3>
            <Eye className="w-5 h-5 text-blue-500" />
          </div>
          <div className="space-y-3 max-h-64 overflow-y-auto">
            {auditEvents.map((event) => (
              <div key={event.id} className="flex items-start space-x-3 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <div className={`w-2 h-2 rounded-full mt-2 ${
                  event.status === 'success' ? 'bg-green-500' : 
                  event.status === 'warning' ? 'bg-yellow-500' : 'bg-red-500'
                }`} />
                <div className="flex-1">
                  <div className="flex items-center justify-between">
                    <p className="text-sm font-medium text-gray-900 dark:text-white">{event.action}</p>
                    <span className="text-xs text-gray-500 dark:text-gray-400">
                      {new Date(event.timestamp).toLocaleTimeString()}
                    </span>
                  </div>
                  <p className="text-xs text-gray-600 dark:text-gray-400">{event.resource}</p>
                  <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">{event.details}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Compliance Alerts */}
      <div className="card">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Compliance Alerts</h3>
          <AlertTriangle className="w-5 h-5 text-yellow-500" />
        </div>
        <div className="space-y-3">
          <div className="flex items-center space-x-3 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg">
            <AlertTriangle className="w-5 h-5 text-yellow-600 dark:text-yellow-400" />
            <div>
              <p className="text-sm font-medium text-yellow-800 dark:text-yellow-200">
                Error Rate Exceeds Target
              </p>
              <p className="text-xs text-yellow-600 dark:text-yellow-400">
                Current error rate (1.2%) is above target threshold (1.0%)
              </p>
            </div>
          </div>
          <div className="flex items-center space-x-3 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
            <CheckCircle className="w-5 h-5 text-blue-600 dark:text-blue-400" />
            <div>
              <p className="text-sm font-medium text-blue-800 dark:text-blue-200">
                Monthly Compliance Review Due
              </p>
              <p className="text-xs text-blue-600 dark:text-blue-400">
                Scheduled for January 31, 2024
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <button className="card hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
          <div className="flex items-center space-x-3">
            <FileText className="w-6 h-6 text-blue-500" />
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">Generate Report</p>
              <p className="text-xs text-gray-500 dark:text-gray-400">Export compliance data</p>
            </div>
          </div>
        </button>
        <button className="card hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
          <div className="flex items-center space-x-3">
            <BarChart3 className="w-6 h-6 text-green-500" />
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">Risk Analysis</p>
              <p className="text-xs text-gray-500 dark:text-gray-400">Run risk assessment</p>
            </div>
          </div>
        </button>
        <button className="card hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
          <div className="flex items-center space-x-3">
            <Zap className="w-6 h-6 text-yellow-500" />
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">System Health</p>
              <p className="text-xs text-gray-500 dark:text-gray-400">Check system status</p>
            </div>
          </div>
        </button>
      </div>
    </div>
  );
}; 