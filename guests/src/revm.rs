use core::convert::Infallible;
use revm::{
    context::transaction::AccessList, primitives::TxKind, Context, ExecuteCommitEvm, MainBuilder,
    MainContext,
};
use revm_database::{CacheDB, StateBuilder};
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::vec::Vec;

use revm::{
    primitives::{Address, Bytes, B256, U256},
    state::{AccountInfo, Bytecode},
    Database, DatabaseRef,
};

#[derive(Serialize, Deserialize)]
pub struct DummyDB {}

impl DummyDB {
    pub fn new() -> Self {
        Self {}
    }
}

impl Database for DummyDB {
    #[doc = " The database error type."]
    type Error = Infallible;

    #[doc = " Gets basic account information."]
    fn basic(&mut self, _address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        todo!()
    }

    #[doc = " Gets account code by its hash."]
    fn code_by_hash(&mut self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        todo!()
    }

    #[doc = " Gets storage value of address at index."]
    fn storage(&mut self, _address: Address, _index: U256) -> Result<U256, Self::Error> {
        todo!()
    }

    #[doc = " Gets block hash by block number."]
    fn block_hash(&mut self, _number: u64) -> Result<B256, Self::Error> {
        todo!()
    }
}

impl DatabaseRef for DummyDB {
    #[doc = " The database error type."]
    type Error = Infallible;

    #[doc = " Gets basic account information."]
    fn basic_ref(&self, _address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        todo!()
    }

    #[doc = " Gets account code by its hash."]
    fn code_by_hash_ref(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        todo!()
    }

    #[doc = " Gets storage value of address at index."]
    fn storage_ref(&self, _address: Address, _index: U256) -> Result<U256, Self::Error> {
        todo!()
    }

    #[doc = " Gets block hash by block number."]
    fn block_hash_ref(&self, _number: u64) -> Result<B256, Self::Error> {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct BlockInfo {
    pub number: u64,
    pub beneficiary: Address,
    pub timestamp: u64,
    pub difficulty: U256,
    pub gas_limit: u64,
    pub basefee: u64,
    pub transactions: Vec<TransactionInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionInfo {
    pub from: Address,
    pub gas_limit: u64,
    pub gas_price: u128,
    pub value: U256,
    pub input: Bytes,
    pub max_priority_fee_per_gas: Option<u128>,
    pub chain_id: Option<u64>,
    pub nonce: u64,
    pub access_list: Option<AccessList>,
    pub to: Option<Address>,
}

const BYTES: &[u8] = include_bytes!("../../utils/block_state_caches/block_10889449.bin");

pub fn trace_ethblock(num_txs: usize) -> bool {
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

        let _res = evm.replay_commit();
    }

    true
}
