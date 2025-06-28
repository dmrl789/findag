// FinDAGClient module for FinDAG SDK
// Handles API calls to FinDAG nodes

import axios from 'axios';
import { createHash } from 'crypto'; // Node.js required for proof verification
import { Buffer } from 'buffer'; // Node.js required for proof verification

export class FinDAGClient {
  baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async submitTransaction(tx: any): Promise<any> {
    // TODO: Implement API call to submit transaction
    const response = await axios.post(`${this.baseUrl}/tx`, tx);
    return response.data;
  }

  async getBalance(address: string, asset: string, shardId = 0): Promise<any> {
    // TODO: Implement API call to query balance
    const response = await axios.get(`${this.baseUrl}/balance/${address}/${asset}?shard_id=${shardId}`);
    return response.data;
  }

  subscribeToBlocks(onBlock: (block: any) => void): void {
    // TODO: Implement block subscription (e.g., via WebSocket or polling)
    // Placeholder: Poll every 5 seconds
    setInterval(async () => {
      // TODO: Replace with real block query
      const response = await axios.get(`${this.baseUrl}/blocks/latest`);
      onBlock(response.data);
    }, 5000);
  }

  async outboundBridge(bridgeTx: any): Promise<any> {
    // Submit outbound cross-chain transfer, receive proof
    const response = await axios.post(`${this.baseUrl}/bridge/outbound`, bridgeTx);
    return response.data; // { status, tx_id, details, proof }
  }

  async inboundBridge(bridgeTx: any, proof: string): Promise<any> {
    // Finalize inbound cross-chain transfer, submit proof
    const txWithProof = { ...bridgeTx, proof };
    const response = await axios.post(`${this.baseUrl}/bridge/inbound`, txWithProof);
    return response.data; // { status, tx_id, details, proof, error }
  }

  async getBridgeStatus(txId: string): Promise<any> {
    // Query bridge status
    const response = await axios.get(`${this.baseUrl}/bridge/status/${txId}`);
    return response.data;
  }

  /**
   * Fetch a block by its hex-encoded ID
   */
  async getBlock(blockId: string): Promise<any> {
    const response = await axios.get(`${this.baseUrl}/block/${blockId}`);
    return response.data;
  }

  /**
   * Fetch a Merkle proof for a transaction in a block
   */
  async getMerkleProof(blockId: string, txHash: string): Promise<any> {
    const response = await axios.get(`${this.baseUrl}/block/${blockId}/merkle_proof/${txHash}`);
    return response.data;
  }

  /**
   * Verify a Merkle proof for a transaction
   * @param txHash - hex string of the transaction hash
   * @param proof - array of hex strings (the Merkle path)
   * @param root - hex string of the Merkle root
   * @param index - index of the transaction in the block
   * @returns true if valid, false otherwise
   * @note Requires Node.js environment (not available in browser)
   */
  static verifyMerkleProof(txHash: string, proof: string[], root: string, index: number): boolean {
    let hash = txHash;
    let idx = index;
    for (const sibling of proof) {
      const h = createHash('sha256');
      if (idx % 2 === 0) {
        h.update(Buffer.from(hash, 'hex'));
        h.update(Buffer.from(sibling, 'hex'));
      } else {
        h.update(Buffer.from(sibling, 'hex'));
        h.update(Buffer.from(hash, 'hex'));
      }
      hash = h.digest('hex');
      idx = Math.floor(idx / 2);
    }
    return hash === root;
  }

  // Example usage flow (end-to-end cross-chain with proof):
  // const client = new FinDAGClient('http://localhost:8080');
  // const bridgeTx = { ... };
  // const outRes = await client.outboundBridge(bridgeTx);
  // const status = await client.getBridgeStatus(outRes.tx_id);
  // const inRes = await client.inboundBridge(bridgeTx, outRes.proof);
  // const inStatus = await client.getBridgeStatus(inRes.tx_id);

  // TODO: Add methods for governance, confidential, etc.
} 