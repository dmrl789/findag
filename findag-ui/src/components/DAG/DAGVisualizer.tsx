import React, { useState, useCallback } from 'react';
import { EnhancedDAGVisualizer, DAGNode } from './EnhancedDAGVisualizer';
import { TransactionDetailsModal, Transaction } from './TransactionDetailsModal';

interface DAGVisualizerProps {
  data: {
    nodes: DAGNode[];
    edges: Array<{ from: string; to: string; arrows?: string }>;
  };
  className?: string;
}

export const DAGVisualizer: React.FC<DAGVisualizerProps> = ({ data, className = '' }) => {
  const [selectedNode, setSelectedNode] = useState<DAGNode | null>(null);
  const [showTransactionModal, setShowTransactionModal] = useState(false);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loadingTransactions, setLoadingTransactions] = useState(false);

  const handleNodeClick = useCallback((node: DAGNode) => {
    setSelectedNode(node);
    setShowTransactionModal(true);
    
    // Simulate loading transactions for the selected node
    setLoadingTransactions(true);
    setTimeout(() => {
      // Generate mock transactions for the node
      const mockTransactions: Transaction[] = Array.from({ length: node.transactionCount }, (_, i) => ({
        id: `${node.id}-tx-${i}`,
        hash: `0x${Math.random().toString(16).substring(2, 66)}`,
        from: `0x${Math.random().toString(16).substring(2, 42)}`,
        to: `0x${Math.random().toString(16).substring(2, 42)}`,
        amount: (Math.random() - 0.5) * 1000,
        currency: 'USD',
        timestamp: node.timestamp - Math.random() * 60000,
        status: Math.random() > 0.1 ? 'confirmed' : 'pending',
        fee: Math.random() * 10,
        nonce: i,
      }));
      setTransactions(mockTransactions);
      setLoadingTransactions(false);
    }, 1000);
  }, []);

  const handleNodeDoubleClick = useCallback((node: DAGNode) => {
    // Handle double-click - could open detailed view or perform other actions
    console.log('Double-clicked node:', node);
  }, []);

  const handleTransactionClick = useCallback((transaction: Transaction) => {
    // Handle transaction click - could open transaction details
    console.log('Clicked transaction:', transaction);
  }, []);

  const handleExportTransactions = useCallback(() => {
    if (transactions.length === 0) return;
    
    const csvContent = [
      'ID,Hash,From,To,Amount,Currency,Status,Fee,Timestamp',
      ...transactions.map(tx => 
        `${tx.id},${tx.hash},${tx.from},${tx.to},${tx.amount},${tx.currency},${tx.status},${tx.fee},${new Date(tx.timestamp).toISOString()}`
      ).join('\n')
    ].join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `transactions-${selectedNode?.id}-${Date.now()}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  }, [transactions, selectedNode]);

  return (
    <>
      <EnhancedDAGVisualizer
        data={data}
        className={className}
        onNodeClick={handleNodeClick}
        onNodeDoubleClick={handleNodeDoubleClick}
        enableAnimations={true}
        enableSearch={true}
        enableFilters={true}
        enableExport={true}
        showControls={true}
      />

      <TransactionDetailsModal
        isOpen={showTransactionModal}
        onClose={() => setShowTransactionModal(false)}
        node={selectedNode}
        transactions={transactions}
        loading={loadingTransactions}
        onTransactionClick={handleTransactionClick}
        onExportTransactions={handleExportTransactions}
      />
    </>
  );
}; 