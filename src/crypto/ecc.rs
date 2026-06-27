use k256::{
    elliptic_curve::{
        bigint::U256,
        ops::Reduce,
        rand_core::OsRng,
        sec1::{FromEncodedPoint, ToEncodedPoint},
    },
    EncodedPoint,
    ProjectivePoint,
    PublicKey,
    Scalar,
    SecretKey,
    FieldBytes,
};

/// Generate a random scalar in Zq
pub fn random_scalar() -> Scalar {
    Scalar::generate_vartime(&mut OsRng)
}

/// Generator point G
pub fn generator() -> ProjectivePoint {
    ProjectivePoint::GENERATOR
}

/// Compute R = kG
pub fn scalar_mul_generator(k: &Scalar) -> ProjectivePoint {
    ProjectivePoint::GENERATOR * k
}

/// Point multiplication: P * k
pub fn point_mul(p: &ProjectivePoint, k: &Scalar) -> ProjectivePoint {
    *p * *k
}

/// Point addition
pub fn point_add(p1: &ProjectivePoint, p2: &ProjectivePoint) -> ProjectivePoint {
    *p1 + *p2
}

/// Encode projective point -> SEC1 bytes
pub fn encode_point(p: &ProjectivePoint, compressed: bool) -> EncodedPoint {
    p.to_affine().to_encoded_point(compressed)
}

/// Decode SEC1 bytes -> projective point
pub fn decode_point(p: &EncodedPoint) -> ProjectivePoint {
    ProjectivePoint::from_encoded_point(p)
        .unwrap()
}

/// Convert secret key -> scalar
pub fn secret_to_scalar(sk: &SecretKey) -> Scalar {
    *sk.to_nonzero_scalar()
}

/// Convert public key -> projective point
pub fn public_key_to_point(pk: &PublicKey) -> ProjectivePoint {
    ProjectivePoint::from(pk)
}

/// Convert bytes -> scalar (mod q reduction)
pub fn bytes_to_scalar(bytes: &[u8; 32]) -> Scalar {
    <Scalar as Reduce<U256>>::reduce_bytes(
        &FieldBytes::from(*bytes),
    )
}

/// Convert scalar -> bytes
pub fn scalar_to_bytes(s: &Scalar) -> [u8; 32] {
    s.to_bytes().into()
}

/// Compare points
pub fn point_equal(p1: &ProjectivePoint, p2: &ProjectivePoint) -> bool {
    p1 == p2
}