use surrealdb::sql::Thing;
use crate::database::generate::DatabaseAsync;
use crate::database::global::{*};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpendError {
    #[error("IO error on loading module")]
    IOError(#[from] std::io::Error)
}

impl From<SpendError> for PyErr {
    fn from(err: SpendError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

/*
If the data originates from the Ergo platform, the corresponding block number is added; otherwise, a value of zero is recorded.
*/
#[tokio::main]
pub async fn store_on_db(proof_id: Option<String>, amount: i64, pointer: Option<String>, ergo_block: Option<usize>, database: DatabaseAsync)
    -> Result<String, SpendError>
{
    match database.await {
        Ok(db) => { 
            let _proof_id = proof_id.unwrap_or(String::from(""));
            let _pointer_id = pointer.unwrap_or(String::from(""));

            let result: Vec<RPBoxDBWithId> = db
                .query(
                    format!(
                        "SELECT * FROM {:?} WHERE pointer='{}' AND proof_id='{}'",
                        RESOURCE, &_pointer_id, &_proof_id
                    )
                )
                .await.expect(DB_ERROR_MSG)
                .take(1).expect(DB_ERROR_MSG);

            match &result[..] {

                [_s] => {  // Match if the query result has exactly one element.

                    let _updated: Option<Thing> = db
                        .update((RESOURCE, _s.id.as_str()))
                        .content(RPBoxDB {
                            proof_id: _s.proof_id.clone(),
                            pointer: _pointer_id.clone(),
                            amount: match &ergo_block {
                                Some(_) => amount,
                                None => _s.amount + amount,
                            },
                            ergo_block: Some(ergo_block.unwrap_or(0))
                        })
                        .await.expect(DB_ERROR_MSG);

                    _s.id.clone()
                },
                _ => {
                    let created: Vec<Record> = db
                        .create(RESOURCE)
                        .content(RPBoxDB {
                            proof_id: _proof_id.clone(),
                            pointer: _pointer_id.clone(),
                            amount,  // TODO could check that amount <= proof->amount
                            ergo_block: Some(ergo_block.unwrap_or(0))
                        })
                        .await.expect(DB_ERROR_MSG);

                    created.first().unwrap().id.to_string()              
                }
            };

            Ok(String::from(""))
        },
        Err(err) => Err(SpendError::IOError(err))
    }
}
