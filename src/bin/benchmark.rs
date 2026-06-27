use o2r::{
    config::BenchmarkConfig,
    benchmark::experiments::Experiments,
    benchmark::csv::CsvExporter,
};

fn main() {

    let cfg =
        BenchmarkConfig::load("config/benchmark.toml");

    println!("Running FULL benchmark suite...");

    // =========================
    // 1. Committee scaling
    // =========================
    let committee_results =
        Experiments::committee_scaling(
            &cfg.committee_sizes,
            cfg.request_counts[0],
        );

    for (i, m) in committee_results.iter().enumerate() {

        let path = format!(
            "{}/committee_{}.csv",
            cfg.output_dir,
            cfg.committee_sizes[i]
        );

        CsvExporter::export(&path, m);
    }

    // =========================
    // 2. Request scaling
    // =========================
    let request_results =
        Experiments::request_scaling(
            cfg.committee_sizes[0],
            cfg.threshold(cfg.committee_sizes[0]),
            &cfg.request_counts,
        );

    for (i, m) in request_results.iter().enumerate() {

        let path = format!(
            "{}/request_{}.csv",
            cfg.output_dir,
            cfg.request_counts[i]
        );

        CsvExporter::export(&path, m);
    }

    // =========================
    // 3. Threshold scaling
    // =========================
    let ratios = vec![0.4, 0.5, 0.6, 0.7];

    let threshold_results =
        Experiments::threshold_scaling(
            cfg.committee_sizes[0],
            &ratios,
            cfg.request_counts[0],
        );

    for (i, m) in threshold_results.iter().enumerate() {

        let path = format!(
            "{}/threshold_{}.csv",
            cfg.output_dir,
            i
        );

        CsvExporter::export(&path, m);
    }

    println!("Done. Full dataset generated in {}", cfg.output_dir);
}