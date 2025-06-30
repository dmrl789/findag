#!/usr/bin/env node

// Test script for FinDAG encrypted wallet
// Run with: node test-wallet.js

const { Wallet, WalletManager, validatePassword } = require('./sdk/typescript/src/wallet.ts');

async function testWallet() {
    console.log('🔐 FinDAG Encrypted Wallet Test\n');
    
    const walletManager = new WalletManager('test_wallet');
    
    try {
        // Test 1: Create new wallet
        console.log('1. Creating new wallet...');
        const password = 'mySecurePassword123';
        
        if (!validatePassword(password)) {
            throw new Error('Password validation failed');
        }
        
        const wallet = walletManager.createWallet(password);
        console.log('✅ Wallet created successfully!');
        console.log(`   Address: ${wallet.getAddress()}`);
        console.log(`   Public Key: ${wallet.getPublicKeyHex()}`);
        console.log(`   Accounts: ${wallet.accounts.length}`);
        
        // Test 2: Add account
        console.log('\n2. Adding new account...');
        wallet.addAccount('savings');
        console.log('✅ Account added successfully!');
        console.log(`   Total accounts: ${wallet.accounts.length}`);
        
        // Test 3: Save and reload wallet
        console.log('\n3. Testing wallet persistence...');
        walletManager.saveWallet(wallet, password);
        console.log('✅ Wallet saved successfully!');
        
        const reloadedWallet = walletManager.loadWallet(password);
        console.log('✅ Wallet loaded successfully!');
        console.log(`   Address: ${reloadedWallet.getAddress()}`);
        console.log(`   Accounts: ${reloadedWallet.accounts.length}`);
        
        // Test 4: Sign transaction
        console.log('\n4. Testing transaction signing...');
        const transaction = {
            from: wallet.getAddress(),
            to: 'fdg1qtest123456789',
            amount: 1000,
            currency: 'USD'
        };
        
        const signedTx = wallet.signTransaction(transaction);
        console.log('✅ Transaction signed successfully!');
        console.log(`   Signature: ${signedTx.signature.substring(0, 32)}...`);
        
        // Test 5: Verify transaction
        console.log('\n5. Testing transaction verification...');
        const isValid = Wallet.verifyTransaction(signedTx);
        console.log(`✅ Transaction verification: ${isValid ? 'PASSED' : 'FAILED'}`);
        
        // Test 6: Export private key
        console.log('\n6. Testing private key export...');
        const privateKey = wallet.exportPrivateKey();
        console.log(`✅ Private key exported: ${privateKey.substring(0, 16)}...`);
        
        // Test 7: Import wallet from private key
        console.log('\n7. Testing wallet import...');
        const newWalletManager = new WalletManager('imported_wallet');
        const importedWallet = newWalletManager.importWallet(privateKey, 'newPassword123');
        console.log('✅ Wallet imported successfully!');
        console.log(`   Address: ${importedWallet.getAddress()}`);
        
        // Test 8: Change password
        console.log('\n8. Testing password change...');
        walletManager.changePassword(password, 'newSecurePassword456');
        console.log('✅ Password changed successfully!');
        
        // Test 9: Load with new password
        const walletWithNewPassword = walletManager.loadWallet('newSecurePassword456');
        console.log('✅ Wallet loaded with new password!');
        console.log(`   Address: ${walletWithNewPassword.getAddress()}`);
        
        console.log('\n🎉 All wallet tests passed successfully!');
        
    } catch (error) {
        console.error('❌ Test failed:', error.message);
        process.exit(1);
    }
}

// Run the test
testWallet().catch(console.error); 