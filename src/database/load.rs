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
pub async fn load_from_db(proof_id: String) -> Result<ReputationProof<'static>, std::io::Error>
{
    let db = Surreal::new::<Ws>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);
        
    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    println!("Id -> {:?}", proof_id);

    let response: Option<ReputationProofDB> = db.select((RESOURCE, proof_id)).await.expect(DB_ERROR_MSG);

    println!("Response -> {:?}", response);

    match response  {
        Some(repdb) => {
            let proof = ReputationProof::create(Vec::new(), repdb.amount, None);
            Ok(proof)
        },
        None => todo!(),
    }
}