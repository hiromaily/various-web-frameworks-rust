use rand_core::{OsRng, RngCore};

// e.g. let secret = generate_secret(32);
pub fn generate_secret(length: usize) -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                  abcdefghijklmnopqrstuvwxyz\
                  0123456789)(*&^%$#@!~";
    let mut rng = OsRng;

    let secret: String = (0..length)
        .map(|_| {
            let idx = (rng.next_u32() as usize) % charset.len();
            charset[idx] as char
        })
        .collect();

    secret
}

/******************************************************************************
 Test
******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        assert_eq!(generate_secret(32).len(), 32);
    }
}
