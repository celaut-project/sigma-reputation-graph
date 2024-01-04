use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use surrealdb::engine::local::{Db, File};
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use crate::database::global::{*};
use crate::proof::reputation_proof::ReputationProof;
use crate::proof::pointer_box::PointerBox;

fn recursive(proof_id: String, db: Surreal<Db>) -> Pin<Box<dyn Future<Output = Result<ReputationProof<'static>, Error>>>>
{
    //  Why Box::pin? ->  https://doc.rust-lang.org/error_codes/E0733.html
    Box::pin(async move {

        let response: Option<ReputationProofDB> = {
            db.select((RESOURCE, proof_id.to_string())).await.expect(DB_ERROR_MSG)
        };

        match response  {
            Some(r) => {
                let pointer_box = r.pointer.map_or_else(
                    || None,
                    |s| Some(PointerBox::String(s)) // TODO add proof enum case.
                );
                let mut proof = ReputationProof::create(Vec::new(),
                                                    r.amount, pointer_box
                );

                // TODO Should be better ->  "SELECT ->leaf.out FROM reputation_proof:{}";
                let query = format!("SELECT out FROM leaf WHERE in = reputation_proof:{}", proof_id.to_string());
                let mut dependencies_response = db.query(query)
                    .await.expect(DB_ERROR_MSG);

                let dependencies: Vec<Thing> = dependencies_response.take("out").expect(DB_ERROR_MSG);
                for dependency in dependencies {
                    let dependency_id = dependency.id.to_raw();

                    match recursive(dependency_id, db.clone()).await {
                        Ok(r) => {

                            if (&proof).can_be_spend(r.total_amount) {
                                proof.outputs.push(r);
                            }
                        },
                        Err(err) => eprintln!("{:?}", err)
                    }
                }
                Ok(proof)
            },
            None => todo!(),
        }
    })
}

#[tokio::main]
pub async fn load_from_db(proof_id: String) -> Result<ReputationProof<'static>, Error>
{
    let db = Surreal::new::<File>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);

    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    match recursive(proof_id, db).await {
        Ok(r) => Ok(r),
        Err(err) => Err(err)
    }
}