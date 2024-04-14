use serde_json::json;
use std::error::Error;

use crate::ergo::box_vector::Root;

use super::contract::ProofContract;
use super::explorer::explorer_api::ExplorerApi;
use super::utils::{string_to_rendered, serialized_to_rendered, generate_pk_proposition};

fn fetch_sync(ergo_tree_template_hash: &str, reputation_token_label: &str, wallet_pk: &str) -> Result<String, Box<dyn Error>> {
    ExplorerApi::new().get_unspend_boxes_search(json!({
        "ergoTreeTemplateHash": ergo_tree_template_hash,
        "registers": {
            "R4": string_to_rendered(reputation_token_label)?,
            "R7": serialized_to_rendered(generate_pk_proposition(wallet_pk)?.as_str()),  // There's no need to remove the script and then add the 07 from the GroupElement.
        },
        "constants": {},
        "assets": []
    }))
}

pub fn fetch_proofs() {
    let contract = ProofContract::new();
    let contract: String = match contract {
        Ok(contract) => {
            let ergo_tree_template_hash: String = match contract.ergo_tree_hash() {
                Ok(_h) => _h,
                Err(_) => {
                    println!("Error en ergo_tree_hash");
                    todo!()
                },
            };
            ergo_tree_template_hash
        },
        Err(_) => String::from("18505af21d77405d225cf586591b23303776fbb1a6e4f2adec2a15ddf83d5684"),  // contract template compiled.
    };
    
    let reputation_token_label = "RPT"; // TODO 
    let wallet_pk = "3WyS9EoJJ4zhJf2Eit5m836F6iYNya5SssKFAYH8crwwbSSLHxri";  //  TODO

    match fetch_sync(&contract.as_str(), reputation_token_label, wallet_pk) {
        Ok(response) => {
            let parsed_value:  Result<Root, serde_json::Error> = serde_json::from_str(&response);
            match parsed_value {
                Ok(parsed_data) => {
                    // Now you can work with the parsed_data object in Rust.
                    println!("  parsed value  {:#?}", parsed_data);
                },
                Err(e) => {
                    // Handle the error, e.g., by logging it or displaying a message to the user.
                    eprintln!("Failed to parse JSON data: {}", e);
                }
            };
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}