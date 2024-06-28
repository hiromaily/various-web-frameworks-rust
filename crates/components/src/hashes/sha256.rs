use base64::prelude::*;
use sha2::{Digest, Sha256};

pub fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut sha256 = Sha256::default();
    sha256.update(data);
    sha256.finalize().to_vec()
}

pub fn sha256_b64_hash(data: &[u8]) -> String {
    let mut sha256 = Sha256::default();
    sha256.update(data);
    let result = sha256.finalize();
    BASE64_STANDARD_NO_PAD.encode(result)
}
