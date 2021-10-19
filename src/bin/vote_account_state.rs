use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_vote_program::vote_state::VoteState;
use std::{env, str::FromStr};

fn main() {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap();
    let vote_account_pubkey = env::var("VOTE_ACCOUNT").unwrap();
    let client = RpcClient::new(rpc_url);
    let vote_account = client.get_account(&Pubkey::from_str(vote_account_pubkey.as_str()).unwrap()).unwrap();
    let vote_state = VoteState::deserialize(&vote_account.data);
    dbg!(&vote_state);
}
