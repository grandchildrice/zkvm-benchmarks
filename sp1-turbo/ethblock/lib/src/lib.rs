use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublicValuesStruct {
    pub result: bool,
}

pub fn trace_block(num_txs: usize) -> bool {
    ethblock_utils::trace_ethblock(num_txs)
}
