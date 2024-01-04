use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use surrealdb::engine::local::{Db, File};
use surrealdb::Surreal;
use crate::database::global::{*};

fn get_proof_db_id(id: String, db: Surreal<Db>) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>
{
    Box::pin(async move {
        let response: Option<Record> = db.select((RESOURCE, id.as_str())).await.expect(DB_ERROR_MSG);
        match response {
            Some(_) => Ok(format!("{}:{}", RESOURCE, id)),
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve from database".to_string(),
            )),
        }
    })
}


#[tokio::main]
pub async fn store_on_db(previous_proof_id: Option<String>, amount: i64, pointer: Option<String>)
    -> Result<String, Error>
{
    let db = Surreal::new::<File>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);
        
    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    let id_result: Result<Option<String>, Error> = match previous_proof_id {
        None => Ok(None),
        Some(id) => {
            match get_proof_db_id(id, db.clone()).await {
                Ok(id) => Ok(Some(id)),
                Err(error) => Err(error)
            }
        }
    };

    match id_result {
        Ok(parent_id) => {
            // Create a new person with a random id
            let created: Vec<Record> = db
                .create(RESOURCE)
                .content(ReputationProofDB {
                    pointer,
                    amount  // TODO could check that amount <= proof->amount
                })
                .await.expect(DB_ERROR_MSG);

            let raw_id = created.first().unwrap().id.to_string();

            match parent_id {
                None => {}
                Some(parent_id) => {
                    db.query(
                        format!("RELATE {}->leaf->{}", parent_id, raw_id.to_string())
                    ).await.expect(DB_ERROR_MSG);
                }
            }

            let proof_id = raw_id.split_at((RESOURCE.to_owned()+":").len()).1.to_string();
            Ok(proof_id)
        }
        Err(error) => Err(error)
    }

}