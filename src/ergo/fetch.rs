use serde_json::json;
use thiserror::Error;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::database::generate::generate;
use crate::database::spend::store_on_db;

use super::box_vector::{Root, Asset};
use super::contract::ProofContract;
use super::explorer::error::ExplorerApiError;
use super::explorer::explorer_api::ExplorerApi;
use super::utils::{UtilError, string_to_rendered, serialized_to_rendered, generate_pk_proposition};

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Unknown error")]
    Unknown,

    #[error("Database error")]
    DatabaseError,

    #[error("Parsing JSON object error")]
    JsonParserError,

    #[error("Explorer API error")]
    ExplorerError(#[from] ExplorerApiError),

    #[error("Error from utility")]
    UtilityError(#[from] UtilError),

    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<FetchError>),
}

impl From<FetchError> for PyErr {
    fn from(err: FetchError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

fn fetch_sync(ergo_tree_template_hash: &str, reputation_token_label: &str, wallet_pk: &str) -> Result<String, FetchError> {
    ExplorerApi::new().get_unspend_boxes_search(
        json!({
            "ergoTreeTemplateHash": ergo_tree_template_hash,
            "registers": {
                "R4": string_to_rendered(reputation_token_label)?,
                "R7": serialized_to_rendered(generate_pk_proposition(wallet_pk)?.as_str()),  // There's no need to remove the script and then add the 07 from the GroupElement.
            },
            "constants": {},
            "assets": []
        })
    ).map_err(|err| FetchError::ExplorerError(err))
}

pub fn fetch_proofs(database_file: Option<String>) -> Result<String, FetchError> {
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

    let fetch_response = fetch_sync(&contract.as_str(), reputation_token_label, wallet_pk)?;
    let fetch_obj: Root  = serde_json::from_str(&fetch_response)?;

    let mut errors: Vec<FetchError> = Vec::new();
    let error_limit = 10;
    'processing: for box_item in &fetch_obj.items {  // Loop label is not strictly needed.
        let assets = match &box_item.assets {
            Some(assets) if !assets.is_empty() => assets,
            _ => {
                errors.push(FetchError::JsonParserError);
                if errors.len() >= error_limit {
                    break 'processing; // Sale del bucle etiquetado
                }
                continue;
            }
        };
        let asset: &Asset = &assets[0];
    
        if let Err(e) = store_on_db(
            asset.token_id.clone(),
            asset.amount.unwrap_or_default() as i64,
            match &box_item.additional_registers {
                Some(p) 
                => p.get("R6").and_then(|register| register.rendered_value.clone()),
                None => None
            },
            box_item.settlement_height,
            generate(database_file.clone())
        ) {
            errors.push(FetchError::DatabaseError);
            if errors.len() >= error_limit {
                break 'processing; // Sale del bucle etiquetado
            }
        }
    }

    if errors.is_empty() {
        Ok("".to_string()) // Retorna Ok si no hubo errores
    } else {
        // Devuelve un error que contiene todos los errores acumulados
        Err(FetchError::MultipleErrors(errors))
    }
}