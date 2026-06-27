use crate::{
    protocol::{
        request::RecoveryRequest,
        state::RecoveryState,
    },
    types::NodeId,
};

#[derive(Clone, Debug)]
pub struct CommitteeNode {
    pub id: NodeId,
}

impl CommitteeNode {
    pub fn new(
        id: NodeId,
    ) -> Self {
        Self { id }
    }

    /// Verify a recovery request.
    ///
    /// The node only verifies the request.
    /// It never changes the request state.
    pub fn verify_request(
        &self,
        request: &RecoveryRequest,
    ) -> bool {
        request.verify()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::crypto::keys::generate_keypair;

    #[test]
    fn test_node_verify_request() {

        let (old_sk, _) =
            generate_keypair();

        let (_, new_pk) =
            generate_keypair();

        let mut request =
            RecoveryRequest::new(
                1,
                old_sk.public_key(),
                new_pk,
                123456,
            );

        request.sign(&old_sk);

        let node =
            CommitteeNode::new(1);

        assert!(
            node.verify_request(
                &request,
            )
        );

        // Node must NOT change request state.
        assert_eq!(
            request.state,
            RecoveryState::Signed,
        );

    }

    #[test]
    fn test_node_reject_invalid_request() {

        let (old_sk, _) =
            generate_keypair();

        let (_, new_pk) =
            generate_keypair();

        let request =
            RecoveryRequest::new(
                1,
                old_sk.public_key(),
                new_pk,
                123456,
            );

        let node =
            CommitteeNode::new(1);

        assert!(
            !node.verify_request(
                &request,
            )
        );

        // State remains unchanged.
        assert_eq!(
            request.state,
            RecoveryState::Created,
        );

    }

}