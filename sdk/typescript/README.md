# FinDAG TypeScript SDK

A TypeScript/JavaScript SDK for interacting with the FinDAG blockchain. Supports browser, Node.js, and dApp environments.

---

## Installation
```sh
npm install findag-sdk
```

## Features
- Transaction creation and signing (including confidential and cross-shard/chain txs)
- API calls (submit tx, query state, governance, bridge, compliance, etc.)
- Wallet integration (key management, address generation, signing)
- Event and block subscription (where supported)
- Utilities for encoding/decoding, asset management, and governance

## Example Usage
```typescript
import { FinDAGClient, Wallet } from 'findag-sdk';

const client = new FinDAGClient('https://node.findag.org');
const wallet = Wallet.fromMnemonic('...');

// Create and sign a transaction
const tx = wallet.createTransaction({
  to: 'fdg1q...',
  amount: 100,
  currency: 'USD',
  shardId: 0,
});
const signedTx = wallet.signTransaction(tx);

// Submit transaction
const result = await client.submitTransaction(signedTx);
console.log(result);
```

## Block and Merkle Proof API

You can fetch a block and its Merkle root, and request a Merkle proof for a transaction:

```typescript
// Fetch a block by ID
const block = await client.getBlock('abcdef...');
console.log('Block Merkle root:', block.merkle_root);

// Fetch a Merkle proof for a transaction in the block
const proofRes = await client.getMerkleProof('abcdef...', 'txhash...');
console.log('Merkle proof:', proofRes.proof);

// Verify the Merkle proof (Node.js only)
const isValid = FinDAGClient.verifyMerkleProof(
  'txhash...', // hex string of tx hash
  proofRes.proof, // array of hex strings
  block.merkle_root, // hex string
  0 // index of tx in block.transactions
);
console.log('Proof valid?', isValid);
```

- `getBlock(blockId)` fetches block info including the Merkle root and transaction hashes.
- `getMerkleProof(blockId, txHash)` fetches a Merkle proof for a transaction in the block.
- `verifyMerkleProof(txHash, proof, root, index)` verifies the proof (Node.js only).

## Contributing
- Pull requests and issues are welcome!
- See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

For questions or support, contact the FinDAG team or open an issue on GitHub. 