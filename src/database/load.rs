use std::future::Future;
use thiserror::Error;
use std::pin::Pin;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::database::generate::DatabaseAsync;
use crate::database::global::{*};
use crate::proof::reputation_proof::ReputationProof;
use crate::proof::pointer_box::{Pointer, PointerBox};

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("IO error on loading module")]
    IOError(#[from] std::io::Error)
}

impl From<LoadError> for PyErr {
    fn from(err: LoadError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

fn recursive(proof_id: Option<String>, db: Surreal<Db>) -> Pin<Box<dyn Future<Output = Result<ReputationProof, LoadError>>>>
{
    //  Why Box::pin? ->  https://doc.rust-lang.org/error_codes/E0733.html
    Box::pin(async move {
        
        let _proof_id = proof_id.unwrap_or(String::from(""));
        let proof_boxes: Vec<RPBoxDB> = 
            db.query(&format!("SELECT amount, pointer, proof_id FROM {} WHERE proof_id=$proof_id AND pointer!=''", RESOURCE))
                .bind(("proof_id", &_proof_id))
                .await.expect(DB_ERROR_MSG).take(0).unwrap();
        
        let mut proof = {
            ReputationProof::create(
                _proof_id.clone().into_bytes(),
                {
                    let r: Vec<i64> =
                        db.query(&format!("SELECT math::sum(amount) AS value FROM {} WHERE proof_id=$proof_id GROUP ALL", RESOURCE))
                            .bind(("proof_id", &_proof_id))
                            .await.expect(DB_ERROR_MSG)
                            .take("value").expect(DB_ERROR_MSG);
                    if let Some(value) = r.get(0) { *value } else { 0 }
                }
            )
        };

        for dependency in proof_boxes {
            let dependency_id: String = dependency.pointer;

            if dependency_id == "" {
                unreachable!("{:?}", "Dependency can't be null at this point.");
            }

            if (&proof).can_be_spend(dependency.amount) {
                proof.outputs.push(
                    PointerBox::new(
                        vec![], 
                        dependency.amount, 
                        {
                            // TODO hay que controlar si existe ese _proof_pointed, de no ser asi retorna un Pointer::String(String::from(dependency_id)), esto antes de hacer el recursive.
                            match check_if_proof_id_exists(dependency_id.clone(), db.clone()).await {
                                true => match recursive(Some(dependency_id), db.clone()).await {
                                    Ok(_proof_pointed) => Pointer::ReputationProof(_proof_pointed),
                                    Err(err) => {
                                        eprint!("{}", err);
                                        Pointer::String(String::from(""))
                                    }
                                },
                                false => Pointer::String(String::from(dependency_id))
                            }
                        }
                    )
                );
            }
        }
        Ok(proof)
    })
}

#[tokio::main]
pub async fn load_from_db(proof_id: Option<String>, database: DatabaseAsync) -> Result<ReputationProof, LoadError>
{
    match database.await {
        Ok(db) => {
            match recursive(proof_id, db).await {
                Ok(r) => Ok(r),
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(LoadError::IOError(err))
    }
}