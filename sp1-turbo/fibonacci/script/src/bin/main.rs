//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```
//!

use fibonacci_lib::PublicValuesStruct;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::time::Duration;
use utils::{benchmark, size};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");
type BenchResult = (Duration, usize, usize);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_once = args.iter().any(|arg| arg == "--once");
    if is_once {
        println!("Profile mode activated: executing bench_fib(100) only...");
        sp1_sdk::utils::setup_logger();
        dotenv::dotenv().ok();
        // Setup the prover client.
        let client = ProverClient::from_env();
        // Setup the inputs.
        let mut stdin = SP1Stdin::new();
        let n = 100;
        stdin.write(&n);
        println!("n: {}", n);
        // Setup the program for proving.
        let (pk, _vk) = client.setup(FIBONACCI_ELF);
        let _proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");
    } else {
        if std::env::var("SP1_PROVER").unwrap_or_default() == "cuda" {
            let n: u32 = args
                .iter()
                .skip_while(|arg| *arg != "--n")
                .nth(1)
                .expect("Please provide a value for --n")
                .parse()
                .expect("Value for --n should be a valid u32");

            benchmark(
                bench_fib,
                &[n],
                format!("../benchmark_outputs/fib_sp1turbo-gpu-{}.csv", n).as_str(),
                "n",
            );
        } else {
            let lengths = [10, 100, 1000, 10000, 100000];
            benchmark(
                bench_fib,
                &lengths,
                "../benchmark_outputs/fib_sp1turbo.csv",
                "n",
            );
        }
    }
}

fn bench_fib(n: u32) -> BenchResult {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    println!("n: {}", n);

    // Execute the program
    let (output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
    println!("Program executed successfully.");

    // Read the output.
    let decoded: PublicValuesStruct = bincode::deserialize(output.as_ref()).unwrap();
    let PublicValuesStruct { n, a, b } = decoded;
    println!("n: {}", n);
    println!("a: {}", a);
    println!("b: {}", b);

    let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
    assert_eq!(a, expected_a);
    assert_eq!(b, expected_b);
    println!("Values are correct!");

    // Record the number of cycles executed.
    println!("Number of cycles: {}", report.total_instruction_count());

    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    let start = std::time::Instant::now();
    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);

    println!("Successfully generated proof!");

    // Verify the proof.
    // client.verify(&proof, &vk).expect("failed to verify proof");

    (
        duration,
        size(&proof),
        report.total_instruction_count() as usize,
    )
}
