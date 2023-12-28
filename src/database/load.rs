use std::fmt::Debug;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use crate::database::global::{*};
use crate::proof::ReputationProof;

fn recursive(proof_id: String, db: Surreal<Client>) -> Pin<Box<dyn Future<Output = Result<ReputationProof<'static>, Error>>>>
{
    //  Why Box::pin? ->  https://doc.rust-lang.org/error_codes/E0733.html
    Box::pin(async move {
        println!("\nId -> {:?}", proof_id);

        let response: Option<ReputationProofDB> = {
            db.select((RESOURCE, proof_id.to_string())).await.expect(DB_ERROR_MSG)
        };

        println!("Response -> {:?}", response);

        match response  {
            Some(r) => {
                let mut proof = ReputationProof::create(Vec::new(),
                                                    r.amount, None);

                // TODO Should be ->  let mut query: String = "SELECT ->leaf.out FROM reputation_proof:".to_owned();
                let mut query: String = "SELECT out FROM leaf WHERE in = reputation_proof:".to_owned();
                query.push_str(&*proof_id.to_string()); // TODO use format! like spend.rs
                let mut dependencies_response = db.query(query)
                    .await.expect(DB_ERROR_MSG);

                let dependencies: Vec<Thing> = dependencies_response.take("out").expect(DB_ERROR_MSG);
                for dependency in dependencies {
                    let dependency_id = dependency.id.to_raw();
                    println!("dependency -> {:?}\n", dependency_id);

                    match recursive(dependency_id, db.clone()).await {
                        Ok(r) => {
                            println!("Reputation proof -> {:?}", r);

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
    let db = Surreal::new::<Ws>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);

    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    match recursive(proof_id, db).await {
        Ok(r) => Ok(r),
        Err(err) => Err(err)
    }
}