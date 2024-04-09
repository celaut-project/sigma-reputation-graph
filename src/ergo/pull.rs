use serde_json::json;
use std::error::Error;

use super::contract::{self, ProofContract};
use super::explorer::explorer_api::ExplorerApi;
use super::utils::{string_to_rendered, serialized_to_rendered, generate_pk_proposition};

fn fetch_sync(ergo_tree_template_hash: &str, reputation_token_label: &str, change_address: &str) -> Result<String, Box<dyn Error>> {
    ExplorerApi::new().get_unspend_boxes_search(json!({
        "ergoTreeTemplateHash": ergo_tree_template_hash,
        "registers": {
            "R4": string_to_rendered(reputation_token_label),
            "R7": serialized_to_rendered(generate_pk_proposition(change_address)),
        },
        "constants": {},
        "assets": []
    }))
}

pub fn pull_proofs() {
    let contract = ProofContract::new();
    match contract {
        Ok(contract) => {
            let ergo_tree_template_hash = match contract.ergo_tree_hash() {
                Ok(_h) => _h,
                Err(_) => {
                    println!("Error en ergo_tree_hash");
                    todo!()
                },
            };
            let reputation_token_label = "your_reputation_token_label"; 
            let change_address = "your_change_address";
        
            match fetch_sync(ergo_tree_template_hash, reputation_token_label, change_address) {
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
