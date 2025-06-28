# FinDAG Developer SDK Overview

FinDAG will provide SDKs in popular languages to accelerate integration, dApp development, and ecosystem growth.

---

## Prerequisites & Setup
- **Node.js & npm:** Download and install from [nodejs.org](https://nodejs.org/)
- **TypeScript:** Installed as a dev dependency in the SDK
- **Windows users:**
  - If you see errors about script execution policy, run PowerShell as administrator and execute:
    ```powershell
    Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
    ```
  - Restart your terminal after changing the policy.

## Installing SDK Dependencies
Navigate to the SDK directory and run:
```sh
npm install --save-dev @types/bip39 @types/node buffer
```

### ed25519-hd-key TypeScript Support
There is no official `@types/ed25519-hd-key` package. To silence TypeScript errors, create a file at `src/ed25519-hd-key.d.ts` with:
```typescript
declare module 'ed25519-hd-key' {
  export function derivePath(path: string, seed: string): { key: Buffer; chainCode: Buffer };
}
```

---

## Planned SDKs
- **TypeScript/JavaScript**: For web, Node.js, and browser-based dApps
- **Python**: For data science, automation, and backend integration
- **Java**: For enterprise and Android applications

## Core Features
- Transaction creation and signing (including confidential and cross-shard/chain txs)
- API calls (submit tx, query state, governance, bridge, compliance, etc.)
- Wallet integration (key management, address generation, signing)
- Event and block subscription (where supported)
- Utilities for encoding/decoding, asset management, and governance

## Example Usage (TypeScript)
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

## dApp Templates & Integration Guides
- Planned: Example dApps (wallet, explorer, asset issuer, governance portal)
- Planned: Integration guides for exchanges, custodians, and fintechs

## Merkle Proofs and Block Endpoints

FinDAG SDK allows you to fetch a block, get its Merkle root, and request a Merkle proof for a transaction. You can then verify the proof using the SDK:

```typescript
const block = await client.getBlock('abcdef...');
const proofRes = await client.getMerkleProof('abcdef...', 'txhash...');
const isValid = FinDAGClient.verifyMerkleProof(
  'txhash...',
  proofRes.proof,
  block.merkle_root,
  0 // index of tx in block.transactions
);
console.log('Proof valid?', isValid);
```

- `getBlock(blockId)` fetches block info including the Merkle root and transaction hashes.
- `getMerkleProof(blockId, txHash)` fetches a Merkle proof for a transaction in the block.
- `verifyMerkleProof(txHash, proof, root, index)` verifies the proof (Node.js only; browser support planned).

---

For feedback or to contribute to SDK development, contact the FinDAG team or visit the repository. 