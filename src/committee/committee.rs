use crate::{
    committee::node::CommitteeNode,
    protocol::{
        request::RecoveryRequest,
        state::RecoveryState,
    },
};

#[derive(Debug)]
pub struct Committee {
    pub nodes: Vec<CommitteeNode>,
    pub threshold: usize,
}

impl Committee {

    pub fn new(
        size: usize,
        threshold: usize,
    ) -> Self {

        assert!(threshold <= size);

        let mut nodes = Vec::new();

        for id in 0..size {

            nodes.push(
                CommitteeNode::new(id as u32)
            );

        }

        Self {

            nodes,

            threshold,

        }

    }

    /// Process a recovery request.
    ///
    /// Returns true if enough committee members approve.
    pub fn process_request(
        &self,
        request: &mut RecoveryRequest,
    ) -> bool {

        request.state = RecoveryState::Pending;

        let mut approvals = 0;

        for node in &self.nodes {

            if node.verify_request(request) {

                approvals += 1;

            }

        }

        if approvals >= self.threshold {

            request.state =
                RecoveryState::Approved;

            true

        } else {

            request.state =
                RecoveryState::Rejected;

            false

        }

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::crypto::keys::generate_keypair;

    #[test]
    fn test_committee_approve() {

        let committee =
            Committee::new(5, 3);

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

        assert!(
            committee.process_request(
                &mut request,
            )
        );

        assert_eq!(
            request.state,
            RecoveryState::Approved,
        );

    }

    #[test]
    fn test_committee_reject() {

        let committee =
            Committee::new(5, 3);

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

        // Không sign.

        assert!(
            !committee.process_request(
                &mut request,
            )
        );

        assert_eq!(
                request.state,
                RecoveryState::Rejected,
            );

        }

}