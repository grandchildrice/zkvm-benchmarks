use std::cell::RefCell;

use ethblock_utils::{BlockInfo, DummyDB};
use getrandom::register_custom_getrandom;
use revm::{primitives::TxKind, Context, ExecuteCommitEvm, MainBuilder, MainContext};
use revm_database::{CacheDB, StateBuilder};

use rand::{rngs::SmallRng, RngCore, SeedableRng};

/* wasm32-unknown-unknown でrandを使用する */
thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(123456789));
}
fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG.with(|rng| rng.borrow_mut().fill_bytes(buf));
    Ok(())
}
register_custom_getrandom!(custom_getrandom);

const BYTES: &[u8] = include_bytes!("../../../sp1-turbo/block_state_caches/block_10889449.bin");

#[no_mangle]
pub fn ethblock(num_txs: usize) -> bool {
    // Params
    let chain_id: u64 = 1;
    // let block_number = 10889449;

    let (block_info, cache_db): (BlockInfo, CacheDB<DummyDB>) =
        bincode::deserialize(BYTES).unwrap();

    let mut state = StateBuilder::new_with_database(cache_db).build();
    let ctx = Context::mainnet()
        .with_db(&mut state)
        .modify_block_chained(|b| {
            b.number = block_info.number;
            b.beneficiary = block_info.beneficiary;
            b.timestamp = block_info.timestamp;

            b.difficulty = block_info.difficulty;
            b.gas_limit = block_info.gas_limit;
            b.basefee = block_info.basefee;
        })
        .modify_cfg_chained(|c| {
            c.chain_id = chain_id;
        });

    let mut evm = ctx.build_mainnet();

    for tx in block_info.transactions.into_iter().take(num_txs) {
        evm.modify_tx(|etx| {
            etx.caller = tx.from;
            etx.gas_limit = tx.gas_limit;
            etx.gas_price = tx.gas_price;
            etx.value = tx.value;
            etx.data = tx.input;
            etx.gas_priority_fee = tx.max_priority_fee_per_gas;
            etx.chain_id = tx.chain_id;
            etx.nonce = tx.nonce;
            if let Some(access_list) = tx.access_list {
                etx.access_list = access_list.clone()
            } else {
                etx.access_list = Default::default();
            }

            etx.kind = match tx.to {
                Some(to_address) => TxKind::Call(to_address),
                None => TxKind::Create,
            };
        });

        let res = evm.transact_commit_previous();

        if let Err(error) = res {
            println!("Got error: {:?}", error);
        }
    }

    true
}
