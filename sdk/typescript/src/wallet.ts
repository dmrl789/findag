// Wallet module for FinDAG SDK
// Handles key management, address generation, transaction signing, and encrypted storage

// NOTE: Ensure bip39 and ed25519-hd-key types are installed (npm install --save-dev @types/bip39 @types/ed25519-hd-key if available)
// For Buffer in browser, use a polyfill (npm install buffer)
import { Buffer } from 'buffer';
import * as bip39 from 'bip39';
import { derivePath } from 'ed25519-hd-key';
import nacl from 'tweetnacl';
import { pbkdf2Sync, randomBytes, createCipher, createDecipher } from 'crypto';

export interface WalletAccount {
  name: string;
  address: string;
  publicKey: string;
  createdAt: Date;
  isDefault: boolean;
}

export interface EncryptedWalletData {
  version: number;
  createdAt: Date;
  salt: string;
  encryptedData: string;
  iv: string;
  iterations: number;
}

export class Wallet {
  private privateKey: Buffer;
  public address: string;
  public accounts: WalletAccount[];

  private constructor(privateKey: Buffer, address: string) {
    this.privateKey = privateKey;
    this.address = address;
    this.accounts = [{
      name: 'default',
      address: address,
      publicKey: this.getPublicKeyHex(),
      createdAt: new Date(),
      isDefault: true
    }];
  }

  static fromMnemonic(mnemonic: string): Wallet {
    // Derive seed from mnemonic
    const seed = bip39.mnemonicToSeedSync(mnemonic);
    // Derive key using BIP44 path for FinDAG (example: m/44'/1234'/0'/0/0)
    const { key } = derivePath("m/44'/1234'/0'/0/0", seed.toString('hex'));
    // Generate address from public key
    const address = 'fdg1q' + key.toString('hex').slice(0, 16);
    return new Wallet(Buffer.from(key), address);
  }

  static fromPrivateKey(privateKeyHex: string): Wallet {
    const privateKey = Buffer.from(privateKeyHex, 'hex');
    if (privateKey.length !== 32) {
      throw new Error('Invalid private key length');
    }
    
    // Generate public key from private key
    const keyPair = nacl.sign.keyPair.fromSeed(privateKey);
    const address = 'fdg1q' + Buffer.from(keyPair.publicKey).toString('hex').slice(0, 16);
    
    return new Wallet(privateKey, address);
  }

  getAddress(): string {
    return this.address;
  }

  getPublicKeyHex(): string {
    const keyPair = nacl.sign.keyPair.fromSeed(this.privateKey);
    return Buffer.from(keyPair.publicKey).toString('hex');
  }

