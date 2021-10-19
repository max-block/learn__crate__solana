use dotenv::dotenv;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter,
};

use std::{env, time::Duration};

fn main() {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap();
    let vote_account_pubkey = env::var("VOTE_ACCOUNT").unwrap();
    let client = RpcClient::new_with_timeout(rpc_url, Duration::from_secs(600));

    let program_accounts_config = RpcProgramAccountsConfig {
        filters: Some(vec![rpc_filter::RpcFilterType::Memcmp(rpc_filter::Memcmp {
            offset: 124,
            bytes: rpc_filter::MemcmpEncodedBytes::Binary(vote_account_pubkey),
            encoding: Some(rpc_filter::MemcmpEncoding::Binary),
        })]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
            ..RpcAccountInfoConfig::default()
        },
        ..RpcProgramAccountsConfig::default()
    };
    let res = client.get_program_accounts_with_config(&solana_stake_program::id(), program_accounts_config);
    dbg!(res);
}
