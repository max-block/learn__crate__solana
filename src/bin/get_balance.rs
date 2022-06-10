use learn::Env;
use solana_client::rpc_client::RpcClient;

fn main() {
    let env = Env::new();
    let client = RpcClient::new(env.rpc_url);
    let res = client.get_balance(&env.address_0).unwrap();
    dbg!(res); // in lamports
}
