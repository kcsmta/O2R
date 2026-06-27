use sha2::{Digest, Sha256};

use k256::{
    EncodedPoint,
    PublicKey,
    Scalar,
    SecretKey,
};
use k256::elliptic_curve::sec1::ToEncodedPoint;

use crate::crypto::ecc::{
    bytes_to_scalar,
    encode_point,
    random_scalar,
    scalar_mul_generator,
    secret_to_scalar,
    decode_point,
    point_add,
    point_equal,
    point_mul,
    public_key_to_point,
};

use crate::crypto::keys::generate_keypair;

/// Schnorr proof (R, s)
#[derive(Clone, Debug)]
pub struct SchnorrProof {
    pub commitment: EncodedPoint,
    pub response: Scalar,
}

/// c = H(R || pk || m)
fn compute_challenge(
    commitment: &EncodedPoint,
    public: &PublicKey,
    message: &[u8],
) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(commitment.as_bytes());
    hasher.update(public.to_encoded_point(true).as_bytes());
    hasher.update(message);

    let digest = hasher.finalize();

    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

/// Prover
pub fn prove(
    secret: &SecretKey,
    message: &[u8],
) -> SchnorrProof {
    // r ← Zq
    let r = random_scalar();

    // R = rG
    let r_point = scalar_mul_generator(&r);

    let commitment = encode_point(&r_point, true);

    // c = H(...)
    let c_bytes = compute_challenge(
        &commitment,
        &secret.public_key(),
        message,
    );

    let c = bytes_to_scalar(&c_bytes);

    let sk = secret_to_scalar(secret);

    // s = r + c·sk
    let s = r + c * sk;

    SchnorrProof {
        commitment,
        response: s,
    }
}

#[cfg(test)]
mod tests_proof {
    use super::*;
    use crate::crypto::keys::generate_keypair;

    #[test]
    fn test_prove() {
        let (sk, _) = generate_keypair();

        let proof = prove(&sk, b"hello");

        println!(
            "R = {}",
            hex::encode(proof.commitment.as_bytes())
        );

        println!(
            "s = {}",
            hex::encode(proof.response.to_bytes())
        );
    }
}

pub fn verify(
    public: &PublicKey,
    message: &[u8],
    proof: &SchnorrProof,
) -> bool {

    // Decode R
    let r_point = decode_point(
        &proof.commitment,
    );

    // c = H(R || pk || m)
    let challenge = compute_challenge(
        &proof.commitment,
        public,
        message,
    );

    let c = bytes_to_scalar(
        &challenge,
    );

    // Left = sG
    let left = scalar_mul_generator(
        &proof.response,
    );

    // Right = R + cP
    let pk = public_key_to_point(
        public,
    );

    let cpk = point_mul(
        &pk,
        &c,
    );

    let right = point_add(
        &r_point,
        &cpk,
    );

    point_equal(
        &left,
        &right,
    )
}

#[cfg(test)]
mod tests_verify {
    use super::*;
    use crate::crypto::keys::generate_keypair;

    #[test]
    fn test_prove_verify_success() {
        let (sk, pk) = generate_keypair();

        let message = b"hello o2r";

        let proof = prove(&sk, message);

        assert!(verify(&pk, message, &proof));
    }

    #[test]
    fn test_wrong_message() {
        let (sk, pk) = generate_keypair();

        let proof = prove(&sk, b"hello");

        assert!(!verify(&pk, b"world", &proof));
    }

    #[test]
    fn test_wrong_public_key() {
        let (sk_alice, _) = generate_keypair();

        let (_, pk_bob) = generate_keypair();

        let proof = prove(&sk_alice, b"hello");

        assert!(!verify(&pk_bob, b"hello", &proof));
    }
}

#[test]
fn test_random_messages() {

    let (sk, pk) = generate_keypair();

    for i in 0..1000 {

        let msg = format!("msg{}", i);

        let proof = prove(
            &sk,
            msg.as_bytes(),
        );

        assert!(
            verify(
                &pk,
                msg.as_bytes(),
                &proof,
            )
        );

    }
}

#[test]
fn stress_test() {
    for k in 0..100 {
        let (sk, pk) = generate_keypair();

        for i in 0..100 {
            let msg = format!("key{}-msg{}", k, i);

            let proof = prove(&sk, msg.as_bytes());

            assert!(verify(&pk, msg.as_bytes(), &proof));
        }
    }
}

#[test]
fn test_modified_response() {
    let (sk, pk) = generate_keypair();

    let message = b"hello";

    let mut proof = prove(&sk, message);

    proof.response += random_scalar();

    assert!(
        !verify(&pk, message, &proof)
    );
}

#[test]
fn test_modified_commitment() {
    let (sk, pk) = generate_keypair();

    let message = b"hello";

    let mut proof = prove(&sk, message);

    let fake_r = scalar_mul_generator(
        &random_scalar()
    );

    proof.commitment = encode_point(
        &fake_r,
        true,
    );

    assert!(
        !verify(&pk, message, &proof)
    );
}

#[test]
fn test_forged_proof() {

    let (_, pk) = generate_keypair();

    let fake_r = random_scalar();

    let fake_point = scalar_mul_generator(
        &fake_r,
    );

    let fake = SchnorrProof {

        commitment: encode_point(
            &fake_point,
            true,
        ),

        response: random_scalar(),
    };

    assert!(
        !verify(
            &pk,
            b"hello",
            &fake,
        )
    );

}

#[test]
fn test_different_secret_same_message() {

    let (sk1, pk1) = generate_keypair();

    let (sk2, pk2) = generate_keypair();

    let message = b"hello";

    let proof1 = prove(
        &sk1,
        message,
    );

    let proof2 = prove(
        &sk2,
        message,
    );

    assert!(
        verify(
            &pk1,
            message,
            &proof1,
        )
    );

    assert!(
        verify(
            &pk2,
            message,
            &proof2,
        )
    );

    assert!(
        !verify(
            &pk1,
            message,
            &proof2,
        )
    );

    assert!(
        !verify(
            &pk2,
            message,
            &proof1,
        )
    );

}