  exportPrivateKey(): string {
    return this.privateKey.toString('hex');
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

  addAccount(name: string): void {
    // Generate new keypair for the account
    const newPrivateKey = randomBytes(32);
    const keyPair = nacl.sign.keyPair.fromSeed(newPrivateKey);
    const newAddress = 'fdg1q' + Buffer.from(keyPair.publicKey).toString('hex').slice(0, 16);
    
    const account: WalletAccount = {
      name: name,
      address: newAddress,
      publicKey: Buffer.from(keyPair.publicKey).toString('hex'),
      createdAt: new Date(),
      isDefault: false
    };
    
    this.accounts.push(account);
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
}

export class WalletManager {
  private storageKey: string;

  constructor(storageKey: string = 'findag_wallet') {
    this.storageKey = storageKey;
  }

  // Encrypt wallet data
  private encryptWallet(wallet: Wallet, password: string): EncryptedWalletData {
    const salt = randomBytes(32).toString('hex');
    const iv = randomBytes(16);
    const iterations = 100000;
    
    // Derive key from password using PBKDF2
    const key = pbkdf2Sync(password, salt, iterations, 32, 'sha256');
    
    // Serialize wallet data
    const walletData = JSON.stringify({
      address: wallet.address,
      accounts: wallet.accounts,
      privateKey: wallet.exportPrivateKey()
    });
    
    // Encrypt data
    const cipher = createCipher('aes-256-cbc', key);
    let encrypted = cipher.update(walletData, 'utf8', 'hex');
    encrypted += cipher.final('hex');
    
    return {
      version: 1,
      createdAt: new Date(),
      salt: salt,
      encryptedData: encrypted,
      iv: iv.toString('hex'),
      iterations: iterations
    };
  }

  // Decrypt wallet data
  private decryptWallet(encryptedData: EncryptedWalletData, password: string): Wallet {
    const key = pbkdf2Sync(password, encryptedData.salt, encryptedData.iterations, 32, 'sha256');
    const iv = Buffer.from(encryptedData.iv, 'hex');
    
    // Decrypt data
    const decipher = createDecipher('aes-256-cbc', key);
    let decrypted = decipher.update(encryptedData.encryptedData, 'hex', 'utf8');
    decrypted += decipher.final('utf8');
    
    // Parse wallet data
    const walletData = JSON.parse(decrypted);
    const wallet = Wallet.fromPrivateKey(walletData.privateKey);
    wallet.accounts = walletData.accounts;
    
    return wallet;
  }

  // Save encrypted wallet to localStorage (browser) or file system (Node.js)
  saveWallet(wallet: Wallet, password: string): void {
    const encryptedData = this.encryptWallet(wallet, password);
    
    if (typeof window !== 'undefined') {
      // Browser environment - use localStorage
      localStorage.setItem(this.storageKey, JSON.stringify(encryptedData));
    } else {
      // Node.js environment - use file system
      const fs = require('fs');
      fs.writeFileSync(`${this.storageKey}.dat`, JSON.stringify(encryptedData));
    }
  }

  // Load encrypted wallet from storage
  loadWallet(password: string): Wallet {
    let encryptedData: EncryptedWalletData;
    
    if (typeof window !== 'undefined') {
      // Browser environment - use localStorage
      const stored = localStorage.getItem(this.storageKey);
      if (!stored) {
        throw new Error('No wallet found');
      }
      encryptedData = JSON.parse(stored);
    } else {
      // Node.js environment - use file system
      const fs = require('fs');
      const stored = fs.readFileSync(`${this.storageKey}.dat`, 'utf8');
      encryptedData = JSON.parse(stored);
    }
    
    return this.decryptWallet(encryptedData, password);
  }

  // Check if wallet exists
  walletExists(): boolean {
    if (typeof window !== 'undefined') {
      return localStorage.getItem(this.storageKey) !== null;
    } else {
      const fs = require('fs');
      return fs.existsSync(`${this.storageKey}.dat`);
    }
  }

  // Create new wallet
  createWallet(password: string): Wallet {
    if (this.walletExists()) {
      throw new Error('Wallet already exists');
    }
    
    const wallet = Wallet.fromMnemonic(bip39.generateMnemonic());
    this.saveWallet(wallet, password);
    return wallet;
  }

  // Import wallet from private key
  importWallet(privateKeyHex: string, password: string): Wallet {
    if (this.walletExists()) {
      throw new Error('Wallet already exists');
    }
    
    const wallet = Wallet.fromPrivateKey(privateKeyHex);
    this.saveWallet(wallet, password);
    return wallet;
  }

  // Change wallet password
  changePassword(oldPassword: string, newPassword: string): void {
    const wallet = this.loadWallet(oldPassword);
    this.saveWallet(wallet, newPassword);
  }

  // Delete wallet
  deleteWallet(): void {
    if (typeof window !== 'undefined') {
      localStorage.removeItem(this.storageKey);
    } else {
      const fs = require('fs');
      if (fs.existsSync(`${this.storageKey}.dat`)) {
        fs.unlinkSync(`${this.storageKey}.dat`);
      }
    }
  }
}

// Utility functions for password validation
export function validatePassword(password: string): boolean {
  return password.length >= 8;
}

export function promptPassword(prompt: string): Promise<string> {
  if (typeof window !== 'undefined') {
    // Browser environment
    return new Promise((resolve) => {
      const password = prompt(prompt);
      resolve(password || '');
    });
  } else {
    // Node.js environment
    const readline = require('readline');
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    
    return new Promise((resolve) => {
      rl.question(prompt + ': ', (answer: string) => {
        rl.close();
        resolve(answer);
      });
    });
  }
} 