use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

use crate::database::generate::DatabaseAsync;
use crate::database::global::{*};
use crate::proof::reputation_proof::ReputationProof;
use crate::proof::pointer_box::{Pointer, PointerBox};



fn recursive(proof_id: Option<String>, db: Surreal<Db>) -> Pin<Box<dyn Future<Output = Result<ReputationProof, Error>>>>
{
    //  Why Box::pin? ->  https://doc.rust-lang.org/error_codes/E0733.html
    Box::pin(async move {


        let proof_boxes: Vec<RPBoxDB> = match proof_id.clone() {
            Some(__proof_id) => {
                db.query("SELECT * FROM $resource WHERE proof_id=$proof_id AND pointer!= NULL")
                    .bind(("resource", RESOURCE))
                    .bind(("proof_id", __proof_id.to_string()))
                    .await.expect(DB_ERROR_MSG).take(0).unwrap()
            },
            None => vec!()
        };

        let mut proof = 
            ReputationProof::create(
                Vec::new(),
                {
                    let r: Vec<i64> =
                                db.query("SELECT math::sum(amount) AS value FROM reputation_proof WHERE proof_id=$proof_id GROUP ALL")
                                    .bind(("proof_id", proof_id.unwrap_or(String::from("")).to_string()))
                                    .await.expect(DB_ERROR_MSG)
                                    .take("value").expect(DB_ERROR_MSG);
                            if let Some(value) = r.get(0) { *value } else { 0 }
                }
            );

        for dependency in proof_boxes {
            let dependency_id: Option<String> = dependency.pointer;

            if dependency_id.is_none() {
                eprintln!("{:?}", "Dependency can't be null at this point.")
            }

            if (&proof).can_be_spend(dependency.amount) {
                proof.outputs.push(
                    PointerBox::new(
                        vec![], 
                        dependency.amount, 
                        match recursive(dependency_id, db.clone()).await {
                            Ok(_proof_pointed) => Pointer::ReputationProof(_proof_pointed),
                            Err(err) => {
                                eprint!("{}", err);
                                Pointer::String(String::from(""))
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
pub async fn load_from_db(proof_id: Option<String>, database: DatabaseAsync) -> Result<ReputationProof, Error>
{
    match database.await {
        Ok(db) => {
            match recursive(proof_id, db).await {
                Ok(r) => Ok(r),
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}