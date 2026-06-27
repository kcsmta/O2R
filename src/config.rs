use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct BenchmarkConfig {
    /// List of committee sizes to test
    pub committee_sizes: Vec<usize>,

    /// Number of requests per experiment
    pub request_counts: Vec<usize>,

    /// Threshold ratio (0.5 = majority)
    pub threshold_ratio: f64,

    /// Output directory for CSV results
    pub output_dir: String,
}

impl BenchmarkConfig {

    /// Load config from TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Self {

        let content = fs::read_to_string(path)
            .expect("Failed to read benchmark config file");

        toml::from_str::<BenchmarkConfig>(&content)
            .expect("Invalid benchmark config TOML format")
    }

    /// Compute threshold from committee size
    ///
    /// Example:
    /// - size = 5, ratio = 0.5 → 3
    /// - size = 31, ratio = 0.67 → ~21
    pub fn threshold(&self, committee_size: usize) -> usize {

        ((committee_size as f64 * self.threshold_ratio).floor() as usize)
            + 1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_threshold_calculation() {

        let cfg = BenchmarkConfig {
            committee_sizes: vec![3, 5],
            request_counts: vec![100],
            threshold_ratio: 0.5,
            output_dir: "results".to_string(),
        };

        assert_eq!(cfg.threshold(5), 3);
        assert_eq!(cfg.threshold(7), 4);
    }
}