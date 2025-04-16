#![no_main]
sp1_zkvm::entrypoint!(main);

use revm_utils::transfer_eth_n_times;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublicValuesStruct {
    pub result: bool,
}

fn main() {
    let num = sp1_zkvm::io::read::<usize>();

    let b = transfer_eth_n_times(num);
    assert!(b);

    sp1_zkvm::io::commit(&PublicValuesStruct { result: b });
}
