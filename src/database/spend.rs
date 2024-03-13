use std::io::Error;
use surrealdb::sql::Thing;
use crate::database::generate::DatabaseAsync;
use crate::database::global::{*};

#[tokio::main]
pub async fn store_on_db(proof_id: Option<String>, amount: i64, pointer: Option<String>, database: DatabaseAsync)
    -> Result<ProofIdType, Error>
{
    match database.await {
        Ok(db) => {        
            let result: Vec<RPBoxDBWithId> = db
                .query(
                    format!(
                        "SELECT * FROM {:?} WHERE pointer='{}' AND proof_id='{}'",
                        RESOURCE, 
                        pointer.clone().unwrap_or(String::from("")),
                        proof_id.clone().unwrap_or(String::from(""))
                    )
                )
                .await.expect(DB_ERROR_MSG)
                .take(1).expect(DB_ERROR_MSG);

            let raw_id: String = match &result[..] {

                [_s] => {  // Match if the query result has exactly one element.

                    let _updated: Option<Thing> = db
                        .update((RESOURCE, _s.id.as_str()))
                        .content(RPBoxDB {
                            proof_id: _s.proof_id.clone(),
                            pointer: pointer,
                            amount: amount + _s.amount
                        })
                        .await.expect(DB_ERROR_MSG);

                    _s.id.clone()
                },
                _ => {
                    let created: Vec<Record> = db
                        .create(RESOURCE)
                        .content(RPBoxDB {
                            proof_id,
                            pointer,
                            amount  // TODO could check that amount <= proof->amount
                        })
                        .await.expect(DB_ERROR_MSG);

                    created.first().unwrap().id.to_string()              
                }
            };

            let proof_id: ProofIdType = raw_id.split_at((RESOURCE.to_owned()+":").len()).1.to_string();
            Ok(proof_id)
        },
        Err(err) => Err(err)
    }
}
