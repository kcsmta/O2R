use sha2::{Digest, Sha256};
use ripemd::Ripemd160;

/// SHA256(data)
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let hash = Sha256::digest(data);

    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);

    out
}

/// RIPEMD160(data)
pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    let hash = Ripemd160::digest(data);

    let mut out = [0u8; 20];
    out.copy_from_slice(&hash);

    out
}

/// HASH160(data) = RIPEMD160(SHA256(data))
pub fn hash160(data: &[u8]) -> [u8; 20] {
    ripemd160(&sha256(data))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash160() {

        let h = hash160(b"hello");

        assert_eq!(h.len(), 20);
    }

}