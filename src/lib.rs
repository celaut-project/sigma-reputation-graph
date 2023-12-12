use std::future::Future;
use pyo3::prelude::*;
use pyo3::types::PyString;
use crate::database::spend::store_on_db;

pub mod proof;
pub mod database;
pub mod tests;

/**
Pyo3 doesn't support wrap structs with lifetimes on the Python interpreter.

https://pyo3.rs/main/class.html?highlight=lifetime#no-lifetime-parameters
*/


/**
    Currently, the library does not use asynchronous runtime.
Instead, each call is a process that will use surrealDB on disk (using async for communication
with the DB, but in isolation for each call).
*/

#[pyfunction]
fn submit(_proof_id: Vec<u8>)
{
    // Verify if all the previous proofs were on-chain.
    // Submit all the proof with proof_id and all the childs.
}

/**
The pointer box parameter must be on-chain.
 */
#[pyfunction]
fn spend<'p>(py: Python<'p>, surreal_id: &PyString, amount: i64)
   -> Result<&'p PyString, std::io::Error>
{
    /*
        Params
        - Ergo node url
        - SurrealDB endpoint  https://surrealdb.com/docs/embedding/rust#connect
        - proof
        - amount
        - pointer
    */

    match store_on_db(
        if surreal_id.len().unwrap() == 0 { None }
            else { Some(surreal_id.to_str().unwrap().parse().unwrap()) },
        amount
    ) {
        Ok(id) => Ok(PyString::new(py, &id)),
        Err(error) => Err(error)
    }
}

#[pyfunction]
fn compute()
{
    /*
        Params
        - Ergo node url
        - SurrealDB endpoint
        - pointer to calculate
    */

    // Reads data from DB and load all the struct on memory.
    println!("Compute function.");
}

/*
   TODO If the desired DB mode is Mem, all the methods should run using Tokio. If not, that's not important.
 */

#[pymodule]
fn compute_reputation_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(submit, m)?)?;
    m.add_function(wrap_pyfunction!(spend, m)?)?;
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    Ok(())
}