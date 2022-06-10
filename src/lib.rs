use std::{env, str::FromStr};

use dotenv::dotenv;
use solana_sdk::pubkey::Pubkey;

pub struct Env {
    pub rpc_url: String,
    pub address_0: Pubkey,
    pub private_key_0: String,
    pub address_1: Pubkey,
    pub private_key_1: String,
    pub token: Pubkey,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            rpc_url: env::var("RPC_URL").unwrap(),
            address_0: Pubkey::from_str(&env::var("ADDRESS_0").unwrap()).unwrap(),
            private_key_0: env::var("PRIVATE_KEY_0").unwrap(),
            address_1: Pubkey::from_str(&env::var("ADDRESS_1").unwrap()).unwrap(),
            private_key_1: env::var("PRIVATE_KEY_1").unwrap(),
            token: Pubkey::from_str(&env::var("TOKEN").unwrap()).unwrap(),
        }
    }
}
