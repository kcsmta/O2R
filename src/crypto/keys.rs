use crate::crypto::hash::hash160;

use k256::{
    elliptic_curve::{
        rand_core::OsRng,
        sec1::ToEncodedPoint,
    },
    PublicKey,
    SecretKey,
};

/// Generate a new secp256k1 key pair.
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secret = SecretKey::random(&mut OsRng);
    let public = secret.public_key();

    (secret, public)
}

/// Derive public key from secret key.
pub fn public_key_from_secret(secret: &SecretKey) -> PublicKey {
    secret.public_key()
}

/// Convert public key to SEC1 encoded bytes.
/// compressed = true  -> 33 bytes
/// compressed = false -> 65 bytes
pub fn public_key_to_bytes(
    public: &PublicKey,
    compressed: bool,
) -> Vec<u8> {
    public.to_encoded_point(compressed).as_bytes().to_vec()
}

/// Restore public key from SEC1 bytes.
pub fn public_key_from_bytes(
    bytes: &[u8],
) -> Result<PublicKey, k256::elliptic_curve::Error> {
    PublicKey::from_sec1_bytes(bytes)
}

/// Compute HASH160(public_key).
pub fn address_from_public_key(
    public: &PublicKey,
) -> [u8; 20] {
    let encoded = public.to_encoded_point(true);

    hash160(encoded.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let (sk, pk) = generate_keypair();

        println!("Secret Key : {:?}", sk.to_bytes());

        println!(
            "Public Key : {}",
            hex::encode(public_key_to_bytes(&pk, false))
        );

        let addr = address_from_public_key(&pk);

        println!("Address    : {}", hex::encode(addr));

        assert_eq!(addr.len(), 20);
    }

    #[test]
    fn test_public_key_restore() {
        let (_, pk) = generate_keypair();

        let bytes = public_key_to_bytes(&pk, true);

        let restored = public_key_from_bytes(&bytes).unwrap();

        assert_eq!(
            public_key_to_bytes(&pk, true),
            public_key_to_bytes(&restored, true)
        );
    }
}