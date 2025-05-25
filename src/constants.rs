use solana_sdk::{pubkey, pubkey::Pubkey};

pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

pub const DEFAULT_BUY: [u8; 8] = [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea];
pub const DEFAULT_SELL: [u8; 8] = [0x33, 0xe6, 0x85, 0xa4, 0x01, 0x7f, 0x83, 0xad];

pub const PUMPFUN_PROGRAM: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
pub const PUMPFUN_GLOBAL: Pubkey = pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
pub const PUMPFUN_FEE_RECIPENT: Pubkey = pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");
pub const PUMPFUN_EVENT_AUTHORITY: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
