import React from 'react';
import { EnhancedDAGVisualizer } from './EnhancedDAGVisualizer';

export const DAGPage: React.FC = () => {
  return (
    <div className="p-6">
      <EnhancedDAGVisualizer 
        data={{nodes: [], edges: []}}
        className=""
      />
    </div>
  );
}; 