
use std::convert::Infallible;
use serde::{Deserialize, Serialize};

use revm::{
    context::AccessList, primitives::{Address, Bytes, B256, U256}, state::{AccountInfo, Bytecode}, Database, DatabaseRef
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
    pub transactions: Vec<TransactionInfo>
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