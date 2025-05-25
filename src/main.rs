use std::{env, str::FromStr, sync::Arc};

use anyhow::Result;
use crusty_fun_crab::{actions::buy::pumpfun_buy, client::create_solana_rpc_client};
use dotenv::dotenv;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Environment variables
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let investment = env::var("INVESTMENT")
        .map(|v| v.parse::<f64>().expect("investment must be a valid f64"))
        .unwrap_or(0.001);

    let token = env::var("TOKEN").expect("TOKEN must be set");

    /* let slippage = env::var("slippage")
    .map(|v| v.parse::<f64>().expect("slippage must be a valid f64"))
    .unwrap_or(0.0);*/

    // Create RPC client
    let client = create_solana_rpc_client()?;

    // Create keypair
    let keypair = Arc::new(Keypair::from_base58_string(&private_key));

    // Token address
    let token_pubkey = Pubkey::from_str(&token).unwrap();

    // Convert investment to lamport and subtract 3% for fees
    let investment_lamported = investment * LAMPORTS_PER_SOL as f64;
    let adjusted_investment_for_fees = investment_lamported + (investment_lamported * 0.03);

    // Need to index all bc pubkeys
    let bc_pubkey = Pubkey::from_str("5RsPXgxFFsYBRjuv42RTo9xZrxc5VajMuH1bfamTE1kY").unwrap();

    pumpfun_buy(
        &client,
        &keypair,
        token_pubkey,
        bc_pubkey,
        adjusted_investment_for_fees,
    )
    .await?;

    Ok(())
}
