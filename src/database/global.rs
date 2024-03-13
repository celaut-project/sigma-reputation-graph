use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Surreal};
use std::future::Future;
use std::pin::Pin;
use surrealdb::engine::local::Db;

pub(crate) const DB_ERROR_MSG: &str = "Invalid response or error connection from the database";
pub(crate) const NAMESPACE: &str = "local";
pub(crate) const DATABASE: &str = "graph";
pub(crate) const ENDPOINT: &str = "reputation.db";
pub(crate) const RESOURCE: &str = "reputation_proof";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RPBoxDB {
    pub(crate) proof_id: String,
    pub(crate) pointer: String,
    pub(crate) amount: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RPBoxDBWithId {
    pub(crate) id: String,
    pub(crate) proof_id: String,
    pub(crate) pointer: String,
    pub(crate) amount: i64
}

#[derive(Debug, Deserialize)]
pub(crate) struct Record {
    #[allow(dead_code)]
    pub(crate) id: Thing,
}

pub(crate) fn check_if_proof_id_exists(id: String, db: Surreal<Db>) -> Pin<Box<dyn Future<Output = bool>>>
{
    Box::pin(async move {
        let response: Option<Record> = db.select((RESOURCE, id.as_str())).await.expect(DB_ERROR_MSG);
        match response {
            Some(_) => true,
            None => false,
        }
    })
}
