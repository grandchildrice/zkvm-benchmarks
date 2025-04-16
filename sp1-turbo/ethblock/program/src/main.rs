//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use ethblock_lib::{trace_block, PublicValuesStruct};

fn main() {
    let num_txs = sp1_zkvm::io::read::<usize>();

    let b = trace_block(num_txs);
    assert!(b);

    sp1_zkvm::io::commit(&PublicValuesStruct { result: b });
}
