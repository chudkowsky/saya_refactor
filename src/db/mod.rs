pub mod sql_lite;

pub enum AtlanticStatus{
    InProgress,
    Failed,
}
impl AtlanticStatus{
    pub fn as_str(&self) -> &str {
        match self {
            AtlanticStatus::InProgress => "IN_PROGRESS",
            AtlanticStatus::Failed => "FAILED",
        }
    }
}
#[allow(async_fn_in_trait)]
pub trait SayaProvingDb {
    async fn insert_block(
        &self,
        block_id: i32,
        query_id: &str,
        status: AtlanticStatus,
    ) -> Result<(), sqlx::Error>;
    async fn check_status(&self, block: u32) -> Result<(u32, String, String), sqlx::Error>;
    async fn list_pending_blocks(&self) -> Result<Vec<(u32, String, String)>, sqlx::Error>;
}

