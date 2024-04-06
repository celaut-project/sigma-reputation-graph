use reqwest;
use serde_json::json;
use std::error::Error;

use super::utils::{string_to_rendered, serialized_to_rendered, generate_pk_proposition};

// Función síncrona que bloquea el hilo actual hasta que se complete la solicitud HTTP.
fn fetch_sync(explorer_uri: &str, ergo_tree_template_hash: &str, reputation_token_label: &str, change_address: &str) -> Result<String, Box<dyn Error>> {
    // Iniciar un nuevo runtime de Tokio para ejecutar la operación asíncrona de manera síncrona
    let response = tokio::runtime::Runtime::new()?.block_on(async {
        let r4_value = string_to_rendered(reputation_token_label); // Implementa esta función
        let r7_value = serialized_to_rendered(generate_pk_proposition(change_address)); // Implementa esta función

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/api/v1/boxes/unspent/search", explorer_uri))
            .json(&json!({
                "ergoTreeTemplateHash": ergo_tree_template_hash,
                "registers": {
                    "R4": r4_value,
                    "R7": r7_value,
                },
                "constants": {},
                "assets": []
            }))
            .send()
            .await?;

        // Manejar la respuesta
        if response.status().is_success() {
            response.text().await
        } else {
            Err(response.status().to_string().into())  // TODO hay que mirar de resolver esto.
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
    let ergo_tree_template_hash = "your_ergo_tree_template_hash"; // Reemplaza con tu valor
    let reputation_token_label = "your_reputation_token_label"; // Reemplaza con tu valor
    let change_address = "your_change_address"; // Reemplaza con la dirección de cambio

    match fetch_sync(explorer_uri, ergo_tree_template_hash, reputation_token_label, change_address) {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
