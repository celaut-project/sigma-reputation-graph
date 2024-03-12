use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub(crate) const DB_ERROR_MSG: &str = "Invalid response or error connection from the database";
pub(crate) const NAMESPACE: &str = "local";
pub(crate) const DATABASE: &str = "graph";
pub(crate) const ENDPOINT: &str = "reputation.db";
pub(crate) const RESOURCE: &str = "reputation_proof";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RPBoxDB {
    pub(crate) proof_id: Option<String>,
    pub(crate) pointer: Option<String>,
    pub(crate) amount: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RPBoxDBWithId {
    pub(crate) id: String,
    pub(crate) proof_id: Option<String>,
    pub(crate) pointer: Option<String>,
    pub(crate) amount: i64
}

#[derive(Debug, Deserialize)]
pub(crate) struct Record {
    #[allow(dead_code)]
    pub(crate) id: Thing,
}