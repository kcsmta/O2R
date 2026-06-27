use crate::metrics::metrics::Metrics;
use std::{fs::File, io::Write, path::Path};

pub struct CsvExporter;

impl CsvExporter {

    pub fn export(
        path: &str,
        metrics: &Metrics,
    ) {

        let p = Path::new(path);

        let mut file = File::create(p)
            .expect("Cannot create CSV file");

        // Header
        writeln!(
            file,
            "committee_size,threshold,total_requests,approved,rejected,avg_latency_ms,approval_rate,rejection_rate"
        ).unwrap();

        // Row
        writeln!(
            file,
            "{},{},{},{},{},{:.6},{:.6},{:.6}",
            metrics.committee_size,
            metrics.threshold,
            metrics.total_requests,
            metrics.approved_requests,
            metrics.rejected_requests,
            metrics.average_latency_ms(),
            metrics.approval_rate(),
            metrics.rejection_rate(),
        ).unwrap();
    }

}