// Wallet module for FinDAG SDK
// Handles key management, address generation, and transaction signing

// NOTE: Ensure bip39 and ed25519-hd-key types are installed (npm install --save-dev @types/bip39 @types/ed25519-hd-key if available)
// For Buffer in browser, use a polyfill (npm install buffer)
import { Buffer } from 'buffer';
import * as bip39 from 'bip39';
import { derivePath } from 'ed25519-hd-key';
import nacl from 'tweetnacl';

export class Wallet {
  privateKey: Buffer;
  address: string;

  private constructor(privateKey: Buffer, address: string) {
    this.privateKey = privateKey;
    this.address = address;
  }

  static fromMnemonic(mnemonic: string): Wallet {
    // Derive seed from mnemonic
    const seed = bip39.mnemonicToSeedSync(mnemonic);
    // Derive key using BIP44 path for FinDAG (example: m/44'/1234'/0'/0/0)
    const { key } = derivePath("m/44'/1234'/0'/0/0", seed.toString('hex'));
    // TODO: Generate address from public key (placeholder)
    const address = 'fdg1q' + key.toString('hex').slice(0, 16);
    return new Wallet(Buffer.from(key), address);
  }

  getAddress(): string {
    return this.address;
  }

  signTransaction(tx: any): any {
    // Serialize transaction (excluding signature fields)
    const txCopy = { ...tx };
    delete txCopy.signature;
    delete txCopy.public_key;
    const message = Buffer.from(JSON.stringify(txCopy));
    // Sign with Ed25519
    const keyPair = nacl.sign.keyPair.fromSeed(this.privateKey);
    const signature = nacl.sign.detached(message, keyPair.secretKey);
    // Attach signature and public key (as hex)
    return {
      ...tx,
      signature: Buffer.from(signature).toString('hex'),
      public_key: Buffer.from(keyPair.publicKey).toString('hex'),
    };
  }

  static verifyTransaction(tx: any): boolean {
    // Extract signature and public key
    const { signature, public_key, ...txCopy } = tx;
    if (!signature || !public_key) return false;
    // Serialize transaction (excluding signature fields)
    const message = Buffer.from(JSON.stringify(txCopy));
    // Convert signature and public key from hex
    const sigBuf = Buffer.from(signature, 'hex');
    const pubBuf = Buffer.from(public_key, 'hex');
    // Verify with Ed25519
    return nacl.sign.detached.verify(message, sigBuf, pubBuf);
  }

  // TODO: Add address generation, encryption, etc.
} 