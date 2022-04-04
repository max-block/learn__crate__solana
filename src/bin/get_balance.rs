use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use learn::Env;

fn main() {
    let env = Env::new();
    let client = RpcClient::new(env.rpc_url);
    let address = Pubkey::from_str(&env.address_0).unwrap();
    let res = client.get_balance(&address).unwrap();
    dbg!(res); // in lamports
}
