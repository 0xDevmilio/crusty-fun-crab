use std::{env, sync::Arc};

use anyhow::Result;
use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

use crate::{
    client::get_latest_blockhash_with_retry, constants::ASSOCIATED_TOKEN_PROGRAM_ID,
    instructions::get_buy_ix,
};

pub async fn pumpfun_buy(
    client: &Arc<RpcClient>,
    keypair: &Arc<Keypair>,
    token_pubkey: Pubkey,
    bc_pubkey: Pubkey,
    investment: f64,
) -> Result<()> {
    dotenv().ok();

    let unit_limit = env::var("UNIT_LIMIT")
        .map(|v| v.parse::<u32>().expect("unit_limit must be a valid u32"))
        .unwrap_or(80_000);

    let unit_price = env::var("UNIT_PRICE")
        .map(|v| v.parse::<u64>().expect("unit_price must be a valid u64"))
        .unwrap_or(100_000);
    // Create compute budget instructions
    let unit_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(unit_limit);
    let unit_price_ix = ComputeBudgetInstruction::set_compute_unit_price(unit_price);

    // Get a recent blockhash
    let recent_blockhash = get_latest_blockhash_with_retry(&client).await?;

    let bc_pk_ata =
        Pubkey::find_program_address(&[bc_pubkey.as_ref()], &ASSOCIATED_TOKEN_PROGRAM_ID).0;

    let token_ata = spl_associated_token_account::get_associated_token_address(
        &keypair.pubkey(),
        &token_pubkey,
    );

    // Only necessary on first buy
    let ix_ata = spl_associated_token_account::instruction::create_associated_token_account(
        &keypair.pubkey(),
        &keypair.pubkey(),
        &token_pubkey,
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    let buy_ix = get_buy_ix(
        0,
        investment as u64,
        token_pubkey,
        bc_pubkey,
        bc_pk_ata,
        token_ata,
        keypair,
    )?;

    // tx info--------------------
    let ixs: Vec<Instruction> = vec![unit_limit_ix.clone(), unit_price_ix.clone(), ix_ata, buy_ix];

    let message = Message::try_compile(&keypair.pubkey(), &ixs, &[], recent_blockhash).unwrap();

    // Create a VersionedTransaction
    let transaction = VersionedTransaction {
        message: VersionedMessage::V0(message.clone()), // Clone the message
        signatures: vec![keypair.sign_message(&message.serialize())],
    };
    let tx = client.send_and_confirm_transaction(&transaction).await;

    println!("tx: {:?}", tx);

    Ok(())
}
