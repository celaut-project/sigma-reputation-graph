use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the root structure that holds everything.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Root {
    pub(crate) items: Vec<BoxItem>,
    total: usize,
}


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BoxItem {
    #[serde(rename = "boxId")]
    box_id: Option<String>,
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
    #[serde(rename = "blockId")]
    block_id: Option<String>,
    value: Option<u64>,
    index: Option<usize>,
    #[serde(rename = "globalIndex")]
    global_index: Option<usize>,
    #[serde(rename = "creationHeight")]
    creation_height: Option<usize>,
    #[serde(rename = "settlementHeight")]
    pub(crate) settlement_height: Option<usize>,
    #[serde(rename = "ergoTree")]
    ergo_tree: Option<String>,
    #[serde(rename = "ergoTreeConstants")]
    ergo_tree_constants: Option<String>,
    #[serde(rename = "ergoTreeScript")]
    ergo_tree_script: Option<String>,
    address: Option<String>,
    pub(crate) assets: Option<Vec<Asset>>,
    #[serde(rename = "additionalRegisters")]
    pub(crate) additional_registers: Option<HashMap<
            String, AdditionalRegister>>,
    #[serde(rename = "spentTransactionId")]
    spent_transaction_id: Option<String>,
    #[serde(rename = "mainChain")]
    main_chain: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Asset {
    #[serde(rename = "tokenId")]
    pub(crate) token_id: Option<String>,
    pub(crate) index: Option<usize>,
    pub(crate) amount: Option<u64>,
    pub(crate) name: Option<String>,
    pub(crate) decimals: Option<u8>,
    #[serde(rename = "type")]
    pub(crate) asset_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AdditionalRegister {
    #[serde(rename = "serializedValue")]
    serialized_value: Option<String>,
    #[serde(rename = "sigmaType")]
    sigma_type: Option<String>,
    #[serde(rename = "renderedValue")]
    pub(crate) rendered_value: Option<String>,
}