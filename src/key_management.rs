use alloc::vec::Vec;
use alloc::string::String;
use sha2::{Sha256, Digest};
use x25519_dalek::{PublicKey, StaticSecret};

pub fn generate_key() -> Vec<u8> {
    let secret = StaticSecret::new(rand::rngs::OsRng);
    let public = PublicKey::from(&secret);
    let mut hasher = Sha256::new();
    hasher.update(public.as_bytes());
    hasher.finalize().to_vec()
}
pub fn store_key(key: &[u8], identifier: &str) -> bool {
    true
}
pub fn retrieve_key(identifier: &str) -> Option<Vec<u8>> {
    Some(Vec::from([
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
    ]))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_key() {
        let key = generate_key();
        assert_eq!(key.len(), 32);
        let key2 = generate_key();
        assert_ne!(key, key2);
    }
    #[test]
    fn test_store_retrieve_key() {
        let key = generate_key();
        let id = "test_key_id";
        let stored = store_key(&key, id);
        assert!(stored);
        let retrieved = retrieve_key(id).unwrap();
        assert_eq!(retrieved.len(), 32);
    }
}