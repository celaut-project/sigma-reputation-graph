use std::fmt::Debug;
use std::io::Error;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::sql::{Thing, Operation};
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
        db.select((RESOURCE, proof_id)).await.expect(DB_ERROR_MSG)
    };

    println!("Response -> {:?}", response);

    match response  {
        Some(r) => {
            let proof = ReputationProof::create(Vec::new(),
                                                r.amount, None);
            /* for dependency_id in db.select(leaf).from(proof_id):
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