use std::str::FromStr;

use learn::Env;
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::pubkey::Pubkey;

fn main() {
    let env = Env::new();
    let client = RpcClient::new(env.rpc_url);

    // json rpc: getTokenAccountsByOwner with jsonParsed; it's possible to take the balance from this query already, but there is no a simple parser func
    let res = client.get_token_accounts_by_owner(&env.address_0, TokenAccountsFilter::Mint(env.token)).unwrap();
    if res.is_empty() {
        println!("there is no token accont");
        return
    }
    let token_account = &res[0].pubkey;
    let token_account = Pubkey::from_str(token_account).unwrap();

    let res = client.get_token_account_balance(&token_account).unwrap();
    dbg!(res.ui_amount_string);

    
}
