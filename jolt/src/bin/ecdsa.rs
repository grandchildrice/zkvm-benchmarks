use std::time::{Duration, Instant};
use jolt::Serializable;
use utils::benchmark;

type BenchResult = (Duration, usize, usize);

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/ecdsa_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "_gpu" } else { "" },
        ""
    );

    let lengths = [1];
    benchmark(
        bench_ecdsa,
        &lengths,
        &csv_file,
        "n",
    );
}

fn bench_ecdsa(_dummy: usize) -> BenchResult {

    let (prove_ecdsa_verify, _verify_ecdsa_verify) = ecdsa_guest::build_ecdsa_verify();

    let program_summary = ecdsa_guest::analyze_ecdsa_verify();

    let start = Instant::now();
    let (_output, proof) = prove_ecdsa_verify();
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}
