use db::{sql_lite::SqliteDb, AtlanticStatus, SayaProvingDb};

use tokio;
pub mod db;
pub mod piltover;
pub mod starknet;
pub mod retry;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = SqliteDb::new("blocks.db").await?;
    db.create_database().await?;
    db.insert_block(200991, "ASD12134XZCAQEQW", AtlanticStatus::InProgress).await?;
    let s = db.check_status(200990).await?;
    println!("{:?}", s);
    Ok(())
}
