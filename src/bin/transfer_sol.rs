use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::system_transaction;

use learn::Env;

fn main() {
    let env = Env::new();
    let client = RpcClient::new(env.rpc_url);
    let from_keypair = Keypair::from_base58_string(&env.private_key_0);
    let recipient = Pubkey::from_str(&env.address_1).unwrap();
    let lamports = (0.1f64 * LAMPORTS_PER_SOL as f64) as u64;
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    dbg!(recent_blockhash);
    let tx = system_transaction::transfer(&from_keypair, &recipient, lamports, recent_blockhash);
    let res = client.send_transaction(&tx).unwrap();
    dbg!(res);
}
