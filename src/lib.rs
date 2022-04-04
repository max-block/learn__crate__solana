use std::env;

use dotenv::dotenv;

pub struct Env {
    pub rpc_url: String,
    pub address_0: String,
    pub private_key_0: String,
    pub address_1: String,
    pub private_key_1: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            rpc_url: env::var("RPC_URL").unwrap(),
            address_0: env::var("ADDRESS_0").unwrap(),
            private_key_0: env::var("PRIVATE_KEY_0").unwrap(),
            address_1: env::var("ADDRESS_1").unwrap(),
            private_key_1: env::var("PRIVATE_KEY_1").unwrap(),
        }
    }
}
