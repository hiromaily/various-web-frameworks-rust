use crate::errors::CustomError;
use crate::hashes::{hash, sha256};
use pbkdf2::{
    password_hash::{PasswordHasher, Salt},
    Params, Pbkdf2,
};

fn extract_pbkdf2(input: &str) -> anyhow::Result<&str> {
    // Find the start of the substring
    if let Some(start_pos) = input.find("i=4096,l=") {
        // Calculate the position where the desired substring starts
        let desired_start_pos = start_pos + "i=4096,l=".len();

        // Extract the substring from the desired position onwards
        if let Some(result) = input.get(desired_start_pos..) {
            return Ok(result);
        }
    }
    anyhow::bail!(CustomError::InvalidData)
}

/*******************************************************************************
 pbkdf2
 - https://crates.io/crates/pbkdf2
*******************************************************************************/

#[derive(Clone, Debug)]
pub struct HashPbkdf2 {
    params: Params,
}

impl Default for HashPbkdf2 {
    fn default() -> Self {
        let params = Params {
            rounds: 4096,
            output_length: 32,
        };

        Self { params }
    }
}

impl HashPbkdf2 {
    pub fn new(params: Params) -> Self {
        Self { params }
    }
}

impl hash::Hash for HashPbkdf2 {
    fn hash(&self, data: &[u8]) -> anyhow::Result<String> {
        let sha_b64_hash = sha256::sha256_b64_hash(data);
        let salt = Salt::from_b64(sha_b64_hash.as_str())?;

        let hash = Pbkdf2
            .hash_password_customized(data, None, None, self.params, salt)
            .map_err(|err| anyhow::Error::msg(format!("Failed to hash, error: {}", err)))?;
        let hash_string = hash.to_string();
        let hash_str = extract_pbkdf2(hash_string.as_str())?;
        Ok(hash_str.to_string())
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
    fn test_pbkdf2_hash() {
        let password = "foobar".as_bytes();
        let pbkdf2hash = HashPbkdf2::default();
        let hashed = pbkdf2hash.hash(password).expect("fail to hash");

        assert_eq!(
            hashed,
            "32$w6uP8Tcg6K2QR905Rms8iXTlksL6OD1KOWBxTK7wxPI$MsNxcLOzLe44T66tld+f3yofeR8PIurs8F9Fd344PIY".to_string()
        );
    }
}
