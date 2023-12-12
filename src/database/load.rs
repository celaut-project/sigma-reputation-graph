use std::fmt::Debug;
use std::io::Error;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::sql::{Thing, Operation};


const DB_ERROR_MSG: &str = "Invalid response or error connection from the database";
const NAMESPACE: &str = "local";
const DATABASE: &str = "graph";
const ENDPOINT: &str = "127.0.0.1:8000";
const RESOURCE: &str = "reputation_proof";


#[derive(Debug, Serialize, Deserialize)]
struct ReputationProof {
    previous_proof_id: Option<String>,
    amount: i64
}

#[tokio::main]
pub async fn compute_from_db(proof_id: String) -> Result<f64, std::io::Error>
{
    let db = Surreal::new::<Ws>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);
        
    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    println!("Id -> {:?}", proof_id);
    
    let response: Option<ReputationProof> = db.select((RESOURCE, proof_id)).await.expect(DB_ERROR_MSG);

    println!("Response -> {:?}", response);
    Ok(1.00)
}