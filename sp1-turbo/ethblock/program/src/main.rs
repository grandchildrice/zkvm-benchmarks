//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use ethblock_lib::{trace_block, PublicValuesStruct};

fn main() {
    let b = trace_block();
    assert!(b);

    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { result: b });

    sp1_zkvm::io::commit_slice(&bytes);
}