import React, { useState, useEffect } from 'react';
import { Clock, CheckCircle, AlertTriangle, TrendingUp, Users, Zap } from 'lucide-react';
import { useAppStore } from '../../store';
import { Round } from '../../types';
import { formatNumber, formatTimestamp } from '../../utils/formatters';

export const RoundsPage: React.FC = () => {
  const { currentRound, isLoading, fetchValidators } = useAppStore();
  const [rounds, setRounds] = useState<Round[]>([]);
  const [selectedRound, setSelectedRound] = useState<Round | null>(null);

  useEffect(() => {
    // Fetch rounds data - this would be implemented in the store
    fetchValidators(); // Placeholder for now
  }, [fetchValidators]);

  // Mock data for demonstration
  const mockRounds: Round[] = [
    {
      number: 45678,
      startTime: Date.now() - 30000,
      endTime: Date.now() - 5000,
      validators: ['validator-01', 'validator-02', 'validator-03'],
      finalizedBlocks: ['block-001', 'block-002', 'block-003'],
      status: 'finalized'
    },
    {
      number: 45677,
      startTime: Date.now() - 60000,
      endTime: Date.now() - 35000,
      validators: ['validator-01', 'validator-02', 'validator-04'],
      finalizedBlocks: ['block-004', 'block-005'],
      status: 'finalized'
    },
    {
      number: 45676,
      startTime: Date.now() - 90000,
      endTime: Date.now() - 65000,
      validators: ['validator-01', 'validator-03', 'validator-04'],
      finalizedBlocks: ['block-006', 'block-007', 'block-008'],
      status: 'finalized'
    }
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'bg-blue-100 text-blue-800';
      case 'finalized': return 'bg-green-100 text-green-800';
      case 'failed': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active': return <Clock className="w-4 h-4" />;
      case 'finalized': return <CheckCircle className="w-4 h-4" />;
      case 'failed': return <AlertTriangle className="w-4 h-4" />;
      default: return <Clock className="w-4 h-4" />;
    }
  };

  const calculateRoundDuration = (round: Round) => {
    if (!round.endTime) return 'Active';
    const duration = round.endTime - round.startTime;
    return `${Math.floor(duration / 1000)}s`;
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Rounds</h1>
          <p className="text-gray-600">Round finalization and history</p>
        </div>
        <div className="flex items-center space-x-3">
          <button className="btn-secondary flex items-center space-x-2">
            <TrendingUp className="w-4 h-4" />
            <span>Round Analytics</span>
          </button>
        </div>
      </div>

      {/* Current Round Status */}
      {currentRound && (
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-gray-900">Current Round</h2>
            <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(currentRound.status)}`}>
              {getStatusIcon(currentRound.status)}
              <span className="ml-1">{currentRound.status}</span>
            </span>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <div className="flex items-center space-x-3">
              <div className="p-2 bg-blue-100 rounded-lg">
                <Clock className="w-5 h-5 text-blue-600" />
              </div>
              <div>
                <p className="text-sm text-gray-500">Round Number</p>
                <p className="font-semibold text-gray-900">#{currentRound.number}</p>
              </div>
            </div>
            
            <div className="flex items-center space-x-3">
              <div className="p-2 bg-green-100 rounded-lg">
                <Users className="w-5 h-5 text-green-600" />
              </div>
              <div>
                <p className="text-sm text-gray-500">Validators</p>
                <p className="font-semibold text-gray-900">{currentRound.validators.length}</p>
              </div>
            </div>
            
            <div className="flex items-center space-x-3">
              <div className="p-2 bg-purple-100 rounded-lg">
                <Zap className="w-5 h-5 text-purple-600" />
              </div>
              <div>
                <p className="text-sm text-gray-500">Finalized Blocks</p>
                <p className="font-semibold text-gray-900">{currentRound.finalizedBlocks.length}</p>
              </div>
            </div>
            
            <div className="flex items-center space-x-3">
              <div className="p-2 bg-yellow-100 rounded-lg">
                <TrendingUp className="w-5 h-5 text-yellow-600" />
              </div>
              <div>
                <p className="text-sm text-gray-500">Duration</p>
                <p className="font-semibold text-gray-900">{calculateRoundDuration(currentRound)}</p>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Round History */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900">Round History</h3>
        </div>
        
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Round
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Validators
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Blocks
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Duration
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Start Time
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  End Time
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {mockRounds.map((round) => (
                <tr key={round.number} className="hover:bg-gray-50">
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm font-medium text-gray-900">
                      #{round.number}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(round.status)}`}>
                      {getStatusIcon(round.status)}
                      <span className="ml-1">{round.status}</span>
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm text-gray-900">
                      {round.validators.length}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm text-gray-900">
                      {round.finalizedBlocks.length}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm text-gray-900">
                      {calculateRoundDuration(round)}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {formatTimestamp(round.startTime)}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {round.endTime ? formatTimestamp(round.endTime) : 'Active'}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <button
                      onClick={() => setSelectedRound(round)}
                      className="text-blue-600 hover:text-blue-900"
                    >
                      View Details
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Round Details Modal */}
      {selectedRound && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">Round #{selectedRound.number} Details</h2>
              <button
                onClick={() => setSelectedRound(null)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-500">Round Number</label>
                  <p className="text-sm text-gray-900">#{selectedRound.number}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Status</label>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(selectedRound.status)}`}>
                    {getStatusIcon(selectedRound.status)}
                    <span className="ml-1">{selectedRound.status}</span>
                  </span>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Start Time</label>
                  <p className="text-sm text-gray-900">{formatTimestamp(selectedRound.startTime)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">End Time</label>
                  <p className="text-sm text-gray-900">{selectedRound.endTime ? formatTimestamp(selectedRound.endTime) : 'Active'}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Duration</label>
                  <p className="text-sm text-gray-900">{calculateRoundDuration(selectedRound)}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Validators</label>
                  <p className="text-sm text-gray-900">{selectedRound.validators.length}</p>
                </div>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-500 mb-2">Validators</label>
                <div className="space-y-1">
                  {selectedRound.validators.map((validator, index) => (
                    <div key={index} className="text-sm text-gray-900 font-mono bg-gray-50 px-2 py-1 rounded">
                      {validator}
                    </div>
                  ))}
                </div>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-500 mb-2">Finalized Blocks</label>
                <div className="space-y-1">
                  {selectedRound.finalizedBlocks.map((block, index) => (
                    <div key={index} className="text-sm text-gray-900 font-mono bg-gray-50 px-2 py-1 rounded">
                      {block}
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}; 