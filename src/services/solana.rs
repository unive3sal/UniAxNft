use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::config::SolanaConfig;

pub struct SolanaClient {
    rpc_client: RpcClient,
    service_wallet: Keypair,
    program_id: Pubkey,
}

impl SolanaClient {
    pub fn new(config: SolanaConfig) -> Self {
        Self {
            rpc_client: RpcClient::new(config.rpc_url.clone()),
            service_wallet: config.service_wallet,
            program_id: config.program_id,
        }
    }
}
