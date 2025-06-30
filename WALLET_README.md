# FinDAG Encrypted Wallet

A secure, encrypted wallet implementation for the FinDAG blockchain with password protection, similar to Bitcoin's wallet.dat functionality.

## Features

- 🔐 **Password-protected encryption** using Argon2id and AES-256-GCM
- 🗝️ **Secure private key storage** - keys are never stored in plain text
- 📱 **Multi-platform support** - Rust CLI and TypeScript SDK
- 👥 **Multi-account support** - manage multiple addresses in one wallet
- 🔄 **Import/Export functionality** - backup and restore wallets
- 🔒 **Password change capability** - update wallet password securely
- ✅ **Transaction signing** - sign transactions with Ed25519
- 🌐 **Node.js and Browser support** - TypeScript SDK works in both environments

## Security Features

- **Argon2id** for password-based key derivation (memory-hard, resistant to GPU attacks)
- **AES-256-GCM** for authenticated encryption
- **Random salt generation** for each wallet
- **Configurable iteration counts** for key derivation
- **Secure password input** (hidden input in CLI)
- **Minimum password requirements** (8+ characters)

## Installation

### Rust CLI Wallet

The encrypted wallet is included in the main FinDAG project. Build it with:

```bash
cargo build --bin encrypted_wallet
```

### TypeScript SDK

Install the SDK dependencies:

```bash
cd sdk/typescript
npm install
```

## Usage

### Rust CLI Wallet

#### Create a new wallet

```bash
cargo run --bin encrypted_wallet -- create
```

This will prompt you to enter and confirm a password, then create an encrypted wallet file (`wallet.dat` by default).

#### Show wallet information

```bash
cargo run --bin encrypted_wallet -- info
```

#### Check balance

```bash
cargo run --bin encrypted_wallet -- balance --currency USD
```

#### Send a transaction

```bash
cargo run --bin encrypted_wallet -- send --to fdg1qrecipient123 --amount 1000 --currency USD
```

#### Export private key (for backup)

```bash
cargo run --bin encrypted_wallet -- export
```

#### Import private key

```bash
cargo run --bin encrypted_wallet -- import --private-key <hex_private_key>
```

#### Add a new account

```bash
cargo run --bin encrypted_wallet -- add-account --name "savings"
```

#### List all accounts

```bash
cargo run --bin encrypted_wallet -- list-accounts
```

#### Change password

```bash
cargo run --bin encrypted_wallet -- change-password
```

### TypeScript SDK

#### Create a new wallet

```typescript
import { WalletManager } from './sdk/typescript/src/wallet';

const walletManager = new WalletManager('my_wallet');
const wallet = walletManager.createWallet('mySecurePassword123');
console.log('Wallet address:', wallet.getAddress());
```

#### Load existing wallet

```typescript
const wallet = walletManager.loadWallet('mySecurePassword123');
```

#### Sign a transaction

```typescript
const transaction = {
    from: wallet.getAddress(),
    to: 'fdg1qrecipient123',
    amount: 1000,
    currency: 'USD'
};

const signedTx = wallet.signTransaction(transaction);
```

#### Add an account

```typescript
wallet.addAccount('savings');
walletManager.saveWallet(wallet, 'mySecurePassword123');
```

#### Import from private key

```typescript
const wallet = walletManager.importWallet(privateKeyHex, 'newPassword123');
```

#### Change password

```typescript
walletManager.changePassword('oldPassword', 'newPassword');
```

## File Structure

### Rust Implementation

- `src/core/wallet.rs` - Core wallet functionality with encryption
- `src/tools/encrypted_wallet.rs` - CLI wallet application
- `Cargo.toml` - Dependencies for encryption (argon2, aes-gcm, rpassword)

### TypeScript Implementation

- `sdk/typescript/src/wallet.ts` - Complete wallet SDK with encryption
- `test-wallet.js` - Test script demonstrating all features

## Security Considerations

### Password Security

- Use strong passwords (8+ characters, mix of letters, numbers, symbols)
- Never share your password or private keys
- Consider using a password manager for wallet passwords
- The wallet file is encrypted, but losing the password means losing access

### Backup Strategy

1. **Wallet file backup**: Copy the encrypted wallet file (`wallet.dat`)
2. **Private key backup**: Export private keys and store securely (offline)
3. **Mnemonic backup**: For wallets created from mnemonics, backup the seed phrase
4. **Multiple locations**: Store backups in different secure locations

### Best Practices

- Test wallet recovery procedures regularly
- Use different passwords for different wallets
- Keep wallet software updated
- Verify transaction details before signing
- Use hardware wallets for large amounts (when available)

## Technical Details

### Encryption Algorithm

The wallet uses a two-layer encryption approach:

1. **Password Derivation**: Argon2id with configurable parameters
   - Memory cost: 64MB
   - Time cost: 3 iterations
   - Parallelism: 4 threads

2. **Data Encryption**: AES-256-GCM with random nonce
   - 256-bit key derived from password
   - 12-byte random nonce
   - Authenticated encryption with associated data

### File Format

The encrypted wallet file contains:

```rust
struct EncryptedWallet {
    version: u32,                    // File format version
    created_at: DateTime<Utc>,       // Creation timestamp
    salt: String,                    // Argon2 salt
    encrypted_data: Vec<u8>,         // AES-GCM encrypted wallet data
    nonce: Vec<u8>,                  // AES-GCM nonce
    argon2_params: Argon2Params,     // Key derivation parameters
}
```

### Supported Assets

The wallet supports all FinDAG whitelisted assets:
- Fiat currencies: EUR, USD, GBP, JPY, CHF, SGD, AED, CNY
- Government bonds: BUND, OAT, BTP, GILT, UST, JGB, T-BILL
- Money market: CP, CD
- Precious metals: XAU, XAG, XPT, XPD
- Securities: XS1234567890, FR0000120271, BE0003796134, DE0001135275
- ETFs: ETF1, UCITS1
- Cryptocurrencies: BTC, ETH, USDT, USDC

## Testing

### Rust Tests

```bash
cargo test --bin encrypted_wallet
```

### TypeScript Tests

```bash
node test-wallet.js
```

## Troubleshooting

### Common Issues

1. **"Wallet file does not exist"**
   - Create a new wallet first with the `create` command

2. **"Failed to decrypt wallet data - wrong password?"**
   - Double-check your password
   - Ensure caps lock is off
   - Try typing password in a text editor first, then copy-paste

3. **"Password must be at least 8 characters long"**
   - Use a longer, more secure password

4. **"Wallet already exists"**
   - Remove existing wallet file or use different filename
   - Use `--wallet-file` option to specify different location

### Recovery

If you lose your password:
- **Cannot recover** - the encryption is designed to be irreversible
- **Backup private keys** - export and store private keys securely
- **Test recovery** - regularly test importing from private keys

## Development

### Adding New Features

1. **New wallet commands**: Add to `Commands` enum in `encrypted_wallet.rs`
2. **New encryption algorithms**: Modify `encrypt_wallet` and `decrypt_wallet` methods
3. **New storage backends**: Implement new storage methods in `WalletManager`

### Security Audits

Before using in production:
- Review encryption implementation
- Test with security tools
- Consider third-party security audit
- Test on multiple platforms

## License

This wallet implementation is part of the FinDAG project and follows the same license terms. 