use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use surrealdb::engine::local::{Db, File};
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use crate::database::global::{*};
use crate::proof::reputation_proof::ReputationProof;
use crate::proof::pointer_box::PointerBox;



fn recursive(proof_id: Option<String>, db: Surreal<Db>) -> Pin<Box<dyn Future<Output = Result<ReputationProof<'static>, Error>>>>
{
    //  Why Box::pin? ->  https://doc.rust-lang.org/error_codes/E0733.html
    Box::pin(async move {


        let response: Option<ReputationProofDB> = match proof_id.clone() {
            Some(__proof_id) => {
                db.select((RESOURCE, __proof_id.to_string())).await.expect(DB_ERROR_MSG)
            },
            None => Some(
                ReputationProofDB {
                    pointer: None,
                    amount: {
                        let r: Vec<i64> = 
                            db.query("SELECT math::sum(amount) AS value FROM reputation_proof WHERE id NOTINSIDE <-leaf.out GROUP ALL")
                                .await.expect(DB_ERROR_MSG)
                                .take("value").expect(DB_ERROR_MSG);
                        if let Some(value) = r.get(0) { *value } else { 0 }
                    }
                }
            )
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
                let query = match proof_id.clone() {
                    Some(__proof_id) => format!("SELECT (out) AS id FROM leaf WHERE in = reputation_proof:{}", __proof_id.to_string()),
                    None => format!("SELECT id FROM reputation_proof WHERE id NOTINSIDE <-leaf.out"),
                };

                let mut dependencies_response = db.query(query)
                    .await.expect(DB_ERROR_MSG);

                let dependencies: Vec<Thing> = dependencies_response.take("id").expect(DB_ERROR_MSG);

                for dependency in dependencies {
                    let dependency_id: String = dependency.id.to_raw();

                    match recursive(Some(dependency_id), db.clone()).await {
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
pub async fn load_from_db(proof_id: Option<String>) -> Result<ReputationProof<'static>, Error>
{
    /*let db = Surreal::new::<File>(ENDPOINT)
        .await.expect(DB_ERROR_MSG); */

    let db = Surreal::new::<File>(ENDPOINT).await.expect("");

    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    match recursive(proof_id, db).await {
        Ok(r) => Ok(r),
        Err(err) => Err(err)
    }
}