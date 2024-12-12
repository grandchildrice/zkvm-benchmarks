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
   git clone --recursive <repository-url>
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

### Run All Examples
Run all examples from every submodule using the Makefile:
```bash
make all
```

### Run Examples for Specific Submodule
Run examples for a specific submodule:
```bash
make sp1
make risc0
make jolt
make nexus
make ceno
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

## Makefile
The repository includes a Makefile to simplify running benchmarks. Update `example_name` in the Makefile to the desired example names specific to each submodule.

## License
Refer to the individual submodules for their respective licenses.
