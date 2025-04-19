use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use alloc::vec::Vec;

pub fn encrypt_buffer(plaintext: &[u8], key: &[u8]) -> Option<Vec<u8>> {

    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    match cipher.encrypt(&nonce, plaintext) {
        Ok(ciphertext) => {

            let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
            result.extend_from_slice(&nonce);
            result.extend_from_slice(&ciphertext);
            Some(result)
        },
        Err(_) => None,
    }
}

pub fn decrypt_buffer(ciphertext: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    if ciphertext.len() <= 12 {  // 12 بایت اندازه nonce است
        return None;
    }
    let nonce = Nonce::from_slice(&ciphertext[0..12]);

    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    match cipher.decrypt(nonce, &ciphertext[12..]) {
        Ok(plaintext) => Some(plaintext),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let plaintext = b"This is a test message";

        let ciphertext = encrypt_buffer(plaintext, &key).unwrap();
        let decrypted = decrypt_buffer(&ciphertext, &key).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }
    #[test]
    fn test_wrong_key() {
        let key1 = [0u8; 32];
        let key2 = [1u8; 32];
        let plaintext = b"This is a test message";

        let ciphertext = encrypt_buffer(plaintext, &key1).unwrap();
        let decrypted = decrypt_buffer(&ciphertext, &key2);

        assert!(decrypted.is_none());
    }
}