use serde::Serialize;
use starknet::{
    accounts::ConnectedAccount,
    core::{
        types::{BlockId, BlockTag, FunctionCall},
        utils::get_selector_from_name,
    },
    providers::Provider,
};
use starknet_types_core::felt::Felt;

use crate::starknet::account::SayaStarknetAccount;

#[derive(Debug, Serialize)]
pub struct PiltoverCalldata {
    pub program_snos_output: Vec<Felt>,
    pub program_output: Vec<Felt>,
    pub onchain_data_hash: Felt,
    pub onchain_data_size: (Felt, Felt), // U256
}

pub struct Piltover {
    contract: Felt,
    account: SayaStarknetAccount,
}
pub struct PiltoverState {
    state_root: Felt,
    block_number: Felt,
    block_hash: Felt,
}

impl Piltover {
    pub async fn update_state(&self, _calldata: PiltoverCalldata) -> () {}

    pub async fn get_state(&self) -> PiltoverState {
        let function_call = FunctionCall {
            contract_address: self.contract,
            entry_point_selector: get_selector_from_name("get_state").unwrap(),
            calldata: vec![],
        };

        let transaction = self
            .account
            .provider()
            .call(function_call, &BlockId::Tag(BlockTag::Latest))
            .await
            .unwrap();
        let state = transaction[0];
        let block_number = transaction[1];
        let block_hash = transaction[2];
        let piltover_state = PiltoverState {
            state_root: state,
            block_number,
            block_hash,
        };
        return piltover_state;
    }
}

#[cfg(test)]
mod tests {
    pub const SEPOLIA: Felt = Felt::from_raw([
        507980251676163170,
        18446744073709551615,
        18446744073708869172,
        1555806712078248243,
    ]);

    use super::*;
    use crate::starknet::account::StarknetAccountData;
    use url::Url;

    #[tokio::test]
    async fn test_piltover() {
        let starknet_url = Url::parse("https://api.cartridge.gg/x/starknet/sepolia").unwrap();
        let signer_address =
            Felt::from_hex("<acount_addres>")
                .unwrap();
        let signer_key =
            Felt::from_hex("<private key>")
                .unwrap();
            
        let starknet_account = StarknetAccountData {
            starknet_url,
            chain_id: SEPOLIA,
            signer_address,
            signer_key,
        };
        let account = starknet_account.get_starknet_account();
        let piltover = Piltover {
            contract: Felt::from_hex(
                "0x443a70746f8f0d0e3b34343e80c079d31b977729773593485da69d88e1bdbd0",
            )
            .unwrap(),
            account,
        };
        let state = piltover.get_state().await;
        println!("{:?}", state.block_hash);
        println!("{:?}", state.block_number);
        println!("{:?}", state.state_root);
    }
}