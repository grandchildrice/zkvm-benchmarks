use tokio::time::{sleep, Duration};

use revm::{
    primitives::{Address, B256, U256},
    state::{AccountInfo, Bytecode},
};
use revm_database_interface::{async_db::DatabaseAsyncRef, DatabaseAsync};

pub struct SleepWrapperDB<ExtDB> {
    inner: ExtDB,
    sleep_duration: Duration
}

impl<ExtDB> SleepWrapperDB<ExtDB> {
    pub fn new(inner: ExtDB, sleep_duration: u64) -> Self {
        Self {inner, sleep_duration: Duration::from_millis(sleep_duration)}
    }
}

// const sleep_duration: u64 = 1;

impl<ExtDB: DatabaseAsync> DatabaseAsync for SleepWrapperDB<ExtDB> {
    #[doc = " The database error type."]
    type Error = ExtDB::Error;

    fn basic_async(
        &mut self,
        address: Address,
    ) -> impl std::future::Future<Output = Result<Option<AccountInfo>, Self::Error>> + Send {
        let fut = self.inner.basic_async(address);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
    
    fn code_by_hash_async(
        &mut self,
        code_hash: B256,
    ) -> impl std::future::Future<Output = Result<Bytecode, Self::Error>> + Send {
        let fut = self.inner.code_by_hash_async(code_hash);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
    
    fn storage_async(
        &mut self,
        address: Address,
        index: U256,
    ) -> impl std::future::Future<Output = Result<U256, Self::Error>> + Send {
        let fut = self.inner.storage_async(address, index);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
    
    fn block_hash_async(
        &mut self,
        number: u64,
    ) -> impl std::future::Future<Output = Result<B256, Self::Error>> + Send {
        let fut = self.inner.block_hash_async(number);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
}

impl<ExtDB: DatabaseAsyncRef> DatabaseAsyncRef for SleepWrapperDB<ExtDB> {
    #[doc = " The database error type."]
    type Error = ExtDB::Error;

    fn basic_async_ref(
        &self,
        address: Address,
    ) -> impl std::future::Future<Output = Result<Option<AccountInfo>, Self::Error>> + Send {
        let fut = self.inner.basic_async_ref(address);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
    
    fn code_by_hash_async_ref(
        &self,
        code_hash: B256,
    ) -> impl std::future::Future<Output = Result<Bytecode, Self::Error>> + Send {
        let fut = self.inner.code_by_hash_async_ref(code_hash);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
    
    fn storage_async_ref(
        &self,
        address: Address,
        index: U256,
    ) -> impl std::future::Future<Output = Result<U256, Self::Error>> + Send {
        let fut = self.inner.storage_async_ref(address, index);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
    
    fn block_hash_async_ref(
        &self,
        number: u64,
    ) -> impl std::future::Future<Output = Result<B256, Self::Error>> + Send {
        let fut = self.inner.block_hash_async_ref(number);
        let sleep_duration = self.sleep_duration;
        async move {
            sleep(sleep_duration).await;
            fut.await
        }
    }
}
