use ergo_lib::chain::ergo_box::box_builder::{ErgoBoxCandidateBuilder, ErgoBoxCandidateBuilderError};
use ergo_lib::chain::transaction::TxIoVec;
use ergo_lib::ergo_chain_types::{Digest32, DigestNError};
use ergo_lib::ergotree_ir::chain::ergo_box::NonMandatoryRegisterId;
use ergo_lib::ergotree_ir::mir::constant::Constant;
use ergo_lib::ergotree_interpreter::sigma_protocol::prover::ContextExtension;
use ergo_lib::ergotree_ir::chain::address::{AddressEncoderError, NetworkPrefix};
use ergo_lib::ergotree_ir::chain::ergo_box::box_value::{BoxValue, BoxValueError};
use ergo_lib::ergotree_ir::chain::ergo_box::{ErgoBox};
use ergo_lib::ergotree_ir::chain::token::{self, Token, TokenAmount, TokenId};
use ergo_lib::ergotree_ir::types::stype::SType;
use ergo_lib::wallet::box_selector::{BoxSelector, BoxSelectorError, SimpleBoxSelector};
use ergo_lib::wallet::miner_fee::MINERS_FEE_ADDRESS;
use ergo_lib::wallet::signing::TxSigningError;
use ergo_node_interface::scanning::NodeError;
use ergo_node_interface::NodeInterface;
use thiserror::Error;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::ergo::contract::{ProofContract, ProofContractError};
use crate::ergo::explorer::error::ExplorerApiError;
use crate::ergo::explorer::explorer_api::ExplorerApi;
use crate::ergo::submit::prover::{SeedPhrase, SigmaProver};
use crate::database::generate::generate;
use crate::database::load::{load_from_db, LoadError as DBLoadError};
use crate::ergo::submit::prover::{get_ergo_state_context, Wallet};

use super::transaction::TransactionCandidate;

#[derive(Error, Debug)]
pub enum SubmitTxError {
    #[error("unknown data store error")]
    Unknown,

