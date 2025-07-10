import React, { useState, useCallback } from 'react';
import { EnhancedDAGVisualizer, DAGNode } from './EnhancedDAGVisualizer';
import { TransactionDetailsModal, Transaction } from './TransactionDetailsModal';
import { finDAGApi } from '../../services/api';
import { useNotifications, createNotification } from '../Common/NotificationSystem';

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

  const { addNotification } = useNotifications();

  const handleNodeClick = useCallback(async (node: DAGNode) => {
    setSelectedNode(node);
    setShowTransactionModal(true);
    
    // Load real transactions for the selected node from backend
    setLoadingTransactions(true);
    try {
      // Fetch transactions for this block/node from backend
      const response = await finDAGApi.getTransactions(1, 100); // Get first 100 transactions
      
      // Convert API response to Transaction format
      const realTransactions: Transaction[] = response.data.map((tx: any) => ({
        id: tx.id,
        hash: tx.hash,
        from: tx.from,
        to: tx.to,
        amount: tx.amount,
        currency: tx.asset || 'USD',
        timestamp: typeof tx.timestamp === 'object' ? tx.timestamp.timestamp : tx.timestamp,
        status: tx.status,
        fee: tx.fee || 0,
        nonce: parseInt(tx.id) || 0,
      }));
      
      setTransactions(realTransactions);
      
      addNotification(createNotification.success(
        'Transactions Loaded',
        `Loaded ${realTransactions.length} transactions for block ${node.id}`,
        { category: 'trading' }
      ));
    } catch (error: any) {
      addNotification(createNotification.error(
        'Transaction Error',
        error.message || 'Failed to load transactions',
        { category: 'trading' }
      ));
      setTransactions([]);
    } finally {
      setLoadingTransactions(false);
    }
  }, [addNotification]);

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