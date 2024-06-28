use crate::hashes::{hash, sha256};
use argon2::Argon2;
use std::fmt::Write;

/*******************************************************************************
 argon2
 - https://docs.rs/argon2/latest/argon2/
*******************************************************************************/

#[derive(Clone, Debug)]
pub struct HashArgon2 {
    byte_length: usize,
}

impl Default for HashArgon2 {
    fn default() -> Self {
        Self { byte_length: 32 }
    }
}

impl HashArgon2 {
    pub fn new(size: usize) -> Self {
        Self { byte_length: size }
    }
}

impl hash::Hash for HashArgon2 {
    fn hash(&self, data: &[u8]) -> anyhow::Result<String> {
        let salt = sha256::sha256_hash(data);
        let mut key: Vec<u8> = vec![0u8; self.byte_length];

        Argon2::default()
            .hash_password_into(data, salt.as_ref(), &mut key)
            .map_err(|err| anyhow::Error::msg(format!("Failed to hash, error: {}", err)))?;

        // convert to String
        // FIXME: better code as performance wise
        let mut hashed_str = String::with_capacity(key.len() * 2); // Each byte is 2 hex characters
        for byte in &key {
            write!(hashed_str, "{:02x}", byte).expect("Unable to write");
        }

        Ok(hashed_str)
    }
}

/******************************************************************************
 Test
******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashes::hash::Hash; // important to use argon2hash.hash()

    #[test]
    fn test_argon2_hash() {
        let password = "foobar".as_bytes();
        let argon2hash = HashArgon2::new(32);
        let hashed = argon2hash.hash(password).expect("fail to hash");

        assert_eq!(
            hashed,
            "1550b10bc2d5591908047861f0a8345b21798855407b5951aee7cf0edd39e318".to_string()
        );
    }
}