    #[error("error loading proofs from database {0}")]
    DatabaseLoadingError(#[from] DBLoadError),

    #[error("tx sign error {0}")]
    TxSigningError(#[from] TxSigningError),

    #[error("node error {0}")]
    NodeError(#[from] NodeError),

    #[error("box value error {0}")]
    BoxValueError(#[from] BoxValueError),

    #[error("digest n error {0}")]
    DigestNError(#[from] DigestNError),

    #[error("address encoder error")]
    AddressEncoderError(#[from] AddressEncoderError),

    #[error("ergo box candidate builder error {0}")]
    ErgoBoxCandidateBuilderError(#[from] ErgoBoxCandidateBuilderError),

    #[error("box selector error {0}")]
    BoxSelectorError(#[from] BoxSelectorError),

    #[error("Explorer API error {0}")]
    ExplorerError(#[from] ExplorerApiError),

    #[error("Contract error {0}")]
    ProofContractError(#[from] ProofContractError)
}

impl From<SubmitTxError> for PyErr {
    fn from(err: SubmitTxError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}


/**
 * The `submit_proofs` function is designed to process and submit proofs to the Ergo blockchain.
 * Currently, the function is a stub and its final implementation will need to be more sophisticated
 * to efficiently handle system resources and network considerations.
 *
 * The function performs the following steps:
 * 1. Read proofs from local storage. At this stage, the function assumes it can load all proofs
 *    into memory without size restrictions. This will likely change in the final implementation
 *    to handle memory limitations and optimize performance.
 * 2. Construct and submit a transaction to the Ergo blockchain that includes all the read proofs.
 *    The current logic simplifies this process by sending each proof in its own transaction, but
 *    the future implementation should allow for batching multiple proofs into a single transaction
 *    to optimize on ERG costs (Ergo's currency) and reduce network usage.
 *
 * The function returns a `Result` which, on success, provides an empty `String`. If an error occurs
 * during the process, it returns an `Error`, which is a generic way to handle different
 * types of errors in Rust.
 *
 * In its final state, the function will need to be capable of parameterizing memory handling and
 * network usage. For instance, if there are memory constraints, the function might opt to send proofs
 * one at a time. If the network is congested, it might choose to load all proofs and send them in one
 * large transaction. The implementation should also consider a balance between these two extremes.
 *
 * For the time being, the function focuses on the simplest implementation, sending each proof
 * individually, to avoid the complexity of bundling multiple proofs into a single transaction.
 */
pub fn submit_proofs(database_file: Option<String>) -> Result<String, SubmitTxError> {
    
    let fee_value = 1000000;
    let seed = "income chaos lunar arrive because jazz tomato burst stock stay hold velvet network weekend invite".to_string();
    let (prover, addr) = Wallet::try_from_seed(seed).expect("Invalid seed");


    /*
        wHAT IS THE TOKEN ID FOR THE PROOFS THAT ARE NOT FETCHED FROM ERGO ??
     */
    let proof = "4b14d26234bfd7e0dc37148ced29e3410eadf3c9c22787e79d310c5de91bd833".to_string();
    let proof = load_from_db(Some(proof), generate(database_file.clone()))?;

    let token_id_str = String::from_utf8(proof.token_id).unwrap_or_default();
    println!("Id of the proof -> {:?}", &token_id_str);

/*
    token_amount: number, object_to_assign?: string, object_type_to_assign: ObjectType = ObjectType.PlainText
*/
    

    /*
    We have selected a reputation test, so we will add a pointer that assigns the selected amount. 
    If a test is not specified, a new token should be created whose maximum amount would be the specified one.
     */
    let token_amount: u64 = 100;
   //  let input_proof = proof.outputs.first().unwrap();  TODO.   Currently, it is not selected from which pointer the reputation is deducted; this way, the reputation is extracted from where the miner decides? Or ergo-lib?
    let object_to_assign = "rust_proof";
    let object_type_to_assign = "plain-text/utf-8";

    let mut new_token = false;
    if (proof.total_amount as u64) < token_amount {
        println!("TOTAL AMOUNT TOO HIGHT. {} {}", proof.total_amount, token_amount);
        new_token = true;
    }

    let node = NodeInterface::new("", "213.239.193.208", "9052");
    match node {
        Ok(node) => {
            let block_height = match node.current_block_height() {
                Ok(value) => {
                    println!("Current height: {}", value);
                    value
                },
                Err(_) => {
                    println!("ERROR1");
                    0
                }
            };

            let contract_ergo_tree = ProofContract::new()?.ergo_tree();
            let addr_ergo_tree = addr.script().unwrap();

            let token = if !new_token {
                let token_id = Some(TokenId::from(
                    Digest32::try_from(token_id_str).unwrap()
                ));

                Some(Token {
                    token_id: token_id.unwrap(), 
                    amount: token_amount.try_into().unwrap()
                })
            } else {
                None
            }; 

            let target_tokens = if token.is_some() {
                vec![token.clone().unwrap()]
            } else {
                vec![]
            };

            // Selector
            let explorer = ExplorerApi::new();
            let input_boxes: Vec<ErgoBox> = explorer.get_utxos(&addr, NetworkPrefix::Testnet)?;  // This MAYBE only returns boxes with the simple address script. No with reputation proof box.
            let box_selector = SimpleBoxSelector::new();
            let box_selection = box_selector.select(
                input_boxes,
                BoxValue::SAFE_USER_MIN, 
                target_tokens.as_slice()
            )?;

            // Inputs
            let inputs = TxIoVec::from_vec(
                box_selection
                    .clone()
                    .boxes
                    .into_iter()
                    .map(|bx| (bx, ContextExtension::empty()))
                    .collect::<Vec<_>>(),
            )
            .unwrap();

            println!("inputs -> {:?}", inputs);
            
            let input_total_value = box_selection
                .boxes
                .iter()
                .map(|bx| bx.value.as_u64())
                .sum::<u64>();

            let non_used_value = BoxValue::new(
                    input_total_value - fee_value - BoxValue::SAFE_USER_MIN.as_u64()
                )?;

            // Output candidates
            let mut pointer_output = ErgoBoxCandidateBuilder::new(
                BoxValue::SAFE_USER_MIN, 
                contract_ergo_tree.clone(),
                block_height.try_into().unwrap()
                );

            let non_used_output = ErgoBoxCandidateBuilder::new(
                non_used_value, 
                addr_ergo_tree.clone(),
                block_height.try_into().unwrap()
                );

            match token {
                Some(t) => {
                    pointer_output.add_token(t.clone());
                },
                None => {
                    let token: Token = Token {
                        token_id: box_selection.boxes.first().box_id().into(),
                        amount: token_amount.try_into().unwrap(),
                    };
                    
                    pointer_output.mint_token(token.clone(), "".into(), "".into(), 0);
                }
            };
 
            /*
            
            pointer_output.set_register_value(NonMandatoryRegisterId::R4, Constant::from(""));     reputation token label ?
            pointer_output.set_register_value(NonMandatoryRegisterId::R5, Constant::from(object_to_assign));
            pointer_output.set_register_value(NonMandatoryRegisterId::R6, Constant::from(object_to_assign));
            pointer_output.set_register_value(NonMandatoryRegisterId::R7, Constant::from(generate_pk_proposition(¿addr?)));
            
            */

            let fee_value = BoxValue::new(fee_value)?;
            let miner_tree = MINERS_FEE_ADDRESS.script().unwrap();
            let miner_output = ErgoBoxCandidateBuilder::new(fee_value, miner_tree.clone(), block_height.try_into().unwrap());
            let output_candidates = vec![
                non_used_output.build()?,
                pointer_output.build()?, 
                miner_output.build()?
                ];
            let output_candidates = TxIoVec::from_vec(output_candidates.clone()).unwrap();

            let tx = TransactionCandidate {
                inputs,
                data_inputs: None,
                output_candidates
            };

            // Sign
            let signed_tx = prover.sign(tx)?;

            // 2. Submit tx.
            let tx_id = node.submit_transaction(&signed_tx)?;

            println!("txId -> {:?}", tx_id);
        },
        Err(_) => println!("ERROR2")
    }

    Ok("".to_string())
}
