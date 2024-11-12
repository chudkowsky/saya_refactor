pub mod db;
pub mod piltover;
pub mod retry;
pub mod starknet;

use std::thread::sleep;

use db::{sql_lite::SqliteDb, SayaProvingDb};
use piltover::Piltover;
use sqlx::Sqlite;
use starknet::account::StarknetAccountData;
use starknet_types_core::felt::Felt;
use url::Url;

pub struct Saya {
    pub config: SayaConfig,
    pub last_settled_block: u32,
    pub last_sent_for_prove_block: u32,
    pub db: SqliteDb,
    pub piltover: Piltover,
}
#[derive(Debug)]
pub struct SayaConfig {
    pub rpc_url: Url,
    pub prover_url: Url,
    pub prover_key: String,
    pub settlement_contract: Felt,
    pub starknet_account: StarknetAccountData,
}

impl Saya {
    pub async fn new(config: SayaConfig) -> Saya {
        let piltover = Piltover{
            contract: config.settlement_contract,
            account: config.starknet_account.get_starknet_account(),
        };
        let last_settled_block = piltover.get_state().await.block_number;

        let db = SqliteDb::new("blocks.db").await.unwrap(); 
        let pending_blocks = db.list_pending_blocks().await.unwrap(); 
        let last_sent_for_prove_block = pending_blocks.iter().map(|(block, _, _)| block).max().unwrap_or(&last_settled_block);
        
        println!("{:?}", pending_blocks);
        
        Saya {
            config,
            last_settled_block,
            last_sent_for_prove_block: *last_sent_for_prove_block,
            db,
            piltover
        }
    }
    pub async fn start(&mut self) {
        let poll_interval_secs = 1;
        let mut block_number = self.last_sent_for_prove_block;
        loop{
            sleep(std::time::Duration::from_secs(poll_interval_secs));
            
        }
        
    }
}
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use starknet::core::chain_id::SEPOLIA;
    use starknet_types_core::felt::Felt;
    use url::Url;

    use crate::{db::{sql_lite::SqliteDb, AtlanticStatus, SayaProvingDb}, starknet::account::StarknetAccountData, Saya, SayaConfig};

    #[tokio::test]
    async fn test_all() -> Result<(), sqlx::Error> {
        let db = SqliteDb::new("blocks.db").await?;
        db.insert_block(280124, "ASD12134XZCAQEQW", AtlanticStatus::InProgress)
            .await?;
        db.insert_block(280125, "ASD12134XZCAQEQW", AtlanticStatus::InProgress)
        .await?;
        db.insert_block(280126, "ASD12134XZCAQEQW", AtlanticStatus::InProgress)
        .await?;
        // println!("{:?}", s);
        Ok(())
    }

    #[tokio::test]
    async fn test_saya_init() {
        let starknet_url = Url::parse("https://api.cartridge.gg/x/starknet/sepolia").unwrap();
        let signer_address =
            Felt::from_hex("0x069A4f598B14F8424F2Ee90B7A55fbc6083635dA13f96a35acae04e6C149798D")
                .unwrap();
        let signer_key =
            Felt::from_hex("0x036e29b89cc49d3ade4c0535b88d7131f39c58aef0c75e76d76e6ccf075b105f")
                .unwrap();
            
        let starknet_account = StarknetAccountData {
            starknet_url,
            chain_id: SEPOLIA,
            signer_address,
            signer_key,
        };
        let saya_config = SayaConfig{rpc_url:Url::parse("https://api.cartridge.gg/x/starknet/sepolia").unwrap(), 
        prover_url: Url::from_str("https://api.cartridge.gg/x/starknet/sepolia").unwrap(), 
        prover_key: "asdasdas".to_string(),
        settlement_contract: Felt::from_hex("0x443a70746f8f0d0e3b34343e80c079d31b977729773593485da69d88e1bdbd0").unwrap(),
        
        starknet_account: starknet_account };
        let saya = Saya::new(saya_config).await;
        println!("{:?}", saya.last_sent_for_prove_block);
        println!("{:?}", saya.last_settled_block);
    }
}
