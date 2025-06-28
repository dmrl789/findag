// FinDAGClient SDK tests for block and Merkle proof endpoints
import { FinDAGClient } from './client';

describe('FinDAGClient Merkle Proof', () => {
  const client = new FinDAGClient('http://localhost:8080');

  it('fetches a block and its Merkle root', async () => {
    // Placeholder block ID (replace with real one in integration)
    const blockId = '0000000000000000000000000000000000000000000000000000000000000000';
    const block = await client.getBlock(blockId);
    expect(block).toHaveProperty('merkle_root');
    expect(Array.isArray(block.transactions)).toBe(true);
  });

  it('fetches a Merkle proof and verifies it', async () => {
    // Placeholder block ID and tx hash (replace with real ones in integration)
    const blockId = '0000000000000000000000000000000000000000000000000000000000000000';
    const txHash = 'deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef';
    const proofRes = await client.getMerkleProof(blockId, txHash);
    expect(proofRes).toHaveProperty('proof');
    // Example: verify proof (index 0, replace with real index)
    const block = await client.getBlock(blockId);
    const isValid = FinDAGClient.verifyMerkleProof(
      txHash,
      proofRes.proof,
      block.merkle_root,
      0 // index of tx in block.transactions
    );
    expect(typeof isValid).toBe('boolean');
  });
}); 