use pyo3::prelude::*;

pub mod proof;
pub mod tests;


#[pyfunction]
fn spend()
{
    /*
        Params
        - Ergo node url
        - SurrealDB endpoint  https://surrealdb.com/docs/embedding/rust#connect
        - proof
        - amount
        - pointer
    */
    println!("Spend function.");
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
    println!("Compute function.");
}


#[pymodule]
fn compute_reputation_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spend, m)?)?;
    m.add_function(wrap_pyfunction!(compute, m)?)?;

    Ok(())
}