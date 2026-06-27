use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Metrics {

    /// Committee configuration
    pub committee_size: usize,

    pub threshold: usize,

    /// Requests
    pub total_requests: usize,

    pub approved_requests: usize,

    pub rejected_requests: usize,

    /// Total processing time
    pub total_latency: Duration,

}

impl Metrics {

    pub fn new(
        committee_size: usize,
        threshold: usize,
    ) -> Self {

        Self {

            committee_size,

            threshold,

            total_requests: 0,

            approved_requests: 0,

            rejected_requests: 0,

            total_latency: Duration::ZERO,

        }

    }

    pub fn record(
        &mut self,
        approved: bool,
        latency: Duration,
    ) {

        self.total_requests += 1;

        self.total_latency += latency;

        if approved {

            self.approved_requests += 1;

        } else {

            self.rejected_requests += 1;

        }

    }

    pub fn average_latency_ms(
        &self,
    ) -> f64 {

        if self.total_requests == 0 {

            return 0.0;

        }

        self.total_latency.as_secs_f64()
            * 1000.0
            / self.total_requests as f64

    }

    pub fn approval_rate(
        &self,
    ) -> f64 {

        if self.total_requests == 0 {

            return 0.0;

        }

        self.approved_requests as f64
            / self.total_requests as f64

    }

    pub fn rejection_rate(
        &self,
    ) -> f64 {

        if self.total_requests == 0 {

            return 0.0;

        }

        self.rejected_requests as f64
            / self.total_requests as f64

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_metrics() {

        let mut metrics =
            Metrics::new(5, 3);

        metrics.record(
            true,
            Duration::from_millis(10),
        );

        metrics.record(
            false,
            Duration::from_millis(20),
        );

        assert_eq!(
            metrics.total_requests,
            2,
        );

        assert_eq!(
            metrics.approved_requests,
            1,
        );

        assert_eq!(
            metrics.rejected_requests,
            1,
        );

        assert!(
            metrics.average_latency_ms()
                > 14.9
        );

        assert!(
            metrics.average_latency_ms()
                < 15.1
        );

        assert_eq!(
            metrics.approval_rate(),
            0.5,
        );

        assert_eq!(
            metrics.rejection_rate(),
            0.5,
        );

    }

}