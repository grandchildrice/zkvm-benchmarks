mod wrapper;

use alloy_consensus::Transaction;
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_provider::{
    network::primitives::{BlockTransactions, BlockTransactionsKind},
    Provider, ProviderBuilder,
};
use indicatif::ProgressBar;
use revm::{primitives::TxKind, Context, ExecuteCommitEvm, MainBuilder, MainContext};
use revm_database::{AlloyDB, CacheDB, StateBuilder};
use revm_database_interface::WrapDatabaseAsync;
use std::{fs::File, io::{BufWriter, Write}, sync::Arc};
use std::time::Instant;

use wrapper::SleepWrapperDB;
use ethblock_utils::DummyDB;
use ethblock_utils::{BlockInfo, TransactionInfo};


/*
NOTE
TXをREVMで実行すると、必要なステートは必要に応じてRPCから読み込まれる。
zkVMで実行する前に一度EVMを実行することで、そのTXに必要なステートをあらかじめ読み込ませておくことが可能である。
revm_database::CacheDB<ExtDB> は繰り返しの読み込みを防ぐための構造であり、これを利用することで効率的にキャッシュを活用できる。
すべての実行が終了すると、CacheDB には ExtDB から読み込まれたステートが記録されている。
zkVM上での実行では、すべてのメソッドが todo!() の DummyDB を CacheDB に渡せばよい。


infuraへのCallはrequest limitがあり、REVMの実行速度では制限に達してしまうので SleepWrapperDB で1秒まつことで回避しているが、
全てのTXの実行をすることはできない。 この実装では、最初の5個のTXだけを実行している。
*/

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://ethereum-rpc.publicnode.com".parse()?;

    // Create ethers client and wrap it in Arc<M>
    let client = ProviderBuilder::new().on_http(rpc_url);

    // Params
    let chain_id: u64 = 1;
    let block_number = 10889449;

    // Fetch the transaction-rich block
    let block = match client
        .get_block_by_number(
            BlockNumberOrTag::Number(block_number),
            BlockTransactionsKind::Full,
        )
        .await
    {
        Ok(Some(block)) => block,
        Ok(None) => anyhow::bail!("Block not found"),
        Err(error) => anyhow::bail!("Error: {:?}", error),
    };
    println!("Fetched block number: {}", block.header.number);
    let previous_block_number = block_number - 1;

    // Use the previous block state as the db with caching
    let prev_id: BlockId = previous_block_number.into();  // Note: 前のブロックを参照して、state_dbとしているのだが、向こうでも必要？
    // SAFETY: This cannot fail since this is in the top-level tokio runtime

    let state_db = WrapDatabaseAsync::new(SleepWrapperDB::new(AlloyDB::new(client, prev_id), 10)).unwrap();
    let cache_db: CacheDB<_> = CacheDB::new(state_db);
    // let wapper_db = WrapperDB::new(cache_db);
    let mut state = StateBuilder::new_with_database(cache_db).build();

    let mut block_info = BlockInfo {
        number: block.header.number,
        beneficiary: block.header.beneficiary,
        timestamp: block.header.timestamp,
        difficulty: block.header.difficulty,
        gas_limit: block.header.gas_limit,
        basefee: block.header.base_fee_per_gas.unwrap_or_default(),
        transactions: vec![]
    };

    let ctx = Context::mainnet()
        .with_db(&mut state)
        .modify_block_chained(|b| {
            b.number = block.header.number;
            b.beneficiary = block.header.beneficiary;
            b.timestamp = block.header.timestamp;

            b.difficulty = block.header.difficulty;
            b.gas_limit = block.header.gas_limit;
            b.basefee = block.header.base_fee_per_gas.unwrap_or_default();
        })
        .modify_cfg_chained(|c| {
            c.chain_id = chain_id;
        });

    let mut evm = ctx.build_mainnet();

    let txs = block.transactions.len();
    println!("Found {txs} transactions.");

    let console_bar = Arc::new(ProgressBar::new(txs as u64));
    let start = Instant::now();

    // Fill in CfgEnv
    let BlockTransactions::Full(transactions) = block.transactions.clone() else {
        panic!("Wrong transaction type")
    };

    for tx in transactions {
        // sleep(Duration::from_secs(10)).await;
        let tx_info = TransactionInfo {
            from: tx.from,
            gas_limit:tx.gas_limit(),
            gas_price: tx.gas_price().unwrap_or(tx.inner.max_fee_per_gas()),
            value: tx.value(),
            input: tx.input().to_owned(),
            max_priority_fee_per_gas: tx.max_priority_fee_per_gas(),
            chain_id: Some(chain_id),
            nonce: tx.nonce(),
            access_list: tx.access_list().cloned(),
            to: tx.to(),
        };
        block_info.transactions.push(tx_info);

        evm.modify_tx(|etx| {
            etx.caller = tx.from;
            etx.gas_limit = tx.gas_limit();
            etx.gas_price = tx.gas_price().unwrap_or(tx.inner.max_fee_per_gas());
            etx.value = tx.value();
            etx.data = tx.input().to_owned();
            etx.gas_priority_fee = tx.max_priority_fee_per_gas();
            etx.chain_id = Some(chain_id);
            etx.nonce = tx.nonce();
            if let Some(access_list) = tx.access_list() {
                etx.access_list = access_list.clone()
            } else {
                etx.access_list = Default::default();
            }

            etx.kind = match tx.to() {
                Some(to_address) => TxKind::Call(to_address),
                None => TxKind::Create,
            };
        });

        let res = evm.transact_commit_previous();

        if let Err(error) = res {
            panic!("Got error: {:?}", error);
        }

        console_bar.inc(1);
    }

    console_bar.finish_with_message("Finished all transactions.");

    let elapsed = start.elapsed();
    println!(
        "Finished execution. Total CPU time: {:.6}s",
        elapsed.as_secs_f64()
    );

    /* 一度使用したcacheを再利用 */
    println!("------------------------------------------------\n");

    let mut cache_db = CacheDB::new(DummyDB::new());
    let database = state.database;
    cache_db.accounts = database.accounts;
    cache_db.contracts = database.contracts;
    cache_db.logs = database.logs;
    cache_db.block_hashes = database.block_hashes;
    println!("accounts {}", cache_db.accounts.len());
    println!("contracts {}", cache_db.contracts.len());
    println!("logs {}", cache_db.logs.len());
    println!("block_hashes {}", cache_db.block_hashes.len());

    /* 保存 */
    std::fs::create_dir_all("block_state_caches")?;
    let path = format!("block_state_caches/block_{block_number}.bin");
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    bincode::serialize_into(&mut writer, &(block_info, cache_db))?;
    writer.flush()?;

    Ok(())
}
