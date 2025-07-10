# ZK-VM Benchmarks

This repo is inspired by grandchildrice ZK-VMs benchmarks, based on a16z ZK-VM benchmarks.
grandchildrice benchmarks focuses on ZK-VMs capable of executing Rust programs (Jolt, Nexus, OpenVM, Pico, RiscZero, SP1, ZKM), aiming at fairness in conditions on powerful machines. The current repo extends these to ZK-VMs targeting zkDSLs, namely CairoM, Noir ProveKit and Miden.

The goal of the current benchmarks is to compare ZK-VMs in a context of client-side proving on consumer devices: laptop and mobile. The on-chain verification is discarded and proof size is less important.

As the language used to define programs might not exactly have the same features, an _applicative_ approach has been taken: the benchmarked program is the most optimized version for a given zkDSL/ZK-VM.
For example, the fibonacci program will use the native field of the ZK-VM when doable.

## Benchmark state

| ZK-VM         | Fibonacci |
| ------------- | --------- |
| Cairo         | ✅        |
| Cairo M       | ✅        |
| Miden         | ✅        |
| Noir ProveKit | ✅        |
| OpenVM        | ✅        |
| RiscZero      | ✅        |
| SP1           | ✅        |
| ZKM           | ✅        |

The following ZK-VMs are yet to be adapted: Ceno, Jolt, Nexus, Noir Barretenberg, Pico, Valida

Benchmarks can be done on ARM64 (MacOS) and x86 architectures.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Scarb](https://docs.swmansion.com/scarb/)
- [uv](https://docs.astral.sh/uv/getting-started/installation/)

### MacOS

- [lld](https://formulae.brew.sh/formula/lld)

## Run Benchmarks

### Local

#### Setup ZK-VM Toolchains

Install all required toolchains:

- [Noir](https://noir-lang.org/docs/getting_started/quick_start)
- [OpenVM](https://book.openvm.dev/getting-started/install.html)
- [R0](https://dev.risczero.com/api/zkvm/install)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)
- [ZKM](https://docs.zkm.io/introduction/installation.html)

#### Launch benchmark

Either launch all benchmarks in a single command:

```bash
make bench-all
```

Or launch benchmark for a given ZK-VM:

```bash
make bench-<cairo|cairo-m|miden|noir-provekit|openvm|risczero|sp1|zkm>
```

## Benchmark Details

### Guest Programs

Not all the benchmarked ZK-VMs are working with similar guest programs.
To still compare the different projects, an applicative approach has been taken: the most optimized version of the guest program for a project is used.

#### Fibonacci

The computation of the 10, 100, 1,000, 10,000 and 100,000 terms of the Fibonacci sequence are benchmarked.

For fibonacci, the program requirements are loose so the native field of a ZK-VM can be used in zkDSL, avoiding range checks while still computing the n-th term of a fibonacci sequence.

### Security Level

To properly compare the various ZK-VM projects, all projects should have the same expected security level, expressed in bits.
The following table summarizes the expected security level of the ZK-VMs in these benchmarks.

| ZK-VM         | Security level (bits) | Security model docs                                                                                                                                                                                                                                                                                                                      |
| ------------- | --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cairo         | 96                    | [link](./cairo/src/main.rs#L26)                                                                                                                                                                                                                                                                                                          |
| Cairo M       | 96                    | [link](https://github.com/kkrt-labs/zkvm-benchmarks/blob/accbfa6a4ad949596936660503bd6ba53e576373/cairo-m/src/main.rs#L114)                                                                                                                                                                                                              |
| Miden         | 96                    | [link](https://github.com/0xMiden/miden-vm/blob/1878ce974a7aa8834e70072b5ef3ca4d299b9873/air/src/options.rs#L182-L186)                                                                                                                                                                                                                   |
| Noir ProveKit | 128                   | [link](https://github.com/worldfnd/ProveKit/blob/77304a3509554ef82025348ecbb660614ac50c0a/noir-r1cs/src/whir_r1cs.rs#L96)                                                                                                                                                                                                                |
| OpenVM        | 100                   | [link1](./openvm/src/bin/fibonacci.rs#L79), [link2](https://github.com/openvm-org/stark-backend/blob/b0bec8739d249370f91862f99c2ecc2c03d33240/crates/stark-sdk/src/config/fri_params.rs#L29)                                                                                                                                             |
| RiscZero      | 96                    | [link](https://github.com/risc0/risc0/blob/bef7bf580eb13d5467074b5f6075a986734d3fe5/website/api/security-model.md#cryptographic-security)                                                                                                                                                                                                |
| SP1           | 100                   | [link1](https://docs.succinct.xyz/assets/files/SP1_Turbo_Memory_Argument-b042ba18b58c4add20a8370f4802f077.pdf), [link2](https://docs.succinct.xyz/docs/sp1/security/security-model#security-of-elliptic-curves-over-extension-fields), [link3](https://docs.succinct.xyz/docs/sp1/security/security-model#conjectures-for-fris-security) |
| ZKM           | 100                   | [link1](https://docs.zkm.io/design/memory-checking.html#elliptic-curve-selection-over-koalabear-prime-extension-field), [link2](https://github.com/ProjectZKM/Ziren/blob/52dd269d475b10b6b2ddc5df3155814633491f24/crates/stark/src/kb31_poseidon2.rs#L202-L203)                                                                          |

For FRI-STARKs related ZK-VMs, the security level is conjectured based on proximity gap proofs and "Toy Problem" related to FRI . This means the soundness is not proven in the traditional cryptographic sense [see paper](https://eprint.iacr.org/2024/1161.pdf).

The security level is tunable: either by increasing the number of queries, which will only increase the proof size ; or by increasing the number of PoW bits (grinding) or the blowup factor, which will only increase the proving time.

## Results

### MacBook M2 Max

Results can be found [here](.outputs/benchmark/simple_benchmarks.ipynb).
Note that any process on your device can influence the results.

## Contributions

If there are inconsistencies, errors, or improvements, contributions are welcome.
