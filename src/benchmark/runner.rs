use crate::{
    committee::committee::Committee,
    crypto::keys::generate_keypair,
    metrics::metrics::Metrics,
    protocol::request::RecoveryRequest,
};

use std::time::Instant;

pub struct BenchmarkRunner;

impl BenchmarkRunner {

    pub fn run(
        committee_size: usize,
        threshold: usize,
        requests: usize,
    ) -> Metrics {

        let committee =
            Committee::new(committee_size, threshold);

        let mut metrics =
            Metrics::new(committee_size, threshold);

        for id in 0..requests {

            let request_id = id as u64;

            let (sk, _) = generate_keypair();
            let (_, pk) = generate_keypair();

            let mut req =
                RecoveryRequest::new(
                    request_id,
                    sk.public_key(),
                    pk,
                    123456,
                );

            req.sign(&sk);

            let start = Instant::now();

            let approved =
                committee.process_request(&mut req);

            let latency = start.elapsed();

            metrics.record(approved, latency);
        }

        metrics
    }
}