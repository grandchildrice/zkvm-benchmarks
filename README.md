# ZKVM Benchmarks Repository

This repository manages multiple ZKVM implementations as submodules to benchmark and compare their performance using Rust `cargo run` examples. Each implementation resides in its dedicated folder for better organization.

## Submodules

The following ZKVM projects are included as submodules:

- [SP1](https://github.com/succinctlabs/sp1)
- [RISC0](https://github.com/risc0/risc0)
- [Jolt](https://github.com/a16z/jolt)
- [Nexus](https://github.com/nexus-xyz/nexus-zkvm)
- [Ceno](https://github.com/scroll-tech/ceno)

## Setup

1. Clone the repository:

   ```bash
   git clone --recursive git@github.com:grandchildrice/zkvm-benchmarks.git
   cd zkvm-benchmarks
   ```

2. If you forgot to clone with `--recursive`, initialize and update submodules manually:

   ```bash
   git submodule update --init --recursive
   ```

3. Install dependencies for each submodule as required by their respective documentation.

## Usage

### Installation

#### Install SP1

```
curl -L https://sp1.succinct.xyz | bash
sp1up
```

#### Install RISC Zero

```
curl -L https://risczero.com/install | bash
rzup install
```

#### Install Jolt

```
rustup target add riscv32i-unknown-none-elf
```

#### Install Nexus

```
rustup target add riscv32i-unknown-none-elf
cargo install --git https://github.com/nexus-xyz/nexus-zkvm cargo-nexus --tag 'v0.2.4'
```

#### Install Ceno

```
cargo install cargo-make
rustup target add riscv32im-unknown-none-elf
```

### Run Examples for Specific Submodule

Run examples for a specific submodule:

```bash
make sp1-fib
make risc0-fib
make jolt-fib
make nexus-fib
make ceno-fib
```

### Run on Docker

#### SP1

```bash
docker build -f Dockerfile-sp1 -t sp1-example .
docker run -it --rm --cpus=2 sp1-example /bin/bash
cd examples/fibonacci/program && cargo prove build && cd ../script && cargo run --release
```

To record the proof generation time, change the file `examples/fibonacci/script/src/main.rs` like:

```rust
use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::time::Instant;

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("fibonacci-program");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '500' to it.
    let n = 1000u32;

    // The input stream that the program will read from using `sp1_zkvm::io::read`. Note that the
    // types of the elements in the input stream must match the types being read in the program.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Create a `ProverClient` method.
    let client = ProverClient::from_env();

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);
    let start = Instant::now();
    let mut proof = client.prove(&pk, &stdin).run().unwrap();
    let duration = start.elapsed();
    println!("proof generation took {:?}", duration);

    println!("generated proof");

    // Read and verify the output.
    //
    // Note that this output is read from values committed to in the program using
    // `sp1_zkvm::io::commit`.
    let _ = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();

    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    // Test a round trip of proof serialization and deserialization.
    proof.save("proof-with-pis.bin").expect("saving proof failed");
    let deserialized_proof =
        SP1ProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client.verify(&deserialized_proof, &vk).expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}
```

#### RISC0

```bash
docker build -f Dockerfile-RISC0 -t risc0-example .
docker run -it --rm --cpus=2 risc0-example /bin/bash
rzup install # don't care if you fail this command on running symlink cargo-risczero toolchain
cd benchmarks && cargo run --release -- fibonacci
```

#### Nexus

```bash
docker build -f Dockerfile-nexus -t nexus-example .
docker run -it --rm --cpus=2 nexus-example /bin/bash
cargo nexus prove --bin fib3
```

To record the proof generation time, change the file `examples/src/bin/fib3.rs` like:

```rust
// Used in the CI as a small example that uses memory store
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

fn fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let mut c = a + b;
        c %= 7919;
        a = b;
        b = c;
    }

    b
}

#[nexus_rt::main]
fn main() {
    let n = 1000;
    let result = fib(n);
}
```

#### Ceno

```bash
docker build -f Dockerfile-ceno -t ceno-example .
docker run -it --rm --cpus=2 ceno-example /bin/bash
cd ceno_zkvm && cargo bench --bench fibonacci
```

### Clean All Projects

Clean build artifacts for all submodules:

```bash
make clean
```

### Submodule Updates

To fetch the latest updates for all submodules:

```bash
git submodule foreach git pull origin main
```

### Make a Report (Jupyter Notebook)

```bash
pip install notebook pandas matplotlib
jupyter notebook
```

## License

Refer to the individual submodules for their respective licenses.
