use std::time::Duration;
use std::{env, sync::Arc};

use anyhow::{anyhow, Result};
use dotenv::dotenv;
use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::{commitment_config::CommitmentConfig, hash::Hash};
use tokio::time::sleep;

pub fn create_solana_rpc_client() -> Result<Arc<RpcClient>, ClientError> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get RPC endpoint from environment variables
    let rpc = env::var("RPC_HTTPS_URL").expect("RPC endpoint must be set in environment variables");

    // Create and return new RPC client with confirmed commitment level
    Ok(Arc::new(RpcClient::new_with_commitment(
        rpc,
        CommitmentConfig::confirmed(),
    )))
}

// Get latest blockhash with retry
pub async fn get_latest_blockhash_with_retry(client: &Arc<RpcClient>) -> Result<Hash> {
    let max_attempts = 10;
    let mut attempts = 0;
    while attempts < max_attempts {
        match client.get_latest_blockhash().await {
            Ok(resp) => {
                return Ok(resp);
            }
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(anyhow!("Max attempts reached. Error: {:?}", e));
                }
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
    Err(anyhow!(
        "Failed to get token account balance after {} attempts",
        max_attempts
    ))
}
