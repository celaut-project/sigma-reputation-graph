use std::time::Duration;
use ergo_lib::chain::transaction::TxId;
use ergo_lib::ergotree_ir::chain::address::NetworkPrefix;

use crate::ergo::endpoints::Endpoints;
use super::error::ExplorerApiError;
use super::explorer_api::ExplorerApi;

pub(crate) fn ergo_explorer_transaction_link(tx_id: TxId, prefix: NetworkPrefix) -> String {
    let url = Endpoints::default()
        .explorer_url
        .clone();
    let tx_id_str = String::from(tx_id);
    url.join("en/transactions/")
        .unwrap()
        .join(&tx_id_str)
        .unwrap()
        .to_string()
}

pub fn wait_for_tx_confirmation(tx_id: TxId) {
    wait_for_txs_confirmation(vec![tx_id]);
}

pub fn wait_for_txs_confirmation(tx_ids: Vec<TxId>) {
    let timeout = Duration::from_secs(1200);
    let explorer_url = Endpoints::default()
        .explorer_url
        .clone();
    let explorer_api = ExplorerApi::new(explorer_url);
    let start_time = std::time::Instant::now();
    println!("Waiting for block confirmation from ExplorerApi for tx ids: {tx_ids:?} ...");
    let mut remaining_txs = tx_ids.clone();
    loop {
        for tx_id in remaining_txs.clone() {
            match explorer_api.get_transaction_v1(tx_id) {
                Ok(tx) => {
                    assert_eq!(tx.id(), tx_id);
                    log::info!("Transaction found: {tx_id}");
                    remaining_txs.retain(|id| *id != tx_id);
                }
                Err(ExplorerApiError::SerdeError(_)) => {
                    // remove after https://github.com/ergoplatform/explorer-backend/issues/249 is fixed
                    log::info!("Transaction found, but failed to parse: {tx_id}");
                    remaining_txs.retain(|id| *id != tx_id);
                }
                Err(_e) => {
                    log::debug!("ExplorerApi error: {_e}");
                }
            }
        }
        if remaining_txs.is_empty() {
            break;
        }
        if start_time.elapsed() > timeout {
            println!("Timeout waiting for transactions");
            break;
        }
        println!(
            "Elapsed: {}s out of {}s (timeout)",
            start_time.elapsed().as_secs(),
            timeout.as_secs()
        );
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}