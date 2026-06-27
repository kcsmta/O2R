use crate::{
    committee::committee::Committee,
    protocol::request::RecoveryRequest,
};

pub struct Simulator {
    pub committee: Committee,
}

impl Simulator {

    pub fn new(
        committee: Committee,
    ) -> Self {

        Self {
            committee,
        }

    }

    /// Process one recovery request.
    pub fn run_request(
        &self,
        request: &mut RecoveryRequest,
    ) -> bool {

        self.committee
            .process_request(request)

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::crypto::keys::generate_keypair;

    #[test]
    fn test_simulator() {

        let committee =
            Committee::new(5, 3);

        let simulator =
            Simulator::new(committee);

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
            simulator.run_request(
                &mut request,
            )
        );

    }

}