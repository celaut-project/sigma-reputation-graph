#[cfg(feature = "python")]
pub mod proof;
#[cfg(feature = "python")]
pub mod database;
#[cfg(feature = "python")]
pub mod ergo;

#[cfg(feature = "python")]
use crate::database::load::{load_from_db, LoadError};
#[cfg(feature = "python")]
use crate::database::spend::{store_on_db, SpendError};
#[cfg(feature = "python")]
use crate::database::generate::generate;

#[cfg(feature = "python")]
use crate::proof::pointer_box::Pointer;

#[cfg(feature = "python")]
use crate::ergo::submit::submit::{submit_proofs, SubmitTxError};
#[cfg(feature = "python")]
use crate::ergo::fetch::{fetch_proofs, FetchError};
#[cfg(feature = "python")]

use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyString, PyFloat};

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

/**
Pyo3 doesn't support wrap structs with lifetimes on the Python interpreter.

https://pyo3.rs/main/class.html?highlight=lifetime#no-lifetime-parameters
*/


/**
    Currently, the library does not use asynchronous runtime.
Instead, each call is a process that will use surrealDB on disk (using async for communication
with the DB, but in isolation for each call).
*/
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (database_file))]
fn fetch<'p>(py: Python<'p>, database_file: Option<&PyString>)
    -> Result<&'p PyString, FetchError>
{

    match fetch_proofs(
        match database_file {
            Some(s) => Some(s.to_string()),
            None => None
        }
    ) {
        Ok(result) => Ok(PyString::new(py, &result)),
        Err(err) => Err(err)
    }
}


#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (database_file))]
fn submit<'p>(py: Python<'p>, database_file: Option<&PyString>)
    -> Result<&'p PyString, SubmitTxError>
{
    match submit_proofs(
        match database_file {
            Some(s) => Some(s.to_string()),
            None => None
        }
    ) {
        Ok(result) => Ok(PyString::new(py, &result)),
        Err(error) => Err(error)
    }
}

/**
    Params
    - proof
    - amount
    - pointer
*/
/**
The pointer box parameter must be on-chain.
 */
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (proof_id, amount, pointer, database_file))]
fn spend<'p>(py: Python<'p>, proof_id: &PyString, amount: i64, pointer: Option<&PyString>, database_file: Option<&PyString>)   // TODO surreal_id can be None
   -> Result<&'p PyString, SpendError>
{
    match store_on_db(
        if proof_id.len().unwrap() == 0 { None }
            else { Some(proof_id.to_str().unwrap().parse().unwrap()) },
        amount,
        match pointer {
            Some(p) => Some(p.to_string()),
            None => None
        },
        None,
        generate(match database_file {
            Some(s) => Some(s.to_string()),
            None => None
        })
    ) {
        Ok(result) => Ok(PyString::new(py, &result)),
        Err(error) => Err(error)
    }
}


/**
Params
- root_id surreal_id of the root proof.
- pointer to calculate
 */
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (root_proof_id, pointer, database_file))]
fn compute<'p>(py: Python<'p>, root_proof_id: Option<&PyString>, pointer: &PyString, database_file: Option<&PyString>)
    -> Result<&'p PyFloat, LoadError>
{
    // Reads data from DB and load all the struct on memory.
    match load_from_db(
        match root_proof_id {
            Some(id) => Some(id.to_string()),
            None => None,
        },
        generate(match database_file {
            Some(s) => Some(s.to_string()),
            None => None
        })
    )
    {
        Ok(proof) => {
            let result = proof.compute(Pointer::String(pointer.to_string()));
            Ok(PyFloat::new(py, result))
        },
        Err(error) => Err(error)
    }
}

/*
   TODO If the desired DB mode is Mem, all the methods should run using Tokio. If not, that's not important.
 */
#[cfg(feature = "python")]
#[pymodule]
fn reputation_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch, m)?)?;
    m.add_function(wrap_pyfunction!(submit, m)?)?;
    m.add_function(wrap_pyfunction!(spend, m)?)?;
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    Ok(())
}

#[cfg_attr(feature = "web", wasm_bindgen)]
pub fn hello_browser() -> String {
    "hello_browser".into()
}
