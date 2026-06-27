use crate::{
    crypto::{
        hash::sha256,
        keys::address_from_public_key,
        schnorr::{prove, verify, SchnorrProof},
    },
    protocol::state::RecoveryState,
    types::{
        Address,
        RequestId,
        Timestamp,
    },
};

use k256::{
    elliptic_curve::sec1::ToEncodedPoint,
    PublicKey,
    SecretKey,
};

use crate::crypto::keys::generate_keypair;

#[derive(Clone, Debug)]
pub struct RecoveryRequest {
    pub request_id: RequestId,

    pub old_address: Address,

    pub old_public_key: PublicKey,

    pub new_public_key: PublicKey,

    pub timestamp: Timestamp,

    pub state: RecoveryState,

    pub proof: Option<SchnorrProof>,
}

impl RecoveryRequest {

    /// Create a new recovery request.
    pub fn new(
        request_id: RequestId,
        old_public_key: PublicKey,
        new_public_key: PublicKey,
        timestamp: Timestamp,
    ) -> Self {

        let old_address =
            address_from_public_key(&old_public_key);

        Self {

            request_id,

            old_address,

            old_public_key,

            new_public_key,

            timestamp,

            state: RecoveryState::Created,

            proof: None,
        }

    }

    /// Serialize request into the message to be signed.
    pub fn signing_message(
        &self,
    ) -> Vec<u8> {

        let mut msg = Vec::new();

        msg.extend_from_slice(
            &self.request_id.to_be_bytes(),
        );

        msg.extend_from_slice(
            &self.old_address,
        );

        msg.extend_from_slice(
            self.old_public_key
                .to_encoded_point(true)
                .as_bytes(),
        );

        msg.extend_from_slice(
            self.new_public_key
                .to_encoded_point(true)
                .as_bytes(),
        );

        msg.extend_from_slice(
            &self.timestamp.to_be_bytes(),
        );

        sha256(&msg).to_vec()

    }

    /// Sign this recovery request.
    pub fn sign(
        &mut self,
        old_secret: &SecretKey,
    ) {

        let msg =
            self.signing_message();

        let proof =
            prove(old_secret, &msg);

        self.proof = Some(proof);

        self.state =
            RecoveryState::Signed;

    }

    /// Verify Schnorr proof.
    pub fn verify(
        &self,
    ) -> bool {

        let proof = match &self.proof {

            Some(p) => p,

            None => return false,

        };

        let msg =
            self.signing_message();

        verify(
            &self.old_public_key,
            &msg,
            proof,
        )

    }

}

#[test]
fn test_state_transition_after_sign() {

    let (old_sk, _) = generate_keypair();

    let (_, new_pk) = generate_keypair();

    let mut request = RecoveryRequest::new(
        1,
        old_sk.public_key(),
        new_pk,
        123456,
    );

    assert_eq!(
        request.state,
        RecoveryState::Created,
    );

    request.sign(&old_sk);

    assert_eq!(
        request.state,
        RecoveryState::Signed,
    );

}