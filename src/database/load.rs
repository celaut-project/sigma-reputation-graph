use std::fmt::Debug;
use std::io::Error;
use std::thread;
use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use crate::proof::ReputationProof;


const DB_ERROR_MSG: &str = "Invalid response or error connection from the database";
const NAMESPACE: &str = "local";
const DATABASE: &str = "graph";
const ENDPOINT: &str = "127.0.0.1:8000";
const RESOURCE: &str = "reputation_proof";

#[derive(Debug, Serialize, Deserialize)]
struct ReputationProofDB {
    proof_id: Option<String>,
    amount: i64
}

#[tokio::main]
pub async fn load_from_db(proof_id: String) -> Result<ReputationProof<'static>, Error>
{
    let db = Surreal::new::<Ws>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);

    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    println!("Id -> {:?}", proof_id);

    let response: Option<ReputationProofDB> = {
        db.select((RESOURCE, proof_id.to_string())).await.expect(DB_ERROR_MSG)
    };

    println!("Response -> {:?}", response);

    match response  {
        Some(r) => {
            let proof = ReputationProof::create(Vec::new(),
                                                r.amount, None);

            // TODO Should be ->  let mut query: String = "SELECT ->leaf.out FROM reputation_proof:".to_owned();
            let mut query: String = "SELECT out FROM leaf WHERE in = reputation_proof:".to_owned();
            query.push_str(&*proof_id.to_string());
            let mut dependencies_response = db.query(query)
                .await.expect(DB_ERROR_MSG);

            let dependencies: Vec<Thing> = dependencies_response.take("out").expect(DB_ERROR_MSG);
            for dependency in dependencies {
                println!("\n\n Go to load from db -> {:?}", dependency);

                load_from_db(dependency.id.to_raw());

                /*thread::spawn(move || {
                    // TODO PERFORMANCE -> https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
                    match load_from_db(dependency.id.to_raw()) {
                        Ok(r) => {
                            println!("{:?}", r);
                        },
                        Err(_) => println!("Errr")
                    }
                }).join().expect("Thread panicked");*/
            }

            /* for dependency_id in dependencies:
                 if let dependency = load_from_db(dependency_id)
                 {
                    if (&proof).can_be_spend(dependency.unwrap().amount) {
                        proof.outputs.push(d);
                    }
                 }
                 else {
                    todo!();
                 }
             */
            Ok(proof)
        },
        None => todo!(),
    }
}