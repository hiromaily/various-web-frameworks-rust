use crate::hashes::hash;
use scrypt::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use scrypt::Scrypt;

/*******************************************************************************
 scrypt
 - https://docs.rs/argon2/latest/argon2/
*******************************************************************************/

#[derive(Clone, Debug)]
pub struct HashScrypt {
    salt: SaltString,
    //params: scrypt::Params,
}

impl Default for HashScrypt {
    fn default() -> Self {
        let salt = SaltString::generate(&mut OsRng);
        //let params = scrypt::Params::new(18, 8, 1, 32).unwrap();

        Self { salt }
    }
}

impl HashScrypt {
    pub fn new(salt: SaltString) -> Self {
        Self { salt }
    }
}

// FIXME: extremely slow
impl hash::Hash for HashScrypt {
    fn hash(&self, data: &[u8]) -> anyhow::Result<String> {
        // [too slow]
        // Ok(Scrypt
        //     .hash_password_customized(data, None, None, self.params, &self.salt)?
        //     .to_string())
        Ok(Scrypt.hash_password(data, &self.salt)?.to_string())
    }
}

/******************************************************************************
 Test
******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashes::hash::Hash; // important to use scypthash.hash()

    #[ignore = "ignore because function is too slow"]
    #[test]
    fn test_scrypt_hash() {
        let password = "foobar".as_bytes();

        let salt = SaltString::from_b64("oKUiVnIPlVQtm1T19IctrA").expect("fail to make salt");
        let scypthash = HashScrypt::new(salt);
        let hashed = scypthash.hash(password).expect("fail to hash");

        assert_eq!(
            hashed,
            "$scrypt$ln=17,r=8,p=1$oKUiVnIPlVQtm1T19IctrA$VLVIRYqGANK8i6Yc9oXsKvPe1BZhFqcq4H5APR28K7Q".to_string()
        );
    }
}
