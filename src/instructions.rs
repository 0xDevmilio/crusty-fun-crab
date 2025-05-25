use anchor_lang::prelude::*;
use anyhow::Result;
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, system_program, sysvar::rent,
};

use crate::constants::{
    ASSOCIATED_TOKEN_PROGRAM_ID, DEFAULT_BUY, DEFAULT_SELL, PUMPFUN_EVENT_AUTHORITY,
    PUMPFUN_FEE_RECIPENT, PUMPFUN_GLOBAL, PUMPFUN_PROGRAM, TOKEN_PROGRAM_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct BuyLayoutpf {
    pub amount: u64,
    pub max_sol_cost: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SellLayoutpf {
    pub amount: u64,
    pub min_sol_output: u64,
}
pub fn get_buy_ix(
    final_with_slippage_int: u64,
    adjusted_investment_for_fees: u64,
    token_pubkey: Pubkey,
    bc_pubkey: Pubkey,
    bc_pk_ata: Pubkey,
    mint_ata: Pubkey,
    payer: &Keypair,
) -> Result<Instruction> {
    // data of ix
    let buy_layout = BuyLayoutpf {
        amount: final_with_slippage_int as u64,
        max_sol_cost: adjusted_investment_for_fees as u64,
    };

    let mut concatenated_data = Vec::from(DEFAULT_BUY);

    let serialized_data = buy_layout.try_to_vec().unwrap();
    concatenated_data.extend_from_slice(&serialized_data);

    // println!("{:?}", concatenated_data);
    let instruction_accounts = vec![
        AccountMeta::new_readonly(PUMPFUN_GLOBAL, false),
        AccountMeta::new(PUMPFUN_FEE_RECIPENT, false),
        AccountMeta::new_readonly(token_pubkey, false), // Token Address
        AccountMeta::new(bc_pubkey, false),             // Bonding Curve Address
        AccountMeta::new(bc_pk_ata, false),             // Derived
        AccountMeta::new(mint_ata, false),              // Derived
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(PUMPFUN_EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMPFUN_PROGRAM, false),
    ];
    let ix_final =
        Instruction::new_with_bytes(PUMPFUN_PROGRAM, &concatenated_data, instruction_accounts);

    Ok(ix_final)
}

pub fn create_sell_ix(
    tokens_2_sell: u64,
    expected_sol: u64,
    mint: Pubkey,
    bc_pk: Pubkey,
    bc_pk_ata: Pubkey,
    mint_ata: Pubkey,
    payer: &Keypair,
) -> Result<Instruction> {
    // data of ix
    let sell_layout: SellLayoutpf = SellLayoutpf {
        amount: tokens_2_sell as u64,
        min_sol_output: expected_sol as u64,
    };

    let mut concatenated_data = Vec::from(DEFAULT_SELL);

    let serialized_data = sell_layout.try_to_vec().unwrap();
    concatenated_data.extend_from_slice(&serialized_data);

    // println!("{:?}", concatenated_data);
    let instruction_accounts = vec![
        AccountMeta::new_readonly(PUMPFUN_GLOBAL, false),
        AccountMeta::new(PUMPFUN_FEE_RECIPENT, false),
        AccountMeta::new_readonly(mint, false),
        AccountMeta::new(bc_pk, false),
        AccountMeta::new(bc_pk_ata, false),
        AccountMeta::new(mint_ata, false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
        AccountMeta::new_readonly(PUMPFUN_EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMPFUN_PROGRAM, false),
    ];
    let ix_final =
        Instruction::new_with_bytes(PUMPFUN_PROGRAM, &concatenated_data, instruction_accounts);

    Ok(ix_final)
}
