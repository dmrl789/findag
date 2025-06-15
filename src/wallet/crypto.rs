use ring::aead::*;
use ring::pbkdf2;
use rand::RngCore;
use rand::rngs::OsRng;
use std::num::NonZeroU32;
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::error::Error;
use std::collections::HashMap;
use std::convert::TryInto;

const KEY_LEN: usize = 32;
const PBKDF2_ITER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };
const SALT_LEN: usize = 16;

pub fn encrypt_secret_key(secret: &[u8], password: &str) -> (Vec<u8>, [u8; 12], [u8; 16]) {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let mut key = [0u8; KEY_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITER,
        &salt,
        password.as_bytes(),
        &mut key,
    );
    let sealing_key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &key).unwrap());

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let mut in_out = secret.to_vec();
    sealing_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out).unwrap();

    (in_out, nonce_bytes, salt)
}

pub fn decrypt_secret_key(
    ciphertext: &[u8],
    nonce: &[u8; 12],
    salt: &[u8; 16],
    password: &str,
) -> Result<Vec<u8>, String> {
    let mut key = [0u8; KEY_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITER,
        salt,
        password.as_bytes(),
        &mut key,
    );
    let opening_key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &key).unwrap());
    let nonce = Nonce::assume_unique_for_key(*nonce);
    let mut data = ciphertext.to_vec();
    opening_key
        .open_in_place(nonce, Aad::empty(), &mut data)
        .map_err(|_| "Failed to decrypt key".to_string())?;
    let plain = &data[..data.len() - AES_256_GCM.tag_len()];
    Ok(plain.to_vec())
}

// Shamir's Secret Sharing implementation
pub fn split_secret(secret: &[u8], n: u8, k: u8) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    if k > n {
        return Err("Threshold k cannot be greater than number of shares n".into());
    }
    if k < 2 {
        return Err("Threshold k must be at least 2".into());
    }

    let mut rng = OsRng;
    let mut shares = Vec::with_capacity(n as usize);
    
    // Generate random coefficients for the polynomial
    let mut coefficients = Vec::with_capacity((k - 1) as usize);
    for _ in 0..(k - 1) {
        let mut coef = [0u8; 32];
        rng.fill_bytes(&mut coef);
        coefficients.push(coef);
    }
    
    // Generate shares
    for i in 1..=n {
        let mut share = [0u8; 32];
        // Evaluate polynomial at point i
        for (j, coef) in coefficients.iter().enumerate() {
            let power = (i as u64).pow(j as u32 + 1);
            for k in 0..32 {
                share[k] ^= coef[k].wrapping_mul(power as u8);
            }
        }
        // Add the secret
        for k in 0..32 {
            share[k] ^= secret[k];
        }
        shares.push(share.to_vec());
    }
    
    Ok(shares)
}

pub fn reconstruct_secret(shares: &[Vec<u8>], k: u8) -> Result<Vec<u8>, Box<dyn Error>> {
    if shares.len() < k as usize {
        return Err("Not enough shares for reconstruction".into());
    }
    
    // Lagrange interpolation
    let mut secret = vec![0u8; 32];
    for i in 0..shares.len() {
        let mut term = shares[i].clone();
        let mut denominator = 1u64;
        let mut numerator = 1u64;
        
        for j in 0..shares.len() {
            if i != j {
                numerator = numerator.wrapping_mul((j + 1) as u64);
                denominator = denominator.wrapping_mul((j as i64 - i as i64).abs() as u64);
            }
        }
        
        let factor = numerator.wrapping_div(denominator);
        for k in 0..32 {
            term[k] = term[k].wrapping_mul(factor as u8);
            secret[k] ^= term[k];
        }
    }
    
    Ok(secret)
}

pub fn encrypt_share(share: &[u8], public_key: &VerifyingKey) -> Result<Vec<u8>, Box<dyn Error>> {
    // In a real implementation, this would use the holder's public key to encrypt the share
    // For now, we'll just use a simple XOR with a random key
    let mut rng = OsRng;
    let mut key = [0u8; 32];
    rng.fill_bytes(&mut key);
    
    let mut encrypted = Vec::with_capacity(share.len());
    for (i, byte) in share.iter().enumerate() {
        encrypted.push(byte ^ key[i % 32]);
    }
    
    Ok(encrypted)
}

pub fn decrypt_share(encrypted_share: &[u8], private_key: &SigningKey) -> Result<Vec<u8>, Box<dyn Error>> {
    // In a real implementation, this would use the holder's private key to decrypt the share
    // For now, we'll just use a simple XOR with the same key
    let mut decrypted = Vec::with_capacity(encrypted_share.len());
    for (i, byte) in encrypted_share.iter().enumerate() {
        decrypted.push(byte ^ private_key.to_bytes()[i % 32]);
    }
    
    Ok(decrypted)
}
