use reqwest;
use serde_json::json;
use std::error::Error;
use std::io::{self, ErrorKind};

use super::utils::{string_to_rendered, serialized_to_rendered, generate_pk_proposition};

fn fetch_sync(explorer_uri: &str, ergo_tree_template_hash: &str, reputation_token_label: &str, change_address: &str) -> Result<String, Box<dyn Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    let response = runtime.block_on(async {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/api/v1/boxes/unspent/search", explorer_uri))
            .json(&json!({
                "ergoTreeTemplateHash": ergo_tree_template_hash,
                "registers": {
                    "R4": string_to_rendered(reputation_token_label),
                    "R7": serialized_to_rendered(generate_pk_proposition(change_address)),
                },
                "constants": {},
                "assets": []
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text().await.map_err(|e| e.into())
                } else {
                    let error_message = format!("Error: {}", resp.status());
                    Err(Box::new(io::Error::new(ErrorKind::Other, error_message)) as Box<dyn Error>)
                }
            },
            Err(e) => {
                Err(Box::new(e) as Box<dyn Error>)
            }
        }
    })?;

    Ok(response)
}

pub fn pull_proofs() {
    let explorer_uri = "https://api-testnet.ergoplatform.com"; // Reemplaza con la URI correcta
    let contract = "{
        proveDlog(SELF.R7[GroupElement].get) &&
        sigmaProp(SELF.tokens.size == 1) &&
        sigmaProp(OUTPUTS.forall { (x: Box) =>
          !(x.tokens.exists { (token: (Coll[Byte], Long)) => token._1 == SELF.tokens(0)._1 }) ||
          (
            x.R7[GroupElement].get == SELF.R7[GroupElement].get &&
            x.tokens.size == 1 &&
            x.propositionBytes == SELF.propositionBytes
          )
        })  
    }";
    // TODO ergo-reputation-system envs.ts  ...

    // let ergoTree = compile(contract, {version: 1})
   
    let ergo_tree_address = ""; // let ergoTreeAddress = ErgoAddress.fromErgoTree(ergoTree.toHex(), Network.Testnet).toString()
    let ergo_tree_hash = ""; // let ergoTreeHash = hex.encode(sha256(ergoTree.template.toBytes()))

    let ergo_tree_template_hash = "your_ergo_tree_template_hash"; // Reemplaza con tu valor
    let reputation_token_label = "your_reputation_token_label"; // Reemplaza con tu valor
    let change_address = "your_change_address"; // Reemplaza con la dirección de cambio

    match fetch_sync(explorer_uri, ergo_tree_template_hash, reputation_token_label, change_address) {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
