use crate::{
    benchmark::runner::BenchmarkRunner,
    metrics::metrics::Metrics,
};

pub struct Experiments;

impl Experiments {

    /// =========================
    /// 1. Committee scaling
    /// =========================
    pub fn committee_scaling(
        sizes: &[usize],
        requests: usize,
    ) -> Vec<Metrics> {

        let mut results = Vec::new();

        for &size in sizes {

            let threshold = size / 2 + 1;

            let metrics = BenchmarkRunner::run(
                size,
                threshold,
                requests,
            );

            results.push(metrics);
        }

        results
    }

    /// =========================
    /// 2. Request load scaling
    /// =========================
    pub fn request_scaling(
        committee_size: usize,
        threshold: usize,
        request_counts: &[usize],
    ) -> Vec<Metrics> {

        let mut results = Vec::new();

        for &req in request_counts {

            let metrics = BenchmarkRunner::run(
                committee_size,
                threshold,
                req,
            );

            results.push(metrics);
        }

        results
    }

    /// =========================
    /// 3. Threshold sensitivity
    /// =========================
    pub fn threshold_scaling(
        committee_size: usize,
        ratios: &[f64],
        requests: usize,
    ) -> Vec<Metrics> {

        let mut results = Vec::new();

        for &r in ratios {

            let threshold =
                ((committee_size as f64 * r).floor() as usize) + 1;

            let metrics = BenchmarkRunner::run(
                committee_size,
                threshold,
                requests,
            );

            results.push(metrics);
        }

        results
    }

    /// =========================
    /// 4. Full experiment suite
    /// (FOR PAPER EVALUATION)
    /// =========================
    pub fn full_suite(
        sizes: &[usize],
        request_counts: &[usize],
        ratios: &[f64],
    ) -> Vec<Metrics> {

        let mut all = Vec::new();

        // 1. committee scaling
        for &size in sizes {

            let threshold = size / 2 + 1;

            all.push(
                BenchmarkRunner::run(size, threshold, request_counts[0])
            );
        }

        // 2. request scaling
        let fixed_size = sizes[0];
        let fixed_threshold = fixed_size / 2 + 1;

        for &req in request_counts {

            all.push(
                BenchmarkRunner::run(fixed_size, fixed_threshold, req)
            );
        }

        // 3. threshold sensitivity
        let fixed_size = sizes[0];

        for &r in ratios {

            let threshold =
                ((fixed_size as f64 * r).floor() as usize) + 1;

            all.push(
                BenchmarkRunner::run(fixed_size, threshold, request_counts[0])
            );
        }

        all
    }
}