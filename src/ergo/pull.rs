use serde_json::json;
use std::error::Error;

use super::contract::ProofContract;
use super::explorer::explorer_api::ExplorerApi;
use super::utils::{string_to_rendered, serialized_to_rendered, generate_pk_proposition};

fn fetch_sync(ergo_tree_template_hash: &str, reputation_token_label: &str, wallet_pk: &str) -> Result<String, Box<dyn Error>> {
    ExplorerApi::new().get_unspend_boxes_search(json!({
        "ergoTreeTemplateHash": ergo_tree_template_hash,
        "registers": {
            "R4": string_to_rendered(reputation_token_label)?,
            "R7": serialized_to_rendered(generate_pk_proposition(wallet_pk)?.as_str()),
        },
        "constants": {},
        "assets": []
    }))
}

pub fn pull_proofs() {
    let contract = ProofContract::new();
    match contract {
        Ok(contract) => {
            let ergo_tree_template_hash: String = match contract.ergo_tree_hash() {
                Ok(_h) => _h,
                Err(_) => {
                    println!("Error en ergo_tree_hash");
                    todo!()
                },
            };
            let reputation_token_label = "your_reputation_token_label"; // TODO 
            let wallet_pk = "your_change_address";  //  TODO
        
            match fetch_sync(ergo_tree_template_hash.as_str(), reputation_token_label, wallet_pk) {
                Ok(response) => println!("Response: {}", response),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Err(_) => {
            println!("Error en contract generation ");
            todo!()
        },
    }
}
