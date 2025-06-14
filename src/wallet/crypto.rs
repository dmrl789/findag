use ring::aead::*;
use ring::pbkdf2;
use rand::RngCore;
use rand::rngs::OsRng;
use std::num::NonZeroU32;

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

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let nonce = Nonce::assume_unique_for_key(nonce);

    let mut in_out = secret.to_vec();
    sealing_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out).unwrap();

    (in_out, *nonce.as_ref(), salt)
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
