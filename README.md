# O2R

This repo is the official implementation of O2R paper.

## Repository Structure

- `Cargo.toml` - Rust package manifest and dependency list
- `config/benchmark.toml` - default benchmark configuration
- `src/bin/benchmark.rs` - benchmark runner binary
- `src/config.rs` - configuration loader and threshold helpers
- `src/benchmark/experiments.rs` - scaling experiments and benchmark workflows
- `src/benchmark/runner.rs` - request generation and committee benchmarking logic
- `src/benchmark/csv.rs` - CSV export for benchmark results
- `src/committee/` - committee and node simulation
- `src/crypto/` - cryptographic helper modules for keys, hashing, and Schnorr proofs
- `src/protocol/` - recovery request and state machinery
- `src/metrics/` - metrics collection and derived statistics
- `results/` - generated CSV result files

## Build

Install Rust and Cargo, then build the benchmark binary:

```bash
cargo build --release --bin benchmark
```

## Run

Run the benchmark binary with the provided configuration file:

```bash
cargo run --release --bin benchmark
```

The benchmark runner loads `config/benchmark.toml` by default, executes the full suite, and writes CSV files into the configured output directory.

## Configuration

The benchmark configuration lives in `config/benchmark.toml` and supports:

- `committee_sizes` - list of committee sizes to test
- `request_counts` - number of requests per experiment
- `threshold_ratio` - base majority threshold ratio for threshold-scaling experiments
- `output_dir` - directory where generated CSV files are stored

Example:

```toml
committee_sizes = [1000, 2000, 5000, 10000]
request_counts = [100, 1000, 5000]
threshold_ratio = 0.5
output_dir = "results"
```

## Output

The benchmark produces CSV files such as:

- `results/committee_<size>.csv`
- `results/request_<count>.csv`
- `results/threshold_<index>.csv`

Each CSV contains these columns:

- `committee_size`
- `threshold`
- `total_requests`
- `approved`
- `rejected`
- `avg_latency_ms`
- `approval_rate`
- `rejection_rate`

## Notes

- The current prototype focuses on benchmarking the committee approval path and not on a production-ready protocol implementation.
- `src/main.rs` is a placeholder entrypoint; the benchmark binary is implemented in `src/bin/benchmark.rs`.
- The code uses `k256`, `sha2`, `ripemd`, `rand`, `hex`, `serde`, and `toml`.

## Contact

...
## License

...