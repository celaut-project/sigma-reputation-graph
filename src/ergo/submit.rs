use thiserror::Error;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::database::generate::generate;
use crate::database::load::{load_from_db, LoadError};

#[derive(Error, Debug)]
pub enum SubmitError {
    #[error("unknown data store error")]
    Unknown,

    #[error("error loading proofs from database")]
    DatabaseLoadingError(#[from] LoadError)
}

impl From<SubmitError> for PyErr {
    fn from(err: SubmitError) -> PyErr {
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
pub fn submit_proofs(database_file: Option<String>) -> Result<String, SubmitError> {
    // 1. Read proofs.
    let proof = load_from_db(None, generate(database_file.clone()))?;

    // 2. Build and submit transaction.
    Ok("".to_string())
}
