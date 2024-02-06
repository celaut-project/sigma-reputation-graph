use crate::database::global::{*};
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use surrealdb::engine::local::{Db, File};
use surrealdb::Surreal;

pub type DatabaseAsync = Pin<Box<dyn Future<Output = Result<Surreal<Db>, Error>>>>;
pub fn generate(file: Option<String>) -> DatabaseAsync {
    Box::pin(async move {
        let endpoint = file.as_ref().map_or(ENDPOINT, String::as_str);
        let db = Surreal::new::<File>(endpoint).await.expect("");
        db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);
        return Ok(db);
    })
}