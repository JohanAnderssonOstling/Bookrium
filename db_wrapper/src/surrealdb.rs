use async_once::AsyncOnce;
use lazy_static::lazy_static;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};
use surrealdb::opt::auth::Root;
use tokio::runtime::Runtime;
pub mod client_db;
pub mod library_db;

lazy_static!(
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref DB: Surreal<Any> = RUNTIME.block_on(connect_db("ws://127.0.0.1:8000"));
);

pub async fn connect_db(url: &str) -> Surreal<Any> {
    let db = connect(url).await.unwrap();
    db.signin(Root { username: "root", password: "root", }).await.unwrap();
    db
}