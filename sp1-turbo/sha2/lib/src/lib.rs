use serde::{Deserialize, Serialize};

extern crate alloc;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct PublicValuesStruct {
    pub input: Vec<u8>,
    pub result: [u8; 32],
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn sha2(input: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    result.into()
}
