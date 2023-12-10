use std::fmt::Debug;
use std::io::Error;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
struct ReputationProof {
    proof_id: Option<String>,
    amount: i64
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

fn get_proof_db_id(id: &str) -> Result<String, Error> {
    // TODO check if the id exists.
    Ok("reputation_proof".to_owned() + id)
}

const DB_ERROR_MSG: &str = "Error with SurrealDB";

#[tokio::main]
pub async fn store_on_db(proof_id: Option<String>, amount: i64) 
    -> Result<String, std::io::Error> 
{
    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await.expect(DB_ERROR_MSG);
        
    db.use_ns("local").use_db("graph").await.expect(DB_ERROR_MSG);

    let id_result: Result<Option<String>, Error> = match proof_id {
        None => Ok(None),
        Some(id) => match get_proof_db_id(id.as_str()) {
            Ok(id) => Ok(Some(id)),
            Err(error) => Err(error)
        }
    };

    match id_result {
        Ok(proof_id) => {
            // Create a new person with a random id
            let created: Vec<Record> = db
                .create("reputation_proof")
                .content(ReputationProof {
                    proof_id,
                    amount
                })
                .await
                .expect(DB_ERROR_MSG);

            let raw_id = created.first().unwrap().id.to_string();
            Ok(raw_id)
        },
        Err(error) => Err(error)
    }

}