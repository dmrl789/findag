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

## Contributing
- Pull requests and issues are welcome!
- See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

For questions or support, contact the FinDAG team or open an issue on GitHub. 