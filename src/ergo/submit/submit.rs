use ergo_lib::chain::ergo_box::box_builder::{ErgoBoxCandidateBuilder, ErgoBoxCandidateBuilderError};
use ergo_lib::chain::transaction::TxIoVec;
use ergo_lib::ergo_chain_types::DigestNError;
use ergo_lib::ergotree_interpreter::sigma_protocol::private_input::PrivateInput;
use ergo_lib::ergotree_interpreter::sigma_protocol::prover::ContextExtension;
use ergo_lib::ergotree_ir::chain::address::{AddressEncoder, AddressEncoderError, NetworkPrefix};
use ergo_lib::ergotree_ir::chain::ergo_box::box_value::BoxValue;
use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;
use ergo_lib::ergotree_ir::chain::token::{Token, TokenId};
use ergo_lib::wallet::box_selector::{BoxSelector, BoxSelectorError, SimpleBoxSelector};
use ergo_lib::wallet::secret_key::SecretKey;
use ergo_lib::wallet::signing::TxSigningError;
use ergo_node_interface::scanning::NodeError;
use ergo_node_interface::NodeInterface;
use thiserror::Error;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::ergo::submit::prover::SigmaProver;
use crate::database::generate::generate;
use crate::database::load::{load_from_db, LoadError as DBLoadError};
use crate::ergo::submit::prover::{get_ergo_state_context, Wallet};

use super::transaction::TransactionCandidate;

#[derive(Error, Debug)]
pub enum SubmitTxError {
    #[error("unknown data store error")]
    Unknown,

    #[error("error loading proofs from database")]
    DatabaseLoadingError(#[from] DBLoadError),

    #[error("tx sign error")]
    TxSigningError(#[from] TxSigningError),

    #[error("node error")]
    NodeError(#[from] NodeError),

    #[error("digest n error")]
    DigestNError(#[from] DigestNError),

    #[error("address encoder error")]
    AddressEncoderError(#[from] AddressEncoderError),

    #[error("ergo box candidate builder error")]
    ErgoBoxCandidateBuilderError(#[from] ErgoBoxCandidateBuilderError),

    #[error("box selector error")]
    BoxSelectorError(#[from] BoxSelectorError),
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
    let proof = "4b14d26234bfd7e0dc37148ced29e3410eadf3c9c22787e79d310c5de91bd833".to_string();
    let proof = load_from_db(Some(proof), generate(database_file.clone()))?;
    println!("Proof -> {:?}", proof);
    println!("Id of the proof -> {:?}", String::from_utf8(proof.token_id));

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
            let token_amount = 100.try_into().unwrap();

            let network_prefix = NetworkPrefix::Testnet;
            let encoder = AddressEncoder::new(network_prefix);
            let addr = encoder.parse_address_from_str("4b14d26234bfd7e0dc37148ced29e3410eadf3c9c22787e79d310c5de91bd833")?;
            let ergo_tree = addr.script().unwrap();
            
            // 1.  Create tx.
            let secret_key = SecretKey::random_dlog();
            let address = secret_key.get_address_from_public_image();

            let ergo_state_context = get_ergo_state_context();
            
            let prover = Wallet {
                secrets: vec![PrivateInput::from(secret_key)],
                ergo_state_context,
            };

            // Output candidates

            let token_id = TokenId::from_base64("4b14d26234bfd7e0dc37148ced29e3410eadf3c9c22787e79d310c5de91bd833")?;
            
            let mut builder = ErgoBoxCandidateBuilder::new((0 as u64).try_into().unwrap(), ergo_tree.clone(),  block_height.try_into().unwrap());
            let token = Token {
                token_id,
                amount: token_amount,
            };
            builder.mint_token(token.clone(), "".into(), "".into(), 0);
            let output_candidates = vec![builder.build()?];
            let output_candidates = TxIoVec::from_vec(output_candidates.clone()).unwrap();

            
            // Inputs
            
            let input_boxes: Vec<ErgoBox> = vec![];
            let box_selector = SimpleBoxSelector::new();
            let box_selection = box_selector.select(input_boxes,  BoxValue::SAFE_USER_MIN, vec![].as_slice())?;
            let inputs = TxIoVec::from_vec(
                box_selection
                    .boxes
                    .into_iter()
                    .map(|bx| (bx, ContextExtension::empty()))
                    .collect::<Vec<_>>(),
            )
            .unwrap();

            
            let tx = TransactionCandidate {
                inputs,
                data_inputs: None,
                output_candidates
            };

            let signed_tx = prover.sign(tx)?;

            // 2. Submit tx.
            let tx_id = node.submit_transaction(&signed_tx)?;

            println!("txId -> {:?}", tx_id);
        },
        Err(_) => println!("ERROR2")
    }

    Ok("".to_string())
}
