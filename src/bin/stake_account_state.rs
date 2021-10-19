use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_stake_program::stake_state::StakeState;
use std::{env, str::FromStr};


fn main() {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap();
    let stake_account_pubkey = env::var("STAKE_ACCOUNT").unwrap();
    let client = RpcClient::new(rpc_url);
    let stake_account = client.get_account(&Pubkey::from_str(stake_account_pubkey.as_str()).unwrap()).unwrap();
    let stake_state: Result<StakeState, _> = stake_account.deserialize_data();
    dbg!(&stake_state.unwrap());
}
