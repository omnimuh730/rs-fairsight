use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};

pub const KEY: [u8; 32] = [0x42; 32]; // Replace with a securely generated key

/// Encrypt a string, returning the encrypted bytes and nonce
pub fn encrypt_string(
    plaintext: &str,
    key_bytes: &[u8; 32]
) -> Result<(Vec<u8>, [u8; 12]), Unspecified> {
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)?;
    let key = LessSafeKey::new(unbound_key);
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let mut data = plaintext.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut data)?;

    // Prepend the length of the encrypted data
    let mut result = Vec::new();
    let len = data.len() as u32;
    result.extend_from_slice(&len.to_le_bytes()); // 4 bytes for length
    result.extend_from_slice(&data);

    Ok((result, nonce_bytes))
}

/// Decrypt a string from encrypted bytes and nonce
pub fn decrypt_string(
    encrypted_data: &mut Vec<u8>,
    key_bytes: &[u8; 32],
    nonce_bytes: [u8; 12]
) -> Result<String, Unspecified> {
    let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)?;
    let key = LessSafeKey::new(unbound_key);
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let decrypted_data = key.open_in_place(nonce, Aad::empty(), encrypted_data)?;
    String::from_utf8(decrypted_data.to_vec()).map_err(|_| Unspecified)
}
